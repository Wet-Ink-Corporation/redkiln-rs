//! `schema_snapshot` — the golden over every exported schema.
//!
//! **This test is the stated exception to golden constraint 12, and it is not evidence.** A golden
//! snapshot has no meaningful red state: its first acceptance is a deliberate, one-time
//! rubber-stamp, and what it pins afterwards is *change*, not *correctness*. A changed snapshot is
//! a schema change and is reviewed as one. The four discriminating tests beside it —
//! `step_error_is_total`, `no_oneof_at_any_node`, `union_and_optional_caps`,
//! `repro_rejects_prose_only` and `companion_kind_has_registered_schema` — are what discharge the
//! criteria. ADR-0084's own residual stands: a reviewer can rubber-stamp `cargo insta accept`, and
//! the guard against that is convention, not mechanism.

use insta::Settings;
use rk_types::companion::CompanionKind;
use rk_types::registry;

/// Snapshots live where Checkpoint 5's `test -d crates/rk-types/tests/it/snapshots` guard looks,
/// and the test pins the path rather than inheriting it — so the guard and the test can never name
/// different directories (PIA-034).
fn settings() -> Settings {
    let mut s = Settings::clone_current();
    s.set_snapshot_path("snapshots");
    s.set_prepend_module_to_snapshot(false);
    s
}

#[test]
fn schema_snapshot() {
    let all = registry::export_all();

    // A snapshot suite over an empty set passes having pinned nothing.
    assert!(
        !all.is_empty(),
        "registry::EXPORTED is empty — this snapshot would pin nothing"
    );
    assert_eq!(
        all.len(),
        registry::EXPORTED.len(),
        "export_all() must cover the whole registry, or a schema ships unsnapshotted"
    );

    settings().bind(|| {
        for (name, schema) in all {
            insta::assert_json_snapshot!(format!("exported__{name}"), schema);
        }
    });
}

#[test]
fn companion_schema_snapshot() {
    settings().bind(|| {
        for kind in CompanionKind::ALL {
            let name = kind.wire_name();
            let schema = kind
                .registered_schema()
                .expect("every kind has a registered sidecar");
            // `_ledger` and `ledger` differ by one character, so the snapshot file name is
            // disambiguated explicitly rather than by the wire name alone.
            let slug = if name.starts_with('_') {
                format!("companion_{name}")
            } else {
                format!("companion_bare_{name}")
            };
            insta::assert_json_snapshot!(slug, schema);
        }
    });
}
