//! **The repro pair** (`.redkiln/intake/<item>.repro.{toml,rs}`; ADR-0204 decision 2, proposed).
//!
//! *(Added 2026-11-08, s8 repro-pair route.)* `ReproPairSpec` is the `.repro.toml` as written,
//! `ReproPair` the validated pair (constructed only through `ReproPair::new` or a re-validating
//! deserialise), and `ReproPairInput` the envelope `rk-cli` binds as `repro_place.pair`, so an
//! absent or invalid pair refuses at `place`, by name. Each `ReproPairError` variant is pinned with
//! its context fields.

use rk_types::digest::sha256;
use rk_types::repro_pair::{
    REPRO_PAIR_MAX_BYTES, ReproPair, ReproPairError, ReproPairInput, ReproPairSpec, ReproTarget,
};
use serde_json::json;

/// The shipped `rk-fix-doctor-margin.repro.toml`'s values.
fn shipped_spec() -> ReproPairSpec {
    ReproPairSpec {
        kind: "it".to_owned(),
        krate: "rk-cli".to_owned(),
        module: "doctor".to_owned(),
        test: "s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error".to_owned(),
        expect: "AC-377: a fixture config with no margin must fail (no default)".to_owned(),
        file: None,
    }
}

/// A source that holds `fn <test>(` once.
fn source_for(test: &str) -> String {
    format!("#[test]\nfn {test}() {{\n    assert!(false, \"red\");\n}}\n")
}

/// A `unit` spec appending module `margin_repro` to `crates/rk-cli/src/doctor.rs`.
fn unit_spec() -> ReproPairSpec {
    ReproPairSpec {
        kind: "unit".to_owned(),
        krate: "rk-cli".to_owned(),
        module: "doctor::margin_repro".to_owned(),
        test: "red".to_owned(),
        expect: "boom".to_owned(),
        file: Some("doctor.rs".to_owned()),
    }
}

/// `new` over `spec` and a source holding the spec's test once.
fn build(spec: ReproPairSpec) -> Result<ReproPair, ReproPairError> {
    let source = source_for(&spec.test);
    ReproPair::new(spec, source, b"spec bytes")
}

#[test]
fn the_shipped_pair_shape_validates() {
    // The `.repro.toml`'s shape, as JSON (this crate has no TOML reader): `crate` is the wire name.
    let spec: ReproPairSpec = serde_json::from_value(json!({
        "kind": "it",
        "crate": "rk-cli",
        "module": "doctor",
        "test": "s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error",
        "expect": "AC-377: a fixture config with no margin must fail (no default)"
    }))
    .expect("the shipped spec's shape deserialises");
    assert_eq!(spec, shipped_spec());
    let source = source_for(&spec.test);
    let pair = ReproPair::new(spec, source.clone(), b"kind = \"it\"\n").expect("validates");
    assert_eq!(pair.krate(), "rk-cli");
    assert_eq!(pair.module(), "doctor");
    assert_eq!(
        pair.test(),
        "s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error"
    );
    assert_eq!(
        pair.expected(),
        "AC-377: a fixture config with no margin must fail (no default)"
    );
    assert_eq!(pair.target(), &ReproTarget::It);
    assert_eq!(pair.source(), source);
    assert_eq!(pair.source_digest(), &sha256(source.as_bytes()));
    assert_eq!(pair.spec_digest(), &sha256(b"kind = \"it\"\n"));
}

#[test]
fn a_unit_pair_carries_its_src_relative_file() {
    let pair = build(unit_spec()).expect("validates");
    match pair.target() {
        ReproTarget::Unit { file } => assert_eq!(file.as_str(), "doctor.rs"),
        ReproTarget::It => panic!("a unit spec gives a Unit target"),
    }
    assert_eq!(pair.module(), "doctor::margin_repro");
}

#[test]
fn an_unknown_kind_is_refused_naming_it() {
    let spec = ReproPairSpec {
        kind: "doc".to_owned(),
        ..shipped_spec()
    };
    assert_eq!(
        build(spec),
        Err(ReproPairError::UnknownKind {
            found: "doc".to_owned()
        })
    );
}

#[test]
fn a_unit_pair_without_a_file_is_refused() {
    let spec = ReproPairSpec {
        file: None,
        ..unit_spec()
    };
    assert_eq!(build(spec), Err(ReproPairError::UnitNeedsFile));
}

#[test]
fn an_it_pair_with_a_file_is_refused_naming_it() {
    let spec = ReproPairSpec {
        file: Some("lib.rs".to_owned()),
        ..shipped_spec()
    };
    assert_eq!(
        build(spec),
        Err(ReproPairError::ItTakesNoFile {
            file: "lib.rs".to_owned()
        })
    );
}

#[test]
fn a_bad_crate_name_is_refused_naming_it() {
    for found in ["", "rk cli", "../rk-cli", "RK"] {
        let spec = ReproPairSpec {
            krate: found.to_owned(),
            ..shipped_spec()
        };
        assert_eq!(
            build(spec),
            Err(ReproPairError::BadCrate {
                found: found.to_owned()
            }),
            "{found:?}"
        );
    }
}

#[test]
fn a_bad_module_is_refused_naming_it() {
    // `it`: one identifier, so a path is refused; `unit`: a path, so a bad segment is refused.
    let it = ReproPairSpec {
        module: "doctor::inner".to_owned(),
        ..shipped_spec()
    };
    assert_eq!(
        build(it),
        Err(ReproPairError::BadModule {
            found: "doctor::inner".to_owned()
        })
    );
    let unit = ReproPairSpec {
        module: "doctor::".to_owned(),
        ..unit_spec()
    };
    assert_eq!(
        build(unit),
        Err(ReproPairError::BadModule {
            found: "doctor::".to_owned()
        })
    );
}

#[test]
fn a_bad_test_name_is_refused_naming_it() {
    let spec = ReproPairSpec {
        test: "1st-test".to_owned(),
        ..shipped_spec()
    };
    assert_eq!(
        ReproPair::new(spec, source_for("x"), b""),
        Err(ReproPairError::BadTest {
            found: "1st-test".to_owned()
        })
    );
}

#[test]
fn a_bad_file_is_refused_naming_it() {
    for found in [
        "/etc/doctor.rs",
        "../lib.rs",
        "a\\b.rs",
        "doctor.txt",
        "",
        "a//b.rs",
    ] {
        let spec = ReproPairSpec {
            file: Some(found.to_owned()),
            ..unit_spec()
        };
        assert_eq!(
            build(spec),
            Err(ReproPairError::BadFile {
                found: found.to_owned()
            }),
            "{found:?}"
        );
    }
}

#[test]
fn an_empty_expect_is_refused() {
    let spec = ReproPairSpec {
        expect: "  ".to_owned(),
        ..shipped_spec()
    };
    assert_eq!(build(spec), Err(ReproPairError::EmptyExpect));
}

#[test]
fn an_over_cap_source_is_refused_naming_its_size() {
    let spec = shipped_spec();
    let mut source = source_for(&spec.test);
    source.push_str(&" ".repeat(REPRO_PAIR_MAX_BYTES));
    let bytes = source.len();
    assert_eq!(
        ReproPair::new(spec, source, b""),
        Err(ReproPairError::SourceTooLarge {
            bytes,
            cap: REPRO_PAIR_MAX_BYTES
        })
    );
}

#[test]
fn a_source_without_exactly_one_test_fn_is_refused_naming_the_count() {
    let spec = shipped_spec();
    let test = spec.test.clone();
    assert_eq!(
        ReproPair::new(spec.clone(), "// nothing here\n".to_owned(), b""),
        Err(ReproPairError::SourceLacksTest {
            test: test.clone(),
            found: 0
        })
    );
    let twice = format!("{}{}", source_for(&test), source_for(&test));
    assert_eq!(
        ReproPair::new(spec, twice, b""),
        Err(ReproPairError::SourceLacksTest { test, found: 2 })
    );
}

#[test]
fn every_refusal_renders_a_reason() {
    let refusals = [
        ReproPairError::UnknownKind {
            found: "doc".to_owned(),
        },
        ReproPairError::UnitNeedsFile,
        ReproPairError::ItTakesNoFile {
            file: "a.rs".to_owned(),
        },
        ReproPairError::BadCrate {
            found: "A".to_owned(),
        },
        ReproPairError::BadModule {
            found: "a::".to_owned(),
        },
        ReproPairError::BadTest {
            found: "1".to_owned(),
        },
        ReproPairError::BadFile {
            found: "..".to_owned(),
        },
        ReproPairError::EmptyExpect,
        ReproPairError::SourceTooLarge { bytes: 9, cap: 8 },
        ReproPairError::SourceLacksTest {
            test: "t".to_owned(),
            found: 0,
        },
    ];
    for refusal in refusals {
        assert!(!refusal.to_string().is_empty(), "{refusal:?}");
    }
}

#[test]
fn a_pair_round_trips_and_a_tampered_one_is_refused_on_decode() {
    let pair = build(unit_spec()).expect("validates");
    let wire = serde_json::to_value(&pair).expect("serialises");
    let back: ReproPair = serde_json::from_value(wire.clone()).expect("decodes");
    assert_eq!(back, pair);

    // Deserialising re-validates: a decoded pair is never one `new` would refuse.
    let mut tampered = wire;
    tampered["spec"]["file"] = json!("../escape.rs");
    let err = serde_json::from_value::<ReproPair>(tampered).expect_err("re-validated");
    assert!(err.to_string().contains("../escape.rs"), "{err}");
}

#[test]
fn the_envelope_round_trips_each_state() {
    let present = ReproPairInput::Present(build(shipped_spec()).expect("validates"));
    let absent = ReproPairInput::Absent {
        looked_for: vec![".redkiln/intake/x.repro.toml".to_owned()],
    };
    let invalid = ReproPairInput::Invalid {
        path: ".redkiln/intake/x.repro.toml".to_owned(),
        why: "bad".to_owned(),
    };
    for input in [present, absent, invalid] {
        let wire = serde_json::to_value(&input).expect("serialises");
        assert!(wire.get("state").is_some(), "tagged by `state`: {wire}");
        let back: ReproPairInput = serde_json::from_value(wire).expect("decodes");
        assert_eq!(back, input);
    }
}
