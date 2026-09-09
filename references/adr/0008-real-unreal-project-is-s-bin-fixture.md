---
id: "0008"
title: A real Unreal project is the S-BIN fixture in Phase 0; the toolchain adapter ships in Phase 2
status: accepted
date: 2026-09-05
legacy_id: D8
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: ["0003"]
evidence: ["S-BIN"]
consumers: []
---

## Context and problem

`ChangeSummary` (the Toolchain port's `change_summary(range)` output, §4.1) has to represent binary and generated changes, not just text diffs, and the review gate has to be able to verify acceptance criteria and catch a seeded regression when there is "no diff" to read — asset manifests, automation reports, captures. Designing that type against a synthetic stand-in risks shaping it around an imagined binary-change shape rather than a real one, and the toolchain gap (Unreal is not yet an adapter) has to be closed before Phase 2's parallel project DAG can include UE items.

## Decision drivers

- `ChangeSummary`'s design has to be tested against artefacts a binary-heavy toolchain actually produces (asset manifest, automation report, captures), not an author's guess at their shape.
- S-BIN, the spike that answers whether the reviewer can verify ACs and catch a seeded regression from those artefacts alone, is marked "gates Phase 1 (type design)" in §8 — it needs a real fixture to run against.
- The full Unreal toolchain adapter (build/test/lint/classify/change_summary for UE) is separately costed work that does not need to land before Phase 1's kernel/store/fix-lane scope closes.

## Considered options

### Option — minimal synthetic fixture
A hand-built stand-in exercising only the binary/no-diff code path would have been cheaper to stand up than a real UE project. Rejected: it would not surface the actual shape of asset manifests, automation reports or captures that `ChangeSummary` has to type against, defeating the purpose of designing the type from real artefacts.

### Option — paper design
Designing `ChangeSummary` from documentation/specification alone, without running S-BIN against anything. Rejected: it would leave the "can the reviewer verify ACs and catch a seeded regression with no diff" question unanswered, and S-BIN is explicitly a gating spike for that type design.

## Decision

"A real UE project exists and is the S-BIN fixture in Phase 0; `ChangeSummary` is designed against it; Unreal toolchain adapter ships in Phase 2." Concretely: the UE fixture is stood up during Phase 0; S-BIN (5 items whose change is asset manifest + automation report + captures, no diff) runs against it and feeds the `ChangeSummary` type; Phase 1 only requires `ChangeSummary` to be "designed against the UE fixture" (per the Phase 1 exit criteria in §7), not a working adapter; the full Unreal toolchain adapter — alongside .NET and Python — ships in Phase 2, whose exit criterion includes "a UE item verified through the adapter."

## Consequences

**Positive**
- `ChangeSummary`'s binary/no-diff handling is grounded in a real toolchain's output shape rather than a synthetic guess.
- Phase 1 is not blocked on a full Unreal adapter; only the type design needs the fixture.

**Negative**
- Standing up and maintaining a real UE project as a fixture is materially more expensive than a synthetic stand-in (S-BIN is costed at ~10 Opus dispatches plus UE automation runs).
- Per the "Still open after rev. 4" note in the source, S-BIN is registered but had **not been run** as of this ledger snapshot — so the type design this decision authorises is not yet backed by a completed spike result.

**What this forbids**
- `ChangeSummary` may not be designed and locked purely against a synthetic or paper fixture; it must be designed against the real UE project's actual artefacts.
- The full Unreal toolchain adapter may not ship before Phase 2 — Phase 1 scope is limited to the type design, not the adapter.

## Evidence and confidence

- [documented] S-BIN's question, fixture shape (5 items, asset manifest + automation report + captures, no diff), and "gates Phase 1? yes (type design)" status are given in the §8 spike table, costed at ~10 Opus dispatches + UE automation runs.
- [documented] The source's own status line states S-BIN is "registered and unrun" as of the rev. 4 snapshot — the decision to use it as the fixture is accepted, but the type-design outcome it is meant to produce is provisional on that spike actually running. This ADR does not claim S-BIN has been executed.
- [documented] Phase 2's exit criterion ("a UE item verified through the adapter", §7) and its scope line naming ".NET, Python and Unreal toolchain adapters (D8)" fix the Phase-2 ship date for the adapter itself, as distinct from the Phase-0/1 fixture-and-type-design work.
