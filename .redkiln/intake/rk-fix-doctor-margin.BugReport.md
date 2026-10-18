# BugReport: rk-fix-doctor-margin

## What is wrong

`redkiln doctor` reports no `RK-D-MARGIN` finding for a pack whose `[margins]` table is absent, lacks one of its two
windows (`five_hour`, `seven_day`), or gives a window a non-integer value. `redkiln run` refuses the same pack at
load, because both members of its margins table are required and neither has a default.

So `doctor` reports green on a configuration that `run` refuses.

## Where

`rk-cli`, `doctor::rules::margin` (`crates/rk-cli/src/doctor/rules.rs`), the rule `RK-D-MARGIN`.

## The contract it breaks

AC-377 (`references/2026-09-08-acceptance-criteria.md`): the configuration *"is refused as a configuration error
before any dispatch AND `redkiln doctor` reaches the same verdict without dispatching a model call"*. Its Verified-by
includes *"a fixture config with no margin that must fail (no default)"*.

## How to reproduce

Take `crates/rk-cli/tests/fixtures/doctor/clean/`, remove `seven_day = 10` from its pack's `[margins]` table, and
run `RK-D-MARGIN` over it. Expected: one error finding naming `seven_day`. Actual: no finding.

The failing test is committed beside this file, never in the crate:

- `.redkiln/intake/rk-fix-doctor-margin.repro.rs`, the test, appended to `crates/rk-cli/tests/it/doctor.rs`;
- `.redkiln/intake/rk-fix-doctor-margin.repro.toml`, its `kind`, `crate`, `module`, `test` and the `expect`
  substring of its failure.

Run it with `cargo test -p rk-cli --test it -- doctor::s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error
--exact` in a tree where the test has been appended.
