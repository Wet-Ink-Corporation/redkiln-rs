//! **`repro_pair::read`: the item's committed repro pair, as the `repro_place.pair` envelope**
//! (ADR-0204 decision 2, proposed; added 2026-11-08, s8 repro-pair route).
//!
//! The composition root reads `.redkiln/intake/<item>.repro.{toml,rs}` from the run tree (the
//! frozen commit's bytes), and never fails: every failure is an `Absent` or `Invalid` value that
//! `place` later refuses by name. Both reads are capped at `REPRO_PAIR_MAX_BYTES`.

use std::path::Path;

use rk_cli::repro_pair::read;
use rk_types::repro_pair::{REPRO_PAIR_MAX_BYTES, ReproPairInput, ReproTarget};

/// A spec naming `words::repro_red` in crate `demo`.
const SPEC: &str = "# a comment the reader ignores\nkind = \"it\"\ncrate = \"demo\"\n\
                    module = \"words\"\ntest = \"repro_red\"\nexpect = \"the defect\"\n";

/// A source holding `fn repro_red(` once.
const SOURCE: &str = "#[test]\nfn repro_red() {\n    assert!(false, \"the defect\");\n}\n";

/// A tree with `.redkiln/intake/` and whichever of the pair's files are given.
#[cfg(test)]
fn tree(spec: Option<&[u8]>, source: Option<&[u8]>) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a temp tree");
    let intake = dir.path().join(".redkiln").join("intake");
    std::fs::create_dir_all(&intake).expect("intake");
    if let Some(bytes) = spec {
        std::fs::write(intake.join("rk-x.repro.toml"), bytes).expect("the spec writes");
    }
    if let Some(bytes) = source {
        std::fs::write(intake.join("rk-x.repro.rs"), bytes).expect("the source writes");
    }
    dir
}

/// The `Invalid` arm's `(path, why)`, or a panic naming what came instead.
#[cfg(test)]
fn invalid(input: ReproPairInput) -> (String, String) {
    match input {
        ReproPairInput::Invalid { path, why } => (path, why),
        other => panic!("an Invalid envelope, got {other:?}"),
    }
}

#[test]
fn both_files_read_as_a_present_pair() {
    let dir = tree(Some(SPEC.as_bytes()), Some(SOURCE.as_bytes()));
    match read(dir.path(), "rk-x") {
        ReproPairInput::Present(pair) => {
            assert_eq!(
                (pair.krate(), pair.module(), pair.test()),
                ("demo", "words", "repro_red")
            );
            assert_eq!(pair.target(), &ReproTarget::It);
            assert_eq!(pair.source(), SOURCE);
            assert_eq!(
                pair.spec_digest(),
                &rk_types::digest::sha256(SPEC.as_bytes()),
                "the digest is the .repro.toml as read"
            );
        }
        other => panic!("a Present pair, got {other:?}"),
    }
}

#[test]
fn a_missing_spec_is_absent_naming_both_paths() {
    let dir = tree(None, Some(SOURCE.as_bytes()));
    assert_eq!(
        read(dir.path(), "rk-x"),
        ReproPairInput::Absent {
            looked_for: vec![
                ".redkiln/intake/rk-x.repro.toml".to_owned(),
                ".redkiln/intake/rk-x.repro.rs".to_owned(),
            ],
        }
    );
}

#[test]
fn malformed_toml_is_invalid_naming_the_path_and_the_toml_error() {
    let dir = tree(Some(b"kind = \"it\"\ncrate = \n"), Some(SOURCE.as_bytes()));
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.toml");
    assert!(why.contains("TOML"), "the toml error is rendered: {why}");
}

#[test]
fn an_unknown_key_is_invalid() {
    let spec = format!("{SPEC}extra = 1\n");
    let dir = tree(Some(spec.as_bytes()), Some(SOURCE.as_bytes()));
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.toml");
    assert!(why.contains("extra"), "{why}");
}

#[test]
fn an_over_cap_source_is_invalid_naming_it() {
    let mut big = SOURCE.as_bytes().to_vec();
    big.resize(REPRO_PAIR_MAX_BYTES + 1, b' ');
    let dir = tree(Some(SPEC.as_bytes()), Some(&big));
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.rs");
    assert!(why.contains(&REPRO_PAIR_MAX_BYTES.to_string()), "{why}");
}

#[test]
fn a_source_at_exactly_the_cap_is_read() {
    let mut exact = SOURCE.as_bytes().to_vec();
    exact.resize(REPRO_PAIR_MAX_BYTES, b' ');
    let dir = tree(Some(SPEC.as_bytes()), Some(&exact));
    assert!(
        matches!(read(dir.path(), "rk-x"), ReproPairInput::Present(_)),
        "the cap is inclusive"
    );
}

#[test]
fn a_spec_without_its_source_is_invalid_naming_the_source() {
    let dir = tree(Some(SPEC.as_bytes()), None);
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.rs");
    assert!(
        why.contains("missing") || why.contains("not found"),
        "{why}"
    );
}

#[test]
fn a_source_lacking_the_test_is_invalid_naming_the_source() {
    let dir = tree(Some(SPEC.as_bytes()), Some(b"// no test here\n"));
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.rs");
    assert!(why.contains("fn repro_red("), "{why}");
}

#[test]
fn a_spec_rule_failure_is_invalid_naming_the_spec() {
    let spec = SPEC.replace("kind = \"it\"", "kind = \"doc\"");
    let dir = tree(Some(spec.as_bytes()), Some(SOURCE.as_bytes()));
    let (path, why) = invalid(read(dir.path(), "rk-x"));
    assert_eq!(path, ".redkiln/intake/rk-x.repro.toml");
    assert!(why.contains("doc"), "{why}");
}

/// Not a file at all: the intake directory missing entirely reads as absent.
#[test]
fn no_intake_directory_is_absent() {
    let dir = tempfile::tempdir().expect("a temp tree");
    assert!(matches!(
        read(Path::new(dir.path()), "rk-x"),
        ReproPairInput::Absent { .. }
    ));
}
