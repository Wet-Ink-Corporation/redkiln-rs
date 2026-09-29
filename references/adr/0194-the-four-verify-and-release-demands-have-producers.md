---
id: "0194"
title: "**The four verify and release demands have producers** (`s8g`'s producers record, R4; D21-42 6a; D21-43 N25; D21-46 R6): `SmokeResult`, `ConfirmationResult`, `ScanReport` and `BuildArtifact` are written by walks of the shipped `smoke-test`, `confirmation-test`, `sca-scan` and `build-package` activities under `s8d`'s resolver, each through one `artifact.write`; `smoke-test` gains a `passing` check (`smoke.status == 'passing'`, `on_false = \"fail\"`, version 1 → 2, D21-46 R6); the smoke checklist is authored at `.redkiln/checklists/smoke.toml` and `[checklist.smoke]` flips from `unauthored` to its `path`; the three Rust verb handlers are `0170`'s (record V), `sca-scan`'s `unrated == 0` edit is `s8b`'s changelog row `sca-scan@2`, and `package_binaries` is `0179` decision 4's row, each cited by id and not decided again. Supersedes nothing"
status: proposed
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (row 6a — *"**Reversed for five demands:** SmokeResult, ConfirmationResult, ScanReport, BuildArtifact and DebtRegisterEntry. … **Supersedes D21-41(c)/(d)'s narrowings.**"*), D21-43 (N25; the integration call *"The Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4)"*), D21-46 (R6 — *"**A check on `status == 'passing'` with `on_false = "fail"`**, as an ADR-0072 edit in s8g."*), D21-63, D21-64 (ruling 7), D21-67 (ruling 7), D21-73 (items 4 and 5); references/adr/0170 (record V), 0179 (decision 4); .redkiln/CHANGELOG.md (`sca-scan@2`); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.6, §8g.8 (R4's row)
supersedes: []
superseded_by: []
related: ["0072", "0114", "0169", "0170", "0179", "0180", "0189", "0191", "0195"]
evidence: []
consumers: []
---

## Context and problem

**D21-42 6a reverses D21-41's pack cuts for five demands.** It reads *"**Reversed for five demands:** SmokeResult,
ConfirmationResult, ScanReport, BuildArtifact and DebtRegisterEntry. … **Supersedes D21-41(c)/(d)'s narrowings.**"*
`DebtRegisterEntry` is `s8f`'s `rk-store` item write. The other four are this stage's, and at `ca8f387` none has a
producer a run reaches: `bug-standard.toml` demands *"`baseline = ["TestReport", "ConfirmationResult",
"SmokeResult"]`"* at verify and *"`baseline = ["BuildArtifact", "ReleaseRecord"]`"* at release, and adds
*"`when = "'text' in change"`"* → `ScanReport` at verify.

**What the Rust verbs do is decided elsewhere.** D21-43's integration call: *"The Rust verb handlers are **s8c**'s
record (V); the activities are **s8g**'s (R4)"*. Record V is ADR-0170 (accepted on D21-70). `sca-scan`'s N25 edit
(*"**`sca-scan` requires `unrated == 0`** or goes to `scan.accept-findings`"*) is `s8b`'s catalog edit, listed in
`.redkiln/CHANGELOG.md` as `sca-scan@2` (2026-09-25, D21-43 N25). This repository's `package_binaries` row is ADR-0179
decision 4's (accepted on D21-86), written at `s8d` under D21-73 item 5.

**`smoke-test` has no check.** At `c5d19d7` (re-read at `178868f`) its steps are `build`, `smoke` and `emit`, version
1, so a `SmokeResult` with `failed >= 1` is emitted and the stage completes on it. D21-46 R6 decides the check.

**`checklist:smoke` refuses by name.** `s8c` declared `[checklist.smoke]` in `.redkiln/registry.toml` as
`unauthored`, naming `s8g` §8g.6 as the stage that replaces it with its path.

## Decision drivers

- Thin producers over thick ports: no handler semantics decided twice (D21-43's integration call).
- ADR-0072: an edited shipped activity is a new version with a changelog row.
- A measurement whose verdict nobody reads is not a gate (D21-46 R6's reason).
- No smoke item dispatches, and no smoke item's working directory is inside the run-owned worktree (ADR-0180).

## Considered options

### Option — walk the shipped activities, with `smoke-test`'s check added and the checklist authored (taken)

Taken: every demand is produced by a catalog walk under `s8d`'s resolver and ADR-0170's registry method, each writing
one `artifact.write`, with this stage's edits limited to one step-list edit, one checklist and one registry entry.

### Option — keep `smoke-test` emit-only and let a later stage read the status

Rejected (D21-46 R6): no stage-exit rule reads a `SmokeResult`'s status, so a failing smoke run would complete the
verify stage.

### Option — re-specify the three handlers' semantics in this record

Rejected: they are ADR-0170's decisions (record V), and a second specification would let the two files disagree.

## Decision

1. **`SmokeResult` — `smoke-test`.** `toolchain:build`, then `toolchain:smoke` with *"`checklist =
   "checklist:smoke"`"*; ADR-0170 decision 4's handler *"runs each declared checklist row against the binaries that build
   produced, never an installed `redkiln`"*. **A `passing` check** sits between `smoke` and `emit` (D21-46 R6): `id = "passing"`, `kind = "check"`,
   `inputs = { smoke = "${smoke.outputs}" }`, `expr = "smoke.status == 'passing'"`, `on_false = "fail"`. The version
   moves 1 → 2 with a `smoke-test@2` changelog row quoting the old and new text. A failing result fails the activity,
   writes no `SmokeResult`, and leaves the verify stage's baseline demand unmet. `smoke` is live, so every step using
   it has `memo = false`.

2. **The smoke checklist** is authored at `.redkiln/checklists/smoke.toml` in the handler's item form (`id`, `argv`,
   `cwd`, `expect_exit`, one stdout predicate). Its first cut, each expectation measured on the built binary before
   it is written: `redkiln --version`; `redkiln-migrate --version`; `redkiln --help` naming the nine verbs;
   `redkiln -C <scratch> init` then `status`; `redkiln-migrate` with no arguments, exiting clap's `2`. **No item
   dispatches** (no `run`, no network), and every item's `cwd` is a scratch repository **outside
   `.redkiln/worktrees/<item>`** (where the handler makes it is ADR-0170's decision). `.redkiln/registry.toml`'s
   `[checklist.smoke]` replaces `unauthored` with `path = "checklists/smoke.toml"` in the same commit.

3. **`ConfirmationResult` — `confirmation-test`.** `toolchain:test` with *"`selection =
   "artifact:ReproCase.command"`"* through `selection_of` (ADR-0162) and the resolver; the `fixed` check reads
   *"`result.status == 'passing'`"*, and a non-passing result opens *"`question = "confirmation.not-fixed"`"*, one of
   `s8d`'s ten loaded questions. At the fix sha a passing result emits; a result that does not pass opens the gate and
   emits nothing. *Owed and not decided at writing:* the composing emit reads `${before.outputs.selection}` and
   `${before.outputs.sha}`, which ADR-0169's `TestOutcome` does not carry (`s8g` §8g.1's D21-73 item 4 receiver note);
   the build settles it by an ADR-0072 catalog edit or by a record widening what the `test` step binds, and records
   the choice in this record by dated note while it is `proposed`.

4. **`ScanReport` — `sca-scan`.** `toolchain:sca_scan`, gated by `scan.accept-findings`. The handler's semantics are
   ADR-0170 decision 5's: both tools, the CVSS bands, `unrated`, the advisory database's commit or a refusal by name,
   and the `cvss-unscorable` refusal for a matched vector neither scorer accepts (D21-64 ruling 7 moved `CVSS:4.0`
   to the `cvss` crate's scorer; D21-67 ruling 7 scores it base-only). The activity's `unrated == 0` requirement is
   `s8b`'s `sca-scan@2`, read and not edited again. The producer walk opens `scan.accept-findings` on an unrated
   advisory and on a high finding, and emits without a gate on a clean scan.

5. **`BuildArtifact` — `build-package`.** `toolchain:build`, the `built` check (*"`report.status == 'passing'`"*, now
   live because N25 widened `build` to an outcome report), then `toolchain:package`, which runs a locked release
   build and takes the plain sha256 of each declared binary (ADR-0170). The declared set is `[toolchain.rust]`'s
   `package_binaries = ["redkiln", "redkiln-migrate"]`, ADR-0179 decision 4's row with its warrant (AC-441's
   evidence: the migrator is a *"separate binary"*); this record cites it and does not write it. A release build also links
   `rk-fixture-child-cli`, `rk-fixture-toolchain` and `rk-fixture-child-claude`, and none of them is the release.

6. **Each of the four writes one `artifact.write`** through `rk-store`'s write path (ADR-0189), and `setup` is the
   precondition of each walk's tree (`s8c`'s `setup_projects` row naming
   `crates/rk-adapter-toolchain-node/tests/fixtures/project`; run at creation by ADR-0180).

## Consequences

**Positive**
- The fix lane's verify and release baselines have reachable producers, and `smoke-test` gates on its own result.
- `checklist:smoke` resolves, and the smoke run exercises the shipped binaries without dispatching.

**Negative**
- The smoke checklist pins `--help` and `--version` text, so a verb or version change moves it.
- `confirmation-test`'s emit fields are owed to the build (decision 3) until settled by dated note.
- `sca-scan` refuses the whole scan on a matched vector neither scorer accepts; the operator rules that vector.

**What this forbids**
- A smoke item that dispatches, runs `redkiln run`, reaches the network, or runs in the run-owned worktree.
- Emitting a `SmokeResult` whose status is not `passing` as the activity's product.
- Counting an unrated advisory as zero, or routing `cvss-unscorable` to `scan.accept-findings`.
- A `package_binaries` set other than ADR-0179's without a dated record edit, and packaging a fixture binary.
- Deciding a Rust verb handler's semantics outside ADR-0170, or re-editing `sca-scan`'s check under this record.

## Evidence and confidence

**Measured** (2026-09-29, at `178868f`): `smoke-test` version 1, step ids `build`, `smoke`, `emit`;
`[checklist.smoke]` declares `unauthored`; `sca-scan`'s check reads *"`report.critical == 0 and report.high == 0 and
report.unrated == 0`"*; `[toolchain.rust]` carries `package_binaries`. **CVSS precondition** (`s8g` §8g.6, as narrowed
by D21-64 ruling 7 and D21-67 ruling 7), read at the open through the no-dispatch helper: `cargo audit --json
--no-fetch --file Cargo.lock` against `~/.cargo/advisory-db` at `f23b768236fe2880e4cfa167da662cad8ca79240` (1277
advisories, 331 dependencies) → `vulnerabilities.found = false`, count 0, so no matched vector reached a scorer and the
named halt does not fire; one informational `unsound` warning (`RUSTSEC-2026-0306`, `faster-hex`, `cvss` null),
which the handler does not count. The databases have moved since `s8c`'s pin `e2111519`. **Confidence:** high on
the producers; the confirmation emit fields are open (decision 3).
