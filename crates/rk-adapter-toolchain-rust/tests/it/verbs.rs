//! **The Rust verb handlers behind `run_verb`** (s8c §8c.4; record V = ADR-0170 decisions 2 and 4-7,
//! proposed; D21-62 C, provisional): `smoke`, `sca_scan`, `package` and `setup`, each answering an
//! erased `VerbOutput { kind, value }` the kernel types at the boundary, and every other verb refused
//! by its bare name.
//!
//! - `smoke` builds, then runs each checklist row against the binaries THAT build produced, in one
//!   scratch directory per call under the adapter's scratch (ADR-0137 clause 6's
//!   `<git-common-dir>/redkiln/tmp` in production), removed on every path. A failing row is `Ok`
//!   with `failed >= 1`.
//! - `sca_scan` runs `cargo audit` and `cargo deny` offline, each against an explicit database, and
//!   reads each database's `HEAD` itself (the shared `rk_git::head` read) before and after its tool
//!   runs. An unrated advisory is counted in `unrated`, never as `low`, and never dropped.
//! - `package` is a locked release build whose DECLARED binaries alone are digested, with the
//!   project root's `HEAD` read through `rk-git` and checked against the input `sha`.
//! - `setup` runs `pnpm install --frozen-lockfile --prefer-offline --dir <project>` per declared
//!   project; a failed install is refused naming the project and the exit code.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::{Path, PathBuf};

use rk_adapter_toolchain_rust::verbs::{db_unmoved, severity_of};
use rk_adapter_toolchain_rust::{
    Config, DEFAULT_PROGRAM, ForcedRunner, RunnerPolicy, RustToolchain,
};
use rk_types::digest::sha256;
use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::{
    BuildArtifact, ScanReport, SetupResult, Severity, SmokeResult, SmokeStatus,
};

use crate::support::{
    GENEROUS, adapter_over, advisory_fixture, copy_tree, fixture_workspace, git_commit_all,
    minimal_env, repo_root, run_verb, same_path, scan_databases, smoke_dirs,
};

fn workspace_adapter(root: &Path, scratch: &Path) -> RustToolchain {
    adapter_over(root, RunnerPolicy::Force(ForcedRunner::CargoTest), scratch)
}

fn refused_naming(err: &StepError, needles: &[&str], what: &str) {
    let text = match err {
        StepError::Unsupported { capability } => capability.clone(),
        StepError::Transport { detail, .. } => detail.clone(),
        other => panic!("{what}: a refusal by name, got {other:?}"),
    };
    for n in needles {
        assert!(text.contains(n), "{what}: the refusal names `{n}`: {text}");
    }
}

// ---------------------------------------------------------------- smoke

/// **`smoke` runs each checklist row against the binary the call's build produced**, in a scratch
/// directory under the adapter's scratch that is gone afterwards (record V decision 4).
#[test]
fn smoke_runs_each_checklist_row_against_the_built_binary() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = workspace_adapter(&fixture_workspace(), scratch.path());
    let out = run_verb(
        &tc,
        "smoke",
        &serde_json::json!({ "checklist": [
            { "id": "version", "argv": ["wsbin", "--version"], "cwd": "scratch", "expect_exit": 0, "stdout_contains": "wsbin 0.0.0" },
            { "id": "exit-three", "argv": ["wsbin", "--exit", "3"], "cwd": "scratch", "expect_exit": 3, "stdout_contains": "" }
        ] }),
    )
    .expect("a smoke run over two passing rows");
    assert_eq!(out.kind, "SmokeResult", "the row's declared outputs kind");
    let result: SmokeResult =
        serde_json::from_value(out.value).expect("the value decodes as SmokeResult");
    assert_eq!((result.passed, result.failed), (2, 0), "{result:?}");
    assert_eq!(result.status, SmokeStatus::Passing);
    assert_eq!(result.rows.len(), 2);
    let built = scratch.path().join("target");
    for row in &result.rows {
        assert!(row.passed && row.predicate_held, "{row:?}");
        let program = Path::new(&row.argv[0]);
        assert!(
            program.starts_with(&built),
            "the row ran the binary the call's build produced (under the adapter's target dir {}), \
             never an installed one: {row:?}",
            built.display()
        );
        let cwd = Path::new(&row.cwd);
        assert!(
            cwd.starts_with(scratch.path())
                && cwd
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with("smoke-")),
            "cwd = \"scratch\" resolved to one smoke-<unique> directory under the adapter's \
             scratch: {row:?}"
        );
    }
    assert_eq!(result.rows[0].exit, 0);
    assert_eq!(result.rows[1].exit, 3);
    assert!(
        smoke_dirs(scratch.path()).is_empty(),
        "the scratch directory is removed on success: {:?}",
        smoke_dirs(scratch.path())
    );
}

/// **A failing smoke row is a failing RESULT, not an error** (record V decision 4, 1a's rule), and
/// the scratch directory is removed on that path too.
#[test]
fn a_failing_smoke_row_is_a_failing_result_not_an_error() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = workspace_adapter(&fixture_workspace(), scratch.path());
    let out = run_verb(
        &tc,
        "smoke",
        &serde_json::json!({ "checklist": [
            { "id": "passes", "argv": ["wsbin"], "cwd": "scratch", "expect_exit": 0, "stdout_contains": "wsbin ok" },
            { "id": "wrong-exit", "argv": ["wsbin", "--exit", "4"], "cwd": "scratch", "expect_exit": 0, "stdout_contains": "" },
            { "id": "wrong-stdout", "argv": ["wsbin", "--version"], "cwd": "scratch", "expect_exit": 0, "stdout_contains": "redkiln" }
        ] }),
    )
    .expect("a failing row is Ok with failed >= 1, never Err");
    let result: SmokeResult = serde_json::from_value(out.value).expect("decodes");
    assert_eq!((result.passed, result.failed), (1, 2), "{result:?}");
    assert_eq!(result.status, SmokeStatus::Failing);
    let wrong_exit = &result.rows[1];
    assert!(!wrong_exit.passed && wrong_exit.exit == 4, "{wrong_exit:?}");
    let wrong_stdout = &result.rows[2];
    assert!(
        !wrong_stdout.passed && !wrong_stdout.predicate_held && wrong_stdout.exit == 0,
        "{wrong_stdout:?}"
    );
    assert!(
        smoke_dirs(scratch.path()).is_empty(),
        "the scratch directory is removed on failure: {:?}",
        smoke_dirs(scratch.path())
    );

    // A row naming a binary the build did not produce cannot run: Err, and still no leak.
    let err = run_verb(
        &tc,
        "smoke",
        &serde_json::json!({ "checklist": [
            { "id": "ghost", "argv": ["redkiln", "--version"], "cwd": "scratch", "expect_exit": 0, "stdout_contains": "" }
        ] }),
    )
    .expect_err("a binary the build did not produce is a spawn that cannot run");
    refused_naming(&err, &["redkiln"], "an unbuilt binary");
    assert!(
        smoke_dirs(scratch.path()).is_empty(),
        "no leak on the Err path"
    );
}

// ---------------------------------------------------------------- sca_scan

fn scan_adapter(scratch: &Path) -> RustToolchain {
    workspace_adapter(&advisory_fixture().join("project"), scratch)
}

fn scan_inputs(audit: &Path, deny_parent: &Path) -> serde_json::Value {
    serde_json::json!({
        "audit_db": audit.to_string_lossy(),
        "deny_db": deny_parent.to_string_lossy(),
    })
}

/// **An unrated advisory is counted in `unrated`, never as zero** (record V decision 5; M8's kill:
/// an unrated advisory counted as `low`).
#[test]
fn sca_scan_counts_an_unrated_advisory_as_unrated_never_zero() {
    let tmp = tempfile::tempdir().expect("tmp");
    let (audit, deny_parent, _, _) = scan_databases(tmp.path());
    let tc = scan_adapter(&tmp.path().join("scratch"));
    let out = run_verb(&tc, "sca_scan", &scan_inputs(&audit, &deny_parent))
        .expect("an offline scan over the fixture databases");
    assert_eq!(out.kind, "ScanReport");
    let report: ScanReport = serde_json::from_value(out.value).expect("decodes");
    assert_eq!(
        report.unrated, 1,
        "RUSTSEC-2099-0002 carries no CVSS: {report:?}"
    );
    assert_eq!(
        report.critical, 1,
        "RUSTSEC-2099-0001 scores 10.0: {report:?}"
    );
    assert_eq!(
        (report.high, report.medium, report.low),
        (0, 0, 0),
        "{report:?}"
    );
    let ids: Vec<(&str, Option<Severity>)> = report
        .advisories
        .iter()
        .map(|a| (a.id.as_str(), a.severity))
        .collect();
    assert_eq!(
        ids,
        [
            ("RUSTSEC-2099-0001", Some(Severity::Critical)),
            ("RUSTSEC-2099-0002", None)
        ],
        "one entry per advisory, the unrated one kept (never dropped)"
    );
    for a in &report.advisories {
        assert_eq!((a.package.as_str(), a.version.as_str()), ("itoa", "1.0.18"));
    }

    // The scorer's bands, over CVSS v3 vectors (record V decision 5).
    for (vector, band) in [
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:H",
            Some(Severity::Critical),
        ),
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
            Some(Severity::Critical),
        ),
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
            Some(Severity::High),
        ),
        (
            "CVSS:3.0/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:H",
            Some(Severity::High),
        ),
        (
            "CVSS:3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:L/I:L/A:L",
            Some(Severity::Medium),
        ),
        (
            "CVSS:3.1/AV:P/AC:H/PR:H/UI:R/S:U/C:L/I:N/A:N",
            Some(Severity::Low),
        ),
        (
            "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:N",
            Some(Severity::Low),
        ),
    ] {
        assert_eq!(
            severity_of(Some(vector)).expect("a v3 vector scores"),
            band,
            "{vector}"
        );
    }
    assert_eq!(
        severity_of(None).expect("no vector"),
        None,
        "no CVSS is unrated"
    );
    let v4 = severity_of(Some(
        "CVSS:4.0/AV:N/AC:L/AT:N/PR:N/UI:N/VC:H/VI:H/VA:H/SC:N/SI:N/SA:N",
    ))
    .expect_err("a CVSS v4.0 vector is refused by name, never scored as unrated or low");
    refused_naming(&v4, &["CVSS:4.0"], "a v4 vector");
}

/// **The scan records each tool's database commit, read by the adapter** (D21-42 6a; D21-62 C):
/// audit's is copy A's `HEAD` and deny's is copy B's, as an independent `git rev-parse HEAD` reads
/// them, which kills a constant commit, a swap and a read of `~/.cargo`.
#[test]
fn sca_scan_records_the_advisory_database_commit() {
    let tmp = tempfile::tempdir().expect("tmp");
    let (audit, deny_parent, audit_commit, deny_commit) = scan_databases(tmp.path());
    let tc = scan_adapter(&tmp.path().join("scratch"));
    let out = run_verb(&tc, "sca_scan", &scan_inputs(&audit, &deny_parent)).expect("a scan");
    let report: ScanReport = serde_json::from_value(out.value).expect("decodes");
    assert_eq!(report.tools.len(), 2, "two tools, two pins: {report:?}");
    let audit_tool = &report.tools[0];
    let deny_tool = &report.tools[1];
    assert_eq!(audit_tool.name, "cargo-audit");
    assert_eq!(deny_tool.name, "cargo-deny");
    assert_eq!(audit_tool.version, "0.22.2", "the pinned cargo-audit");
    assert_eq!(deny_tool.version, "0.20.2", "the pinned cargo-deny");
    assert_eq!(audit_tool.db_commit, audit_commit, "audit read copy A");
    assert_eq!(deny_tool.db_commit, deny_commit, "deny read copy B");
    assert!(
        same_path(&audit_tool.db_path, &audit.to_string_lossy()),
        "{audit_tool:?}"
    );
    assert!(
        same_path(
            &deny_tool.db_path,
            &deny_parent
                .join(crate::support::DENY_DB_CHILD)
                .to_string_lossy()
        ),
        "deny's database is the db-path's only child: {deny_tool:?}"
    );
}

/// **A database that is not a git checkout is refused by name** (D21-62 C refusal (a)); no
/// `ScanReport` is recorded.
#[test]
fn sca_scan_refuses_a_non_git_database() {
    let tmp = tempfile::tempdir().expect("tmp");
    let (_, deny_parent, _, _) = scan_databases(tmp.path());
    let plain = tmp.path().join("plain-db");
    copy_tree(&advisory_fixture().join("db"), &plain);
    let tc = scan_adapter(&tmp.path().join("scratch"));
    let err = run_verb(&tc, "sca_scan", &scan_inputs(&plain, &deny_parent))
        .expect_err("a plain directory has no commit to record");
    refused_naming(
        &err,
        &["database-not-a-checkout", "plain-db"],
        "a non-git audit database",
    );
}

/// **A deny `db-path` holding other than one database is refused by name** (D21-62 C refusal (d)).
#[test]
fn sca_scan_refuses_a_deny_db_path_with_other_than_one_database() {
    let tmp = tempfile::tempdir().expect("tmp");
    let (audit, deny_parent, _, _) = scan_databases(tmp.path());
    std::fs::create_dir_all(deny_parent.join("advisory-db-second")).expect("a second child");
    let tc = scan_adapter(&tmp.path().join("scratch"));
    let err = run_verb(&tc, "sca_scan", &scan_inputs(&audit, &deny_parent))
        .expect_err("two databases under deny's db-path");
    refused_naming(&err, &["deny-db-path-children", "2"], "a two-child db-path");
    let empty = tmp.path().join("empty-dbs");
    std::fs::create_dir_all(&empty).expect("an empty parent");
    let err = run_verb(&tc, "sca_scan", &scan_inputs(&audit, &empty))
        .expect_err("no database under deny's db-path");
    refused_naming(&err, &["deny-db-path-children", "0"], "an empty db-path");
}

/// **A database that moved mid-scan is refused by name** (D21-62 C refusal (b)). Asserted over the
/// COMPARATOR only, and said so: the adapter carries no seam that moves `HEAD` between its two reads
/// (record V leaves that seam to the builder; none is added), so the end-to-end case is not claimed.
#[test]
fn sca_scan_refuses_a_database_moved_mid_scan() {
    let a = "a".repeat(40);
    let b = "b".repeat(40);
    db_unmoved("cargo-audit", Path::new("D:/db"), &a, &a).expect("an unmoved database passes");
    let err = db_unmoved("cargo-deny", Path::new("D:/db"), &a, &b)
        .expect_err("a moved database is refused");
    refused_naming(
        &err,
        &["database-moved-mid-scan", "cargo-deny", &a, &b],
        "a database whose HEAD moved",
    );
}

// ---------------------------------------------------------------- package

/// A git copy of the workspace fixture, committed; returns its root and `HEAD`.
fn committed_workspace(tmp: &Path) -> (PathBuf, String) {
    let root = tmp.join("ws");
    copy_tree(&fixture_workspace(), &root);
    let head = git_commit_all(&tmp.join("gh"), &root, "the workspace");
    (root, head)
}

/// **`package` digests only the DECLARED binaries of a locked release build** (record V decision 6),
/// and refuses a declared binary the build did not produce, a `sha`/`HEAD` mismatch, and an
/// undeclared set, each by name.
#[test]
fn package_digests_only_the_declared_binaries_of_a_locked_release_build() {
    let tmp = crate::support::repo_tempdir();
    let (root, head) = committed_workspace(tmp.path());
    let scratch = tmp.path().join("scratch");
    let tc = workspace_adapter(&root, &scratch);
    let out = run_verb(
        &tc,
        "package",
        &serde_json::json!({ "sha": head, "binaries": ["wsbin"] }),
    )
    .expect("a locked release build of the declared set");
    assert_eq!(out.kind, "BuildArtifact");
    let artifact: BuildArtifact = serde_json::from_value(out.value).expect("decodes");
    assert_eq!(artifact.profile, "release");
    assert_eq!(artifact.sha, head);
    assert_eq!(
        artifact.head, head,
        "the project root's HEAD, read through rk-git"
    );
    assert_eq!(
        &artifact.argv[1..],
        ["build", "--release", "--locked", "--message-format=json"],
        "record V decision 6's vector"
    );
    assert_eq!(
        artifact.binaries.len(),
        1,
        "only the declared set: {artifact:?}"
    );
    let bin = &artifact.binaries[0];
    assert_eq!(bin.name, "wsbin");
    let exe = if cfg!(windows) { "wsbin.exe" } else { "wsbin" };
    assert_eq!(
        bin.path,
        format!("release/{exe}"),
        "relative to the target directory"
    );
    let bytes = std::fs::read(scratch.join("target").join("release").join(exe))
        .expect("the built binary is there");
    assert_eq!(
        bin.sha256,
        sha256(&bytes).as_str(),
        "the plain sha256 of the binary's bytes"
    );

    let refuse = |inputs: serde_json::Value, needles: &[&str], what: &str| {
        let err = run_verb(&tc, "package", &inputs).expect_err(what);
        refused_naming(&err, needles, what);
    };
    refuse(
        serde_json::json!({ "sha": head, "binaries": ["wsbin", "nosuchbin"] }),
        &["binary-not-built", "nosuchbin"],
        "a declared binary the build did not produce",
    );
    refuse(
        serde_json::json!({ "sha": "0".repeat(40), "binaries": ["wsbin"] }),
        &["sha-head-mismatch", &head],
        "a sha that is not the tree's HEAD",
    );
    refuse(
        serde_json::json!({ "sha": head, "binaries": [] }),
        &["undeclared-binary-set"],
        "an empty declared set",
    );
    refuse(
        serde_json::json!({ "sha": head }),
        &["undeclared-binary-set"],
        "no declared set at all",
    );
}

// ---------------------------------------------------------------- setup

const NODE_PROJECT: &str = "crates/rk-adapter-toolchain-node/tests/fixtures/project";

fn setup_adapter(root: &Path, scratch: &Path, pnpm: Option<PathBuf>) -> RustToolchain {
    let mut config = Config::new(
        DEFAULT_PROGRAM.into(),
        GENEROUS,
        root,
        minimal_env(),
        scratch.to_path_buf(),
    )
    .expect("a Config constructs")
    .with_runner(RunnerPolicy::Force(ForcedRunner::CargoTest));
    if let Some(pnpm) = pnpm {
        config = config.with_tool_program("pnpm", pnpm);
    }
    RustToolchain::new(config).expect("the shipped registry loads")
}

/// **`setup` installs each declared Node project, frozen and offline** (record V decision 7), and
/// records the lockfile it installed from.
#[test]
fn setup_installs_each_declared_node_project_offline() {
    let root = repo_root();
    assert!(
        root.join(NODE_PROJECT).join("node_modules/.bin").is_dir(),
        "PREFLIGHT: the Node fixture is installed once (Checkpoint 8c (2))"
    );
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = setup_adapter(&root, scratch.path(), None);
    let out = run_verb(
        &tc,
        "setup",
        &serde_json::json!({ "projects": [NODE_PROJECT] }),
    )
    .expect("the declared project installs from its lockfile");
    assert_eq!(out.kind, "SetupResult");
    let result: SetupResult = serde_json::from_value(out.value).expect("decodes");
    assert_eq!(result.toolchain, "rust");
    assert_eq!(result.projects.len(), 1);
    let row = &result.projects[0];
    assert_eq!(row.path, NODE_PROJECT);
    assert_eq!(
        &row.argv[1..],
        [
            "install",
            "--frozen-lockfile",
            "--prefer-offline",
            "--dir",
            NODE_PROJECT
        ],
        "record V decision 7's vector"
    );
    let lock = std::fs::read(root.join(NODE_PROJECT).join("pnpm-lock.yaml")).expect("the lockfile");
    assert_eq!(row.lockfile_sha256, sha256(&lock).as_str());
}

/// **A failing setup is refused naming the project** and the exit code; never `Ok` with a failed
/// row (M16's kill). The project declares a dependency and carries no lockfile, so a frozen
/// install refuses before any fetch.
#[test]
fn a_failing_setup_is_refused_naming_the_project() {
    let tmp = tempfile::tempdir().expect("tmp");
    let root = tmp.path().join("root");
    std::fs::create_dir_all(root.join("web")).expect("mkdir the project");
    std::fs::write(
        root.join("web/package.json"),
        "{ \"name\": \"web\", \"version\": \"0.0.0\", \"private\": true, \"dependencies\": { \"left-pad\": \"1.3.0\" } }\n",
    )
    .expect("write package.json");
    let tc = setup_adapter(&root, &tmp.path().join("scratch"), None);
    let err = run_verb(&tc, "setup", &serde_json::json!({ "projects": ["web"] }))
        .expect_err("a frozen install with no lockfile fails, and setup refuses");
    refused_naming(&err, &["setup", "`web`", "exit"], "a failed install");
}

/// **An empty declared set is `Ok` with no projects**, and spawns nothing: the `pnpm` program is an
/// absolute path that does not exist, so any spawn would be `Transport { Spawn }`.
#[test]
fn an_empty_setup_set_is_ok_with_no_projects() {
    let tmp = tempfile::tempdir().expect("tmp");
    let tc = setup_adapter(
        &repo_root(),
        &tmp.path().join("scratch"),
        Some(tmp.path().join("no-such-pnpm.exe")),
    );
    let out =
        run_verb(&tc, "setup", &serde_json::json!({ "projects": [] })).expect("an empty set is Ok");
    let result: SetupResult = serde_json::from_value(out.value).expect("decodes");
    assert!(result.projects.is_empty(), "{result:?}");
    assert_eq!(result.toolchain, "rust");
    // The control: the same absent program over a non-empty set IS a spawn failure.
    match run_verb(
        &tc,
        "setup",
        &serde_json::json!({ "projects": [NODE_PROJECT] }),
    ) {
        Err(StepError::Transport {
            kind: TransportKind::Spawn,
            ..
        }) => {}
        other => {
            panic!("CONTROL: an absent pnpm over a declared project cannot start; got {other:?}")
        }
    }
}

// ---------------------------------------------------------------- refusals

/// **An undeclared verb is refused by its bare name** (record V decision 2; ADR-0135 clause 2): the
/// registry's `store` and `git` rows, the rowless verbs and `mutate` (no patch applier).
#[test]
fn an_undeclared_verb_is_refused_by_name() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = workspace_adapter(&fixture_workspace(), scratch.path());
    for verb in [
        "item_create",
        "release_tag",
        "mutate",
        "release_record",
        "deploy",
        "test",
        "build",
        "",
    ] {
        match run_verb(&tc, verb, &serde_json::json!({})) {
            Err(StepError::Unsupported { capability }) => {
                assert_eq!(capability, verb, "refused by the BARE name");
            }
            other => panic!("`{verb}` is refused by name; got {other:?}"),
        }
    }
}
