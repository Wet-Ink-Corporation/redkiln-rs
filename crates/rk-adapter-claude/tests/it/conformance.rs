//! **The Harness real row of AC-499** — `port_conformance_real_row_claude`: `rk-ports`' own
//! `conform_harness` body (the `conformance` feature module, over the DEV edge — RT-16 as
//! re-amended, s7c §7c.13, §7c.14) run over the REAL adapter with the fixture child as the
//! program, NOT a real `claude`. **Local-only, named-runner** (the register's Harness row; D17-5):
//! this test is what `PHASE_1_REAL_ROWS`' Claude row names, and it is never wired into CI as a
//! matrix row (that instrument is ARMED, §7c.22). The live probe (`live.rs`) runs the same body
//! over the real CLI before its own prints.

use rk_ports::conformance::conform_harness;
use rk_ports::fakes::VecStore;

use crate::support::{Markers, bind_probe, harness, ok_transcript, spawn_config};

/// The shared body, over the real adapter and the fixture child: the record on the Ok arm (the
/// instrument version present; a `Pinned` arm equal to the pin), the `Typed` carrying its schema
/// id; and, on the error arm, the store grew (AC-360's shape). Prints `PROBE_RAN_CONFORMANCE=1`
/// itself as its last statement.
///
/// RED (observed 2026-09-18) against the skeleton: the body's error-arm assertion — `a dispatch
/// terminated in Transport { kind: Spawn, … } without appending an event (AC-360, register
/// :5192-5197): the store held 0 events before and 0 after`.
#[test]
fn port_conformance_real_row_claude() {
    // the Ok arm
    let t = ok_transcript();
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(
        spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "conformance",
    );
    let h = harness(config);
    let mut store = VecStore::default();
    conform_harness(&h, &mut store, &spec);
    assert_eq!(
        markers.count(),
        1,
        "the body dispatched once through the real adapter"
    );
    // the error arm: a child that exits non-zero — the body asserts the store grew
    let markers = Markers::new();
    let (config, spec) = bind_probe(
        spawn_config(&markers, "exit:3"),
        30,
        wt.path(),
        "conformance",
    );
    let h = harness(config);
    let mut store = VecStore::default();
    conform_harness(&h, &mut store, &spec);
    assert_eq!(markers.count(), 1);
}
