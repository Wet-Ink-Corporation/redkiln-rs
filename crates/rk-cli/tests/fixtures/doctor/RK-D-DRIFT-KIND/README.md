# `RK-D-DRIFT-KIND` - the rejecting fixture

**Severity:** `error` · **Pins:** AC-127's *Verified by* ("a `doctor` rule-table row
distinguishing the two drift kinds"), AC-067 and ADR-0025's first forbids bullet (`s8f` §8f.9;
ADR-0189 decision 13, proposed).

**The defect: arm (a).** In one entry (after one `reconcile_started`), a vouched root's own
change is recorded as `drift_advisory` (`root = "Findings"`, `upstream = null`), and a
`render.structured-drift` gate naming that root is opened after it: an advisory promoted to a
blocking gate. The rule reads the blocking property from `rk_types::drift::Drift::blocks()`.

Arm (b) (a `structured_drift` with no drift gate after it in its entry) and the control (a real
lane journal carrying both kinds, each handled correctly) are `doctor_drift`'s named tests;
this fixture adds no `#[test]` to `doctor.rs` (§8f.9).

Generated 2026-09-29 by S8F-P8's `S8F-P8-gen-fixtures.py`, LF only.
