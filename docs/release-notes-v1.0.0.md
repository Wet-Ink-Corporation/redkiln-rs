# redkiln-rs v1.0.0 (draft)

*Draft, agent-written 2026-11-24 for the GitHub release (`runbook/phase-1/s8-exit-criterion.md`, §Landing step 4). It
records no pass. Fill the two `CP8` references below from Ryan's Checkpoint 8 pass lines before the release is
created; until then this file claims nothing.*

**Phase 1 exits (M5).** v1.0.0 is the tree that passed Checkpoint 8 (`CP8` record: PROGRESS row 8 and the DECISIONS
entry, to be filled), plus the workspace version bump and nothing else (`kb-decision-ed8ddcbb85`, `wi-cecc23c1ff`).

## What it is

`redkiln` is a single Rust binary that owns the loop: a deterministic kernel (stage selection, gate evaluation,
budgets, retry, replay) that dispatches one harness process per activity. The design is
`references/2026-09-08-architecture-design.md`; the decisions are the ADRs under `references/adr/`.

## What v1.0.0 certifies

Exactly what `runbook/INDEX.md`'s M5 row states (hybrid (c), `kb-decision-a96d6b8999`, #26): **the Step 8 exit checks,
as run and recorded at Checkpoint 8**, and nothing else.

- **(1) Self-hosting, fix lane** (AC-473): one real fix-lane run, `rk-fix-doctor-margin-8`, closed end to end
  through the Rust toolchain adapter.
- **(1b) Self-hosting, story lane** (AC-521): one real story-lane run, `rk-story-harden-spike-pack`, authored a
  Phase-2 story.
- **(2) Replay** (AC-184, SM-124) and the kernel replay test.
- **(3) Deterministic resume across a real kill** at more than one step.
- **(4) The determinism ban list fails the build.**
- **(6) `redkiln lint --facet-stability`** (AC-071) and **`--facet-provenance`** (AC-520) pass.
- **(7) Every dispatch event carries a cost row** (AC-394), and headroom is reported only as a batch delta.
- **Checkpoint 8's mechanical lines**, including `ci.yml` green on both OS legs.

## What v1.0.0 does not claim

- **No AC that no Step 8 check runs.** Those residual ACs are the Phase-2 entry backlog; their disposition table
  follows via #63 and #66. Passing M5 says nothing about them either way.
- **The mutation-pinned verify state (AC-334, ARMED) is not claimed.** In the exit runs a verify-stage AC closed on
  the human `verify.ac-discriminates` answer alone; that state is not reachable through the real harness, and its
  wiring is Phase 2's (`runbook/INDEX.md`, M5 scope note; D21-159; #33).
- **A spec's verification command is not checked for matching any test.** `rk-fix-doctor-margin-8`'s spec
  (`f10871e`) gave AC-4 the filter `doctor::rk_d_margin`, which matches no test, so that verification passes
  vacuously. Verify refusing a zero-match filter is v1.x work (#37). AC-4's floor behaviour itself is tested by
  `margin_floor_of_two_is_lawful` (`47fb9de`).
- **The design is not final.** S-H1c and P8's unrun arms still gate that label (Checkpoint 8's standing lines).
- **No facet "leaves the model".** No facet clears METHOD-P §1.4's 0.95 bar (F39).
- **Not the hardening backlog.** The held PRs (#148, #149, #150, #151, #153, #154, and #147 for the runbook) are not
  in v1.0.0; they land afterwards as v1.0.1. Post-M5 hardening (#37) and Phases 2 to 4 (#38 to #40) are later work.
- **Not a measured-everywhere build.** The corpus-bound suite (`--features corpus`) and the live-dispatch tests
  (`#[ignore]` with reasons) run only on the operator's machine, not in CI. The Step 8 runs were made on one Windows
  machine with a pinned Claude Code CLI.
- **No published crates.** Every crate is `publish = false`; the version number names this tree, not a crates.io API.

## Known issues carried into v1.0.x

#155 (a guard-3 denial cannot be read back), #162 (the margin floor test, now joined by `margin_floor_of_two_is_lawful`),
#163 (a `step_refused` halt is never cleared), #164 (fixture `run` spawns are slow under full-suite load on Windows),
and the vacuous-filter issue above. See #37.

## Landing

`main` takes a fresh `--no-ff` merge of `build/phase-0-to-1` made on `353ea01` (`kb-decision-49a1fa0545`), tagged
`v1.0.0` (annotated) on `main`.
