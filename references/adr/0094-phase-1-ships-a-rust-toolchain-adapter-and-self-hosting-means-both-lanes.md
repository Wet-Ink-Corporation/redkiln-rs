---
id: "0094"
title: Phase 1 ships a Rust toolchain adapter beside Node, and self-hosting means the fix lane and one story-lane run on Redkiln-RS itself
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-004 (§3.2), PIA-072 (§4.1); references/adr/0002 :53; references/adr/0011 :47; references/2026-09-08-architecture-design.md §10.2 :246
supersedes: ["0003"]
superseded_by: []
related: ["0002", "0010", "0011", "0029", "0073", "0076"]
evidence: ["PIA-004", "PIA-072", "F12", "F19"]
consumers: []
---

## Context and problem

Phase 1's exit criterion (1), AC-473 and the runbook's Step 8 require a recorded fix-lane run on
`D:/repos/redkiln-rs` — a Cargo workspace — with every lane stage present, and the verify and review stages reach a
toolchain only through the `Toolchain` port (AC-328/AC-329 make the adapter the sole producer of `VerifyReport`;
AC-497 routes `classify` and `change_summary` through it). Yet design §10.2 scopes Phase 1 to "exactly one adapter
behind each port it uses — SQLite, Claude Code and Node" [documented, :246]; ADR-0003's port table lists Rust only as
a **named alternative** for `Toolchain` ("Node (vitest/pnpm) | .NET, Python, Unreal, Rust"); §9.1's crate list has
no `rk-adapter-toolchain-rust`; Phases 2–4 add .NET/Python/Unreal, GitHub/mdBook, Codex/Linear/Astro and never Rust;
and no Rust technology pack is mentioned anywhere in the design, the registers, the runbook or the 90 records
[measured, PIA-004]. Phase 1 as written cannot run its own fix lane, because no adapter can run `cargo test` on it.

A second disagreement sits between two accepted records. ADR-0002:53 says the cutover "is gated **solely** on
Redkiln-RS running its own fix lane end to end (self-hosting)"; ADR-0011:47 says "the exit bar is specifically 'runs
its own fix lane end to end (self-hosting) **and Phase 2's stories are authored under it**'". Design §10.2's exit,
Step 8's seven checks and Phase 2's "Enter when" encode only the fix-lane half; ADR-0011's four criteria (AC-400,
AC-405, AC-437, AC-473) contain no story-lane criterion; `story` appears in the runbook only in its scope line
[measured, PIA-004]. The story lane ships in Phase 1 (ADR-0011's title) with nothing checking that it works.

A third, smaller one: ADR-0003's column header reads as if every port has a Phase-1 real adapter, so AC-499's
`{fake, real}` matrix row is unbuildable for `Tracker` and `DocsSite` (no real adapter until Phase 3), and 7a's
acceptance set sweeps in criteria whose real half cannot exist yet [measured, PIA-072].

This record supersedes ADR-0003 **in part**: the `Toolchain` row's Phase-1 column and the reading of the "named real
adapter" column. The five ports, the no-Postgres decision, git-is-not-a-port and the fake-per-port rule stand;
ADR-0003 stays `accepted`. It also settles the ADR-0002:53 / ADR-0011:47 disagreement in favour of ADR-0011's
wording, without editing either.

## Decision drivers

- Self-hosting is the clean-break proof ADR-0002 was written for; a Phase 1 that can only self-host on a Node repo
  is not self-hosting.
- The `Toolchain` port is fixture-driven (ADR-0003) and the Node adapter is one implementation of a small trait —
  `build`, `test`, `classify`, `change_summary`; a `cargo` adapter is the same shape over `cargo build`/`cargo test`
  (or `nextest`) and `cargo metadata`, and the fake count per port is unchanged.
- ADR-0011 titled itself "fix and story lanes only"; an exit bar that checks one of the two leaves the other
  unmeasured for the whole phase.
- AC-499's matrix is per port; a port whose real adapter is Phase 3 needs its row to say so, not to fail.

## Decision

**1. Phase 1 ships four real adapters: SQLite (`EventStore`), Claude Code (`Harness`), Node (`Toolchain`) and Rust
(`Toolchain`).** The crate is `rk-adapter-toolchain-rust`; it implements the same `Toolchain` port as
`rk-adapter-toolchain-node`, with `build` = `cargo build --workspace`, `test` = `cargo test --workspace` (or
`cargo nextest run` where present, recorded on the report), `classify` and `change_summary` over `cargo metadata` and
the changeset — `change_summary` remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4). A
**Rust technology pack** binds the adapter's artifact kinds (`TestReport`, `Changeset`) per ADR-0029. Design §9.1's
crate table, §10.2's Phase-1 row, the runbook's Step 7c and Checkpoint 7c gain the adapter, its fake-conformance row
and the pack. ADR-0003's `Toolchain` row now reads "Node, Rust | .NET, Python, Unreal | fixture-driven".

**2. ADR-0003's "named real adapter" column is read as "named real adapter, in the phase design §10.2 assigns".**
Phase 1 instantiates `{fake, real}` for `EventStore`, `Harness` and `Toolchain` (two reals); `Tracker` and
`DocsSite` have a fake only until Phase 3, and AC-499 carries a phase clause saying so. The Harness port's second real
adapter (Codex) is Phase 2 and stays behind the CI decision ADR-0010 defers, as a named open item with
`rk-adapter-codex` as consumer.

**3. Self-hosting means both lanes.** Phase 1 is complete when (a) a recorded fix-lane run on `D:/repos/redkiln-rs`
carries every lane stage from intake to a landed commit with no stage performed by the old redkiln (AC-473), **and**
(b) at least one recorded `story`-lane run under Redkiln-RS authors a Phase-2 story — a new criterion **AC-521**,
twin of AC-473, checked by a release check over the event log, and a Step 8 check (1b). ADR-0002:53's "solely" is
read as "solely on self-hosting, not on a calendar", which is what its sentence is about; ADR-0011:47's wording is
the exit bar.

## Considered options

### Option — Rust adapter, fix lane only
Keep the adapter and settle the exit as ADR-0002's narrower reading by superseding ADR-0011's clause. Rejected: it
ships a lane no criterion exercises, and the Phase-2 stories that lane must author are the first thing Phase 2 needs.

### Option — redefine the self-hosting target as a Node repository
Supersede ADR-0002/0011 to name the frozen redkiln corpus as the Phase-1 run target. Rejected: "Redkiln-RS runs its
own fix lane" stops being literally true, and the corpus is frozen (ADR-0013) — a fix lane cannot land a commit on it.

### Option — a generic `cargo`/`npm` shell adapter
One adapter parameterised by command strings. Rejected: `classify` and `change_summary` are toolchain-specific
(ADR-0073's toolchain-keyed kinds exist because of it), and one adapter with two behaviours is two adapters with one
name.

## Consequences

**Positive**
- Step 8 (1) becomes passable on the repository it names.
- The story lane has an exit check; Phase 2 starts with a story authored by the system that will build it.
- AC-499's matrix is buildable per port in Phase 1.

**Negative**
- One more crate and one more technology pack before the exit; a fourth fake-conformance row in Checkpoint 7c.
- Two toolchain adapters in Phase 1 means ADR-0073's mixed-toolchain instance expansion is exercised by the
  self-hosting run itself (redkiln-rs carries Python scripts under `spikes/`); that is a feature and a risk.
- Step 8 gains an eighth check.

**What this forbids**
- Forbids declaring Phase 1 complete on a fix-lane run alone, or on a run whose toolchain stage was performed by
  anything but the Rust adapter through the `Toolchain` port.
- Forbids a Phase-1 real adapter for `Tracker` or `DocsSite`, and forbids AC-499 failing on their absence.
- Forbids adding an adapter in code that no record names — ADR-0003's table, as amended here, is the list.

## Evidence and confidence

- [measured, PIA-004] "Rust" as a Toolchain adapter occurs only in ADR-0003:42's alternatives column and design §3.3
  across the design, AC, SM, RUNBOOK, 90 ADRs and the proposal; `story` occurs in the RUNBOOK only in its scope line;
  ADR-0011's four criteria carry no story-lane check.
- [documented] ADR-0002:53 and ADR-0011:47, quoted above; design §10.2 :246.
- [measured, PIA-072] AC-499's matrix row is unbuildable for two ports in Phase 1; 7a's set (:1902) sweeps in
  fourteen Phase-2/3, CONDITIONAL or retired rows.
- [measured, F12, F19] the fix lane's stages and their replay are the measured substrate; nothing measured bears on a
  story-lane run, which is why AC-521 is a release check and not a threshold.
- **What this evidence does not license.** It does not license any claim about the Rust adapter's behaviour — none
  exists; the fake-conformance row and AC-328/AC-329 are what will. It does not decide the Codex adapter's phase or
  the CI decision ADR-0010 defers.
