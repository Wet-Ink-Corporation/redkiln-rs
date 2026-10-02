---
id: "0197"
title: "**Cards, models and the run environment: least-privilege cards, pinned model ids, typed schemas read by name, in-band principle-7 guards, isolated settings and one launcher** (`s8h`'s Record A; D21-41 (g); D21-42 15a, 15b, 16, 17a, 17b, 21b, 21d; D21-43 N17 Q1; D21-45's standing limit; D21-47; D21-48 X12): the eight shipped cards carry the least-privilege grant table and a mode from a closed seven-member set; the argv spells a card's `default` from one closed table keyed by the `Config`'s pin; `[models]` pins three full ids, and adoption requires exact equality of the reported and requested id (RT-27 settled); every lane-reached kind is an `EXPORTED` Rust type read by the bound step's `value:<Kind>`; the init's `permissionMode`, `model` and `apiKeySource` and both denial channels are read back from the child; the child runs under `--setting-sources \"\"` and one `--settings` confinement element; `tools/live-run.sh` and its launch record are the only entry to a real run; the CLI is frozen for the real-run window. Supersedes `0126` decision 1 in part (its payload table gains `observed_model`) and `0074` decision 1 in part (its finding shape gains `severity`)"
status: proposed
date: 2026-10-02
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (g), D21-42 15a, 15b, 16, 17a, 17b, 21b, 21c, 21d, D21-43 N17 (Q1, Q4) and its integration call on AC-133, D21-44, D21-45 (the standing limit), D21-46's integration calls, D21-47 (the s8h review findings and cross #4, #5, #7), D21-48 X12, D21-129 (owed forward to `s8h`); spikes/DECISIONS-2026-09-17.md RT-27; runbook/phase-1/s8h-cards-and-run-environment.md §8h.1 … §8h.8 and the stage-open readings of 2026-10-02; references/2026-09-08-acceptance-criteria.md AC-133, AC-156, AC-202, AC-204, AC-214; references/adr/0168 decision 1
supersedes: ["0126", "0074"]
superseded_by: []
related: ["0010", "0014", "0080", "0084", "0092", "0095", "0112", "0130", "0146", "0156", "0162", "0164", "0168", "0180", "0188", "0192"]
evidence: []
consumers: []
---

## Context and problem

**The shipped registry cannot bind a dispatch.** D21-40 decision 1 names it as the last of four structural reasons
criterion (1) cannot be met: *"the shipped registry cannot bind a dispatch (0 of 8 cards carry `permission_mode`, all 8
grant only `StructuredOutput`, no `.redkiln/schemas/`, 0 `model` rows)"*. `s8h` §8h.0's re-measurement at `ca8f387`
(2026-09-24) found each clause true, and found five more gaps beside them:

- **The cards.** All eight `.redkiln/cards/*.json` hold exactly `cache_profile`, `description`, `prompt` and `tools`,
  every `tools` is `["StructuredOutput"]`, and `AuthoredCard::from_file` refuses each with `PermissionModeAbsent`. The
  only check on a mode's value is `card.rs`'s `if permission_mode.trim().is_empty() {`, so a misspelt `"acceptEdit"`
  would bind.
- **The models.** Each pack's eight `[role.*]` tables carry one key, `family = "claude"`. `RoleExtras.model` is the
  closed `card::Tier` (`Haiku | Sonnet | Opus`), `--model` receives the tier word, and adoption is read through
  `init.rs`'s segment match `reported == tier || reported.split('-').any(|seg| seg == tier)`, which is RT-27's
  composite reading (*"`model` belongs to the bound card's tier (a `-`-delimited segment of the reported id, or
  exact)"*). `init.rs` says of it *"A record settling the reading is STILL OWED"*. `DispatchRecord` has no model member.
- **The schemas.** `bind_one` in `crates/rk-cli/src/verbs/run.rs` reads `.join("schemas")` then
  `.join(format!("{kind}.json"))`, a directory that does not exist, while `rk_types::registry::EXPORTED` carries two
  entries, `Findings` and `AcceptanceCriterion` [measured 2026-10-02 at `2770bda`].
- **Principle 7.** Nothing in `rk-adapter-claude/src` reads `permissionMode`, `apiKeySource` or `permission_denials`
  back from the child, and `command.rs` emits neither `--setting-sources` nor `--settings`, so the child discovers the
  operator's user settings.
- **The pin and the environment.** `TESTED_CLI_VERSION` is `2.1.278` and `RUNBOOK-instrument.txt` reads
  `2.1.278 (Claude Code)` (22 bytes), while the installed `claude --version` printed `2.1.281 (Claude Code)` at
  authoring and `2.1.287 (Claude Code)` at this record's writing [measured 2026-10-02; `~/.local/bin/claude.exe` dated
  2026-10-01]. **Halt 2 is live, and the instrument has moved twice under the tree.** No launcher exists, a Claude Code
  session exports `CLAUDECODE`, `CLAUDE_CODE_*`, `CLAUDE_PID`, `CLAUDE_EFFORT` and `AI_AGENT`, and `CLAUDE_CONFIG_DIR`
  is inherited rather than set.

D21-41 (g) took the shape: *"**Least privilege.** Only the implementer writes … Three principle-7 guards are added …
Schemas are Rust types in rk-types `EXPORTED` (ADR-0084), not hand-written JSON. … The record is accepted only after two
live probes pass, and those probes run after the fixture proofs."* D21-42 15a, 15b, 16, 17a, 17b, 21b and 21d took the
rest, and D21-43 N17, D21-45, D21-47 and D21-48 X12 took what the stage authors surfaced. This record is the one record
those decisions name. Two of its decisions move accepted `## Decision` text: ADR-0126 decision 1's payload table gains a
member, and ADR-0074 decision 1's finding shape gains a member. ADR-0092 rule 1 makes each a supersession in part.

## Decision drivers

- **Design principle 7**: never *"inferring from the parent side that a flag took effect — an ignored flag looks
  identical to an honoured one"*. Every guard here reads the child's own report (AC-202's rule), never the argv.
- **ADR-0010**: *"The adapter must refuse to proceed (assert, not warn)"*. An auth mode is checked in-band, on the init.
- **ADR-0084**: *"Every type that crosses the closed-output boundary has its generated `schemars` output snapshotted
  with `insta`"*. A hand-written JSON schema crosses the boundary unsnapshotted.
- **ADR-0112** closes `StepError` at eleven and *"Forbids a twelfth variant without a successor to this record"*.
  D21-43 N17 Q1: *"reuse `NotAdopted` and `NotIsolated { surface: Env }`; nothing superseded."*
- **AC-133**: *"the enum appears in the judgement-enum registry with an explicit judgement-bearing yes/no AND every enum
  marked yes has a sibling free-text `rationale` field on the same object"*, and its limit: *"the lint still cannot
  tell whether the *value* of the yes/no is right; that classification is the human gate AC-156"*. D21-43: *"AC-133's
  judgement-enum registry is built once, by **s8g**. s8h consumes it."*
- **D21-48 X12**: *"The adapter emits the spelling the PINNED CLI's `--help` lists, fixed at s8j's pin move (`manual`
  on 2.1.281). Cards keep `default`, and the init comparison reads `default`. The golden argv snapshots move once, at
  the pin move. Probe 0 confirms rather than decides."*
- **D21-45's standing limit**: *"inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/`
  stay protected. An implementer edit there is denied, and s8h's typed `permission_denials` failure names it as a known
  limit, not a defect."*
- **ADR-0130 decision 1's byte-stability**: the launch record is not the kernel's run record, so the CLI version never
  enters the `run_id` preimage.

## Considered options

### Option — least-privilege cards, pinned full ids, typed schemas by name, in-band guards, isolated settings and one launcher (taken)

Everything below under `## Decision`. It is D21-41 (g) and D21-42 15–17, 21b and 21d as taken, with N17 Q1's reuse of
the closed error set and X12's argv table.

### Option — keep the tier words and RT-27's segment match

Cheapest: no `[models]` table, `--model haiku`, adoption by segment. Rejected. D21-42 15a took *"Full model ids are
pinned … the run refuses when the init-reported model differs from the requested one"*. A segment match lets a
reported `claude-sonnet-4-6` "belong" to a requested `sonnet`, so a model change under the same tier word moves every
recorded measurement without a refusal, and nothing records which model ran.

### Option — hand-written JSON schemas under `.redkiln/schemas/`

What `bind_one` reads today. Rejected by D21-41 (g) (*"not hand-written JSON"*) and by ADR-0084's snapshot clause:
a JSON file has no `schemars` derivation to snapshot, and ADR-0095 decision 1's caps (no `oneOf`, no nested union,
`additionalProperties: false`, required first, ≤ 16 union-typed and ≤ 24 optional parameters) would be checked against
text rather than against the type the kernel decodes into.

### Option — two new `IsolationSurface` members for the mode and the denials

The `s8h` draft's recommendation. It would have superseded ADR-0095 decision 2's payload cell
(`{ surface: tools | mcp_servers | env, diff_digest }`) in part. N17 Q1's *"nothing superseded"* rules it out, so the
mode, the auth source and the denials take `Env`, the surface of the child's run environment.

### Option — the argv carries the card's own spelling whatever the pin

Simplest: `--permission-mode default` always. Rejected by X12: the 2.1.281 and 2.1.287 `--help` choices list `manual`
and not `default`, so a parser that drops `default` would refuse every read-only card at the next pin, and the
decision would be made by whichever release did it first.

## Decision

**1. The grant table (D21-41 (g)).** The eight shipped cards are exactly:

| card | `tools` | `permission_mode` | reached by |
| --- | --- | --- | --- |
| `implementer` | `Read`, `Glob`, `Grep`, `Edit`, `Write`, `StructuredOutput` | `acceptEdits` | both lanes |
| `investigator` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `reviewer` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `architect` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `spec-author` | `Read`, `Glob`, `Grep`, `StructuredOutput` | `default` | both |
| `planner` | `StructuredOutput` | `default` | both |
| `releaser` | `StructuredOutput` | `default` | both |
| `operator` | `StructuredOutput` | `default` | neither (`operate` is `forbidden` in both packs) |

No card is granted `Bash`, `PowerShell`, a `Web*` tool or an `mcp__*` tool; build and test stay on the `Toolchain`
port. The card record stays closed (`deny_unknown_fields`), and `model` and `effort` on a card stay a tier leak
(AC-118, AC-418). The child's `cwd` is the run-owned worktree the spec already carries (RT-21's `WORKTREE_KEY`;
`runner.rs`'s `.current_dir(&prepared.worktree)`), which is `.redkiln/worktrees/<item>` under ADR-0180. The doctor row
`RK-D-CARD-GRANT` refuses any shipped card tool outside the union `{Read, Glob, Grep, Edit, Write, StructuredOutput}`,
naming `Bash`, `PowerShell`, `Web*` and `mcp__*` in its message, and any shipped card mode outside `{default,
acceptEdits}` (D21-42 21c). A union allowlist is chosen over a denylist because a denylist goes stale with every tool a
CLI release adds; the 2.1.287 binary carries `CLAUDE_CODE_USE_POWERSHELL_TOOL` [measured 2026-10-02 by string search].

**2. The closed permission-mode set and the shipped pair.** `lint_card` refuses a mode outside the CLI's own set, read
at load and again at bind. The set is cited to `claude --help`, the CLI's argv parser, and not to an SDK enum: *"(choices:
"acceptEdits", "auto", "bypassPermissions", "manual", "dontAsk", "plan")"* [measured 2026-09-24 on 2.1.281; re-read
2026-10-02 on 2.1.287, the same six, captured to
`crates/rk-adapter-claude/tests/fixtures/cli-help/2.1.287.permission-mode.txt`]. The binary maps `manual` to `default`
[measured by string search: the literal `"manual"?"default"`, present in 2.1.287], so `default` is the name the init
reports for that mode and the name D21-41 (g) uses. **The admitted set is the help's six plus `default`, seven in
all.** Only the pair `default` / `acceptEdits` is admitted in a **shipped** card, and that is the doctor row's job
(decision 1), not the adapter's: a test pack may use another mode.

**3. The argv spelling of a card's `default` is one closed table keyed by the pin (X12).** It is a construction under
D21-41 (g), and X12 is its warrant.
- One table in `command.rs`, declared in exactly this form: `pub(crate) const PERMISSION_MODE_ARGV: &[(&str, &str)] =
  &[`, one row per line as `    ("<CLI version>", "<argv spelling of a card's default>"),`, then `];`. Beside it,
  `pub(crate) const LIVE_WARRANTED: &[&str] = &[…];` names the rows warranted by a live acceptance rather than a help
  capture.
- The lookup is keyed by the `Config`'s pin, `tested_cli_version` (D17-11's `Config` value, which `TESTED_CLI_VERSION`
  fills in `real_harness_config`), never by the constant, so a test can build a `Config` at another pin. `assemble`
  writes the row's spelling on the `--permission-mode` element when the card's mode is `default`, and the card's own
  spelling for every other mode.
- **A pin with no row is refused** by `assemble`, before any spawn, with a typed error naming the version. There is no
  fallback to the card's spelling.
- **Only the argv element moves.** The card file, `AgentCard.permission_mode`, the `--agents` JSON's `permissionMode`,
  the card digest (and so the memo key), `lint_card`'s set, the doctor row's pair and guard 2's init comparison all
  keep `default`.
- **The rows at this record's writing are two:** `("2.1.278", "default")`, the pin, warranted live by the 2026-09-21
  `live_dispatch_probe` (`tests/fixtures/cards/catalog/probe.json`'s `"permission_mode": "default"` on its argv) and
  the single entry of `LIVE_WARRANTED`; and `("2.1.287", "manual")`, warranted by the help capture named in decision 2
  and dormant until the pin moves. `s8h` §8h.2 drafted this row as `2.1.281`, and its own rule keys the help-warranted
  row by the version installed at stage open, which read `2.1.287`. Outside `LIVE_WARRANTED`, rows and help captures
  are one to one.
- At `s8j`'s pin move, and nowhere earlier, the `2.1.278` row is deleted, `LIVE_WARRANTED` becomes `&[]`, and the
  golden argv fixtures and snapshots whose card says `default` move in their `--permission-mode` element only.

**4. `[models]` and the three ids (D21-42 15a, 15b).** Each installed pack carries one table, so each pack stays
independently complete:

```text
[models.haiku]    id = "claude-haiku-4-5-20251001"   accepts_effort = false
[models.sonnet]   id = "claude-sonnet-5"             accepts_effort = true
[models.opus]     id = "claude-opus-5-5"             accepts_effort = true
```

Each row is a TOML table with exactly the two keys `id` (a string) and `accepts_effort` (a boolean). **`Tier` becomes
the open set of `[models]` row keys**: `RoleExtras.model` is the row key (a string), the closed `card::Tier` leaves the
binding path, `Tier::accepts_effort`'s lifted rule (*"haiku rejects the parameter"*) becomes the row's
`accepts_effort` with its warrant beside the haiku row, and `EffortUntraceable` / `EffortRefusedByTier` keep their
meaning keyed on the row. The `Effort` enum stays closed. A role naming a key absent from `[models]` is refused at
bind, naming the role and the key. The role rows are D21-41 (g)'s, kept by D21-42 15b: `implementer` and
`investigator` `sonnet`/`high`; `spec-author` `sonnet`/`medium`; `reviewer` `opus`/`high`; `architect` `opus`/`medium`;
`planner`, `operator` and `releaser` `haiku` with no effort. `family = "claude"` stays, and the breaker's partition key
(D-7) does not move.

**5. The full id travels everywhere the tier word did, and RT-27 is settled by exact equality.** The id goes into
`--model`, into `AgentCard.model`, and so into `to_agents_json()` and `AgentCard::digest()`, which is
`ActivitySpec.card_digest` and enters the per-step memo key D21-42 3a names. **Adoption requires that the init's
`model` equal the requested id**; `model_belongs_to_tier` is deleted, not left beside the new check. This retires
RT-27's segment conjunct; its `agent`/`agents` and tool-set conjuncts stand. **`DispatchRecord` gains a typed
`observed_model`**: `Some` on every path that read a `system/init` carrying `model`, `None` on every path that
terminated before one was read, and never the requested id copied in, which would be Halt 11's shape (the reasoning that
gives `cli_version` its `Observed`/`Pinned` arms in ADR-0126 decision 1). This member is the supersession in part of
ADR-0126 decision 1 (decision 12).

**6. The schema route: Rust types in `EXPORTED`, read by name.**
- The binder resolves a schema as the `rk_types::registry::EXPORTED` entry named by **the bound step's** output kind,
  the `value:<Kind>` of its `outputs`, never `provides[0]`. `s8d`'s per-step binding already reads the step's kind
  [measured 2026-10-02: `bind_nodes` passes each agent step's `step.output_type` to `bind_one`]; the file read in
  `bind_one` is deleted and `.redkiln/schemas/` is not created. A kind absent from `EXPORTED` is a bind-time refusal
  naming the kind and the step (`Exit::ConfigError`), before any spawn.
- **The universe is `s8h` §8h.4's 24 kinds**, enumerated by its stated rule (every non-`forbidden` stage's baseline
  and `require` artifacts, closed over `requires` through the lane-tagged catalog activities, collecting each agent
  step's `value:<Kind>`). It is a candidate universe, not a selection run, so the direction is fail-closed. Every kind
  gets a type registered in `EXPORTED` under its kind name and snapshotted (ADR-0084). A type that already exists is
  registered, never redeclared: at writing, `Findings` is the only one of the 24 that exists [measured 2026-10-02].
- **Field shapes** are transcribed from `spikes/p9/schemas.toml`'s `[value.<Kind>]` rows under ADR-0095 decision 1's
  rule. A field the stand-in lists is required, not an `Option`, unless this record names why not; an `Option` is never
  a default.
- **`Spec` embeds `rk_types::AcceptanceCriterion` unchanged** as `acceptance_criteria:
  Vec<rk_types::AcceptanceCriterion>` (`s8g` §8g.3: *"`s8h`'s `Spec` must embed `AcceptanceCriterion` unchanged, under
  that member name"*; ADR-0192). `AcceptanceCriterion.verification` stays `Option<Text>` (AC-137 accepts a criterion
  with none). **Every authored criterion carries a non-empty `verification`** (D21-43 N24 as `s8g` hands it on): the
  `spec-author` card's output contract says so, and the adapter's structured-output validation refuses, after the
  schema validation, a `Spec` output any criterion of which has an absent or blank `verification`. The class is
  `StepError::Schema`, the one a structured output that fails its contract already takes, so ADR-0112's set does not
  move. A `Spec` read back from the store is not re-checked.
- **`Finding` gains `severity`** (D21-47 cross #4): a **required** member typed by a closed enum whose spellings are
  ADR-0168 decision 1's, *"The severity vocabulary is `blocking | advisory`"*, read from that record by its id. The
  reviewer card's structured output fills it, because `Findings` is that card's output kind. This member is the
  supersession in part of ADR-0074 decision 1 (decision 12).
- **`RootCause` carries `discriminating_test`** as a required string (D21-47 cross #5), the test selection `confirm`
  re-runs; the stand-in's `[value.RootCause]` row gains it by dated comment.

**7. AC-133's judgement classification, per enum, for the operator's signature (AC-156).** Every enum the types of
decision 6 bring into `EXPORTED` is entered in `s8g`'s registry (`rk_types::schema::judgement::REGISTRY`; ADR-0192),
and no second registry is written. `s8g`'s completeness test goes red until each is entered, and that red is the
instrument. **Where an enum is drafted judgement-bearing and its object carries no `rationale` member in the stand-in,
the type gains a required free-text `rationale` member**, because AC-133's sibling rule binds and the stand-in lists
fields, not the rule. The yes/no **values** below are drafted, and each is the operator's to sign (AC-156: *"routed to a
human gate; not machine-decidable"*); the tests check that an entry exists, never that its value is right.

| enum (drafted name; owning field) | drafted judgement-bearing | drafted reason | status |
| --- | --- | --- | --- |
| `EvidenceLabel` (`Finding.evidence`) | yes | registered by `s8g`; the model's own label, `Finding.rationale` its sibling | `s8g`'s entry, cited |
| `ReproKind` (`Repro.kind`) | no | registered by `s8g`; the mechanism, not the judgement | `s8g`'s entry, cited |
| `Expected` (`Repro.expected`) | no | registered by `s8g`; machine-compared | `s8g`'s entry, cited |
| `MutationDeclaration` (`AcceptanceCriterion.mutation`) | no | registered by `s8g`; the ledger's outcome is the judgement | `s8g`'s entry, cited |
| ~~`Severity`~~ `FindingSeverity` (`Finding.severity`; *(dated 2026-10-02, `s8h` build, package P2-rk-types)* renamed: `rk_types::verb_output::Severity` is the CVSS band, and the registry keys an enum by its schema name) | yes | the reviewer's call on whether a finding blocks; `Finding.rationale` is its sibling | ~~drafted~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered in `REGISTRY` as drafted, spelled `blocking \| advisory` (ADR-0168 decision 1, read by `findings::` from the record by its id) |
| `Confidence` (`RootCause.confidence`, `CausalChain.confidence`, `ReproCase.confidence`, `Estimate.confidence`) | yes | the author's confidence is a judgement about evidence | ~~drafted; *placeholder: one enum or several is the build's (`s8h` P2), completed by dated row*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: ONE enum, `low \| medium \| high` (`high` is the spelling `differential-debugging` and `instrumented-trace-analysis` compare). `RootCause`, `CausalChain`, `ReproCase` and `Estimate` each gain a required `rationale` |
| `AdrStatus` (`ADR.status`) | no | a lifecycle state the author records, not a judgement on evidence | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: `proposed \| accepted \| rejected \| deprecated \| superseded` (`references/adr/README.md`'s lifecycle) |
| ~~`ImplStatus`~~ (`ImplReport.status`) | yes | the implementer's own claim that the work is done; the verify pass, not the label, measures it | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* **struck: typed as free `Text`.** No shipped row reads it (the `status == 'passing'` rows read a `TestReport`), and no source names its vocabulary |
| `ReproStatus` (`ReproAttempt.status`, `ReproCase.status`) | yes | whether the defect reproduced is the investigator's reading of an observation | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: `reproduced \| not-reproduced` (`reproduced` is the spelling `minimal-repro` compares). `ReproAttempt` and `ReproCase` gain a required `rationale` |
| `HypothesisStatus` (`HypothesisLog.status`) | yes | whether a hypothesis is confirmed is a judgement | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: `open \| confirmed \| refuted` (`confirmed` is the spelling `root-cause` compares). `HypothesisLog` gains a required `rationale` |
| ~~`ChangeStatus`~~ / `ChangeType` (`ChangeRecord.status`, `ChangeRecord.type`) | no / no | change-management classification recorded, not judged | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* `ChangeType` entered: `standard \| normal \| emergency` (`change-authorization`'s rationale). `ChangeStatus` **struck: free `Text`**, because the shipped emit composes it from `${authorize.answer}` and `change.authorize` has no question row naming answers |
| ~~`PostmortemStatus`~~ (`Postmortem.status`) | no | a document lifecycle state | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* **struck: typed as free `Text`.** No shipped row reads it, and no source names its vocabulary |
| `DocsQuadrant` (`Docs.quadrant`) | yes | the Diátaxis quadrant is the author's classification, and the checklist catches a wrong one | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: `tutorial \| how-to \| reference \| explanation` (the `docs-diataxis` checklist). `Docs` gains a required `rationale` |
| ~~`ApiFormat`~~, `ArchitectureLevel`, `CausalMethod`, `EstimateMethod`, `RolloutStrategy`, ~~`TestTechnique`~~ | no | the method or notation chosen, not a judgement on evidence | ~~drafted; *placeholder*~~ *(dated 2026-10-02, `s8h` build, package P2-rk-types)* entered: `ArchitectureLevel` `context \| container \| component` (the three `c4-*` activities); `CausalMethod` `five-whys \| fishbone \| fault-tree` (its three producers); `EstimateMethod` `appetite \| t-shirt \| relative` (its three producers); `RolloutStrategy` `blue-green \| canary \| flag-progressive \| rolling` (the release stage's four deployment patterns). `ApiFormat` and `TestTechnique` **struck: free `Text`** (no source names a closed vocabulary; the `keywords` rows are selection aids, not vocabularies) |

A row marked *placeholder* names a stand-in field that is a candidate enum. Whether the build types it as an enum or as
free `Text`, and the enum's spelling, are the build's; a candidate the build types as `Text` is struck from this table
by dated line, and an enum the build adds that is not here is added by dated line, both before the operator signs.

*(dated 2026-10-02, `s8h` build, package P2-rk-types)* **The table is complete; no enum was added beyond it.** The registry rows are `rk_types::schema::judgement::REGISTRY`'s eleven new entries (five drafted `yes`: `FindingSeverity`, `Confidence`, `ReproStatus`, `HypothesisStatus`, `DocsQuadrant`; six `no`), and `s8g`'s three `judgement_registry::` tests pass over them. The typing rule the build used is stated in `crates/rk-types/src/artifact/lane.rs`'s module comment: a scalar is a closed enum only when a shipped `expr`/`when`/`until` compares it to a literal or a catalog row, checklist or record names its complete vocabulary. Every `yes` value above is still drafted and the operator's to sign (AC-156).

**8. The three principle-7 guards, and the auth check (D21-41 (g); D21-42 15a, 17a).** All read the child's own
report.
1. **The closed-set lint** of decision 2, in `lint_card`.
2. **Init equality.** `init::decide` gains three comparisons beside its tool-set and MCP checks: `permissionMode` must
   equal the **card's** mode (never the argv's spelling of it; X12), and an absent key is refused; `model` must equal
   the requested full id (decision 5); `apiKeySource` must equal `"none"`, and an absent key is refused. The last is
   ADR-0010 checked in-band, on the field D17-11's amendment named: *"AC-511's auth-mode field is **`apiKeySource`**,
   read as `"none"` on a subscription session"*.
3. **Denials are a typed failure, read from two channels.** The dispatch fails on (a) a non-empty result
   `permission_denials` or (b) any stream frame of the `permission_denied` system event keyed by its
   `decision_reason_code`, whatever the exit code. It **outranks a valid `structured_output`** and sits in
   `classify.rs`'s order after the platform arms and before the `structured_output` validation. The digest is over the
   union of both channels' `(tool_name, path)` entries, de-duplicated by `tool_use_id`. A denial's path is extracted per
   tool: `file_path` for `Read`, `Edit` and `Write`; `path` for `Glob` and `Grep` (the working directory when absent);
   `pattern` for `Glob` only when absolute; otherwise `(tool_name, "")`. An errored `tool_result` naming a denial is not
   a third detector. Channel (b)'s frame shape is drafted from the binary's strings and is **unmeasured as a live
   frame**; `s8h` §8h.11's probe 0 retains the real frame, and a difference is corrected by dated line, never by
   widening the arm to match text.

**N17 Q1's mapping**, taken and stated: a model unequal to the requested id is `NotAdopted { expected_agent, observed
}`; an `apiKeySource` absent or not `"none"`, a `permissionMode` unequal or absent, and a non-empty denial set are each
`NotIsolated { surface: Env, diff_digest }`, the digest taken over the reported value's set against the expected one
(`{"none"}`, `{the card's mode}`, or the empty set). The variant count stays eleven, the retry class is `NotIsolated`'s
`Never`, and the exit table does not move.

**D21-45's protected-segment limit is named as a known limit, not a defect.** A denial whose path resolves inside the
spec's worktree, with a segment below the worktree root on the list
`[".git",".vscode",".idea",".claude",".husky",".cargo",".devcontainer",".yarn",".mvn"]` (case-folded) or the
consecutive pair `.config`, `git`, keeps its class and digest, and classification carries a typed note
`ProtectedSegmentDenial { segment, tool_name }`, printed as a warning naming D21-45's limit. The list is one `const` in
`rk-adapter-claude`, copied from D21-45 with that citation; a release that changes it is Halt 2's concern, through the
pin.

**9. Settings isolation and confinement (D21-42 21b, 21d).** Every argv carries `--setting-sources ""` (the empty value
loads none of `user`, `project`, `local`; the 2.1.287 parser still begins `if(e==="")return[];` [measured 2026-10-02 by
string search]) and one `--settings <json>` element assembled per dispatch from the spec's worktree. **Position:** the
four elements `--setting-sources`, `""`, `--settings`, `<json>` follow `--permission-prompts none` and precede
`--system-prompt`, which stays LAST under the static profile (h1-sysprompt's position). The JSON carries:
- `permissions.blockReadsOutsideWorkingDirectories: true`, the primary read fence;
- `permissions.deny` rules for `Read`, `Edit` and `Write` written as **the complement along the worktree's ancestor
  chain** (at each ancestor level every sibling entry not on the path to the worktree, `//<drive>/**` for every other
  drive, and `~/**`), in the documented Windows form, because a `!` carve-out cannot reach an anchored rule and an allow
  rule cannot carve an exception out of a deny rule. A sibling created after assembly is outside the set, and that
  residual is stated, not claimed away;
- `autoMemoryEnabled: false`.

The effect of each is asserted from the child (the init line and probe 2's memory file set), never inferred from the
argv. Nothing is carried into the worktree for the child's settings.

**10. The launcher and its launch record are the only entry to a real run (D21-42 17a, 17b).** `tools/live-run.sh`,
committed, LF, POSIX for Git Bash, with modes `run`, `probe`, `wrap`, `answer` and `--check`, performs `s8h` §8h.7's
nine steps in order: the platform; no `CLAUDECODE`, `CLAUDE_CODE_*`, `CLAUDE_PID`, `CLAUDE_EFFORT` or `AI_AGENT`; no
denylisted name case-insensitively, read from `env_denylist.rs`'s two `pub const` arrays by anchored extraction;
`CLAUDE_CONFIG_DIR` **set, not inherited**, to `~/.claude-personal`; `DISABLE_AUTOUPDATER=1`; the resolved `claude`'s
version and sha256, compared to the pin for `run`, `probe` and `answer` and recorded only for `wrap`; the child; the
launch record at `<git-common-dir>/redkiln/launches/<UTC>-<mode>.txt`, created without clobbering, with the members
`MODE`, `ARGV`, `CLAUDE_PATH`, `CLI_VERSION_BEFORE`, `CLI_SHA256_BEFORE`, `CLAUDE_CONFIG_DIR`, `DISABLE_AUTOUPDATER`,
`PIN`, `HEAD`, `CLI_VERSION_AFTER`, `CLI_SHA256_AFTER` and `RC`, exported as `REDKILN_LAUNCH_RECORD`; and a before/after
comparison that fails the launch naming Halt 2 when the binary moved. Every refusal prints `live-run.sh: refused:
<NAME> — <reason>`. `wrap` screens its argv for a `redkiln run`/`answer` and a live `--ignored` cargo run.
**`run::run` with the real `ClaudeHarness`, and the binary's `answer::run`, refuse at entry with `Exit::ConfigError`**
unless the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file, before `reconcile_started` and before
any worktree operation, read through `env.rs` (ADR-0083). A fake-harness path and `answer::run_with` are not subject.
Every binary spawn in `rk-cli`'s `tests/it` is built by one helper that removes the variable and sets it only for an
explicit fixture record, and every `live::` test opens with the same guard. It is a guard against accident, not a
security boundary: the variable can be forged, and that is stated. The launcher runs from a plain terminal, never from
a Claude Code session, whose names step 2 refuses rather than strips. The launch record is not the kernel's run record
(ADR-0130 decision 1).

**11. The freeze (D21-42 16).** The child gets `DISABLE_AUTOUPDATER=1` from the launcher. The operator freezes their
own interactive sessions by an `env` entry in their user settings for the whole real-run window; that is user
configuration, and this record writes the instruction, never the file. The detector is the launcher's before/after
sha256, so a move is caught rather than trusted away. The pin moves once, at `s8j`, in D17-11's form (`s8h` §8h.8), with
`MismatchPolicy::Refuse` hard-coded in `real_harness_config`.

**12. What this supersedes, in part, and what it does not.**
- **ADR-0126 decision 1**, in part: its `dispatch_completed` payload table gains `observed_model` (decision 5). A member
  added to a `## Decision` table is a supersession (ADR-0092 rule 1). Every other member, obligation and rule of
  decision 1 stands, and decisions 2 onward stand whole.
- **ADR-0074 decision 1**, in part: its finding shape *"`{ ac, claim, evidence (advisory label), rationale, files[],
  repro … }`"* gains `severity` (decision 6). Its other clauses stand, as does ADR-0096's prior supersession of clause
  1's repro shape.
- **Nothing else.** N17 took Q1 as reuse, so ADR-0095 decision 2 and ADR-0112 stand. ADR-0084 is executed, not
  superseded. ADR-0156's and ADR-0162's bars on `.redkiln/schemas/` are honoured (no such directory is created).
- **Not decided here:** landing (ADR-0188), the run-worktree placement and lifecycle (ADR-0180, D21-44), the probes'
  spend (D21-40 decision 2), and the RT-57 scalars, the envelope and gate expiry (`s8h`'s Record B, ADR-0198).
- **The back-links** on ADR-0126 and ADR-0074 are written in this record's flip commit, not before: an accepted record
  is not marked superseded by a `proposed` one (ADR-0092).

**13. This record flips only after both `s8h` §8h.11 probes pass at `s8j`**, on the operator's dated ledger line
(D21-41 (g): *"The record is accepted only after two live probes pass"*), and after the operator has signed decision 7's
table enum by enum.

## Consequences

- **Every shipped card binds, and binds to least privilege.** Criterion (1)'s fourth structural reason is removed, and
  a mode outside the CLI's own set is refused at load rather than handed to a parser.
- **A pin move is a visible edit.** The argv spelling, the help capture and the golden snapshots move in one commit at
  `s8j`, and a pin with no spelling row fails red rather than re-spelling silently.
- **Which model ran is recorded and enforced.** `observed_model` is on every row that read an init, and a reported id
  unequal to the requested one refuses. The memo key moves with the full id, so a model change is a cache miss, not a
  silent reuse.
- **The schema snapshot moves once for 23 new types (the 24 less `Findings`)** (`schema_snapshot`), and `s8g`'s completeness test is the
  instrument that every new enum was classified.
- **Three refusals the child can trigger now fail the dispatch**: an unequal or absent mode, an auth source other than
  `"none"`, and any denial on either channel. A denial under a protected segment of the worktree still fails, and says
  why.
- **A real run needs a plain terminal and the launcher.** Old checkpoints that run `redkiln run` against the real harness
  become provisional and are re-scoped as `s8h` §8h.14 states; `s8j` routes `s8`'s three `redkiln run` sites and every
  real `answer` through the launcher.
- **A judgement-bearing enum without a `rationale` member grows one** in the type (decision 7), beyond the stand-in.

**What this forbids**

- Forbids a shipped card that grants `Bash`, `PowerShell`, any `Web*` tool or any `mcp__*` tool, or that carries a mode
  outside `{default, acceptEdits}`.
- Forbids admitting a permission mode outside the seven-member set of decision 2, and forbids checking a mode by
  non-emptiness alone.
- Forbids spelling a card's `default` on the argv by any route other than `PERMISSION_MODE_ARGV`'s row for the
  `Config`'s pin, forbids a fallback when the pin has no row, and forbids moving any element other than
  `--permission-mode`'s value with the pin.
- Forbids passing a tier word to `--model`, forbids adoption by segment or prefix match, and forbids writing the
  requested id into `observed_model`.
- Forbids reading a schema from a file under `.redkiln/schemas/`, and forbids selecting a step's schema by
  `provides[0]` rather than by the bound step's `value:<Kind>`.
- Forbids redeclaring a type that `rk-types` already exports, forbids a second judgement-enum registry, and forbids an
  enum reachable from `EXPORTED` without a registry entry.
- Forbids inferring the child's mode, model, auth source or isolation from the argv; forbids treating an absent
  `permissionMode` or `apiKeySource` as equal; forbids letting a valid `structured_output` outrank a denial.
- Forbids a twelfth `StepError` variant or a new `IsolationSurface` member for these refusals (N17 Q1).
- Forbids a child that loads user, project or local setting files, and forbids a deny rule set written as a `!`
  carve-out.
- Forbids a real-harness `run` or a binary `answer` without a launch record, a `wrap` that compares to the adapter pin,
  a launcher that inherits `CLAUDE_CONFIG_DIR` or `DISABLE_AUTOUPDATER`, and a launcher that strips session names
  rather than refusing them.
- Forbids writing the CLI version into the `run_id` preimage.
- Forbids flipping this record before both live probes pass at `s8j`, and forbids writing the back-links on ADR-0126
  and ADR-0074 before the flip.

## Evidence and confidence

- [measured 2026-10-02 at `2770bda`] `cargo test -p <crate> --test it -- --list` for `rk-adapter-claude` (112),
  `rk-types` (77), `rk-kernel` (415) and `rk-cli` (310), each through D21-47's no-dispatch wrapper; every regression
  name `s8h` §8h.12 reuses is listed.
- [measured 2026-10-02] `claude --version` → `2.1.287 (Claude Code)`; `RUNBOOK-instrument.txt` → 22 bytes,
  `2.1.278 (Claude Code)\n`; `claude --help`'s `--permission-mode` lines through `(choices: …)` → the six choices above,
  written by `printf` to the 2.1.287 capture (282 bytes, LF only). Neither spawn dispatches.
- [measured 2026-10-02 by string search of the 2.1.287 binary] the three model ids (`"claude-sonnet-5"` 51,
  `"claude-opus-5-5"` 47, `"claude-haiku-4-5-20251001"` 15); `apiKeySource`, `autoMemoryEnabled`,
  `blockReadsOutsideWorkingDirectories`, `permission_denied`, `decision_reason_code`, `outside_reads_blocked`,
  `DISABLE_AUTOUPDATER`, `AI_AGENT`, `checkPathSafetyForAutoEdit`, `"manual"?"default"` and `if(e==="")return[];` each
  present. A string's presence is not its behaviour: what each does is asserted by the probes, not by this search.
- [measured 2026-10-02] `bind_nodes` binds per agent step by `step.output_type` (`s8d`'s R1), and `bind_one` still reads
  `.redkiln/schemas/<kind>.json`; `EXPORTED` holds `Findings` and `AcceptanceCriterion`; of the 24 kinds only `Findings`
  exists as a type; `spikes/p9/schemas.toml`'s `[value.RootCause]` lists `cause`, `contributing_factors`,
  `confidence`, `summary` and no `discriminating_test`; `grep -rn "severity in" .redkiln/catalog` → 0.
- [documented] `permissions.md`'s `blockReadsOutsideWorkingDirectories` sentence, its `!`-scope rule and *"An allow rule
  can't carve an exception out of a deny rule."*; `headless.md`'s `--settings` *"can set any key your user settings
  file can set"*. `--setting-sources`, `DISABLE_AUTOUPDATER` and `DISABLE_UPDATES` are undocumented on the docs site and
  rest on the binary's strings (`s8h` finding 9).
- [measured 2026-09-21] the 2.1.278 init reported `model` as the full `claude-haiku-4-5-20251001` when the argv carried
  `haiku` (`PROGRESS.md`, the Claude Code instrument row). **Whether the child reports exactly the requested id when
  `--model` carries a full id is UNMEASURED**; probe 2 measures it, one dispatch per id (N17 Q4).
- **Unmeasured, and the reason this record is `proposed`:** the denial stream frame's live shape (probe 0), the
  confinement's effect on a real child and the in-worktree edit (probe 1), and the init's mode, model and auth readings
  under the shipped cards (probe 2). Confidence in the construction is high; confidence that the CLI honours each
  setting is what the probes buy, and the record waits for them.
