//! AC-499's Node real row — **the same body the fake row runs** (RT-16 as re-amended, s7c §7c.14):
//! `rk_ports::conformance::conform_toolchain`, from `rk-ports`' `conformance` feature module over
//! this crate's dev edge. No body of this crate's own.
//!
//! The body is a required core plus a capability arm. The core — `detect`, `build` / `test(&[])`
//! / `lint` over the fixture project **with the count assertion the row parameterises** (2: the
//! two passing tests, the decoy among them; the failing and slow tests are skip-marked and
//! reachable only by `-t`), then `classify` over the row's own `(path, classification)` pairs —
//! runs over the real adapter; the arm takes the tokens this row's registry declares unsupported
//! (`change_summary`, D17-9) and asserts `Err(Unsupported { capability })` naming it, where 7a's
//! unchanged `.expect("change_summary(range)")` would panic on this row.
//!
//! **The spawn marker** (§7c.18): the map carries `RK_SPAWN_MARKER_DIR`, and the fixture's
//! `filterable passes` writes one file there per run, so an adapter that answered `Ok(2)` without
//! running `vitest` is red on the marker count as well as on the count.

use rk_adapter_toolchain_node::registry;
use rk_ports::conformance::conform_toolchain;
use rk_ports::toolchain::Classification;

use crate::support::{GENEROUS, adapter_with_env, fixture_project};

/// **AC-499 (Node row).** The shared body over the real adapter and the fixture project.
#[test]
fn port_conformance_real_row_node() {
    let scratch = tempfile::tempdir().expect("scratch");
    let markers = tempfile::tempdir().expect("marker dir");
    let tc = adapter_with_env(
        rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
        GENEROUS,
        &fixture_project(),
        &[(
            "RK_SPAWN_MARKER_DIR",
            markers.path().to_string_lossy().into_owned(),
        )],
        scratch.path(),
    );

    // The tokens come from the registry the adapter answers from (D17-9), never a literal here.
    let unsupported = registry::unsupported_tokens().expect("the shipped registry loads");
    let arm: Vec<&str> = unsupported.iter().map(String::as_str).collect();

    conform_toolchain(
        &tc,
        &[
            ("test/suite.test.ts", Classification::Text),
            ("Content/Maps/Lobby.umap", Classification::Binary),
            ("pnpm-lock.yaml", Classification::Generated),
        ],
        2,
        &arm,
    );

    let count = std::fs::read_dir(markers.path())
        .expect("readable")
        .filter(|e| {
            e.as_ref()
                .is_ok_and(|e| e.file_name().to_string_lossy().ends_with(".marker"))
        })
        .count();
    assert!(
        count >= 1,
        "the body's `test(&[])` reported 2 but no spawn marker appeared: the fixture's \
         `filterable passes` did not run under this adapter (Ok without a spawn — §7c.18)"
    );
}
