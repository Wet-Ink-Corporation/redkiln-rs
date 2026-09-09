---
id: "0002"
title: Clean break from old redkiln; self-hosting is the cutover switch
status: accepted
date: 2026-09-05
legacy_id: D2
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §5)
supersedes: []
superseded_by: []
related: ["0001", "0013"]
evidence: []
consumers: []
---

## Context and problem

Redkiln-RS needs a relationship to the old redkiln's code, data and running practice. The proposal frames a "YAML byte-parity risk" (R3) that a compatibility path would have to carry, and a self-hosting milestone (Redkiln-RS running its own fix lane end to end) as the natural point at which continuing to operate two systems stops making sense.

## Decision drivers

- Redkiln-RS "shares no code, file format or runtime with the old redkiln" (§5); §5's table then fixes, row by row, what crosses over and in what form. The source states this as the premise of the section, not as a conclusion argued from the inversion architecture.
- The old redkiln's 101 CLI-behaviour test files and recorded bugs (lock/id collisions, `--stay`, RK-B0013, the `basis` flip, F25's per-AC gap) are valuable as black-box specifications and test cases, but their logic does not need to survive as code — with one exception, `reconcile.rs`, which was rewritten in the spike and validated on 360 commits / 74 slices (F12).
- The YAML byte-parity risk (R3) is "dissolved rather than mitigated" only if the new store owns its own serialiser and a single migration commit normalises everything once, rather than a runtime compatibility layer trying to stay byte-compatible indefinitely.

## Considered options

### Option — phased cutover

Rejected. §5 states the outcome directly — "that milestone, not a phased cutover, is the switch" — with self-hosting as the switch. The source does not spell out a losing reason; on the plain reading of §5, a phased cutover would require the two systems to interoperate on shared data or a shared runtime for a transitional period, which the section's no-shared-code/format/runtime premise excludes. That reading is this ADR's, not the source's wording.

### Option — engine adapter

Rejected. The source names "engine adapter" only in the D2 alternatives column and does not define it; read as a compatibility layer letting the new kernel work against the old `.bklg`/`.kb` format at runtime, §5's table rules it out explicitly — that content crosses as "input to `redkiln-migrate`, a separate tool that emits the new typed layout in one reviewed commit, with a differential report", and expressly *not* as "a runtime compatibility path".

## Decision

Redkiln-RS is a clean break: it shares no code, file format or runtime with the old redkiln. `redkiln-migrate` is a separate tool that consumes old `.bklg`/`.kb` content and emits the new typed layout in one reviewed commit with a differential report — never a live compatibility path. Process packs cross as content (ported into the new TOML format), not as code. The 101 CLI-behaviour test files and recorded bugs cross as black-box specifications and test cases, not as ported tests or carried-over fixes. `reconcile.rs` is the one code import, rewritten in the spike and validated on the corpus (F12). The old redkiln keeps running its own development until Redkiln-RS is self-hosting — it runs its own fix lane end to end — and that milestone, not a phased cutover, is the switch.

## Consequences

**Positive**
- The YAML byte-parity risk (R3) is dissolved: the new store owns its own canonical serialiser and normalises everything once in the migration commit, rather than carrying a permanent compatibility burden.
- The old redkiln's operational history (101 test files, recorded bugs) is preserved as specification and regression-test material without dragging its implementation forward.
- `reconcile.rs` carries forward only after being rewritten and validated on a real corpus (F12), rather than trusted as-is.

**Negative**
- The source does not name a cost or risk of the clean-break choice itself beyond the byte-parity point; no quantified downside of "two systems running in parallel until self-hosting" is given.

**What this forbids**
- Redkiln-RS may not share code, file format, or runtime with the old redkiln.
- `redkiln-migrate` may not become a runtime compatibility path; it produces one reviewed commit with a differential report, not an ongoing adapter.
- Old redkiln logic (other than `reconcile.rs`, rewritten and revalidated) may not port as code; only its tests and recorded bugs may cross, and only as specifications/test cases.
- The cutover may not happen on a calendar-driven "phased" basis; it is gated solely on Redkiln-RS running its own fix lane end to end (self-hosting).

## Evidence and confidence

- [measured] `reconcile.rs` was rewritten in the spike and validated on 360 commits / 74 slices (F12).
- [documented] The old redkiln's 101 CLI-behaviour test files and specific recorded bugs (lock/id collisions, `--stay`, RK-B0013, the `basis` flip, F25's per-AC gap) are named in §5 as crossing as specifications/test cases.
- [inferred] "Self-hosting is the switch" is stated as a milestone definition rather than measured; whether the milestone will in fact occur before or after some calendar-driven pressure to cut over is not addressed by the source.
- This decision does not license a claim that the migration tool (`redkiln-migrate`) has been built or validated yet — S-MIG (§8) is the gating spike for that, and it is out of scope for this ADR.
