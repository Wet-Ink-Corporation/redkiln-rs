
/// *(s8j section 8j.5 step 3: the reserved fix item `rk-fix-doctor-margin`'s repro. It is appended
/// to `crates/rk-cli/tests/it/doctor.rs` in a throwaway tree only, and never committed into the
/// crate.)*
///
/// **AC-377's doctor half over an absent window.** The run's loader refuses a pack whose
/// `[margins]` lacks a window (`verbs/run.rs`'s `Margins` has both members required, with no
/// `#[serde(default)]`), and AC-377's Verified-by names *"a fixture config with no margin that must
/// fail (no default)"*. `doctor` must reach the same verdict: `RK-D-MARGIN` reports an error that
/// names the missing window.
#[test]
fn s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error() {
    let rule = RULES
        .iter()
        .find(|r| r.id == "RK-D-MARGIN")
        .expect("RK-D-MARGIN is in RULES");
    let dir = materialise("clean");
    let pack = dir.path().join(".redkiln").join("packs").join("demo.toml");
    let src = std::fs::read_to_string(&pack).expect("the clean fixture's pack reads");
    assert!(
        src.contains("\nseven_day = 10\n"),
        "PREMISE: the clean fixture's pack declares `[margins] seven_day = 10`; re-point this repro"
    );
    std::fs::write(&pack, src.replace("\nseven_day = 10\n", "\n")).expect("the edited pack writes");

    let found = (rule.check)(&ctx_for(dir.path()));
    let named: Vec<_> = found
        .iter()
        .filter(|f| f.detail.contains("seven_day"))
        .collect();
    assert!(
        !named.is_empty()
            && named
                .iter()
                .all(|f| f.severity == rk_cli::doctor::Severity::Error),
        "AC-377: a fixture config with no margin must fail (no default) and doctor reaches the same \
         verdict as the load; [margins] lacks seven_day and RK-D-MARGIN found {found:?}"
    );
}
