//! **The configuration root** — `ConfigFile`, `Env` and `AdapterSet`: what the process reads once,
//! at the edge, and hands on as values (ADR-0083; `s7d` §7d.1, §7d.5, §7d.9).
//!
//! AC-426 (the required key), AC-474 (config half), AC-422/AC-488 (the one list) and AC-486 (the
//! refusal before spawn or store write, in-process arm).
//!
//! **Every fixture here is a complete `.redkiln/config.toml`**, derived from §7d.9's scaffold by one
//! edit each, so a refusal is attributable to that edit and to nothing else. Reading them as BYTES
//! through `config::parse` rather than from a repository keeps these tests off the filesystem and
//! off `init` — which is what lets them run at Slice 2, before any verb but `init` exists.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_cli::adapters::{self, Refusal};
use rk_cli::config;
use rk_cli::env::Env;

use crate::{code_lines, lint_controls_dir, rs_files, src_dir};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn config_fixture(name: &str) -> String {
    let path = fixtures().join("config").join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the fixture {} must exist: {e}", path.display()))
}

/// The bytes `init` writes, read from the pinned fixture rather than re-typed here.
fn scaffold() -> String {
    let path = fixtures().join("init").join("config.toml.expected");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("the pinned scaffold must exist: {e}"))
}

/// An `Env` with nothing in it. **No test calls `Env::capture()`** (§7d.5's AC-486 row): a test that
/// read the real process environment would pass or fail on whatever this box happens to export —
/// and on this box `ANTHROPIC_FOUNDRY_API_KEY` is persisted at user scope, so it would fail.
fn clean_env() -> Env {
    Env::from_map(BTreeMap::new())
}

/// AC-426's *"a required key, no default"*.
///
/// Three arms over three fixtures, each the scaffold minus or plus one thing, plus a source scan for
/// the three spellings that would re-introduce a default behind the type's back. Red by
/// `missing-doctor.toml` or `missing-key.toml` loading, by `unknown-key.toml` loading under
/// `deny_unknown_fields`, and by a `Default` impl, an `unwrap_or(` or a `serde(default` on the field;
/// a weaker red tests the missing table only.
#[test]
fn ac426_unread_window_runs_required_no_default() {
    let scaffold = scaffold();
    let ok = config::parse(&scaffold).expect("the scaffold is a complete ConfigFile");
    assert_eq!(
        ok.doctor.unread_window_runs, 50,
        "the scaffold's window is the one literal init writes"
    );

    for (fixture, what) in [
        ("missing-doctor.toml", "the whole [doctor] table is absent"),
        ("missing-key.toml", "[doctor] is present and the key is not"),
        (
            "unknown-key.toml",
            "the key is misspelled, so the real one is absent",
        ),
    ] {
        let err = config::parse(&config_fixture(fixture))
            .expect_err(&format!("{fixture} must be refused: {what}"));
        let rendered = err.to_string();
        assert!(
            rendered.contains("unread_window_runs") || rendered.contains("doctor"),
            "{fixture}'s refusal names neither the key nor its table; it read:\n{rendered}"
        );
    }

    // The source half. A field that loads only because something supplied a default is the same
    // defect one indirection away, and none of the three spellings is visible to the fixtures.
    let control = lint_controls_dir().join("ac426_default_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let spellings = ["impl Default for ConfigFile", "unwrap_or(", "serde(default"];
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| spellings.iter().any(|s| l.contains(s)))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line matching a default spelling — the scan is \
         vacuous",
        control.display()
    );

    // Scoped to the module that DECLARES the field, which is what §7d.14's bullet says: "by a
    // `Default` impl, an `unwrap_or(` or any `serde(default` attribute ON THE FIELD IN `config.rs`".
    // Scoped to the whole crate it is a ban wider than its warrant, and it blocks valid work —
    // measured here: it fired on `clock.rs`'s `unwrap_or(i64::MAX)`, a saturating conversion with
    // nothing to do with a required configuration key (golden constraint 9).
    let config_rs = src_dir().join("config.rs");
    let config_text = std::fs::read_to_string(&config_rs).expect("config.rs is readable");
    let hits: Vec<String> = code_lines(&config_text)
        .into_iter()
        .filter(|(_, l)| spellings.iter().any(|s| l.contains(s)))
        .map(|(n, l)| format!("{n}: {}", l.trim()))
        .collect();
    assert!(
        hits.is_empty(),
        "a default for a required key is declared in {} (AC-426: \"a required key, no default\"):\n{}",
        config_rs.display(),
        hits.join("\n")
    );
}

/// AC-474's config half.
///
/// Two conjuncts: the red fixture must not load, and the type's own key surface must carry no
/// dual-write-shaped key. The second is a snapshot **of the struct**, not of a list someone keeps by
/// hand — which is what §7d.1 means by *"a closed struct so the key list is enumerable"*.
#[test]
fn ac474_config_surface_snapshot_has_no_dual_write_key() {
    config::parse(&config_fixture("dual-write.toml"))
        .expect_err("a [compat] dual_write key must be refused by deny_unknown_fields");

    let keys = config::key_surface();
    assert!(
        keys.len() >= 8,
        "the enumerated key surface is {} keys, which is fewer than the scaffold alone writes — the \
         enumerator is reading less than the struct and every absence below would be vacuous: {keys:?}",
        keys.len()
    );
    for key in &keys {
        let lower = key.to_ascii_lowercase();
        assert!(
            !(lower.contains("dual") || lower.contains("compat") || lower.contains("legacy")),
            "the config surface carries `{key}` — no dual-write, compat or legacy route exists \
             (AC-474)"
        );
    }
    let rendered = keys.join("\n");
    insta::assert_snapshot!(rendered);
}

/// AC-486's positive control (D-14): the scaffold resolves the four reals.
///
/// Red by `resolve` refusing the §7d.9 scaffold, or resolving fewer than four names. This is the
/// control that makes the two refusals below discriminating rather than the behaviour of a resolver
/// that refuses everything.
#[test]
fn ac486_init_scaffold_resolves_four_reals() {
    let file = config::parse(&scaffold()).expect("the scaffold loads");
    let resolved =
        adapters::resolve(&file, &clean_env()).expect("the scaffold's four reals resolve");
    assert_eq!(
        resolved.names(),
        vec![
            "rk-adapter-claude".to_owned(),
            "rk-adapter-sqlite".to_owned(),
            "rk-adapter-toolchain-node".to_owned(),
            "rk-adapter-toolchain-rust".to_owned(),
        ],
        "the four reals ADR-0094 d1 names, resolved once at startup"
    );
}

/// AC-486 — the in-process arm (§7d.16 item 2). The spawned-`run` arm is Slice 3's.
///
/// Two arms, and they refuse at **two different layers**, which is a departure from §7d.16 item 2's
/// phrasing and is recorded as one. That sentence says both fixtures make *"`AdapterSet::resolve`
/// return `Err(Refusal)`"* — but `missing-store.toml` is the scaffold with the `store =` line
/// REMOVED, so it cannot reach `resolve` at all: `ConfigFile` has no optional `store`, and
/// §7d.5 step (2) puts a missing required key at the LOAD (*"an unknown key, a missing `[doctor]
/// unread_window_runs`, or any dual-write-shaped key is exit `3`"*). Making `store` an `Option` to
/// satisfy the sentence would invent a `Refusal` arm that can never fire on a loaded file — golden
/// constraint 12's *right reason*, inverted. Both layers exit `3`, which is what AC-486's THEN
/// actually asserts.
///
/// Each arm asserts TWO things in order (§7d.14): the refusal, and then that nothing happened —
/// no marker file, no store file. The side-effect half is what turns a warn-and-continue
/// implementation red on the second assertion rather than passing on the first.
#[test]
fn ac486_misconfigured_adapter_refuses_before_spawn_or_store_write() {
    let scratch = tempfile::tempdir().expect("a temp dir");

    // Arm 1: a named adapter that is not one of the reals. A COMPLETE ConfigFile, so the load
    // succeeds and the refusal is `resolve`'s.
    let file = config::parse(&config_fixture("unknown-adapter.toml"))
        .expect("unknown-adapter.toml is a complete ConfigFile: only the harness NAME is wrong");
    let refusal = adapters::resolve(&file, &clean_env())
        .expect_err("`rk-adapter-linear` is not one of the four reals");
    assert!(
        matches!(refusal, Refusal::UnknownAdapter { .. }),
        "the refusal is not an unknown-adapter refusal: {refusal}"
    );
    assert!(
        refusal.to_string().contains("rk-adapter-linear"),
        "the refusal does not name the adapter it refused; it read:\n{refusal}"
    );

    // Arm 2: the required key absent. Refused at the LOAD — see the note above.
    let err = config::parse(&config_fixture("missing-store.toml"))
        .expect_err("missing-store.toml has no `store` key and `store` is required");
    assert!(
        err.to_string().contains("store"),
        "the refusal does not name the absent key; it read:\n{err}"
    );

    // The side-effect half, for both arms: nothing was created. `resolve` constructs no port
    // implementor (§7d.5 step (4)) and neither call touches the filesystem, so the scratch
    // directory must be exactly as empty as it was.
    let entries: Vec<_> = std::fs::read_dir(scratch.path())
        .expect("the scratch dir reads")
        .map(|e| e.expect("an entry").file_name())
        .collect();
    assert!(
        entries.is_empty(),
        "a refused configuration created something: {entries:?} — AC-486 refuses BEFORE any spawn \
         or store write"
    );
}

/// AC-422's one-list half, AC-488's edge half.
///
/// Red by a second `"ANTHROPIC_API_KEY"` literal in `crates/rk-cli/src` (a source scan), or by
/// `resolve` not refusing a captured map carrying `ANTHROPIC_FOUNDRY_API_KEY` — a name only the
/// adapter's list carries, so a resolver that had quietly grown its own short list would pass a test
/// written against `ANTHROPIC_API_KEY` alone and fail here.
#[test]
fn ac422_config_reads_the_one_denylist() {
    let control = lint_controls_dir().join("ac422_api_key_literal_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let needle = "\"ANTHROPIC_API_KEY\"";
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| l.contains(needle))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line with the literal — the scan is vacuous",
        control.display()
    );

    let mut hits = Vec::new();
    for file in rs_files(&src_dir()) {
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (line_no, line) in code_lines(&text) {
            if line.contains(needle) {
                hits.push(format!("{}:{line_no}: {}", file.display(), line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "rk-cli carries its own credential-name literal. There is ONE list — \
         `rk_adapter_claude::env_denylist` — and this crate reads it (AC-422, AC-488):\n{}",
        hits.join("\n")
    );

    // The edge half, through a name ONLY the adapter's list carries.
    let file = config::parse(&scaffold()).expect("the scaffold loads");
    let mut map = BTreeMap::new();
    map.insert(
        "ANTHROPIC_FOUNDRY_API_KEY".to_owned(),
        "a-credential".to_owned(),
    );
    let refusal = adapters::resolve(&file, &Env::from_map(map))
        .expect_err("a denylisted credential in the captured environment is refused");
    assert!(
        matches!(refusal, Refusal::DenylistedEnv { .. }),
        "the refusal is not a denylisted-environment refusal: {refusal}"
    );
    assert!(
        refusal.to_string().contains("ANTHROPIC_FOUNDRY_API_KEY"),
        "the refusal does not name the variable; it read:\n{refusal}"
    );
}
