---
id: "0204"
title: "**The fix lane reproduces from the item's committed repro pair, placed on the run branch by the `repro_place` toolchain verb; an agent-produced selection is refused unless it is a plain command** (`s8`'s repro-pair route; the operator's route ruling of 2026-11-08 after relaunch 3 stopped at rc 14): `minimal-repro@2` places `.redkiln/intake/<item>.repro.{toml,rs}` in the run tree and commits it on `rk-run/<item>` at the step grain, `confirm` runs exactly that test and it must be red, `ReproCase.command` is the placement's command, `root-cause@6` confirms on it, `parse_selection` is the one parser and refuses prose by name, and `explain` names a run that stopped at a failed step. Supersedes ADR-0200 decision 4 in part, for the repro pair only"
status: proposed
date: 2026-11-08
legacy_id: none
source: the 2026-11-08 relaunch 3 of `rk-fix-doctor-margin` (`minimal-repro@1`'s `confirm` ran the investigator's prose, `NonZero`, `StepFailedAfterCommit`, exit 14, and `explain` then refused on the missing `run.json`); Ryan's route ruling, 2026-11-08 ("the kernel uses the committed repro pair"); his approval of the plan's decisions D1-D13, 2026-11-08 06:12Z; spikes/plans/s8-repro-pair-route-2026-11-08.md; spikes/DECISIONS-2026-09-21.md D21-174; references/adr/0170 decisions 1, 2 and 7, 0173, 0188 decisions 3 and 5, 0200 decision 4; .redkiln/cards/investigator.json (Read, Glob, Grep, StructuredOutput)
supersedes: ["0200"]
superseded_by: []
related: ["0072", "0170", "0173", "0188", "0200", "0202"]
evidence: []
consumers: ["crates/rk-types/src/repro_pair.rs", "crates/rk-types/src/verb_output.rs", "crates/rk-adapter-toolchain-rust/src/verbs/repro.rs", "crates/rk-kernel/src/runtime/reference/selection.rs", "crates/rk-cli/src/repro_pair.rs", "crates/rk-cli/src/verbs/explain/stopped.rs", ".redkiln/registry.toml", ".redkiln/catalog/minimal-repro.toml", ".redkiln/catalog/root-cause.toml"]
---

## Context and problem

**The fix lane's reproduction could only ever be prose.** `minimal-repro@1`'s `confirm` read
`${attempt.outputs.command}`, and `attempt` is an `investigator` step. The investigator card grants Read, Glob, Grep
and StructuredOutput: it can neither write a test nor run a command. Its `command` is therefore prose, or names a test
that does not exist. On 2026-11-08 relaunch 3, `selection_of` split that prose after ` -- ` into libtest arguments,
`` --exact` `` failed `--list` in every target, and the run stopped with exit 14. `root-cause@5`
(`${hypothesise.outputs.discriminating_test}`) and verify's `confirmation-test@2` (`artifact:ReproCase.command`,
produced by the investigator's `reduce`) carry the same defect one and two stages later.

**The fixture proofs passed because the fake did what the real card cannot.** The lane proofs' `attempt` and
`hypothesise` responses wrote test files into the run tree. No real investigator dispatch can do that.

**The defect is reproduced already, by the operator.** §8j.5 step 3 had the operator write the repro pair,
`.redkiln/intake/<item>.repro.toml` (kind, crate, module, test, expect) and `.repro.rs` (the test), red at HEAD in a
throwaway tree. ADR-0200 decision 4 kept that directory away from every run (*"Nothing reads the directory"*).

**`explain` hid the stop.** `run` writes the run record only on success, so `explain` on the stopped run refused on a
missing `run.json` and named nothing.

## Decision drivers

- **The repro a fix lane runs is a test a human committed**, never an agent's description of one.
- **Cards stay read-only.** `s8h`'s card checks and the paid confinement probes are untouched.
- **The kernel knows no language** (`rk-ports/src/toolchain.rs`). `crates/<crate>/tests/it`, `--test it` and `--lib`
  are Rust facts, so they live in `rk-adapter-toolchain-rust`.
- **Fail closed, by name.** A missing or invalid pair refuses at the step that needs it, naming the path.
- **One parser for a selection** (ADR-0173), which refuses more rather than guessing more.
- **No port change, no new `StepError` variant, no new dependency.**

## Considered options

### Option — A `repro_place` toolchain verb and a row-input envelope (taken)

rk-cli reads the pair from the run tree and binds it as the row input `repro_place.pair`; the Rust adapter appends
it to its target and answers a typed `ReproPlacement`. Taken: plan Decision D1, option A.

### Option — A human-attached `ReproPair` root through a gate

The ADR-0200/0202 route. Not taken: two files do not fit one `--attach`, and a mandatory human kind would block every
bug that has no pair.

### Option — A kernel seam that writes files

Not taken: it breaks "the kernel knows no language".

### Option — rk-cli places the pair at run entry

ADR-0188's entry landing. Not taken: implicit, active from triage on, and with no typed output in the journal.

## Decision

**1. The route.** A fix-lane run reproduces from the item's committed repro pair. `minimal-repro@2`'s first step,
`place`, runs `toolchain:repro_place`. A bug with no pair refuses at `place`, by name; there is no fallback to an
agent's command (plan D3).

**2. The envelope and its refusals.** `run` reads `.redkiln/intake/<item>.repro.toml` and `.repro.rs` from the run
tree (the fork commit plus the run's commits, so the committed, frozen bytes), each capped at
`rk_types::repro_pair::REPRO_PAIR_MAX_BYTES` (64 KiB), and always binds `repro_place.pair` to a
`ReproPairInput`: `present` (a validated `ReproPair`), `absent` (naming both paths looked for) or `invalid` (naming
the path and why). The spec is `deny_unknown_fields`; `kind` is `it` or `unit`; crate, module and test names are
identifiers; a `unit` file is a plain relative path under `src/` ending in `.rs`. The pair carries the sha256 of
both files.

**3. The registry row.** `repro_place = { outputs = "ReproPlacement", live = false, timeout = "build_timeout",
inputs = ["pair"], route = "toolchain", side_effecting = true }`, on the existing `toolchain` route. ADR-0170
decision 1 provides for a later record to add a row, so nothing there is superseded. The Rust handler appends the
source to `crates/<crate>/tests/it/<module>.rs` (`it`) or `crates/<crate>/src/<file>` (`unit`), refuses a target
outside the project root or not a regular file, refuses a file that already defines `fn <test>(` elsewhere, answers
`appended: false` and writes nothing when the file already ends with the block (re-entry), and answers
`ReproPlacement { path, test, command, expect, source_sha256, spec_sha256, appended }`. The command is
`cargo test -p <crate> --test it -- <module>::<test> --exact` (`it`) or `cargo test -p <crate> --lib --
<module>::<test> --exact` (`unit`).

**4. `minimal-repro@2` and `root-cause@6`.** `minimal-repro@2`: `place`; `confirm` on `${place.outputs.command}`; a
`red` check (`report.failed > 0 and report.passed == 0`); the investigator's `attempt` judges the red run, with no
repeat and no escalate; `triage-gate` opens when the run is not red or the agent says not reproduced; `emit` composes
`ReproCase` with `command = "${place.outputs.command}"`. `root-cause@6`: `confirm` selects
`artifact:ReproCase.command`, and `emit` composes `RootCause` with `discriminating_test = "artifact:ReproCase.command"`.
Verify's `confirmation-test@2` is unchanged: it now re-runs the placed test.

**5. The commit.** `repro_place` is side-effecting, so the placed test is committed on `rk-run/<item>` at `place`'s
step grain (ADR-0188 decision 3), lands with the fix, and is the fix's regression test (plan D2).

**6. `parse_selection` is the one parser.** `rk_kernel::runtime::reference::parse_selection` refuses, in order, a
newline (`Multiline`), a control character (`Control`), a shell character (`ShellSyntax`, naming it and its byte),
no ` -- ` (`NoFilter`) and an empty tail (`EmptyFilter`); each refusal's text starts with its variant name.
`selection_of` is its `.ok()`. A `${…}` list element is checked token by token (`check_selection_token`, plan D5).
ADR-0173 stands; its parser refuses more.

**7. `explain` names a stopped run, and `run` names the class.** When the item's newest `activity_committed` run id
has no record, `explain` prints `run: <id> (no run record: the run stopped at a failed step before its record was
written)`, a `stopped:` line when a `step_failed` or `step_refused` row follows that run's last commit, then
`last recorded run:` and the newest earlier recorded run, and exits 0 (plan D7, option a). `run` writes the failed
step's class to stderr beside its exit, so a refusal reaches the operator by name.

**Supersedes ADR-0200 decision 4 in part**: its bullet *"Nothing reads the directory"* and the bullet's *"which
Checkpoint 8j (4) reads and no run reads"*, for the repro pair only. Human roots still reach a run only through
`--attach`. This answers ADR-0200's rejected option *"read the root from a committed file at `run`"*: the pair is not
an artifact kind and not a root, it is read from the frozen commit's bytes, and its digests are journalled on
`ReproPlacement`. ADR-0200's `superseded_by` gains 0204 at this record's flip commit only (ADR-0092).

## Consequences

- The fix lane can reproduce under read-only cards, and the landed fix carries the committed test.
- A bug item needs a pair to pass analysis. Without one, `place` refuses and the operator writes the pair.
- `explain` survives a stopped run. A `stopped:` line appears only when the store holds a failure row for the run;
  a toolchain refusal journals none today, so `run`'s stderr line is the operator's channel for it.
- **Known violators, outside the lanes.** `differential-debugging`'s and `instrumented-trace-analysis`'s `confirm`
  select a read-only investigator's output. No lane selects either. rk-cli's
  `catalog::no_test_selection_reads_a_read_only_roles_output` lists them exactly (plan D13), and they are tracked by
  the issue below.
- **Open at this record's writing (2026-11-08):** `root-cause@6`'s `policy` still loops back to `hypothesise` on a
  red-free confirm, but `confirm` no longer reads anything `hypothesise` produces, so the loop varies nothing.
  `rk-kernel`'s `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` is red on it, and two counts
  moved (`ac527_empty_set_terminal_condition` 93 → 94 check steps, `review::ac303_confirm_step_survives_escalation`
  (4, 16) → (4, 15)). The plan did not settle the policy's shape; it is the operator's decision, and this record
  is amended when it is taken.

### Owed issue (D13)

The implementer does not create issues. The operator files this one, with the backlog's issue body contract
(`docs/BACKLOG.md`):

> **Title:** `differential-debugging` and `instrumented-trace-analysis` select a read-only investigator's output
>
> **Body:** Both activities' `confirm` step (`toolchain:test`) reads `${compare.outputs.discriminating_test}` /
> `${analyse.outputs.discriminating_test}`, produced by an `investigator` step. The investigator card grants Read,
> Glob, Grep and StructuredOutput, so the selection can only be prose or a test that does not exist: the 2026-11-08
> `minimal-repro@1` failure (ADR-0204). No lane selects either today. Fix: confirm on `artifact:ReproCase.command`, as
> `root-cause@6` does, or on a tool step's output. Done when both leave `KNOWN_READ_ONLY_SELECTIONS` in
> `crates/rk-cli/tests/it/catalog.rs` and `catalog::no_test_selection_reads_a_read_only_roles_output` stays green.

### Owed issue (D4, option c)

Plan Decision D4 took option a now and option c as a follow-up issue. The operator files it:

> **Title:** Refuse a selection-feeding agent field that is not a plain command at the agent step, as a retryable
> `Schema` error
>
> **Body:** ADR-0204 decision 6 refuses a non-plain selection at the `toolchain:test` step, by name, through
> `parse_selection`. The agent that produced it is not asked again. Validating the selection-feeding fields
> (`ReproAttempt.command`, `ReproCase.command`, `HypothesisLog.discriminating_test` and co.) when the agent step
> answers would turn the refusal into a `Schema` error the step's retry can correct. Done when an agent answer whose
> selection-feeding field fails `parse_selection` is retried as `Schema`, with a test per field.

## Evidence and confidence

- rk-cli `lane_proof::fix_lane_reproduces_through_the_committed_pair_not_agent_prose` walks the fix lane with every
  agent `command` and `discriminating_test` set to prose and no agent writing a file; it passes through `place`, a
  red confirm, a passing verify and a landed repro. `lane_proof::fix_lane_without_a_repro_pair_refuses_at_place_by_name`
  shows the refusal by name. The mutants `M-repro-pair-ignored`, `M-repro-place-noop` and `M-selection-shell-accepted`
  were killed on 2026-11-08 in the build container.
- Confidence: high for the route; the real box runs nextest, and `--exact` there is measured by §8j.6 step 1b's
  re-run, not here.
