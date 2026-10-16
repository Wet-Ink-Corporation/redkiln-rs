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
//!
//! *(Amended 2026-10-14, s8i, ADR-0199, proposed: S-BIN has reported (F43, proposed), and the
//! second bullet's test is superseded by name, as it said it would be. The `Binary` arm now has
//! its four recorded fields and one checked constructor:
//! `binary_is_constructible_only_through_its_recorded_constructor`,
//! `binary_rejects_a_text_patch_for_a_binary_classified_path`, and one
//! `binary_field_<name>` test per field, which Checkpoint 8i (8) counts against the record's
//! `Binary field set (4 fields)`. The construction rule's `trybuild` case is
//! `ui::trybuild_pins_the_binary_change_summary_construction_rule`.)*

use rk_ports::toolchain::{
    BinaryAsset, BinaryCapture, BinaryChangeSummary, BinaryChangeSummaryError, ChangeKind,
    ChangeSummary, Classification, ManifestEntry, PointerDefect, TextChangeSummary,
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

// (Superseded by name 2026-10-14, s8i, ADR-0199 decision 2, proposed: `s8a`'s
// `binary_arm_is_not_constructible_outside_rk_ports` and its `trybuild` case
// `tests/ui/change_summary_binary_not_constructible.rs` pinned the sealed slot. ADR-0165 decision
// 7 provides that the slot opens by a later record once S-BIN reports, and that the seal pins are
// superseded by name in the commit that lands the constructor. The construction rule is now
// pinned by `binary_is_constructible_only_through_its_recorded_constructor` below and by
// `ui::trybuild_pins_the_binary_change_summary_construction_rule`.)

/// A git-lfs pointer, as a range's blob holds it, for an asset whose size is `size`.
fn pointer(oid: &str, size: u64) -> Vec<u8> {
    format!("version https://git-lfs.github.com/spec/v1\noid sha256:{oid}\nsize {size}\n")
        .into_bytes()
}

const OID: &str = "833ae52fd0db7a460a02931993a8c38bcb4bf356f84efbe79f8f8073247d9325";
const ASSET: &str = "OurGame/Content/UI/Inventory/WBP_LoadoutScreen.uasset";

/// One asset row read from its pointer, as S-BIN's item 4 carried it.
fn asset() -> Result<BinaryAsset, BinaryChangeSummaryError> {
    BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        &pointer(OID, 50_724),
    )
}

fn capture() -> BinaryCapture {
    BinaryCapture {
        path: "_s-bin/captures/capture-1.png".to_owned(),
        viewport: "capture_scene_preview".to_owned(),
        note: "/Game/UI/Inventory/WBP_LoadoutScreen".to_owned(),
    }
}

/// The recorded constructor over S-BIN item 4's shape, with `range_diff` as the producer read it.
fn binary(range_diff: &str) -> Result<BinaryChangeSummary, BinaryChangeSummaryError> {
    BinaryChangeSummary::new(
        vec![asset()?],
        vec![capture()],
        "_s-bin/report/index.json".to_owned(),
        "_s-bin/report/headless.log".to_owned(),
        range_diff,
    )
}

/// git's text patch of the asset's pointer: what AC-497's AND forbids the value to carry.
fn pointer_patch() -> String {
    format!(
        "diff --git a/{ASSET} b/{ASSET}\nindex 1111111..2222222 100644\n--- a/{ASSET}\n+++ \
         b/{ASSET}\n@@ -1,3 +1,3 @@\n version https://git-lfs.github.com/spec/v1\n-oid \
         sha256:{}\n+oid sha256:{OID}\n-size 50710\n+size 50724\n",
        "a".repeat(64)
    )
}

/// The `Binary` payload is built through ONE recorded constructor (ADR-0199 decision 2), and that
/// constructor is CHECKED: a clean input builds a value whose four fields read back and which
/// `ChangeSummary::Binary` carries and serialises, and the same input with a text patch of the
/// asset's path is refused. No other route compiles outside `rk-ports` (the `trybuild` case).
#[test]
fn binary_is_constructible_only_through_its_recorded_constructor() {
    let built = binary("").expect("a clean Binary input builds through the recorded constructor");
    assert_eq!(
        built.assets(),
        [asset().expect("a well-formed git-lfs pointer reads")].as_slice()
    );
    assert_eq!(built.captures(), [capture()].as_slice());
    assert_eq!(built.tests_report_ref(), "_s-bin/report/index.json");
    assert_eq!(built.build_log_ref(), "_s-bin/report/headless.log");

    let value = serde_json::to_value(ChangeSummary::Binary(built))
        .expect("the Binary arm serialises once S-BIN has fixed what it carries (ADR-0199)");
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("a Binary ChangeSummary serialises to an object, got {value}"));
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        vec!["assets", "build", "captures", "tests"],
        "the Binary value carries exactly ADR-0199's four fields at its top level, and none of \
         the Text arm's members: {value}"
    );

    assert_eq!(
        binary(&pointer_patch()),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: ASSET.to_owned(),
        }),
        "the recorded constructor is checked: an input carrying a text patch of a \
         Binary-classified path is refused naming the path, never built (AC-497's AND)"
    );
}

/// The recorded constructor over one asset at `path`, read from the same pointer as [`asset`].
fn binary_at(
    path: &str,
    range_diff: &str,
) -> Result<BinaryChangeSummary, BinaryChangeSummaryError> {
    BinaryChangeSummary::new(
        vec![BinaryAsset::from_pointer(
            path.to_owned(),
            "uasset".to_owned(),
            ChangeKind::Modified,
            &pointer(OID, 50_724),
        )?],
        Vec::new(),
        "_s-bin/report/index.json".to_owned(),
        "_s-bin/report/headless.log".to_owned(),
        range_diff,
    )
}

/// git's text patch of the pointer at `path`, with each name as git writes it: `a_name` and
/// `b_name` on the `diff --git` line and on `---`/`+++`, C-quoted by the caller when git quotes.
fn named_pointer_patch(a_name: &str, b_name: &str, tail: &str) -> String {
    format!(
        "diff --git {a_name} {b_name}\nindex 1111111..2222222 100644\n--- {a_name}{tail}\n+++ \
         {b_name}{tail}\n@@ -1,3 +1,3 @@\n version https://git-lfs.github.com/spec/v1\n oid \
         sha256:{OID}\n-size 50710\n+size 50724\n"
    )
}

/// AC-497's AND, at the constructor: an input whose range diff carries a TEXT patch of a
/// `Binary`-classified path is refused naming the path, whether git wrote it as a hunk over the
/// pointer, as a plain unified diff, or as a `GIT binary patch`. git's content-free
/// `Binary files … differ` record of the path, and a patch of a path the value does not carry,
/// are not a text patch of the binary path and are admitted.
#[test]
fn binary_rejects_a_text_patch_for_a_binary_classified_path() {
    let named = BinaryChangeSummaryError::TextPatchForBinaryPath {
        path: ASSET.to_owned(),
    };
    assert_eq!(
        binary(&pointer_patch()),
        Err(named.clone()),
        "git's hunk over the asset's pointer is a text patch of the binary path"
    );
    let plain = format!("--- a/{ASSET}\n+++ b/{ASSET}\n@@ -3 +3 @@\n-size 50710\n+size 50724\n");
    assert_eq!(
        binary(&plain),
        Err(named.clone()),
        "a plain unified diff of the path, with no `diff --git` header, is refused too"
    );
    let crlf = pointer_patch().replace('\n', "\r\n");
    assert_eq!(binary(&crlf), Err(named.clone()), "a CRLF diff is refused");
    let base85 = format!(
        "diff --git a/{ASSET} b/{ASSET}\nindex 1111111..2222222 100644\nGIT binary patch\nliteral \
         5\nMcmZ?l00001\n\nliteral 0\nHcmV?d00001\n"
    );
    assert_eq!(
        binary(&base85),
        Err(named.clone()),
        "git's `GIT binary patch` is a textual encoding of the binary path's content"
    );
    let message = named.to_string();
    assert!(
        message.contains(ASSET) && message.contains("AC-497"),
        "the refusal names the path and the criterion: {message}"
    );

    // git C-quotes a name holding a `"`, a backslash, a control byte or a non-ASCII byte
    // (`core.quotePath`, on by default), and writes a TAB after a `---`/`+++` name holding a space.
    let quoted = "Content/Props/Crate \"B\".uasset";
    assert_eq!(
        binary_at(
            quoted,
            &named_pointer_patch(
                r#""a/Content/Props/Crate \"B\".uasset""#,
                r#""b/Content/Props/Crate \"B\".uasset""#,
                "",
            ),
        ),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: quoted.to_owned(),
        }),
        "a C-quoted path holding a `\"` is read unquoted, and its text patch is refused"
    );
    let non_ascii = "Content/Props/Caf\u{e9}.uasset";
    assert_eq!(
        binary_at(
            non_ascii,
            &named_pointer_patch(
                r#""a/Content/Props/Caf\303\251.uasset""#,
                r#""b/Content/Props/Caf\303\251.uasset""#,
                "",
            ),
        ),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: non_ascii.to_owned(),
        }),
        "a non-ASCII path, which git writes as octal escapes, is read unquoted and refused"
    );
    let quoted_rename_away = concat!(
        r#"diff --git "a/Content/Props/Caf\303\251.uasset" b/Content/Props/Cafe.uasset"#,
        "\nsimilarity index 88%\n",
        r#"rename from "Content/Props/Caf\303\251.uasset""#,
        "\nrename to Content/Props/Cafe.uasset\nindex 1111111..2222222 100644\nGIT binary patch\n",
        "literal 5\nMcmZ?l00001\n\nliteral 0\nHcmV?d00001\n",
    );
    assert_eq!(
        binary_at(non_ascii, quoted_rename_away),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: non_ascii.to_owned(),
        }),
        "a `GIT binary patch` in a rename section whose quoted a-side is the binary path (no \
         `---`/`+++` to match) is refused"
    );
    let spaced = "Content/Props/Crate B.uasset";
    let plain_spaced =
        format!("--- a/{spaced}\t\n+++ b/{spaced}\t\n@@ -3 +3 @@\n-size 50710\n+size 50724\n");
    assert_eq!(
        binary_at(spaced, &plain_spaced),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: spaced.to_owned(),
        }),
        "the TAB git writes after a `---`/`+++` name holding a space is not part of the name"
    );
    let renamed = format!(
        "diff --git a/OurGame/Content/UI/Inventory/WBP_Old.uasset b/{ASSET}\nsimilarity index \
         88%\nrename from OurGame/Content/UI/Inventory/WBP_Old.uasset\nrename to {ASSET}\nindex \
         1111111..2222222 100644\nGIT binary patch\nliteral 5\nMcmZ?l00001\n\nliteral 0\n\
         HcmV?d00001\n"
    );
    assert_eq!(
        binary(&renamed),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: ASSET.to_owned(),
        }),
        "a `GIT binary patch` in a rename section whose b-side is the binary path is refused"
    );
    let renamed_away = format!(
        "diff --git a/{ASSET} b/OurGame/Content/UI/Inventory/WBP_New.uasset\nsimilarity index \
         88%\nrename from {ASSET}\nrename to OurGame/Content/UI/Inventory/WBP_New.uasset\nindex \
         1111111..2222222 100644\nGIT binary patch\nliteral 5\nMcmZ?l00001\n\nliteral 0\n\
         HcmV?d00001\n"
    );
    assert_eq!(
        binary(&renamed_away),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: ASSET.to_owned(),
        }),
        "a `GIT binary patch` in a rename section whose a-side is the binary path is refused"
    );

    let clean = binary("").expect("a clean input builds");
    let no_content = format!(
        "diff --git a/{ASSET} b/{ASSET}\nindex 1111111..2222222 100644\nBinary files a/{ASSET} \
         and b/{ASSET} differ\n"
    );
    assert_eq!(
        binary(&no_content),
        Ok(clean.clone()),
        "git's content-free record that the binary path changed carries no text patch"
    );
    let other = "diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n@@ -1 +1 \
                 @@\n-a\n+b\n";
    assert_eq!(
        binary(other),
        Ok(clean.clone()),
        "a patch of a path the value does not carry is not a patch of the binary path"
    );
    let header_lookalike = format!(
        "diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n@@ -1,2 +1,2 \
         @@\n--- a/{ASSET}\n+++ b/{ASSET}\n unchanged\n@@ -9 +9 @@\n--- a/{ASSET}\n+++ \
         b/{ASSET}\n"
    );
    assert_eq!(
        binary(&header_lookalike),
        Ok(clean),
        "a hunk line of another path that reads like a `---`/`+++` header is hunk content, not \
         a header naming the binary path"
    );
}

/// A C-quoted name the check cannot read -- its quote never closes, or it holds an escape git
/// does not write -- might name any asset, so it counts as naming each one: the constructor
/// refuses more, never less. Each case patches no path the value carries in readable form.
#[test]
fn binary_refuses_a_hunk_whose_counts_overstate_its_length() {
    let refused = Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
        path: ASSET.to_owned(),
    });
    let overstated = "--- a/README.md\n+++ b/README.md\n@@ -1,5 +1,5 @@\n-a\n+b\n\
                      --- a/Content/Crate.uasset\n+++ b/Content/Crate.uasset\n@@ -1 +1 @@\n-x\n+y\n";
    assert_eq!(
        binary(overstated),
        refused,
        "a hunk whose counts swallow the next file's headers ends short, and is refused"
    );
    let unfinished = "--- a/README.md\n+++ b/README.md\n@@ -1,3 +1,3 @@\n-a\n+b\n";
    assert_eq!(
        binary(unfinished),
        refused,
        "a hunk the input ends before its counts are spent is refused"
    );
    let exact = "--- a/README.md\n+++ b/README.md\n@@ -1,2 +1,2 @@\n-a\n+b\n c\n";
    assert_eq!(
        binary(exact),
        Ok(binary("").expect("a clean input builds")),
        "a hunk whose counts match its lines, of another path, is admitted"
    );
}

#[test]
fn binary_refuses_a_patch_whose_quoted_name_it_cannot_read() {
    let refused = Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
        path: ASSET.to_owned(),
    });
    let unclosed_headers = "--- \"a/Content/Crate.uasset\n+++ \"b/Content/Crate.uasset\n@@ -1 \
                            +1 @@\n-a\n+b\n";
    assert_eq!(
        binary(unclosed_headers),
        refused,
        "a `---`/`+++` pair whose quoted names never close is refused"
    );
    let unclosed_new_side = "--- /dev/null\n+++ \"b/Content/Crate.uasset\n@@ -0,0 +1 @@\n+a\n";
    assert_eq!(
        binary(unclosed_new_side),
        refused,
        "a `+++` header whose quoted name never closes is refused, with `/dev/null` opposite"
    );
    let unknown_escape = "--- \"a/Content/Caf\\q.uasset\"\n+++ /dev/null\n@@ -1 +0,0 @@\n-a\n";
    assert_eq!(
        binary(unknown_escape),
        refused,
        "a quoted `---` name holding an escape git does not write is refused"
    );
    let unclosed_section = "diff --git \"a/Content/Crate.uasset b/Content/Crate.uasset\nindex \
                            1111111..2222222 100644\nGIT binary patch\nliteral 5\nMcmZ?l00001\n\n\
                            literal 0\nHcmV?d00001\n";
    assert_eq!(
        binary(unclosed_section),
        refused,
        "a `GIT binary patch` in a section whose quoted a-side never closes is refused"
    );
    let unclosed_b_side = "diff --git a/Content/Crate.uasset \"b/Content/Crate.uasset\nindex \
                           1111111..2222222 100644\nGIT binary patch\nliteral 5\nMcmZ?l00001\n\n\
                           literal 0\nHcmV?d00001\n";
    assert_eq!(
        binary(unclosed_b_side),
        refused,
        "a `GIT binary patch` in a section whose quoted b-side never closes is refused"
    );
    let readable = "diff --git \"a/Content/Caf\\303\\251.uasset\" \"b/Content/Caf\\303\\251.uasset\"\n\
                    index 1111111..2222222 100644\nGIT binary patch\nliteral 5\nMcmZ?l00001\n\n\
                    literal 0\nHcmV?d00001\n";
    assert_eq!(
        binary(readable),
        Ok(binary("").expect("a clean input builds")),
        "a quoted name the check reads, of a path the value does not carry, is admitted"
    );
}

/// The producer contract (`BinaryChangeSummary::new`): the diff carries git's default `a/` and
/// `b/` prefixes, and each is stripped once. An asset whose own path opens with `a/` is matched
/// under its prefix, and a patch of the path's last component alone is not a patch of it.
#[test]
fn binary_reads_names_under_gits_default_prefixes() {
    let nested = "a/b.uasset";
    let deleted = "--- a/a/b.uasset\n+++ /dev/null\n@@ -1 +0,0 @@\n-x\n";
    assert_eq!(
        binary_at(nested, deleted),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: nested.to_owned(),
        }),
        "`--- a/a/b.uasset` names the path `a/b.uasset`"
    );
    let nested_b = "b/c.uasset";
    let added = "--- /dev/null\n+++ b/b/c.uasset\n@@ -0,0 +1 @@\n+x\n";
    assert_eq!(
        binary_at(nested_b, added),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: nested_b.to_owned(),
        }),
        "`+++ b/b/c.uasset` names the path `b/c.uasset`"
    );
    let section = "diff --git a/a/b.uasset b/a/b.uasset\nindex 1111111..2222222 100644\nGIT binary \
                   patch\nliteral 5\nMcmZ?l00001\n\nliteral 0\nHcmV?d00001\n";
    assert_eq!(
        binary_at(nested, section),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: nested.to_owned(),
        }),
        "`diff --git a/a/b.uasset b/a/b.uasset` names the path `a/b.uasset`"
    );
    let other = "diff --git a/b.uasset b/b.uasset\n--- a/b.uasset\n+++ b/b.uasset\n@@ -1 +1 \
                 @@\n-x\n+y\n";
    assert_eq!(
        binary_at(nested, other),
        Ok(binary_at(nested, "").expect("a clean input builds")),
        "a patch of `b.uasset` is not a patch of `a/b.uasset`: the prefix is stripped once"
    );
}

/// Many assets against one diff: the refusal names the FIRST asset, in the order the producer
/// passed them, that the diff patches as text, whatever order the diff patches them in; and a
/// diff that patches none of them builds a value carrying every asset, in order.
#[test]
fn binary_checks_every_asset_against_the_diff() {
    let paths: Vec<String> = (0..300)
        .map(|i| format!("Content/Bulk/Asset_{i:03}.uasset"))
        .collect();
    let assets = || {
        paths
            .iter()
            .map(|p| {
                BinaryAsset::from_pointer(
                    p.clone(),
                    "uasset".to_owned(),
                    ChangeKind::Modified,
                    &pointer(OID, 50_724),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("every pointer reads")
    };
    let build = |diff: &str| {
        BinaryChangeSummary::new(
            assets(),
            Vec::new(),
            "_s-bin/report/index.json".to_owned(),
            "_s-bin/report/headless.log".to_owned(),
            diff,
        )
    };
    let records: String = paths
        .iter()
        .map(|p| {
            format!(
                "diff --git a/{p} b/{p}\nindex 1..2 100644\nBinary files a/{p} and b/{p} differ\n"
            )
        })
        .collect();
    let built = build(&records).expect("content-free records of every asset build");
    assert_eq!(
        built.assets(),
        assets().as_slice(),
        "every asset is carried, in the order passed"
    );

    let late = &paths[250];
    let early = &paths[40];
    let patched = format!(
        "{records}--- a/{late}\n+++ b/{late}\n@@ -1 +1 @@\n-x\n+y\n--- a/{early}\n+++ \
         b/{early}\n@@ -1 +1 @@\n-x\n+y\n"
    );
    assert_eq!(
        build(&patched),
        Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
            path: early.clone(),
        }),
        "the refusal names the first asset in the producer's order, not the diff's"
    );
}

/// `assets`: one row per `Binary` path, `size` and `lfs_oid` READ from the git-lfs pointer, never
/// inferred (ADR-0199 decision 1; `s8i` §8i.2 item 6). The pointer here is 130 bytes long and
/// names a 50,724-byte asset, so a size taken from anything but the pointer's `size` line reads
/// wrong. A blob that is not a pointer is refused naming the path.
#[test]
fn binary_field_assets() {
    let row = asset().expect("a well-formed git-lfs pointer reads");
    assert_eq!(row.path(), ASSET);
    assert_eq!(row.kind(), "uasset");
    assert_eq!(row.action(), ChangeKind::Modified);
    assert_eq!(row.size(), 50_724, "size is the pointer's `size` line");
    assert_eq!(
        row.lfs_oid(),
        OID,
        "lfs_oid is the pointer's `oid sha256:` value"
    );

    let value = serde_json::to_value(binary("").expect("a clean input builds"))
        .expect("the Binary payload serialises");
    assert_eq!(
        value["assets"],
        serde_json::json!([{
            "path": ASSET,
            "kind": "uasset",
            "action": "modified",
            "size": 50_724,
            "lfs_oid": OID,
        }]),
        "one asset row carries its five members: {value}"
    );

    let refused = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        b"\x00\x01not a pointer",
    );
    assert_eq!(
        refused,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::NoVersionLine,
        }),
        "a blob that is not a git-lfs pointer is refused naming the path and the defect"
    );
    let not_utf8 = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        b"version \xff\xfe",
    );
    assert_eq!(
        not_utf8,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::NotUtf8,
        }),
        "a blob that is not UTF-8 is not a pointer"
    );
    let sizeless = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        format!("version https://git-lfs.github.com/spec/v1\noid sha256:{OID}\n").as_bytes(),
    );
    assert_eq!(
        sizeless,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::Size,
        }),
        "a pointer with no `size` line is refused, never given a size"
    );
    let oidless = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        b"version https://git-lfs.github.com/spec/v1\nsize 50724\n",
    );
    assert_eq!(
        oidless,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::Oid,
        }),
        "a pointer with no `oid` line is refused, never given an lfs_oid"
    );
    let short_oid = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        &pointer(OID.get(..63).expect("OID is 64 ASCII hex"), 50_724),
    );
    assert_eq!(
        short_oid,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::Oid,
        }),
        "an `oid` that is not 64 hex is refused, never carried as the lfs_oid"
    );
    let signed = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        format!("version https://git-lfs.github.com/spec/v1\noid sha256:{OID}\nsize +50724\n")
            .as_bytes(),
    );
    assert_eq!(
        signed,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::Size,
        }),
        "a `size` that is not ASCII digits alone (here a leading `+`) is refused"
    );
    let mut long = pointer(OID, 50_724);
    long.resize(1024, b'\n');
    let too_long = BinaryAsset::from_pointer(
        ASSET.to_owned(),
        "uasset".to_owned(),
        ChangeKind::Modified,
        &long,
    );
    assert_eq!(
        too_long,
        Err(BinaryChangeSummaryError::NotAnLfsPointer {
            path: ASSET.to_owned(),
            defect: PointerDefect::TooLong,
        }),
        "a blob of 1024 bytes or more is not a pointer and is never scanned"
    );
}

/// `captures`: one row per capture, its `path`, `viewport` and `note` (ADR-0199 decision 1).
#[test]
fn binary_field_captures() {
    let value = serde_json::to_value(binary("").expect("a clean input builds"))
        .expect("the Binary payload serialises");
    assert_eq!(
        value["captures"],
        serde_json::json!([{
            "path": "_s-bin/captures/capture-1.png",
            "viewport": "capture_scene_preview",
            "note": "/Game/UI/Inventory/WBP_LoadoutScreen",
        }]),
        "one capture row carries its path, viewport and note: {value}"
    );
}

/// `tests.report_ref`: the raw test report's reference, serialised as `tests: { report_ref }`
/// (ADR-0199 decision 1), and none of the `tests` counts S-BIN's reviewers never cited.
#[test]
fn binary_field_tests_report_ref() {
    let value = serde_json::to_value(binary("").expect("a clean input builds"))
        .expect("the Binary payload serialises");
    assert_eq!(
        value["tests"],
        serde_json::json!({ "report_ref": "_s-bin/report/index.json" }),
        "`tests` carries the report reference only: {value}"
    );
}

/// `build.log_ref`: the build log's reference, serialised as `build: { log_ref }` (ADR-0199
/// decision 1), and neither the build command nor its exit, which no reviewer cited.
#[test]
fn binary_field_build_log_ref() {
    let value = serde_json::to_value(binary("").expect("a clean input builds"))
        .expect("the Binary payload serialises");
    assert_eq!(
        value["build"],
        serde_json::json!({ "log_ref": "_s-bin/report/headless.log" }),
        "`build` carries the log reference only: {value}"
    );
}
