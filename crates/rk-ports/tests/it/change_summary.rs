//! `ChangeSummary`'s two arms (D21-42 decision 2; `s8a` §8a.2, §8a.9): the **`Text`** arm is
//! finalised now, and the **`Binary`** arm's payload is a sealed slot S-BIN fills at `s8i`.
//!
//! Two tests, one per arm:
//!
//! - `text_variant_serialises_sha_diff_and_manifest` — the serialised `Text` value carries the
//!   members the shipped catalog already reads: `sha` (`adversarial-review.toml`'s
//!   `artifact:ChangeSummary.sha`), `diff` (`change-summary.toml`'s `${summarise.outputs.diff}`)
//!   and `manifest` (per path: the path, its `classify` result, its change kind), plus `files`, the
//!   manifest's paths (`mutation-testing.toml`'s `artifact:Changeset.files`; amended 2026-09-25, s8a
//!   fix round 10: this said "three members"). A field set that misses a member the catalog reads
//!   is a value that resolves nothing.
//! - `binary_arm_is_not_constructible_outside_rk_ports` — a `trybuild` case that must fail for the
//!   **privacy** reason, with a committed `.stderr`. It is its OWN harness test on purpose:
//!   Checkpoint 7a counts `trybuild_pins_the_reason_each_port_fixture_fails`'s four cases. **`s8i`
//!   supersedes this test by name** when S-BIN finalises `Binary` (`s8a` §8a.2 item 1); that is
//!   expected, not drift.

use rk_ports::toolchain::{
    ChangeKind, ChangeSummary, Classification, ManifestEntry, TextChangeSummary,
};

/// The `Text` arm serialises exactly the members the catalog reads, at the top level of the
/// `ChangeSummary` value (`artifact:ChangeSummary.sha` reads `sha` off the value itself, so the
/// arm is not wrapped in a tag), and each manifest row carries the path, its classification and
/// its change kind.
#[test]
fn text_variant_serialises_sha_diff_and_manifest() {
    let text = TextChangeSummary {
        sha: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        diff: "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-one\n+two\n".to_owned(),
        manifest: vec![
            ManifestEntry {
                path: "src/lib.rs".to_owned(),
                classification: Classification::Text,
                change: ChangeKind::Modified,
            },
            ManifestEntry {
                path: "Cargo.lock".to_owned(),
                classification: Classification::Generated,
                change: ChangeKind::Added,
            },
        ],
    };
    let value = serde_json::to_value(ChangeSummary::Text(text))
        .expect("the Text arm serialises (it is the finalised arm)");

    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("a Text ChangeSummary serialises to an object, got {value}"));
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    // (amended 2026-09-25, s8a fix round 10: this read `["diff", "manifest", "sha"]`. The catalog
    // also reads `artifact:Changeset.files` -- `mutation-testing.toml`'s `scope` input -- and
    // `Changeset` is declared `schema = "ChangeSummary"` and bound to `toolchain:change_summary`'s
    // value, so the Text value carries `files`: the manifest's paths, in manifest order.)
    assert_eq!(
        keys,
        vec!["diff", "files", "manifest", "sha"],
        "the Text value carries exactly the members the shipped catalog reads (sha, diff, \
         files, manifest) at its top level: {value}"
    );
    assert_eq!(
        value["files"],
        serde_json::json!(["src/lib.rs", "Cargo.lock"]),
        "`artifact:Changeset.files` resolves to the manifest's paths, in manifest order: {value}"
    );
    assert_eq!(
        value["sha"], "0123456789abcdef0123456789abcdef01234567",
        "`artifact:ChangeSummary.sha` resolves to the range's head commit"
    );
    assert!(
        value["diff"].as_str().is_some_and(|d| d.contains("+two")),
        "`${{summarise.outputs.diff}}` resolves to the textual diff: {value}"
    );
    assert_eq!(
        value["manifest"],
        serde_json::json!([
            {"path": "src/lib.rs", "classification": "text", "change": "modified"},
            {"path": "Cargo.lock", "classification": "generated", "change": "added"},
        ]),
        "one manifest row per path: the path, its classify result and its change kind"
    );
}

/// The `Binary` arm's payload cannot be constructed outside `rk-ports` (the sealed slot, `s8a`
/// §8a.2 item 1). A grep over the declaration cannot prove non-constructibility; this case does,
/// and its committed `.stderr` pins the PRIVACY reason, so a case that starts failing for another
/// reason goes red.
#[test]
fn binary_arm_is_not_constructible_outside_rk_ports() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/change_summary_binary_not_constructible.rs");
}
