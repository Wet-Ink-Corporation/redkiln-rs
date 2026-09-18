//! The one compile-fail harness of this crate, named after its rule and not after the tool
//! (s7c §7c.19: *"named for its rule — `trybuild_pins_the_reason_each_claude_fixture_fails`, in
//! `tests/it/ui.rs` over `tests/ui/`, AC-381's case with a committed `.stderr`"*; the checkpoint
//! filters on this name with `--test it`, never on `trybuild`).
//!
//! `tests/ui/` is this glob's root and therefore holds **compile-FAIL cases only** — never a
//! lint control (`tests/fixtures/lint-controls/`, §7c.17). Each case commits a `.stderr`
//! snapshot, so a case that starts failing for a DIFFERENT reason goes red rather than silently
//! continuing to pass (`s7a` §7a.8). One case at Slice 5: AC-381's type-level separation — the
//! headroom calculator's input struct (`throttle::Sample`) cannot name a cache field.

#[test]
fn trybuild_pins_the_reason_each_claude_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/ac381_cache_field_on_headroom_input.rs");
}

/// Every case in `tests/ui/` is registered above and commits a non-empty `.stderr` (the kernel's
/// precedent: an unregistered file never runs, and an empty `.stderr` is green for ANY failure).
#[test]
fn every_claude_compile_fail_case_is_registered_and_commits_a_stderr() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    let harness = include_str!("ui.rs");
    let mut on_disk: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("tests/ui is readable") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("rs") {
            continue;
        }
        let stderr = path.with_extension("stderr");
        assert!(
            stderr.is_file() && std::fs::metadata(&stderr).expect("meta").len() > 0,
            "{} commits no non-empty .stderr (s7a §7a.8)",
            stderr.display()
        );
        on_disk.push(
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .expect("utf-8")
                .to_owned(),
        );
    }
    on_disk.sort();
    let mut registered: Vec<String> = harness
        .lines()
        .filter_map(|l| l.trim().strip_prefix("t.compile_fail(\"tests/ui/"))
        .filter_map(|l| l.split('"').next())
        .map(str::to_owned)
        .collect();
    registered.sort();
    assert_eq!(
        registered, on_disk,
        "every case registered, every registration on disk"
    );
    assert!(!on_disk.is_empty(), "at least AC-381's case");
}
