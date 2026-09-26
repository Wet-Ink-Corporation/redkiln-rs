//! The single integration binary for `rk-types` (ADR-0083: at most one per crate under test,
//! structured as `tests/it/main.rs` with `mod` declarations for the cases, never one file per case).

mod companion_kinds;
mod digest_compute;
mod dispatch_record;
mod event_kinds;
mod fixtures;
mod repro_ingest;
mod review_gate;
mod schema_caps;
mod schema_no_oneof;
mod schema_snapshot;
mod step_error_is_total;
mod support;
mod usage;
mod verb_outputs;
mod verify_types;
