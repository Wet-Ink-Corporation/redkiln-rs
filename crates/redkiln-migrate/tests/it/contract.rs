//! The run contract: the refusal, the worktree, the write phase and the one commit
//! (AC-447, AC-448, AC-522, D18-11, D18-12; `s7d` §7d.11).

use std::path::Path;

use redkiln_migrate::run_contract::{
    self, Completed, Hooks, MigrateError, PHASE_WRITTEN_MARKER, Refusal,
};

use crate::{git_out, legacy_mini_repo, porcelain, refs};

/// The migration branch this target's runs land on.
fn migration_branch(repo: &Path) -> String {
    format!(
        "{}{}",
        run_contract::MIGRATION_BRANCH_PREFIX,
        git_out(repo, &["rev-parse", "--short", "HEAD"])
    )
}

/// `<common-dir>/redkiln/migrate-wt`.
fn migrate_wt(repo: &Path) -> std::path::PathBuf {
    Path::new(&git_out(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    ))
    .join("redkiln")
    .join("migrate-wt")
}

/// Commits on the migration branch that are not on the target's HEAD.
fn commits_ahead(repo: &Path, branch: &str) -> u32 {
    let head = git_out(repo, &["rev-parse", "HEAD"]);
    let range = format!("{head}..{branch}");
    git_out(repo, &["rev-list", "--count", &range])
        .parse()
        .expect("a count")
}

// ---- AC-522 -----------------------------------------------------------------------------------

/// AC-522, the 153-line file's name, kept.
///
/// Two variants — one untracked file, then one modified file — each refused **naming the path**,
/// with `git status --porcelain` equal before and after and **no** `rk-migrate/*` ref and **no**
/// `migrate-wt` directory afterwards.
///
/// The exit/`Err` alone is the weak red: a warn-and-continue implementation that wrote the layout
/// and *then* complained would also return an error. The side-effect assertions are what make it
/// discriminating, so both are asserted, in that order (§7d.14's two-things rule).
#[test]
fn refuses_dirty_target() {
    for (label, dirty) in [("untracked", true), ("modified", false)] {
        let repo = legacy_mini_repo();
        let path = repo.path();

        if dirty {
            std::fs::write(path.join("UNTRACKED.md"), b"an untracked file\n").expect("write");
        } else {
            let existing = path.join(".redkiln").join("config.yaml");
            let mut bytes = std::fs::read(&existing).expect("read");
            bytes.extend_from_slice(b"# modified\n");
            std::fs::write(&existing, bytes).expect("write");
        }

        let before = porcelain(path);
        assert!(
            !before.is_empty(),
            "[{label}] the target is not actually dirty — the refusal below would be vacuous"
        );

        // NOT `expect_err("[{label}] …")`: that takes a plain &str, so the label would never
        // interpolate and the message would name no variant. clippy caught it.
        let err = match run_contract::run(path, false, Hooks::default()) {
            Ok(c) => panic!("[{label}] a dirty target was NOT refused: {c:?}"),
            Err(e) => e,
        };

        // (a) the refusal, and it NAMES the dirty path.
        let MigrateError::Refused(Refusal::DirtyTarget { paths }) = &err else {
            panic!("[{label}] the error is not a DirtyTarget refusal: {err:?}");
        };
        assert!(
            !paths.is_empty(),
            "[{label}] the refusal named no path — AC-522 requires every dirty path by name"
        );
        let named = paths.join("\n");
        let expected = if dirty { "UNTRACKED.md" } else { "config.yaml" };
        assert!(
            named.contains(expected),
            "[{label}] the refusal does not name {expected}:\n{named}"
        );

        // (b) the side effects, which are what a warn-and-continue implementation fails.
        assert_eq!(
            porcelain(path),
            before,
            "[{label}] the porcelain listing changed across a refusal — something was written"
        );
        let migration_refs: Vec<String> = refs(path)
            .into_iter()
            .filter(|r| r.contains("rk-migrate/"))
            .collect();
        assert!(
            migration_refs.is_empty(),
            "[{label}] the refused run created {migration_refs:?}"
        );
        assert!(
            !migrate_wt(path).exists(),
            "[{label}] the refused run created the migration worktree"
        );
    }
}

/// AC-522, D18-11, ADR-0105 clause 6.
///
/// A stale `migrate-wt/` left by a killed run is **removed and re-added**, never reused, and the
/// target's own branch and working tree are byte-identical before and after.
#[test]
fn ac522_worktree_recreated_never_reused() {
    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);

    // A first run, to create the branch and leave a worktree behind.
    run_contract::run(path, false, Hooks::default()).expect("the first run commits");
    let wt = migrate_wt(path);
    assert!(wt.exists(), "the first run left no worktree to reuse");

    // A stale file, of the kind a killed run leaves: written, uncommitted, and one `git worktree
    // remove` without --force refuses to delete.
    let stale = wt.join("STALE-FROM-A-KILLED-RUN.md");
    std::fs::write(&stale, b"left behind\n").expect("write");

    let head_before = git_out(path, &["rev-parse", "HEAD"]);
    let branch_name_before = git_out(path, &["rev-parse", "--abbrev-ref", "HEAD"]);
    let porcelain_before = porcelain(path);

    run_contract::run(path, false, Hooks::default()).expect("the second run completes");

    // (a) the stale file is GONE: the worktree was re-created, not reused.
    assert!(
        !stale.exists(),
        "a stale file from a killed run survived into the next run's worktree — the worktree was \
         REUSED. ADR-0105 clause 6 re-creates it, and `rk_git::worktree::remove` passes no \
         --force, which is why the teardown takes the raw `worktree remove --force` form"
    );
    assert!(wt.exists(), "the second run left no worktree at all");

    // (b) the target is untouched: its branch, its HEAD and its working tree.
    assert_eq!(
        git_out(path, &["rev-parse", "HEAD"]),
        head_before,
        "the target's HEAD moved — the commit must land on the migration branch only"
    );
    assert_eq!(
        git_out(path, &["rev-parse", "--abbrev-ref", "HEAD"]),
        branch_name_before,
        "the target's checked-out branch changed"
    );
    assert_eq!(
        porcelain(path),
        porcelain_before,
        "the target's working tree changed across the run"
    );
    assert!(
        refs(path).iter().any(|r| r.ends_with(&branch)),
        "the migration branch {branch} does not exist after two runs"
    );
}

// ---- AC-447 -----------------------------------------------------------------------------------

/// AC-447, AC-522: *"exactly one commit is created on the target branch — the dedicated migration
/// branch/worktree AC-522 requires."*
#[test]
fn ac447_exactly_one_commit_on_migration_branch() {
    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);
    let head_before = git_out(path, &["rev-parse", "HEAD"]);

    let completed = run_contract::run(path, false, Hooks::default()).expect("the run commits");
    let Completed::Committed { branch: on, .. } = &completed else {
        panic!("the run did not commit: {completed:?}");
    };
    assert_eq!(on, &branch, "the commit landed on the wrong branch");

    assert_eq!(
        commits_ahead(path, &branch),
        1,
        "the migration branch is not exactly ONE commit ahead of the target's HEAD"
    );
    assert_eq!(
        git_out(path, &["rev-parse", "HEAD"]),
        head_before,
        "a commit landed on the target's ORIGINAL branch — it never may (AC-476's AND clause, \
         which AC-522 instruments)"
    );
}

/// AC-447's AND: *"a run that would produce more than one commit aborts before writing."*
#[test]
fn ac447_second_write_phase_aborts_before_writing() {
    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);

    run_contract::run(path, false, Hooks::default()).expect("the first run commits");
    let after_first = git_out(path, &["rev-parse", &branch]);
    let wt = migrate_wt(path);
    let listing_before = listing(&wt);

    let err = run_contract::run(
        path,
        false,
        Hooks {
            request_second_write_phase: true,
            ..Hooks::default()
        },
    )
    .expect_err("a second write phase is refused");
    assert!(
        matches!(err, MigrateError::Refused(Refusal::SecondWritePhase)),
        "the error is not a SecondWritePhase refusal: {err:?}"
    );

    // BEFORE writing: nothing in the worktree moved, and no commit landed.
    assert_eq!(
        listing(&wt),
        listing_before,
        "the second write phase wrote into the worktree before aborting — AC-447 aborts BEFORE \
         writing, which is the half a post-hoc check would miss"
    );
    assert_eq!(
        git_out(path, &["rev-parse", &branch]),
        after_first,
        "a second commit landed on the migration branch"
    );
    assert_eq!(commits_ahead(path, &branch), 1, "more than one commit");
}

/// Every file under `root`, as `(relative path, bytes)`, sorted — the instrument for
/// "nothing was written".
fn listing(root: &Path) -> Vec<(String, Vec<u8>)> {
    if !root.is_dir() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a worktree directory") {
            let p = entry.expect("an entry").path();
            // `.git` in a linked worktree is a FILE pointing at the common dir, and git rewrites
            // its own bookkeeping; the assertion is about what the migrator wrote.
            if p.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p
                    .strip_prefix(root)
                    .expect("under root")
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push((rel, std::fs::read(&p).expect("a file")));
            }
        }
    }
    out.sort();
    out
}

// ---- AC-448 -----------------------------------------------------------------------------------

/// AC-448, AC-467: the report lands **in the same commit** and validates against the committed
/// schema.
#[test]
fn ac448_report_lands_in_the_same_commit_and_validates() {
    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);

    run_contract::run(path, false, Hooks::default()).expect("the run commits");

    // The COMMIT'S TREE, not the worktree: a report written and not committed would pass a
    // filesystem check and fail the criterion.
    let tree = git_out(path, &["ls-tree", "-r", "--name-only", &branch]);
    assert!(
        tree.lines().any(|l| l == "report.json"),
        "report.json is not in the migration commit's tree:\n{tree}"
    );

    let blob = git_out(path, &["show", &format!("{branch}:report.json")]);
    let instance: serde_json::Value =
        serde_json::from_str(&blob).expect("the committed report is JSON");
    let schema: serde_json::Value =
        serde_json::from_str(redkiln_migrate::report::REPORT_SCHEMA).expect("the schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the schema compiles");
    let errors: Vec<String> = validator
        .iter_errors(&instance)
        .map(|e| e.to_string())
        .collect();
    assert!(
        errors.is_empty(),
        "the COMMITTED report does not validate against the committed schema: {errors:?}"
    );
}

/// AC-448's AND: *"a run that writes the new layout without emitting a report is refused."*
#[test]
fn ac448_write_without_report_is_refused() {
    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);

    let err = run_contract::run(
        path,
        false,
        Hooks {
            disable_report_step: true,
            ..Hooks::default()
        },
    )
    .expect_err("a run with the report step disabled is refused");
    assert!(
        matches!(err, MigrateError::Refused(Refusal::NoReport)),
        "the error is not a NoReport refusal: {err:?}"
    );

    // No commit landed. The branch may exist (the worktree created it) but must carry nothing.
    let exists = refs(path).iter().any(|r| r.ends_with(&branch));
    if exists {
        assert_eq!(
            commits_ahead(path, &branch),
            0,
            "a commit landed with the report step disabled (AC-448)"
        );
    }
}

// ---- D18-12 -----------------------------------------------------------------------------------

/// D18-12: the `phase: written` marker is printed **before** the commit.
///
/// In process, the ordering is read off the contract itself: the marker is printed inside the run,
/// and the branch carries no commit until the run returns. The spawned, kill-timed arm is
/// `idempotent_across_kill`'s.
#[test]
fn phase_written_marker_is_printed_before_commit() {
    assert_eq!(
        PHASE_WRITTEN_MARKER, "phase: written",
        "the marker's text is what a killer reads on stdout; changing it is a contract change"
    );

    let repo = legacy_mini_repo();
    let path = repo.path();
    let branch = migration_branch(path);

    // Before the run: no ref at all.
    assert!(
        !refs(path).iter().any(|r| r.ends_with(&branch)),
        "the migration branch exists before the first run"
    );

    let completed = run_contract::run(path, false, Hooks::default()).expect("the run commits");
    assert!(matches!(completed, Completed::Committed { .. }));

    // The marker is a PRINT, never a flag or an environment toggle (D18-12): it must not appear in
    // the binary's flag surface, which `ac474_…` snapshots at cycle 5.
    let src = std::fs::read_to_string(crate::src_dir().join("run_contract.rs")).expect("src");
    let code = crate::code_lines(&src);
    let printed = code
        .iter()
        .filter(|(_, l)| l.contains("PHASE_WRITTEN_MARKER") && l.contains("println!"))
        .count();
    assert_eq!(
        printed, 1,
        "the marker is printed {printed} times on a non-comment line, not exactly once — it is a \
         print, never a flag (D18-12)"
    );
    // And it is printed BEFORE the commit call, in the file's own order.
    let marker_line = code
        .iter()
        .find(|(_, l)| l.contains("println!") && l.contains("PHASE_WRITTEN_MARKER"))
        .map(|(n, _)| *n)
        .expect("the marker print");
    let commit_line = code
        .iter()
        .find(|(_, l)| l.contains("commit_with_trailers("))
        .map(|(n, _)| *n)
        .expect("the commit call");
    assert!(
        marker_line < commit_line,
        "the marker is printed at line {marker_line}, AFTER the commit at {commit_line} — a \
         killer that waits for the marker would then be racing a commit that already landed"
    );
}
