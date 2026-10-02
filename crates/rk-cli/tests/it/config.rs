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

use rk_cli::adapters::{AdapterSet, Refusal};
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
        AdapterSet::resolve(&file, &clean_env()).expect("the scaffold's four reals resolve");
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
/// **The side-effect half observes a REAL repository, and it did not at first.** The first version
/// minted a bare `tempfile::tempdir()` and asserted it was empty — but that path was never handed to
/// `parse` or `resolve`, so no implementation of either could have put anything in it. It was a
/// guard that could not fail. §7d.16 item 2 asks for "zero marker files, no store file", and a store
/// file has a real address: `<common-dir>/redkiln/store.sqlite3`. The arms now run against a
/// repository where that address is live, and the **positive control** is at the end — the same
/// address goes non-empty when `init` legitimately opens a store, so the absence asserted before it
/// is an absence of something that demonstrably can appear.
#[test]
fn ac486_misconfigured_adapter_refuses_before_spawn_or_store_write() {
    let repo = crate::temp_repo();
    let common = crate::common_dir(repo.path());
    let store_path = common.join("redkiln").join("store.sqlite3");
    assert!(
        !store_path.exists(),
        "the fixture repository already has a store before anything ran"
    );

    // Arm 1: a named adapter that is not one of the reals. A COMPLETE ConfigFile, so the load
    // succeeds and the refusal is `resolve`'s.
    let file = config::parse(&config_fixture("unknown-adapter.toml"))
        .expect("unknown-adapter.toml is a complete ConfigFile: only the harness NAME is wrong");
    let refusal = AdapterSet::resolve(&file, &clean_env())
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

    // The side-effect half, for both arms: no store was written. `resolve` returns names and
    // constructs no port implementor (§7d.5 step (4)), so a store file appearing here would mean the
    // refusal happened AFTER something opened one — which is exactly what "before any spawn or store
    // write" forbids.
    assert!(
        !store_path.exists(),
        "a refused configuration wrote a store at {} — AC-486 refuses BEFORE any spawn or store \
         write",
        store_path.display()
    );
    assert!(
        crate::porcelain(repo.path()).is_empty(),
        "a refused configuration left something in the working tree: {:?}",
        crate::porcelain(repo.path())
    );

    // ---- Arms 3 and 4: THE SPAWNED `run`, added at Slice 3 when a `run` exists. ----------------
    //
    // §7d.16 item 2 fixes the split: "Slice 2 lands the in-process arm … and at Slice 3, when `run`
    // exists, the spawned arm". They are arms of THIS test and not a second `#[test]`: the block
    // filters by name, and a second name would be a second checkpoint line.
    //
    // 🛑 §7d.5's "the harness fixture child's marker count is 0" IS A FLOOR AT THIS CYCLE, and is
    // therefore NOT written as an assertion. Nothing in the binary spawns `rk-fixture-child-claude`
    // until `run`'s dispatch path exists, so an empty marker directory is true of every possible
    // implementation and could not fail. Writing it down as a passing check would be the
    // guard-that-cannot-fail shape this stage has already shipped three times. It becomes an
    // instrument in the cycle that makes the child spawnable, and is asserted there.
    //
    // 🛑 THE MESSAGE IS THE DISCRIMINATOR, NOT THE CODE. §7d.5 says why in terms: the fixture's
    // `[packs] enabled = []` means "a warn-and-continue implementation also exits `3` at step (7)
    // with *no process pack enabled* — the message and the store-file absence are what tell the two
    // apart". So each arm asserts the adapter is NAMED and that step (7)'s wording is ABSENT.
    for (fixture, needle) in [
        ("unknown-adapter.toml", "rk-adapter-linear"),
        ("missing-store.toml", "store"),
    ] {
        let spawned = crate::temp_repo();
        let spawned_common = crate::common_dir(spawned.path());
        let spawned_store = spawned_common.join("redkiln").join("store.sqlite3");
        std::fs::create_dir_all(spawned.path().join(".redkiln")).expect("the .redkiln dir is made");
        std::fs::write(
            spawned.path().join(".redkiln").join("config.toml"),
            config_fixture(fixture),
        )
        .expect("the misconfigured configuration installs");
        assert!(
            !spawned_store.exists(),
            "PREMISE: the fixture repository is prepared WITHOUT a store, so the absence below is \
             observable. {} already exists",
            spawned_store.display()
        );

        let out = spawn_run_for_ac486(spawned.path(), "bug-1");
        let stderr = String::from_utf8_lossy(&out.stderr);

        assert_eq!(
            out.status.code(),
            Some(3),
            "the spawned `run` over {fixture} exited {:?}; stderr:\n{stderr}",
            out.status.code()
        );
        assert!(
            stderr.contains(needle),
            "the refusal over {fixture} does not name `{needle}`. Exit 3 alone cannot tell this \
             refusal from step (7)'s, which the same fixture also reaches; stderr:\n{stderr}"
        );
        assert!(
            !stderr.contains("no process pack enabled"),
            "the spawned `run` over {fixture} got as far as step (7), so the step-(4) refusal did \
             NOT happen first — which is the whole of what AC-486 claims; stderr:\n{stderr}"
        );
        assert!(
            !spawned_store.exists(),
            "the refused `run` created a store at {} — a `SqliteStore::open` that ran would have \
             created it, and AC-486 refuses BEFORE any store write",
            spawned_store.display()
        );
    }

    // THE POSITIVE CONTROL for both assertions above. `init` on the same repository opens the store
    // eagerly (D-14), so the address the absences were asserted over goes non-empty — which is what
    // makes those absences observations rather than a guard over ground nothing could reach.
    assert_eq!(crate::run_init(repo.path()).status.code(), Some(0));
    assert!(
        store_path.exists(),
        "the control failed: `init` did not create a store at {}, so the absence asserted above was \
         over an address nothing writes and proved nothing",
        store_path.display()
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
    let refusal = AdapterSet::resolve(&file, &Env::from_map(map))
        .expect_err("a denylisted credential in the captured environment is refused");
    assert!(
        matches!(refusal, Refusal::DenylistedEnv { .. }),
        "the refusal is not a denylisted-environment refusal: {refusal}"
    );
    assert!(
        refusal.to_string().contains("ANTHROPIC_FOUNDRY_API_KEY"),
        "the refusal does not name the variable; it read:\n{refusal}"
    );

    // **The precedence arm.** `AdapterSet::resolve` documents that the environment is checked FIRST
    // and that "the first refusal wins and nothing after it runs" — and until this arm existed that
    // sentence had no instrument: moving the denylist check after all three adapter-name checks left
    // every test in the crate green. A repository whose configuration is ALSO wrong must still be
    // told about the credential, because the credential is the thing that must not reach a child.
    let bad_config = config::parse(&config_fixture("unknown-adapter.toml"))
        .expect("unknown-adapter.toml loads; only the harness NAME is wrong");
    let mut both = BTreeMap::new();
    both.insert(
        "ANTHROPIC_FOUNDRY_API_KEY".to_owned(),
        "a-credential".to_owned(),
    );
    let first = AdapterSet::resolve(&bad_config, &Env::from_map(both))
        .expect_err("a wrong configuration AND a denylisted credential are both refusals");
    assert!(
        matches!(first, Refusal::DenylistedEnv { .. }),
        "the environment is checked FIRST, so the credential wins over the adapter name; got: \
         {first}"
    );
    assert!(
        !first.to_string().contains("rk-adapter-linear"),
        "the refusal named the adapter as well as the credential; only the first refusal runs"
    );
}

/// Spawn `redkiln -C <repo> run <item>` with stdin closed (D-15), for AC-486's spawned arms.
///
/// Module level: `clippy::items_after_statements` is DENIED workspace-wide.
fn spawn_run_for_ac486(repo: &std::path::Path, item: &str) -> std::process::Output {
    let mut cmd = crate::redkiln_command(crate::Launch::Fixture);
    cmd.arg("-C").arg(repo).arg("run").arg(item);
    crate::support::output_bounded(&mut cmd)
}
