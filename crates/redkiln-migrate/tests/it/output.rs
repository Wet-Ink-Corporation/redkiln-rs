//! What the migration commit's tree carries, and what it may not
//! (AC-449, AC-450, AC-470, AC-471's write half, AC-475; D18-12).

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Stdio;

use redkiln_migrate::lift::{self, DirOrder, Natural};
use redkiln_migrate::run_contract::{self, Completed, Hooks, PHASE_WRITTEN_MARKER};

use crate::{
    code_lines, git, git_out, legacy_mini_repo, lint_controls_dir, migrate_bin, rs_files, src_dir,
};

fn migration_branch(repo: &Path) -> String {
    format!(
        "{}{}",
        run_contract::MIGRATION_BRANCH_PREFIX,
        git_out(repo, &["rev-parse", "--short", "HEAD"])
    )
}

fn commits_ahead(repo: &Path, branch: &str) -> u32 {
    let head = git_out(repo, &["rev-parse", "HEAD"]);
    git_out(repo, &["rev-list", "--count", &format!("{head}..{branch}")])
        .parse()
        .expect("a count")
}

/// Every path in the migration commit's tree.
fn tree_paths(repo: &Path, branch: &str) -> Vec<String> {
    git_out(repo, &["ls-tree", "-r", "--name-only", branch])
        .lines()
        .map(str::to_owned)
        .filter(|l| !l.trim().is_empty())
        .collect()
}

/// One blob out of the migration commit's tree.
fn blob(repo: &Path, branch: &str, path: &str) -> String {
    git_out(repo, &["show", &format!("{branch}:{path}")])
}

/// Run the migrator to a commit and hand back the repo and its branch.
fn migrated() -> (tempfile::TempDir, String) {
    let repo = legacy_mini_repo();
    let branch = migration_branch(repo.path());
    let done = run_contract::run(repo.path(), false, Hooks::default()).expect("the run commits");
    assert!(matches!(done, Completed::Committed { .. }));
    (repo, branch)
}

/// Every migrated artifact in the commit's tree, as `(path, parsed JSON)`.
fn artifacts(repo: &Path, branch: &str) -> Vec<(String, serde_json::Value)> {
    tree_paths(repo, branch)
        .into_iter()
        .filter(|p| p.ends_with(".json") && p != "report.json")
        .map(|p| {
            let raw = blob(repo, branch, &p);
            let v = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("{p} is not JSON in the commit's tree: {e}"));
            (p, v)
        })
        .collect()
}

// ---- AC-450 -----------------------------------------------------------------------------------

/// AC-450, and AC-447's no-op exemption.
///
/// *"the run is a no-op, defined as tree-digest equality with the prior migration commit's tree —
/// the re-run's report is written to stdout … AND the tool never emits a second normalisation
/// commit, and AC-447's zero-commit abort does not fire on this case."*
///
/// # AC-447's zero-commit abort is UNREACHABLE here, and that is disclosed rather than claimed
///
/// AC-447 requires that *"a run that would produce zero commits aborts before writing unless it is
/// AC-450's no-op case"*. On this design the exception swallows the rule: a run produces zero
/// commits exactly when the worktree's tree equals the prior migration commit's tree, and that IS
/// the no-op case, which returns before the commit. There is no reachable non-no-op zero-commit
/// run, so no test here can exercise that clause and none pretends to.
///
/// Measured, not assumed: with the no-op check mutated to `if false`, the second run reaches
/// `commit_with_trailers` with nothing staged and git refuses —
/// `NonZeroExit { op: "commit", code: Some(1) }` — so the run exits `1` rather than landing a
/// second commit. This test is red on that (§7d.14's *"by exit ≠ `0`"*, one of its four stated
/// reds), which is how the mutation was confirmed to be detected at all.
#[expect(
    clippy::disallowed_types,
    reason = "a test spawning the migrator binary to read a real stdout stream; the PRODUCT spawns nothing but git, through \n              rk-git's one GitExec wrapper (ADR-0077)"
)]
#[test]
fn ac450_second_run_is_noop_report_to_stdout_no_commit() {
    let (repo, branch) = migrated();
    let path = repo.path();
    let after_first = git_out(path, &["rev-parse", &branch]);
    assert_eq!(
        commits_ahead(path, &branch),
        1,
        "the first run did not commit once"
    );

    // The second run, through the BINARY, so "written to stdout" is read off a real stream.
    let out = std::process::Command::new(migrate_bin())
        .args(["--target", &path.to_string_lossy()])
        .stdin(Stdio::null())
        .output()
        .expect("the migrator runs");
    assert!(
        out.status.success(),
        "the no-op run exited {:?}: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );

    // The report is on stdout and is a whole JSON document.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let json_line = stdout
        .lines()
        .find(|l| l.trim_start().starts_with('{'))
        .unwrap_or_else(|| panic!("the no-op run printed no report:\n{stdout}"));
    let report: serde_json::Value =
        serde_json::from_str(json_line).expect("the re-run's report is JSON");
    assert!(
        report.get("per_kind_slot").is_some(),
        "the re-run's stdout is not a report"
    );

    // No SECOND commit, and no zero-commit abort.
    assert_eq!(
        git_out(path, &["rev-parse", &branch]),
        after_first,
        "the migration branch moved — the tool emitted a second normalisation commit"
    );
    assert_eq!(
        commits_ahead(path, &branch),
        1,
        "not exactly one commit after two runs"
    );

    // `git diff --exit-code` on the target: nothing changed.
    let diff = git_out(path, &["status", "--porcelain"]);
    assert!(
        diff.trim().is_empty(),
        "the second run dirtied the target:\n{diff}"
    );
}

/// AC-450's kill case, AC-522, D18-12 — the 153-line file's name, kept.
///
/// Spawn the real binary, read stdout until `phase: written`, kill it through `rk-containment`,
/// and assert the kill landed INSIDE the window: no commit yet, and the worktree holds written
/// files. Then re-run and assert the count is now 1 and the tree digest equals an uninterrupted
/// run's.
///
/// **A digest that differs across the boundary is Halt 4, not a retry.**
#[expect(
    clippy::disallowed_types,
    reason = "a test spawning the migrator binary to read a real stdout stream; the PRODUCT spawns nothing but git, through \n              rk-git's one GitExec wrapper (ADR-0077)"
)]
#[test]
fn idempotent_across_kill() {
    // Two repositories whose trees are IDENTICAL: `temp_repo`'s README carries a per-call nonce so
    // that two repositories never share a root commit, which is right for store identity and wrong
    // here — this test compares a TREE DIGEST across two repositories, so the nonce is overwritten
    // with fixed bytes in both.
    let killed = fixed_root_repo();
    let control = fixed_root_repo();
    let kpath = killed.path();
    let cpath = control.path();
    let kbranch = migration_branch(kpath);
    let cbranch = migration_branch(cpath);

    let head_before = git_out(kpath, &["rev-parse", "HEAD"]);
    let tree_before = git_out(kpath, &["rev-parse", "HEAD^{tree}"]);

    // ---- the killed run -----------------------------------------------------------------------
    let mut child = std::process::Command::new(migrate_bin())
        .args(["--target", &kpath.to_string_lossy()])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("the migrator spawns");
    let pid = child.id();

    let mut saw_marker = false;
    {
        let stdout = child.stdout.take().expect("piped stdout");
        for line in BufReader::new(stdout).lines() {
            let line = line.expect("a stdout line");
            if line.trim() == PHASE_WRITTEN_MARKER {
                saw_marker = true;
                rk_containment::terminate_pid(pid, rk_containment::CANCEL_EXIT_CODE)
                    .expect("the kill lands");
                break;
            }
        }
    }
    let _ = child.wait();
    assert!(
        saw_marker,
        "the run never printed `{PHASE_WRITTEN_MARKER}` — the kill seam D18-12 fixes is absent, \
         and a kill without it would be timing, not a seam"
    );

    // The kill landed INSIDE the window. A commit that raced ahead would make this an
    // uninterrupted run measured under the kill test's name.
    let refs_now = crate::refs(kpath);
    assert!(
        refs_now.iter().any(|r| r.ends_with(&kbranch)),
        "the killed run left no migration branch at all"
    );
    assert_eq!(
        commits_ahead(kpath, &kbranch),
        0,
        "a commit had already landed when the kill fired — the kill did not land in the window \
         between `{PHASE_WRITTEN_MARKER}` and the commit"
    );
    let wt = Path::new(&git_out(
        kpath,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    ))
    .join("redkiln")
    .join("migrate-wt");
    let written = rs_or_json_count(&wt);
    assert!(
        written > 0,
        "the worktree holds no written files after the kill — the write phase had not run, so the \
         marker was printed too early"
    );

    // The TARGET branch is untouched across the kill.
    assert_eq!(
        git_out(kpath, &["rev-parse", "HEAD"]),
        head_before,
        "the target's HEAD moved"
    );
    assert_eq!(
        git_out(kpath, &["rev-parse", "HEAD^{tree}"]),
        tree_before,
        "the target's tree moved across the kill"
    );

    // ---- the re-run, and the uninterrupted control ----------------------------------------------
    run_contract::run(kpath, false, Hooks::default()).expect("the re-run completes");
    assert_eq!(
        commits_ahead(kpath, &kbranch),
        1,
        "the re-run did not reach exactly one commit"
    );

    run_contract::run(cpath, false, Hooks::default()).expect("the uninterrupted run completes");
    assert_eq!(commits_ahead(cpath, &cbranch), 1);

    let resumed = git_out(kpath, &["rev-parse", &format!("{kbranch}^{{tree}}")]);
    let uninterrupted = git_out(cpath, &["rev-parse", &format!("{cbranch}^{{tree}}")]);
    assert_eq!(
        resumed, uninterrupted,
        "the re-run's tree digest differs from an uninterrupted run's. THIS IS HALT 4, NOT A \
         RETRY: a migration that does not converge to the same tree after an interruption is not \
         idempotent, and re-running it would hide that rather than fix it"
    );
}

/// A `legacy-mini` repository whose root file is FIXED, so two of them have identical trees.
fn fixed_root_repo() -> tempfile::TempDir {
    let repo = legacy_mini_repo();
    std::fs::write(
        repo.path().join("README"),
        b"fixed for the tree-digest comparison\n",
    )
    .expect("the fixed root writes");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "--quiet", "-m", "fixed root"]);
    repo
}

fn rs_or_json_count(root: &Path) -> usize {
    if !root.is_dir() {
        return 0;
    }
    let mut n = 0;
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a directory") {
            let p = entry.expect("an entry").path();
            if p.file_name().is_some_and(|f| f == ".git") {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|e| e == "json") {
                n += 1;
            }
        }
    }
    n
}

// ---- AC-449 -----------------------------------------------------------------------------------

/// A reversing [`DirOrder`] — the second arm of AC-449's read-order sweep.
#[derive(Default)]
struct Reversed;

impl DirOrder for Reversed {
    fn order(&mut self, entries: &mut Vec<std::path::PathBuf>) {
        entries.sort();
        entries.reverse();
    }
}

/// AC-449 — determinism across the swept nuisance parameters.
///
/// # What each arm is worth, disclosed rather than implied
///
/// * **read order** is the strong arm, and it is driven IN PROCESS through [`DirOrder`]. A spawned
///   process cannot be made to read in a different order without a flag or an environment
///   variable, and AC-474 forbids both — so driving this seam from a spawn is not merely awkward,
///   it is unreachable.
/// * **`TZ`** is **DISCLOSED AS WEAK**: the migrator reads no clock (Checkpoint 7d bans
///   `SystemTime::now|Instant::now|Utc::now` over its `src`), so `TZ` can move only git's commit
///   metadata, which a TREE digest excludes by construction. The arm is run because the criterion
///   names it, and its weakness is stated because a test may not claim more than it asserts.
/// * **hash-map seed** is moot and is asserted as such: `HashMap`/`HashSet` are clippy-banned
///   workspace-wide and this crate uses `BTreeMap` throughout.
#[expect(
    clippy::disallowed_types,
    reason = "a test spawning the migrator binary to read a real stdout stream; the PRODUCT spawns nothing but git, through \n              rk-git's one GitExec wrapper (ADR-0077)"
)]
#[test]
fn ac449_two_targets_byte_identical_under_tz_and_read_order() {
    // ---- arm 1: read order, in process ----------------------------------------------------------
    let a = legacy_mini_repo();
    let b = legacy_mini_repo();

    let natural = lift::run(a.path(), &mut Natural).expect("the natural lift");
    let reversed = lift::run(b.path(), &mut Reversed).expect("the reversed lift");

    // The reversing order must actually have reversed something, or the arm is vacuous.
    let n_files: Vec<&std::path::PathBuf> = natural.files.iter().map(|f| &f.source).collect();
    let r_files: Vec<&std::path::PathBuf> = reversed.files.iter().map(|f| &f.source).collect();
    assert_ne!(
        n_files, r_files,
        "the reversed DirOrder produced the same walk order as the natural one — the seam is not \
         wired into the walk and this arm would compare a run with itself"
    );

    assert_eq!(
        natural.artifacts, reversed.artifacts,
        "the artifact set differs across read order — the output is not order-independent"
    );
    let nd = rk_store::canon::canonical_digest(
        &serde_json::to_value(&natural.report).expect("serialise"),
    )
    .expect("digest");
    let rd = rk_store::canon::canonical_digest(
        &serde_json::to_value(&reversed.report).expect("serialise"),
    )
    .expect("digest");
    assert_eq!(nd, rd, "the report digest differs across read order");

    // ---- arm 2: TZ, in the CHILD's environment (set_var is banned workspace-wide) ---------------
    let c = fixed_root_repo();
    let d = fixed_root_repo();
    for (repo, tz) in [(&c, "UTC"), (&d, "Pacific/Kiritimati")] {
        let out = std::process::Command::new(migrate_bin())
            .args(["--target", &repo.path().to_string_lossy()])
            .env("TZ", tz)
            .stdin(Stdio::null())
            .output()
            .expect("the migrator runs");
        assert!(out.status.success(), "the run under TZ={tz} failed");
    }
    let ctree = git_out(
        c.path(),
        &[
            "rev-parse",
            &format!("{}^{{tree}}", migration_branch(c.path())),
        ],
    );
    let dtree = git_out(
        d.path(),
        &[
            "rev-parse",
            &format!("{}^{{tree}}", migration_branch(d.path())),
        ],
    );
    assert_eq!(
        ctree, dtree,
        "two targets built from the same fixture under different TZ produced different trees"
    );

    // ---- arm 3: the hash-map seed, moot by construction and asserted as such --------------------
    let hits: Vec<String> = rs_files(&src_dir())
        .iter()
        .flat_map(|f| {
            let text = std::fs::read_to_string(f).expect("a source file");
            code_lines(&text)
                .into_iter()
                .filter(|(_, l)| l.contains("HashMap") || l.contains("HashSet"))
                .map(|(n, l)| format!("{}:{n}: {}", f.display(), l.trim()))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        hits.is_empty(),
        "the migrator names HashMap/HashSet, so the hash-seed arm is NOT moot by construction:\n{}",
        hits.join("\n")
    );
}

// ---- AC-470, AC-471's write half, AC-475 -------------------------------------------------------

/// AC-470 — *"the bytes are identical for 100% of migrated artifacts."*
#[test]
fn ac470_migrated_artifacts_round_trip_through_canonical() {
    let (repo, branch) = migrated();
    let arts = artifacts(repo.path(), &branch);
    assert!(
        !arts.is_empty(),
        "the migration commit's tree carries no artifacts — the round trip below would compare \
         nothing (Halt 8 shape (a))"
    );

    for (path, value) in &arts {
        let committed = blob(repo.path(), &branch, path);
        let re = rk_store::canon::canonical(value).expect("the artifact re-canonicalises");
        assert_eq!(
            String::from_utf8_lossy(&re).trim(),
            committed.trim(),
            "{path} is not byte-identical after a read-back and re-serialisation through \
             rk_store::canon"
        );
    }
    assert_eq!(arts.len(), 23, "not every legacy file became an artifact");
}

/// AC-471's write half, over the migrator's own output.
///
/// *"every artifact schema declares a `schema_version`, and no schema declares a template- or
/// rendered-signature field."* (AC-478 is ARMED — §7d.11 — because its WHEN needs a render path
/// `rk-store` does not carry at this pin.)
#[test]
fn migrated_artifacts_carry_schema_version_and_no_sig() {
    let (repo, branch) = migrated();
    let arts = artifacts(repo.path(), &branch);
    assert!(!arts.is_empty(), "no artifacts to read");

    for (path, value) in &arts {
        assert!(
            value
                .get("schema_version")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "{path} carries no schema_version"
        );
        let mut sig_keys = Vec::new();
        collect_sig_keys(value, &mut sig_keys);
        assert!(
            sig_keys.is_empty(),
            "{path} carries signature field(s) {sig_keys:?} — AC-471 says no schema declares one, \
             and AC-474 calls the break clean"
        );
    }

    // And the literals are nowhere in the MIGRATED LAYOUT — not merely absent from the artifacts.
    // A migration that left each legacy `.md` beside its new `.json` would pass the per-artifact
    // check above and fail this one.
    //
    // **Scoped to `.bklg/` and `.redkiln/`, and that scope is the claim's real extent.** The
    // migrator converts what it finds under those two roots and does not touch anything else in
    // the target, so a sweep over the WHOLE commit tree would assert something the migration does
    // not control. This test originally did exactly that and failed on `PROVENANCE.md` — the
    // fixture's own provenance note, which the harness copies to the target root and whose PROSE
    // names both literals by design. That was the test over-claiming, not the migrator leaking.
    let migrated_layout: Vec<String> = tree_paths(repo.path(), &branch)
        .into_iter()
        .filter(|p| p.starts_with(".bklg/") || p.starts_with(".redkiln/"))
        .collect();
    assert!(
        !migrated_layout.is_empty(),
        "no path under .bklg/ or .redkiln/ in the commit's tree — the sweep would read nothing"
    );
    for p in migrated_layout {
        let body = blob(repo.path(), &branch, &p);
        for literal in ["template_sig", "rendered_sig"] {
            assert!(
                !body.contains(literal),
                "{p} in the migrated layout still carries `{literal}`"
            );
        }
        assert!(
            p.ends_with(".json"),
            "{p} survived the migration as a non-artifact — each legacy file is REPLACED by its \
             canonical .json, which is what makes the break clean (AC-474)"
        );
    }
}

fn collect_sig_keys(value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                if k.ends_with("_sig") {
                    out.push(k.clone());
                }
                collect_sig_keys(v, out);
            }
        }
        serde_json::Value::Array(items) => {
            for v in items {
                collect_sig_keys(v, out);
            }
        }
        _ => {}
    }
}

/// AC-475 — the tripwire.
///
/// *"it is carried as opaque content with a migration-loss marker AND no `ChangeSummary` value is
/// synthesised."*
///
/// **The second conjunct is VACUOUS and is disclosed as the register discloses it**: it is
/// trivially true while `rk_ports::toolchain::ChangeSummary` has no decided contents and no `Ok`
/// is constructible (S-BIN unrun). It is kept as the tripwire against the type appearing in the
/// migration path before then — which is a real risk, because `redkiln-migrate` may not depend on
/// `rk-ports` at all (AC-466), so the identifier could only arrive by someone re-declaring it.
///
/// *(Noted 2026-09-25, s8a fix round 5, ADR-0165: the premise moved. The type is now an enum in
/// `rk-ports` whose `Text` arm is finalised and constructed by `RustToolchain::change_summary`, so
/// a value CAN exist and the second conjunct is a real constraint for `Text` values from `s8a`. It
/// stays vacuous only for the sealed `Binary` arm, until S-BIN at `s8i`. The tripwire and its
/// assertion do not move.)*
///
/// §7d.14's mutant list names **no declared mutant** for any migrator test. This one is
/// negative-only, so its red is taken from its own stated failure mode rather than from a
/// declared mutant, and the source-scan half is **self-controlled** first.
#[test]
fn ac475_zero_change_summary_values_in_output() {
    // ONE spelling of the token, and a plain single-line string literal on purpose.
    //
    // Checkpoint 5 bans the ChangeSummary TYPE in a code position under `crates/` outside
    // `rk-ports` and strips string literals before matching -- but its stripper cannot cross a
    // backslash, so the token spelled inside a WRAPPED message, or one carrying an escape,
    // survives the strip and fires a ban that does not mean to catch it. Naming it once here
    // keeps every message below interpolated and the single literal strippable.
    // [Found 2026-09-21 by Checkpoint 5's re-run at Step 7d's close.]
    const TOKEN: &str = "ChangeSummary";

    // (a) THE CONTROL, FIRST: the scanner must be seen to find the token before it reports zero.
    let control = lint_controls_dir().join("ac475_change_summary_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| l.contains(TOKEN))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line naming {TOKEN} — the ban below is \
         vacuous",
        control.display()
    );

    // (b) the identifier appears nowhere under the migrator's src/.
    let mut hits = Vec::new();
    for file in rs_files(&src_dir()) {
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (n, line) in code_lines(&text) {
            if line.contains(TOKEN) {
                hits.push(format!("{}:{n}: {}", file.display(), line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "redkiln-migrate names {TOKEN} (AC-475's tripwire):\n{}",
        hits.join("\n")
    );

    // (c) no artifact in the migration commit's tree carries a ChangeSummary VALUE.
    let (repo, branch) = migrated();
    for p in tree_paths(repo.path(), &branch) {
        let body = blob(repo.path(), &branch, &p);
        assert!(
            !body.contains(TOKEN),
            "{p} in the migration commit's tree carries a {TOKEN} value"
        );
    }
}

/// AC-466's run-level half, stated as the tree fact §7d.11 names.
///
/// The register's *"zero `dispatch` events in the event log"* is vacuous here in the exact sense it
/// means: the migrator writes no event log at all. What is assertable is that the migration commit
/// carries no store.
#[test]
fn ac466_migration_commit_carries_no_store() {
    let (repo, branch) = migrated();
    let paths = tree_paths(repo.path(), &branch);
    assert!(!paths.is_empty(), "the commit's tree is empty");
    let stores: Vec<&String> = paths
        .iter()
        .filter(|p| p.contains("store.sqlite3") || p.ends_with(".sqlite3"))
        .collect();
    assert!(
        stores.is_empty(),
        "the migration commit's tree carries a store: {stores:?} — the migrator dispatches nothing \
         and has no port to dispatch through (AC-466)"
    );
}
