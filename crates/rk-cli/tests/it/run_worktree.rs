//! **The run-owned worktree, as `run` creates, re-enters, carries and reviews it** (`s8d-ii`
//! §8d-ii.1, §8d-ii.3 and §8d-ii.4; D21-44; D21-45 W2 and W3; D21-48 X14; ADR-0180, proposed).
//!
//! Every path here is derived through `rk_git::worktree::derive` ([`crate::run_tree`]), never a
//! joined literal: Checkpoint 8d-ii (3)'s root floor reads `crates/*/tests` as well as `src`.
//!
//! *(dated 2026-09-27, `s8d-ii` build, P6.)*

use std::path::{Path, PathBuf};

use crate::support::{PackVariant, pack_min_repo};

/// `git <args>` in `cwd`, its trimmed stdout. Panics on a non-zero exit, naming stderr. Through
/// `rk-git`'s one process-exec site (`support::git_ok`; ADR-0077; D21-128 A1).
fn git_out(cwd: &Path, args: &[&str]) -> String {
    let out = crate::support::git_ok(cwd, args);
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// `git <args>` in `cwd`, its exit code only (`support::git_code`; D21-128 A1/A2).
fn git_rc(cwd: &Path, args: &[&str]) -> Option<i32> {
    crate::support::git_code(cwd, args)
}

/// A path as git prints it and as the OS spells it, folded to one comparable form.
fn fold(p: &str) -> String {
    p.replace('\\', "/").trim_end_matches('/').to_lowercase()
}

/// The `git worktree list --porcelain` record for `path`, as its lines, or `None`.
fn worktree_record(root: &Path, path: &Path) -> Option<Vec<String>> {
    let listing = git_out(root, &["worktree", "list", "--porcelain"]);
    let want = fold(&path.display().to_string());
    listing
        .split("\n\n")
        .map(|rec| rec.lines().map(str::to_owned).collect::<Vec<String>>())
        .find(|lines| {
            lines
                .first()
                .and_then(|l| l.strip_prefix("worktree "))
                .is_some_and(|p| fold(p) == want)
        })
}

/// The `RK_BASE` marker in the tree's own gitdir, as `(fork, item)`.
fn rk_base(tree: &Path) -> Option<(String, String)> {
    let gitdir = git_out(tree, &["rev-parse", "--path-format=absolute", "--git-dir"]);
    let text = std::fs::read_to_string(PathBuf::from(gitdir).join("RK_BASE")).ok()?;
    let mut fork = None;
    let mut item = None;
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("fork ") {
            fork = Some(v.to_owned());
        } else if let Some(v) = line.strip_prefix("item ") {
            item = Some(v.to_owned());
        }
    }
    fork.zip(item)
}

/// **D21-44 / ADR-0180 decision 1: `run` creates the run tree at `.redkiln/worktrees/<item>` under
/// the main worktree, on `rk-run/<item>`, marked and unlocked at the run's end; and a second run
/// re-enters it, discards its in-flight changes, and names each discarded path on stderr** (W2;
/// §8d-ii.3: *"`run` prints each discarded path on stderr, one line per path"* until `s8e`).
///
/// Red at stage entry because the tree is created under the common dir
/// (`<common-dir>/redkiln/worktrees/<item>`) and re-created by force on every run. Red under
/// `S8D-II-M1` (the root derived under the common dir again).
#[test]
fn the_run_worktree_is_created_under_dot_redkiln_worktrees() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();
    let head = git_out(root, &["rev-parse", "HEAD"]);
    let tree = crate::run_tree(root, "bug-1");
    let root_name = tree
        .path
        .parent()
        .and_then(Path::file_name)
        .expect("the derived tree has a parent directory")
        .to_owned();
    let old_root = crate::common_dir(root).join("redkiln").join(&root_name);

    let first = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&first.stderr);
    assert!(
        tree.path.is_dir(),
        "no run tree at the derived path {} (the old common-dir root {} {}); stderr:\n{stderr}",
        tree.path.display(),
        old_root.display(),
        if old_root.join("bug-1").is_dir() {
            "HOLDS it: the tree was created under the common dir"
        } else {
            "does not hold it either"
        }
    );
    assert!(
        !old_root.exists(),
        "the old common-dir root {} exists after a run; nothing may be created there",
        old_root.display()
    );

    // Registered, on `rk-run/bug-1`, and unlocked once the run has ended (W1's lock is released).
    let record = worktree_record(root, &tree.path).unwrap_or_else(|| {
        panic!(
            "{} is on disk and is not a registered worktree: {}",
            tree.path.display(),
            git_out(root, &["worktree", "list", "--porcelain"])
        )
    });
    assert!(
        record.iter().any(|l| l == "branch refs/heads/rk-run/bug-1"),
        "the run tree is not on rk-run/bug-1: {record:?}"
    );
    assert!(
        !record.iter().any(|l| l.starts_with("locked")),
        "the run's lock was not released at its end: {record:?}"
    );
    // Marked: RK_BASE records the fork (the frozen HEAD) and the item (W1; ADR-0109 decision 4).
    assert_eq!(
        rk_base(&tree.path),
        Some((head.clone(), "bug-1".to_owned())),
        "RK_BASE does not record the fork sha and the item"
    );
    // Ignored in the user's checkout, so AC-518's `git status --porcelain` holds with it inside.
    let rel = fold(
        &tree
            .path
            .strip_prefix(root)
            .expect("the derived tree lies under the root")
            .display()
            .to_string(),
    );
    assert_eq!(
        git_rc(root, &["check-ignore", "-q", &rel]),
        Some(0),
        "the run tree is not git-ignored in the checkout it sits in"
    );

    // ---- Re-entry: reused, discarded, and each discarded path named on stderr.
    std::fs::write(tree.path.join("in-flight.txt"), b"not landed\n").expect("an in-flight file");
    std::fs::write(tree.path.join("README"), b"edited in flight\n").expect("an in-flight edit");
    let second = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&second.stderr);
    for path in ["in-flight.txt", "README"] {
        assert!(
            stderr
                .lines()
                .any(|l| l.contains("discarded") && l.contains(path)),
            "re-entry did not name the discarded path `{path}` on its own stderr line; \
             stderr:\n{stderr}"
        );
    }
    assert!(
        !tree.path.join("in-flight.txt").exists(),
        "the untracked in-flight file survived re-entry; it is discarded (N10)"
    );
    assert_eq!(
        rk_base(&tree.path),
        Some((head, "bug-1".to_owned())),
        "the re-entered tree is not the same marked tree (re-created rather than reused?)"
    );
}

/// **ADR-0180 decision 17: no redkiln reader sees a planted run tree.** A run tree carries a full
/// copy of `.redkiln/`, so a reader that recursed would read a second catalog, a second set of
/// cards and a second set of items. This plants `.redkiln/worktrees/planted/.redkiln/{catalog,
/// cards,specs,items}` with contents each reader would flag or refuse, and asserts that `doctor`'s
/// findings, the input set (catalog, registries and tree hash) and the item resolution are equal
/// with and without it.
///
/// Green at entry by design: no reader recurses (§8d-ii.0 item 4). `S8D-II-M9` (doctor's card
/// reader made recursive) shows it can fail.
#[test]
fn no_redkiln_reader_sees_a_planted_run_worktree() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();

    let doctor = || {
        let cli = rk_cli::cli::Cli {
            cwd: Some(root.to_path_buf()),
            command: rk_cli::cli::Command::Doctor(rk_cli::cli::DoctorArgs { rebuild: false }),
        };
        let mut sink: Vec<u8> = Vec::new();
        let exit = rk_cli::verbs::doctor::run_to(
            &cli,
            &rk_cli::cli::DoctorArgs { rebuild: false },
            &crate::support::clean_env(),
            rk_cli::verbs::Startup::real(),
            &mut sink,
        )
        .expect("doctor does not halt");
        (
            exit.code(),
            String::from_utf8(sink).expect("doctor renders UTF-8"),
        )
    };
    let reads = || {
        let resolved = crate::support::resolved_over_pack_min(root, "bug-1");
        let item = format!(
            "{:?} {} {}",
            resolved.item, resolved.stage, resolved.pack_src
        );
        (item, crate::support::inputs_over_pack_min(&resolved))
    };

    let doctor_before = doctor();
    // CONTROL: doctor reads the same twice over an unchanged repository, so a difference below is
    // the planted tree's and not the store's growth.
    assert_eq!(
        doctor(),
        doctor_before,
        "PREMISE: doctor's findings differ across two reads of one repository"
    );
    let reads_before = reads();

    let planted = crate::run_tree(root, "planted").path.join(".redkiln");
    for (sub, name, body) in [
        ("catalog", "planted.toml", "this is = not [a catalog\n"),
        (
            "cards",
            "planted.json",
            "{\"description\": \"planted\", \"model\": \"opus\"}\n",
        ),
        (
            "specs",
            "planted.toml",
            "item = \"bug-1\"\nstage = \"implement\"\nverification = \"None\"\n",
        ),
        ("items", "bug-1.toml", "id = \"bug-1\"\nkind = \"story\"\n"),
    ] {
        std::fs::create_dir_all(planted.join(sub)).expect("the planted directory is created");
        std::fs::write(planted.join(sub).join(name), body).expect("the planted file writes");
    }

    assert_eq!(
        doctor(),
        doctor_before,
        "doctor's findings changed once a run tree was planted under the run-tree root — a \
         reader walks into the run tree"
    );
    let reads_after = reads();
    assert_eq!(
        reads_after.0, reads_before.0,
        "the item resolution changed once a run tree was planted"
    );
    assert!(
        reads_after.1 == reads_before.1,
        "the input set (catalog, registries, tree hash) changed once a run tree was planted"
    );
}

/// The review fixture's committing transcript (AC-438's `exit/findings-block/`), as the child's
/// replay mode: a run of `review-1` over [`PackVariant::Review`] reaches the review pass and exits
/// `4` on its one blocking finding.
fn review_mode() -> String {
    let transcript = crate::fixtures_dir()
        .join("exit")
        .join("findings-block")
        .join("frames.jsonl");
    assert!(
        transcript.is_file(),
        "PREMISE: {} exists",
        transcript.display()
    );
    format!("replay:{}", transcript.display())
}

/// The reference arm's records, one `(HEAD, RK_BASE fork, RK_BASE item)` per pass, as the rigged
/// stub toolchain appends them to `<common-dir>/rk-fixture-reference-heads`.
fn reference_records(root: &Path) -> Vec<(String, String, String)> {
    std::fs::read_to_string(crate::common_dir(root).join("rk-fixture-reference-heads"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let mut f = l.split_whitespace();
            Some((
                f.next()?.to_owned(),
                f.next()?.to_owned(),
                f.next()?.to_owned(),
            ))
        })
        .collect()
}

/// **ADR-0180 decision 10: the review reference tree is created at the fork sha the run tree's
/// `RK_BASE` records, not at `HEAD`, and it carries its own `RK_BASE`** (§8d-ii.3; the adversarial
/// review's M8). Under reuse the run branch forked at an earlier `HEAD` and the operator's checkout
/// may have moved since, so `HEAD` is no longer the merge base `reference_worktree`'s doc argued.
///
/// Two invocations of `run review-1`, with the operator's `HEAD` moved between them. The rigged
/// stub records the reference tree's `HEAD` and marker on each pass. Red at stage entry because the
/// reference tree is created at `HEAD`; red under `S8D-II-M19`.
#[test]
fn the_review_reference_tree_is_created_at_the_rk_base_fork_sha_after_head_moves() {
    let repo = pack_min_repo(PackVariant::Review);
    let root = repo.path();
    let fork = git_out(root, &["rev-parse", "HEAD"]);
    let mode = review_mode();

    let first = crate::run::spawn_run_in_mode(root, "review-1", None, &mode);
    assert_eq!(
        first.status.code(),
        Some(4),
        "PREMISE: the first run reaches the review pass and blocks; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );

    // The operator's checkout moves on while the run tree stands.
    crate::git(
        root,
        &[
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            "the operator's checkout moves on",
        ],
    );
    let moved = git_out(root, &["rev-parse", "HEAD"]);
    assert_ne!(moved, fork, "PREMISE: HEAD moved");

    let second = crate::run::spawn_run_in_mode(root, "review-1", None, &mode);
    let stderr = String::from_utf8_lossy(&second.stderr);
    let records = reference_records(root);
    assert_eq!(
        records.len(),
        2,
        "PREMISE: each run's review pass ran its reference arm once (exit {:?}); records: \
         {records:?}; stderr:\n{stderr}",
        second.status.code()
    );
    assert_eq!(
        records[1].0,
        fork,
        "after HEAD moved to {moved}, the review reference tree was created at {} — not at the \
         run tree's fork {fork} that RK_BASE records (ADR-0180 decision 10){}",
        records[1].0,
        if records[1].0 == moved {
            ": it was created at HEAD"
        } else {
            ""
        }
    );
    assert_eq!(
        (records[1].1.as_str(), records[1].2.as_str()),
        (fork.as_str(), "review-1"),
        "the reference tree carries no RK_BASE of its own naming the fork and the item"
    );
    assert_eq!(
        rk_base(&crate::run_tree(root, "review-1").path),
        Some((fork, "review-1".to_owned())),
        "the run tree's RK_BASE does not record the fork the reference tree was created at"
    );
    // ADR-0162 decision 10: the reference tree is removed on every path.
    let reference = crate::run_tree(root, "review-1").reference;
    assert!(
        !reference.exists() && worktree_record(root, &reference).is_none(),
        "the review reference tree {} outlived the pass (ADR-0162 decision 10)",
        reference.display()
    );
}

/// **ADR-0180 decision 10: a leftover at `<item>.review-ref` is removed only if redkiln made it.**
/// A leftover without `RK_BASE` is refused by name and kept; one that carries the marker is
/// force-removed as scratch, as ADR-0162 decision 10's removal has always done (the control half).
///
/// Red at stage entry because a leftover is force-removed whoever made it.
#[test]
fn a_leftover_reference_tree_without_rk_base_is_refused_and_kept() {
    let repo = pack_min_repo(PackVariant::Review);
    let root = repo.path();
    let mode = review_mode();
    let reference = crate::run_tree(root, "review-1").reference;
    let reference_arg = reference.display().to_string();

    // A tree at the reference path that redkiln did not make: registered, detached, no RK_BASE.
    crate::git(
        root,
        &[
            "worktree",
            "add",
            "--quiet",
            "--detach",
            &reference_arg,
            "HEAD",
        ],
    );
    std::fs::write(reference.join("operator-notes.txt"), b"not redkiln's\n")
        .expect("the leftover holds a file of its own");

    let out = crate::run::spawn_run_in_mode(root, "review-1", None, &mode);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        reference.join("operator-notes.txt").is_file()
            && worktree_record(root, &reference).is_some(),
        "a leftover reference tree WITHOUT RK_BASE was removed (exit {:?}); redkiln removes only \
         a tree it made; stderr:\n{stderr}",
        out.status.code()
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "a leftover reference tree without RK_BASE was not refused; stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("review-1.review-ref") && stderr.contains("RK_BASE"),
        "the refusal does not name the leftover and its missing RK_BASE; stderr:\n{stderr}"
    );

    // CONTROL: the same leftover, once it carries RK_BASE, is scratch redkiln made — it is
    // force-removed, the pass runs, and nothing is left registered (ADR-0162 decision 10).
    let gitdir = git_out(
        &reference,
        &["rev-parse", "--path-format=absolute", "--git-dir"],
    );
    let head = git_out(root, &["rev-parse", "HEAD"]);
    std::fs::write(
        PathBuf::from(gitdir).join("RK_BASE"),
        format!("fork {head}\nitem review-1\n"),
    )
    .expect("the marker writes");
    let control = crate::run::spawn_run_in_mode(root, "review-1", None, &mode);
    assert_eq!(
        control.status.code(),
        Some(4),
        "CONTROL: a marked leftover is scratch and the pass runs; stderr:\n{}",
        String::from_utf8_lossy(&control.stderr)
    );
    assert!(
        !reference.exists() && worktree_record(root, &reference).is_none(),
        "CONTROL: the marked leftover {} was not removed as scratch",
        reference.display()
    );
}

/// The declared Node project the carry tests install into, relative to the repository root.
const SETUP_PROJECT: &str = "web";

/// **A repository whose run creates a tree that the carry reaches** (§8d-ii.4; D21-45 W3; D21-48
/// X14): [`PackVariant::Review`]'s committed `Cargo.toml` and rigged `cargo`, so the Rust adapter
/// detects in the fresh tree; a committed `.gitignore` (`.env`, `node_modules/`) and
/// `.worktreeinclude` (`.env`); an ignored `.env` in the checkout; a committed `web/` project; and a
/// `.redkiln/registry.toml` whose `[toolchain.rust]` declares `setup_projects = ["web"]` and a
/// `setup_timeout`. `web/` is the Node fixture project's `package.json` and `pnpm-lock.yaml` when
/// `installable`, and otherwise a `package.json` with a dependency and no lockfile, which a frozen
/// install refuses before any fetch (`s8c`'s own failing shape).
fn carry_repo(installable: bool) -> tempfile::TempDir {
    let repo = pack_min_repo(PackVariant::Review);
    let root = repo.path();
    std::fs::write(root.join(".gitignore"), b".env\nnode_modules/\n").expect(".gitignore writes");
    std::fs::write(root.join(".worktreeinclude"), b".env\n").expect(".worktreeinclude writes");
    std::fs::create_dir_all(root.join(SETUP_PROJECT)).expect("the project directory is made");
    if installable {
        let node = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("rk-adapter-toolchain-node")
            .join("tests")
            .join("fixtures")
            .join("project");
        for name in ["package.json", "pnpm-lock.yaml"] {
            std::fs::copy(node.join(name), root.join(SETUP_PROJECT).join(name))
                .unwrap_or_else(|e| panic!("the Node fixture's {name} copies: {e}"));
        }
    } else {
        std::fs::write(
            root.join(SETUP_PROJECT).join("package.json"),
            b"{ \"name\": \"web\", \"version\": \"0.0.0\", \"private\": true, \"dependencies\": { \"left-pad\": \"1.3.0\" } }\n",
        )
        .expect("the failing project's package.json writes");
    }
    crate::git(
        root,
        &["add", ".gitignore", ".worktreeinclude", SETUP_PROJECT],
    );
    crate::git(root, &["commit", "--quiet", "-m", "the carry fixture"]);
    std::fs::write(root.join(".env"), b"SECRET=carried\n").expect("the ignored .env writes");
    write_setup_registry(root, SETUP_PROJECT);
    repo
}

/// The carry fixture's `.redkiln/registry.toml`: the `setup` verb row, and `[toolchain.rust]`
/// declaring `setup_projects = [project]` with a `setup_timeout`. It lives in the operator's
/// checkout, which `run` reads it from, so a test can re-point it between runs.
fn write_setup_registry(root: &Path, project: &str) {
    let pin = "0123456789abcdef0123456789abcdef01234567";
    std::fs::write(
        root.join(".redkiln").join("registry.toml"),
        format!(
            "[verb]\nsetup = {{ outputs = \"SetupResult\", live = true, timeout = \
             \"setup_timeout\", inputs = [\"projects\"], route = \"toolchain\", side_effecting = \
             false }}\n[toolchain.rust]\nsetup_timeout = {{ secs = 60, measured = [30], rule = \
             \"r\", pin = \"{pin}\", command = \"c\" }}\nsetup_projects = {{ value = \
             [\"{project}\"], source = \"the carry fixture's one project\" }}\n"
        ),
    )
    .expect("the registry writes");
}

/// **D21-45 W3 / ADR-0180 decision 11: creation carries the `.worktreeinclude` files and runs the
/// `setup` verb** (§8d-ii.4; D21-48 X14: *"s8c builds the reader, and s8d-ii calls it at
/// creation"*), after the scan and before the run proceeds; a reused tree is not re-carried.
///
/// Red at stage entry because creation carries nothing and calls no `setup`.
#[test]
fn creation_carries_worktreeinclude_files_and_runs_setup() {
    let repo = carry_repo(true);
    let root = repo.path();
    let out = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let tree = crate::run_tree(root, "bug-1").path;
    assert!(
        tree.is_dir(),
        "PREMISE: the run created its tree; stderr:\n{stderr}"
    );
    assert_eq!(
        std::fs::read(tree.join(".env")).ok().as_deref(),
        Some(&b"SECRET=carried\n"[..]),
        "the ignored, matched .env was not carried into the fresh run tree; stderr:\n{stderr}"
    );
    let installed = tree
        .join(SETUP_PROJECT)
        .join("node_modules")
        .join(".modules.yaml");
    assert!(
        installed.is_file(),
        "setup did not install the declared project in the run tree ({} is absent); \
         stderr:\n{stderr}",
        installed.display()
    );
    assert!(
        !root.join(SETUP_PROJECT).join("node_modules").exists(),
        "setup installed into the operator's checkout, not the run tree"
    );

    // A reused tree is not re-carried: the carried copy is changed, and re-entry keeps the change
    // (ignored files survive the discard, and no second carry overwrites them).
    std::fs::write(tree.join(".env"), b"SECRET=edited-in-the-tree\n").expect("the copy edits");
    let second = crate::run::spawn_run(root, "bug-1", None);
    assert_eq!(
        std::fs::read(tree.join(".env")).ok().as_deref(),
        Some(&b"SECRET=edited-in-the-tree\n"[..]),
        "re-entry re-carried .env over the tree's own copy; stderr:\n{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(
        installed.is_file(),
        "re-entry lost setup's node_modules (the discard is `clean -fd` without -x)"
    );
}

/// **ADR-0180 decision 11: a `setup` failure is `s8c`'s by-name refusal, and the run surfaces it
/// and stops** (§8d-ii.4): exit `3`, the refusal naming `setup` and the project, and no dispatch.
///
/// Red at stage entry because creation calls no `setup`, so the run goes on to dispatch.
#[test]
fn a_failed_setup_refuses_by_name() {
    let repo = carry_repo(false);
    let root = repo.path();
    let out = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("setup") && stderr.contains(&format!("`{SETUP_PROJECT}`")),
        "the failed install was not refused naming `setup` and the project (exit {:?}); \
         stderr:\n{stderr}",
        out.status.code()
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "a failed setup did not stop the run with exit 3; stderr:\n{stderr}"
    );
    assert!(
        !stderr.contains("the dispatch of"),
        "the run dispatched after its setup failed; stderr:\n{stderr}"
    );
    // *(dated 2026-09-27, s8d-ii build, package D2188.)* This asserted that the fresh tree stayed
    // registered and unlocked after the refusal. D21-88 ruling 2 reverses it: the fresh tree is
    // rolled back before the run refuses, so it is neither registered nor on disk.
    let tree = crate::run_tree(root, "bug-1").path;
    let record = worktree_record(root, &tree);
    assert!(
        record.is_none() && std::fs::symlink_metadata(&tree).is_err(),
        "after the refusal the fresh tree was not rolled back (D21-88 ruling 2): registered \
         {record:?}, on disk {}; stderr:\n{stderr}",
        tree.exists()
    );
}

/// The project a rollback fixture's first `setup` fails on: a `package.json` with a dependency
/// and no lockfile, which a frozen install refuses before any fetch.
const FAILING_PROJECT: &str = "broken";

/// **D21-88 ruling 2 (ADR-0180 decisions 9 and 11, proposed): a FRESH tree whose `setup` failed is
/// rolled back before the run refuses, so the next run creates it anew and re-carries; a reused
/// tree is still not re-carried.** *(dated 2026-09-27, s8d-ii build, package D2188.)*
///
/// [`carry_repo`]'s installable fixture plus a committed failing project. The first run declares
/// the failing project: `setup` refuses by name (exit 3), and the tree is neither registered nor on
/// disk, while `rk-run/bug-1` is kept. The operator then fixes the declaration (the registry is
/// read from the checkout) and edits the carried `.env`: the second run creates the tree anew on
/// the existing branch, re-carries the NEW `.env`, and `setup` installs. The control: a third run
/// reuses that tree and does not re-carry the `.env` edited again.
///
/// Red at entry to package D2188 because the failed tree stays registered, and the second run
/// re-enters it as reused: no carry, no `setup`. Red under `S8D-II-M27` (the rollback skipped).
#[test]
fn a_fresh_tree_whose_setup_failed_is_rolled_back_and_the_next_run_recarries() {
    let repo = carry_repo(true);
    let root = repo.path();
    std::fs::create_dir_all(root.join(FAILING_PROJECT)).expect("the failing project is made");
    std::fs::write(
        root.join(FAILING_PROJECT).join("package.json"),
        b"{ \"name\": \"broken\", \"version\": \"0.0.0\", \"private\": true, \"dependencies\": { \"left-pad\": \"1.3.0\" } }\n",
    )
    .expect("the failing project's package.json writes");
    crate::git(root, &["add", FAILING_PROJECT]);
    crate::git(
        root,
        &["commit", "--quiet", "-m", "a project whose install fails"],
    );
    write_setup_registry(root, FAILING_PROJECT);
    let tree = crate::run_tree(root, "bug-1").path;

    // (1) setup fails: refused by name, and the fresh tree is rolled back; the branch is kept.
    let first = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&first.stderr);
    assert!(
        first.status.code() == Some(3)
            && stderr.contains("setup")
            && stderr.contains(&format!("`{FAILING_PROJECT}`")),
        "PREMISE: the first run's setup was not refused by name with exit 3 (exit {:?}); \
         stderr:\n{stderr}",
        first.status.code()
    );
    let record = worktree_record(root, &tree);
    assert!(
        record.is_none() && std::fs::symlink_metadata(&tree).is_err(),
        "the fresh tree whose setup failed was not rolled back: registered {record:?}, on disk \
         {}; stderr:\n{stderr}",
        tree.exists()
    );
    assert_eq!(
        git_rc(
            root,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                "refs/heads/rk-run/bug-1"
            ]
        ),
        Some(0),
        "the rollback deleted the run branch; it is kept"
    );

    // (2) the declaration is fixed and the carried file changed: the next run re-creates and
    // re-carries, and setup installs.
    write_setup_registry(root, SETUP_PROJECT);
    std::fs::write(root.join(".env"), b"SECRET=second\n").expect("the .env edits");
    let second = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert_eq!(
        std::fs::read(tree.join(".env")).ok().as_deref(),
        Some(&b"SECRET=second\n"[..]),
        "the next run did not re-carry into a re-created tree; stderr:\n{stderr}"
    );
    let installed = tree
        .join(SETUP_PROJECT)
        .join("node_modules")
        .join(".modules.yaml");
    assert!(
        installed.is_file(),
        "the next run did not run setup in the re-created tree; stderr:\n{stderr}"
    );

    // (3) CONTROL: a reused tree is not re-carried.
    std::fs::write(root.join(".env"), b"SECRET=third\n").expect("the .env edits again");
    let third = crate::run::spawn_run(root, "bug-1", None);
    assert_eq!(
        std::fs::read(tree.join(".env")).ok().as_deref(),
        Some(&b"SECRET=second\n"[..]),
        "a reused tree was re-carried; stderr:\n{}",
        String::from_utf8_lossy(&third.stderr)
    );
    assert!(
        worktree_record(root, &tree).is_some(),
        "the reused tree is no longer registered"
    );
}

// ---------------------------------------------------------------------------------------------
// s8d-ii round 1, a tests-mutants minor: the two values `rk-cli` hands `rk-git` — the lock's
// liveness probe (ADR-0180 decision 6: *"`rk-cli` builds the predicate from
// `rk_containment::is_alive` and a new `rk_containment::process_start_time`"*) and
// `GIT_LFS_SKIP_SMUDGE` (§8d-ii.5 read (4)) — had no `rk-cli` test, so `S8D-II-M21`'s kill covered
// `rk-git`'s side only. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)*
// ---------------------------------------------------------------------------------------------

/// `redkiln -C <repo> run <item>` with `env` added to the child's environment (never `set_var`,
/// §7d.13), and the fixture child's mode set as [`crate::run::spawn_run`] sets it: an absent mode
/// is a 600 s park.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS, so the value rk-cli reads from its own \
              environment and hands rk-git is the one under test"
)]
fn spawn_run_with_env(repo: &Path, item: &str, env: &[(&str, &str)]) -> std::process::Output {
    let markers = repo.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(repo).arg("run").arg(item);
    cmd.env("RK_FIXTURE_MODE", "exit:0")
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    for (k, v) in env {
        cmd.env(k, v);
    }
    crate::support::output_bounded(&mut cmd)
}

/// **The liveness probe `run` injects reads the real process table** (ADR-0180 decision 6). A run
/// tree locked with THIS test process's pid and its OS start time, in redkiln's own reason form,
/// is held by a live run: the next `run` refuses by name (exit 3) and leaves that lock as it is.
/// The control: the same lock naming a pid no process holds is stale, and the run releases it,
/// proceeds, and releases its own lock at its end.
///
/// Green at its writing, the wiring being built (P6): a pin. `R1FIX-PROBE-NONE` (`process_probe`
/// reads nothing, so every lock is stale) shows it can fail.
#[test]
fn a_run_tree_locked_by_a_live_process_refuses_the_next_run_by_name() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();
    let first = crate::run::spawn_run(root, "bug-1", None);
    let tree = crate::run_tree(root, "bug-1").path;
    assert!(
        worktree_record(root, &tree).is_some(),
        "PREMISE: the first run created no registered tree; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let id = rk_git::worktree::ItemId::parse("bug-1").expect("the item id parses");
    let me = std::process::id();
    let start = rk_containment::process_start_time(me)
        .expect("PREMISE: the OS reports this test process's start time");
    let live = rk_git::worktree::lock_reason(&id, &rk_git::worktree::LockOwner { pid: me, start });
    let tree_s = tree.to_str().expect("utf-8");
    git_out(root, &["worktree", "lock", "--reason", &live, tree_s]);

    let out = spawn_run_with_env(root, "bug-1", &[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("run-tree-lock-held") && stderr.contains(&format!("pid {me} ")),
        "a lock held by a LIVE process (this test, pid {me}) did not refuse the run by name \
         (exit {:?}); stderr:\n{stderr}",
        out.status.code()
    );
    assert_eq!(out.status.code(), Some(3), "stderr:\n{stderr}");
    let record = worktree_record(root, &tree).expect("the tree is still registered");
    assert!(
        record.iter().any(|l| l == &format!("locked {live}")),
        "the live run's lock was touched: {record:?}"
    );

    // CONTROL: a pid no process holds is stale; the run releases it and proceeds.
    git_out(root, &["worktree", "unlock", tree_s]);
    let dead = rk_git::worktree::lock_reason(
        &id,
        &rk_git::worktree::LockOwner {
            pid: u32::MAX - 1,
            start,
        },
    );
    git_out(root, &["worktree", "lock", "--reason", &dead, tree_s]);
    let out = spawn_run_with_env(root, "bug-1", &[]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("run-tree-lock-held") && !stderr.contains("run-tree-foreign-lock"),
        "CONTROL: a stale lock (no such pid) refused the run; stderr:\n{stderr}"
    );
    let record = worktree_record(root, &tree).expect("the tree is still registered");
    assert!(
        !record.iter().any(|l| l.starts_with("locked")),
        "CONTROL: the stale lock was not released, or the run's own was kept: {record:?}"
    );
}

/// **`GIT_LFS_SKIP_SMUDGE` in `run`'s own environment refuses creation by name** (§8d-ii.5 read
/// (4) and refusal (b): `rk-cli` reads it through `env.rs` and hands it to `rk-git`). The fixture
/// commits `*.bin filter=fx` and an `a.bin`, with `fx` effective at `local` scope; with
/// `GIT_LFS_SKIP_SMUDGE=1` the run exits 3 naming the variable, and no tree exists. The control:
/// `GIT_LFS_SKIP_SMUDGE=0` creates the tree.
///
/// Green at its writing, the wiring being built (P6): a pin. `R1FIX-LFS-NONE` (`run` hands `rk-git`
/// `None`) shows it can fail.
#[test]
fn git_lfs_skip_smudge_in_runs_environment_refuses_creation_by_name() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();
    std::fs::write(root.join(".gitattributes"), b"*.bin filter=fx\n").expect("attributes write");
    std::fs::write(root.join("a.bin"), b"payload\n").expect("a.bin writes");
    crate::git(root, &["add", ".gitattributes", "a.bin"]);
    crate::git(root, &["commit", "--quiet", "-m", "a filtered path"]);
    crate::git(root, &["config", "filter.fx.smudge", "tr a-z A-Z"]);
    crate::git(root, &["config", "filter.fx.clean", "tr A-Z a-z"]);
    let tree = crate::run_tree(root, "bug-1").path;

    let out = spawn_run_with_env(root, "bug-1", &[("GIT_LFS_SKIP_SMUDGE", "1")]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("run-tree-smudge-skipped") && stderr.contains("GIT_LFS_SKIP_SMUDGE=1"),
        "GIT_LFS_SKIP_SMUDGE=1 in run's environment did not refuse creation by name (exit {:?}); \
         stderr:\n{stderr}",
        out.status.code()
    );
    assert_eq!(out.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        worktree_record(root, &tree).is_none() && std::fs::symlink_metadata(&tree).is_err(),
        "a tree exists after the refusal"
    );

    // CONTROL: `0` skips nothing, and the same repository creates its tree.
    let out = spawn_run_with_env(root, "bug-1", &[("GIT_LFS_SKIP_SMUDGE", "0")]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        worktree_record(root, &tree).is_some(),
        "CONTROL: GIT_LFS_SKIP_SMUDGE=0 did not create the tree; stderr:\n{stderr}"
    );
}
