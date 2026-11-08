//! **`repro_place`** (ADR-0204 decisions 1-3, proposed; added 2026-11-08, s8 repro-pair route).
//!
//! The verb reads the `pair` row input (the `ReproPairInput` envelope `rk-cli` binds), appends the
//! pair's `.repro.rs` to its target under the project root, and answers a `ReproPlacement` whose
//! `command` is the one plain command that runs the placed test. Every precondition refusal is
//! `Unsupported` (a missing pair does not improve when repeated) and writes nothing. Each test runs
//! over its own temp project root, never over the workspace running the suite.

use std::path::Path;

use rk_adapter_toolchain_rust::RustToolchain;
use rk_types::digest::sha256;
use rk_types::error::{StepError, TransportKind};
use rk_types::repro_pair::{ReproPair, ReproPairInput, ReproPairSpec};
use rk_types::verb_output::ReproPlacement;

use crate::support::{config_with, minimal_env, run_verb};

/// The `it` module file's bytes before placement.
const MODULE_BEFORE: &str = "use super::*;\n\n#[test]\nfn already_here() {}\n";

/// The repro's bytes: one red test.
const REPRO: &str = "#[test]\nfn repro_red() {\n    assert!(false, \"the defect\");\n}\n";

/// A temp project root holding `crates/demo/tests/it/words.rs` and `crates/demo/src/lib.rs`.
#[cfg(test)]
fn project() -> tempfile::TempDir {
    let root = tempfile::tempdir().expect("a temp project root");
    let it = root.path().join("crates/demo/tests/it");
    std::fs::create_dir_all(&it).expect("tests/it");
    std::fs::write(it.join("words.rs"), MODULE_BEFORE).expect("the module file");
    let src = root.path().join("crates/demo/src");
    std::fs::create_dir_all(&src).expect("src");
    std::fs::write(src.join("lib.rs"), "pub fn f() {}\n").expect("lib.rs");
    root
}

/// The adapter over `root`.
#[cfg(test)]
fn adapter(root: &Path, scratch: &Path) -> RustToolchain {
    RustToolchain::new(config_with(
        "cargo".into(),
        std::time::Duration::from_secs(60),
        root,
        minimal_env(),
        scratch,
    ))
    .expect("the shipped registry loads")
}

/// A spec for `kind` with the given `module` and `file`.
fn spec(kind: &str, module: &str, file: Option<&str>) -> ReproPairSpec {
    ReproPairSpec {
        kind: kind.to_owned(),
        krate: "demo".to_owned(),
        module: module.to_owned(),
        test: "repro_red".to_owned(),
        expect: "the defect".to_owned(),
        file: file.map(str::to_owned),
    }
}

/// The `pair` input carrying a present pair.
#[cfg(test)]
fn present(spec: ReproPairSpec) -> serde_json::Value {
    let pair = ReproPair::new(spec, REPRO.to_owned(), b"spec").expect("a valid pair");
    serde_json::json!({ "pair": ReproPairInput::Present(pair) })
}

/// Place `inputs` over `root` and decode the placement. A scratch or decode failure is reported as
/// a `Transport` error, which every test's assertion then shows.
#[cfg(test)]
fn place(root: &Path, inputs: &serde_json::Value) -> Result<ReproPlacement, StepError> {
    let transport = |kind, detail: String| StepError::Transport { kind, detail };
    let scratch = tempfile::tempdir().map_err(|e| transport(TransportKind::Io, e.to_string()))?;
    let out = run_verb(&adapter(root, scratch.path()), "repro_place", inputs)?;
    assert_eq!(out.kind, "ReproPlacement");
    serde_json::from_value(out.value).map_err(|e| transport(TransportKind::Decode, e.to_string()))
}

/// The refusal's text, which must be an `Unsupported`.
#[cfg(test)]
fn unsupported(result: Result<ReproPlacement, StepError>) -> String {
    match result {
        Err(StepError::Unsupported { capability }) => capability,
        other => panic!("a precondition refusal is Unsupported (never retried): {other:?}"),
    }
}

#[test]
fn an_it_pair_is_appended_to_its_module_file_and_named_by_command() {
    let root = project();
    let placed = place(root.path(), &present(spec("it", "words", None))).expect("placed");
    assert_eq!(
        placed,
        ReproPlacement {
            path: "crates/demo/tests/it/words.rs".to_owned(),
            test: "words::repro_red".to_owned(),
            command: "cargo test -p demo --test it -- words::repro_red --exact".to_owned(),
            expect: "the defect".to_owned(),
            source_sha256: sha256(REPRO.as_bytes()).as_str().to_owned(),
            spec_sha256: sha256(b"spec").as_str().to_owned(),
            appended: true,
        }
    );
    let bytes = std::fs::read_to_string(root.path().join(&placed.path)).expect("reads");
    assert_eq!(bytes, format!("{MODULE_BEFORE}\n{REPRO}"));
}

#[test]
fn a_unit_pair_is_appended_to_its_src_file_under_lib() {
    let root = project();
    let placed =
        place(root.path(), &present(spec("unit", "repro", Some("lib.rs")))).expect("placed");
    assert_eq!(placed.path, "crates/demo/src/lib.rs");
    assert_eq!(placed.test, "repro::repro_red");
    assert_eq!(
        placed.command,
        "cargo test -p demo --lib -- repro::repro_red --exact"
    );
    let bytes = std::fs::read_to_string(root.path().join(&placed.path)).expect("reads");
    assert_eq!(bytes, format!("pub fn f() {{}}\n\n{REPRO}"));
}

#[test]
fn a_second_place_is_idempotent() {
    let root = project();
    let inputs = present(spec("it", "words", None));
    let first = place(root.path(), &inputs).expect("placed");
    let after_first = std::fs::read(root.path().join(&first.path)).expect("reads");
    let second = place(root.path(), &inputs).expect("re-placed");
    assert!(!second.appended, "a re-entry writes nothing");
    assert_eq!(
        ReproPlacement {
            appended: true,
            ..second
        },
        first,
        "the same placement"
    );
    assert_eq!(
        std::fs::read(root.path().join(&first.path)).expect("reads"),
        after_first,
        "byte-identical"
    );
}

#[test]
fn a_prior_different_fn_of_the_test_name_is_refused() {
    let root = project();
    let module = root.path().join("crates/demo/tests/it/words.rs");
    let prior = format!("{MODULE_BEFORE}\nfn repro_red() {{ /* another body */ }}\n");
    std::fs::write(&module, &prior).expect("writes");
    let why = unsupported(place(root.path(), &present(spec("it", "words", None))));
    assert!(
        why.contains("repro_place") && why.contains("crates/demo/tests/it/words.rs"),
        "{why}"
    );
    assert!(why.contains("fn repro_red("), "{why}");
    assert_eq!(std::fs::read_to_string(&module).expect("reads"), prior);
}

#[test]
fn a_missing_target_is_refused_naming_it() {
    let root = project();
    let why = unsupported(place(root.path(), &present(spec("it", "nowhere", None))));
    assert!(
        why.contains("repro_place") && why.contains("crates/demo/tests/it/nowhere.rs"),
        "{why}"
    );
    assert!(!root.path().join("crates/demo/tests/it/nowhere.rs").exists());
}

#[test]
fn an_absent_pair_is_refused_naming_the_intake_paths() {
    let root = project();
    let inputs = serde_json::json!({ "pair": ReproPairInput::Absent {
        looked_for: vec![
            ".redkiln/intake/x.repro.toml".to_owned(),
            ".redkiln/intake/x.repro.rs".to_owned(),
        ],
    }});
    let why = unsupported(place(root.path(), &inputs));
    assert!(
        why.contains("repro_place")
            && why.contains(".redkiln/intake/x.repro.toml")
            && why.contains(".redkiln/intake/x.repro.rs"),
        "{why}"
    );
    assert_eq!(
        std::fs::read_to_string(root.path().join("crates/demo/tests/it/words.rs")).expect("reads"),
        MODULE_BEFORE,
        "nothing is written"
    );
}

#[test]
fn an_invalid_pair_is_refused_with_its_reason() {
    let root = project();
    let inputs = serde_json::json!({ "pair": ReproPairInput::Invalid {
        path: ".redkiln/intake/x.repro.toml".to_owned(),
        why: "kind `doc` is neither `it` nor `unit`".to_owned(),
    }});
    let why = unsupported(place(root.path(), &inputs));
    assert!(
        why.contains(".redkiln/intake/x.repro.toml") && why.contains("kind `doc`"),
        "{why}"
    );
    assert_eq!(
        std::fs::read_to_string(root.path().join("crates/demo/tests/it/words.rs")).expect("reads"),
        MODULE_BEFORE,
        "nothing is written"
    );
}

#[test]
fn no_pair_input_is_refused_naming_it() {
    let root = project();
    let why = unsupported(place(root.path(), &serde_json::json!({})));
    assert!(
        why.contains("repro_place") && why.contains("`pair`"),
        "{why}"
    );
}

#[cfg(unix)]
#[test]
fn a_target_symlinked_outside_the_root_is_refused() {
    let root = project();
    let outside = tempfile::tempdir().expect("outside");
    let elsewhere = outside.path().join("words.rs");
    std::fs::write(&elsewhere, MODULE_BEFORE).expect("writes");
    let module = root.path().join("crates/demo/tests/it/words.rs");
    std::fs::remove_file(&module).expect("removed");
    std::os::unix::fs::symlink(&elsewhere, &module).expect("symlinked");
    let why = unsupported(place(root.path(), &present(spec("it", "words", None))));
    assert!(
        why.contains("crates/demo/tests/it/words.rs") && why.contains("outside"),
        "{why}"
    );
    assert_eq!(
        std::fs::read_to_string(&elsewhere).expect("reads"),
        MODULE_BEFORE,
        "nothing is written outside the root"
    );
}

#[test]
fn an_over_cap_target_is_refused() {
    let root = project();
    let module = root.path().join("crates/demo/tests/it/words.rs");
    let over = rk_git::landing::MAX_LANDED_BLOB_BYTES.saturating_add(1);
    let big = "/".repeat(usize::try_from(over).expect("the cap fits usize"));
    std::fs::write(&module, &big).expect("writes");
    let why = unsupported(place(root.path(), &present(spec("it", "words", None))));
    assert!(
        why.contains("crates/demo/tests/it/words.rs") && why.contains("landing bound"),
        "{why}"
    );
    assert_eq!(
        std::fs::metadata(&module).expect("stat").len(),
        over,
        "nothing is written"
    );
}

#[test]
fn a_directory_target_is_refused() {
    let root = project();
    let dir = root.path().join("crates/demo/tests/it/folder.rs");
    std::fs::create_dir(&dir).expect("a directory where the module file would be");
    let why = unsupported(place(root.path(), &present(spec("it", "folder", None))));
    assert!(
        why.contains("crates/demo/tests/it/folder.rs") && why.contains("not a regular file"),
        "{why}"
    );
    assert!(dir.is_dir(), "the directory is left as it was");
}

#[test]
fn a_target_holding_the_block_before_later_edits_is_already_placed() {
    let root = project();
    let module = root.path().join("crates/demo/tests/it/words.rs");
    let prior = format!("{MODULE_BEFORE}\n{REPRO}\n#[test]\nfn added_later() {{}}\n");
    std::fs::write(&module, &prior).expect("writes");
    let placed = place(root.path(), &present(spec("it", "words", None))).expect("placed");
    assert!(!placed.appended, "the block is already in the file");
    assert_eq!(placed.test, "words::repro_red");
    assert_eq!(std::fs::read_to_string(&module).expect("reads"), prior);
}
