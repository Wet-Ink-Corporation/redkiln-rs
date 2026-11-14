/// *(s8j section 8j.5 step 3: the reserved fix item `rk-fix-doctor-margin-5`'s repro, minted on
/// 2026-11-14 by the route back after a failed first real run (D21-163, D21-179). It is
/// `rk-fix-doctor-margin-4`'s repro unchanged but for this note: one test asserts all three of the
/// BugReport's cases (plan `spikes/plans/s8-repro-aware-implement-2026-11-08.md` Decision D13 A),
/// so a fix for one case of three stays red. It is appended to `crates/rk-cli/tests/it/doctor.rs`
/// in a throwaway tree only, and never committed into the crate on the integration branch.
/// ADR-0204 (proposed): the fix lane's `minimal-repro@2` `place` step appends it in the run tree and
/// commits it on `rk-run/rk-fix-doctor-margin-5`, where it lands with the fix.)*
///
/// **AC-377's doctor half over a malformed `[margins]`.** The run's loader refuses a pack whose
/// `[margins]` is absent, lacks a window, or gives a window a non-integer value (`verbs/run.rs`'s
/// `PackExtras::margins` and both `Margins` members are required `u16`s, with no
/// `#[serde(default)]`), and AC-377's Verified-by names *"a fixture config with no margin that must
/// fail (no default)"*. `doctor` must reach the same verdict on each: `RK-D-MARGIN` reports an
/// error that names what is missing or malformed.
#[test]
fn s8j_repro_a_malformed_margins_table_is_a_doctor_error() {
    let rule = RULES
        .iter()
        .find(|r| r.id == "RK-D-MARGIN")
        .expect("RK-D-MARGIN is in RULES");
    let margins = "\n[margins]\nfive_hour = 10\nseven_day = 10\n";
    // (case, the edited `[margins]` text, the word each error finding must name)
    let cases = [
        ("the table absent", "\n", "margins"),
        ("a window missing", "\n[margins]\nfive_hour = 10\n", "seven_day"),
        (
            "a non-integer window",
            "\n[margins]\nfive_hour = 10\nseven_day = \"ten\"\n",
            "seven_day",
        ),
    ];
    for (case, edited, names) in cases {
        let dir = materialise("clean");
        let pack = dir.path().join(".redkiln").join("packs").join("demo.toml");
        let src = std::fs::read_to_string(&pack).expect("the clean fixture's pack reads");
        assert!(
            src.contains(margins),
            "PREMISE: the clean fixture's pack declares `[margins] five_hour = 10, seven_day = 10`; \
             re-point this repro"
        );
        std::fs::write(&pack, src.replace(margins, edited)).expect("the edited pack writes");

        let found = (rule.check)(&ctx_for(dir.path()));
        let named: Vec<_> = found.iter().filter(|f| f.detail.contains(names)).collect();
        assert!(
            !named.is_empty()
                && named
                    .iter()
                    .all(|f| f.severity == rk_cli::doctor::Severity::Error),
            "AC-377: a fixture config with no margin must fail (no default) and doctor reaches the \
             same verdict as the load; case `{case}`: no error naming `{names}`, RK-D-MARGIN found \
             {found:?}"
        );
    }
}
