//! **`init` — non-interactive, eager, idempotent** (AC-437 as amended by D-14, AC-531, AC-412;
//! ADR-0107 decision 2; `s7d` §7d.9).
//!
//! `redkiln init` does exactly two things: writes `.redkiln/config.toml` **if and only if the file
//! is absent**, and opens the store, which creates it in the common dir and stamps its identity rows
//! on first open. It installs no hook and creates no git ref.
//!
//! **Every test here mints its own temp repository** (D18-9): `tempfile::tempdir()`, `git init`, one
//! commit — the commit because `SqliteStore::open` needs a root commit for its identity row. **The
//! product repository is never touched by this stage**, which is why nothing below resolves a path
//! relative to the working directory.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use rk_cli::verbs::init::{CreatedNames, INIT_UNREAD_WINDOW_RUNS, NAME_REGEX, name_is_admissible};

/// The `redkiln` binary, located the one way §7d.13 fixes.
fn redkiln() -> &'static str {
    env!("CARGO_BIN_EXE_redkiln")
}

/// A temp repository with one commit, and nothing else.
fn temp_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a temp dir");
    git(dir.path(), &["init", "--quiet"]);
    git(
        dir.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(dir.path(), &["config", "user.name", "Test"]);
    std::fs::write(dir.path().join("README"), b"root\n").expect("the root file writes");
    git(dir.path(), &["add", "README"]);
    git(dir.path(), &["commit", "--quiet", "-m", "root"]);
    dir
}

/// Run git in `cwd`. The tests build repositories; the PRODUCT never spawns git outside `rk-git`'s
/// one wrapper, which is what Checkpoint 7d's `Command::new("git")` ban over `src/` enforces.
#[expect(
    clippy::disallowed_types,
    reason = "a test building its own fixture repository; the product's git goes through rk-git's \
              one GitExec wrapper (ADR-0077), which the block's src/ ban enforces"
)]
fn git(cwd: &Path, args: &[&str]) {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Spawn `redkiln -C <repo> init` with stdin closed (D-15: no verb reads stdin).
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS, which is the only way golden constraint 4's \
              'a stub never replaced' is caught (s7d §7d.13's temp-repository rule)"
)]
fn run_init(repo: &Path) -> std::process::Output {
    std::process::Command::new(redkiln())
        .args(["-C", &repo.display().to_string(), "init"])
        .stdin(Stdio::null())
        .output()
        .expect("the redkiln binary runs")
}

/// `git status --porcelain` over the whole tree, as a sorted listing.
#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository's own porcelain listing; see `git` above"
)]
fn porcelain(repo: &Path) -> Vec<String> {
    let out = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()
        .expect("git status runs");
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    lines.sort();
    lines
}

/// Every path under `root`, relative and sorted, with `.git/` excluded — the working-tree digest
/// AC-437 compares across the second `init`.
fn tree_listing(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a readable dir") {
            let path = entry.expect("an entry").path();
            let rel = path
                .strip_prefix(root)
                .expect("under root")
                .to_string_lossy()
                .replace('\\', "/");
            if rel.starts_with(".git/") || rel == ".git" {
                continue;
            }
            if path.is_dir() {
                out.push(format!("{rel}/"));
                stack.push(path);
            } else {
                let bytes = std::fs::read(&path).expect("a readable file");
                out.push(format!(
                    "{rel} {}",
                    rk_types::digest::sha256(&bytes).as_str()
                ));
            }
        }
    }
    out.sort();
    out
}

/// The store file's digest, or `None` if there is no store.
fn store_digest(common_dir: &Path) -> Option<String> {
    let path = common_dir.join("redkiln").join("store.sqlite3");
    std::fs::read(&path)
        .ok()
        .map(|b| rk_types::digest::sha256(&b).as_str().to_owned())
}

/// The repository's common dir, read from git rather than assumed — a linked worktree's is not
/// `<root>/.git`.
#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository's own common dir; see `git` above"
)]
fn common_dir(repo: &Path) -> PathBuf {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()
        .expect("git rev-parse runs");
    PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// AC-437 as amended (D-14), ADR-0107 decision 2: *"a second `init` on an initialised repository
/// changes nothing and exits 0."*
///
/// Red by a second `init` that rewrites `config.toml` (bytes differ), that changes the porcelain
/// listing, that re-stamps an identity row — observable only as a changed store-file digest, since
/// the three rows are deterministic and equal either way — or that exits non-zero. A weaker red
/// compares mtimes, or runs `init` once.
#[test]
fn ac437_init_twice_identical_tree_and_identity_rows() {
    let repo = temp_repo();
    let root = repo.path();
    let common = common_dir(root);

    let first = run_init(root);
    assert_eq!(
        first.status.code(),
        Some(0),
        "the first init exited {:?}; stderr:\n{}",
        first.status.code(),
        String::from_utf8_lossy(&first.stderr)
    );
    let porcelain_1 = porcelain(root);
    let tree_1 = tree_listing(root);
    let store_1 = store_digest(&common).expect("init creates the store eagerly (D-14)");

    let second = run_init(root);
    assert_eq!(
        second.status.code(),
        Some(0),
        "the second init exited {:?}; stderr:\n{}",
        second.status.code(),
        String::from_utf8_lossy(&second.stderr)
    );

    assert_eq!(porcelain(root), porcelain_1, "the porcelain listing moved");
    assert_eq!(
        tree_listing(root),
        tree_1,
        "the working tree moved across the second init — bytes, not mtimes"
    );
    assert_eq!(
        store_digest(&common).as_deref(),
        Some(store_1.as_str()),
        "the store file's bytes moved across the second init, which is how a re-stamped identity \
         row shows: the three rows are deterministic and compare equal either way, so row equality \
         alone cannot see it"
    );

    // **The discriminating arm, and it was added because a mutant survived without it.** Replacing
    // the `if !config.exists()` guard with `if true` — a second `init` that rewrites the scaffold
    // unconditionally — passed every assertion above, because rewriting identical bytes produces an
    // identical tree, an identical porcelain listing and an identical store. §7d.9's guarantee is
    // stronger than "nothing changed": the file is left untouched "byte for byte, WHATEVER IT SAYS"
    // (ADR-0107 d2). So the only way to see the difference is to make the file say something else.
    // An operator who has edited their configuration must not have the edit reverted by a command
    // that claims to change nothing.
    let config = root.join(".redkiln").join("config.toml");
    let edited = b"# an operator edited this\n\n[adapters]\nharness = \"rk-adapter-claude\"\n\
                   store = \"rk-adapter-sqlite\"\ntoolchains = []\n\n[packs]\nenabled = []\n\n\
                   [doctor]\nunread_window_runs = 7\n";
    std::fs::write(&config, edited).expect("the edited config writes");

    let third = run_init(root);
    assert_eq!(
        third.status.code(),
        Some(0),
        "init over an edited configuration exited {:?}; stderr:\n{}",
        third.status.code(),
        String::from_utf8_lossy(&third.stderr)
    );
    assert_eq!(
        std::fs::read(&config).expect("the config is readable"),
        edited,
        "a second `init` REVERTED an operator's edit. The file is written if and only if it is \
         absent; a present one is left alone byte for byte, whatever it says (ADR-0107 d2, §7d.9)"
    );
}

/// AC-437's byte half. Red by any byte of the written file differing from the pinned fixture; a
/// weaker red parses both and compares values, under which a reordered key or a dropped comment
/// would pass.
#[test]
fn ac437_init_scaffold_bytes_pinned() {
    let repo = temp_repo();
    assert_eq!(run_init(repo.path()).status.code(), Some(0));

    let written = std::fs::read(repo.path().join(".redkiln").join("config.toml"))
        .expect("init wrote .redkiln/config.toml");
    let expected = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("init")
            .join("config.toml.expected"),
    )
    .expect("the pinned fixture exists");

    assert_eq!(
        String::from_utf8_lossy(&written),
        String::from_utf8_lossy(&expected),
        "the scaffold init writes is not the pinned bytes"
    );
    assert_eq!(
        written, expected,
        "the scaffold differs in bytes but not in text — a line-ending \
                                   difference; the .gitattributes row exists to stop exactly this"
    );
    assert!(
        !written.contains(&b'\r'),
        "the written scaffold carries CR bytes; it is LF only (§7d.9)"
    );
    assert!(
        String::from_utf8_lossy(&written)
            .contains(&format!("unread_window_runs = {INIT_UNREAD_WINDOW_RUNS}")),
        "the written window does not read the one literal the source declares (H-4, D18-5)"
    );
}

/// AC-531's lint half. Red by `foo`, `redkiln-foo`, `rk-Foo` or `rk-` passing the lint, by `rk-foo`
/// failing it, or by a name outside `.redkiln/` appearing in the tree diff after `init`; a weaker
/// red tests the regex on one string.
#[test]
fn ac531_init_created_names_match_regex() {
    assert_eq!(
        NAME_REGEX, "^rk-[a-z0-9][a-z0-9-]*$",
        "the committed regex is AC-506's, quoted once"
    );
    assert!(
        name_is_admissible("rk-foo"),
        "the admissible shape must pass"
    );
    assert!(
        name_is_admissible("rk-a1-b2"),
        "digits and inner hyphens are admissible"
    );
    for bad in [
        "foo",
        "redkiln-foo",
        "rk-Foo",
        "rk-",
        "rk--",
        "rk-A",
        "",
        "rk-foo/",
    ] {
        assert!(
            !name_is_admissible(bad),
            "`{bad}` passed the naming lint and must not (AC-506's regex edges)"
        );
    }

    // The real tree diff after `init`. Exactly the two paths inside the exempt directory, and
    // nothing outside it — an assertion that goes red the day `init` creates one, and one an
    // enumerator that lists nothing cannot pass.
    let repo = temp_repo();
    assert_eq!(run_init(repo.path()).status.code(), Some(0));
    let created: Vec<String> = porcelain(repo.path())
        .into_iter()
        .map(|l| l[3..].to_owned())
        .collect();
    assert_eq!(
        created,
        vec![".redkiln/".to_owned()],
        "init created something other than its own exempt directory"
    );
    let inside: Vec<String> = tree_listing(repo.path())
        .into_iter()
        .filter(|p| p.starts_with(".redkiln"))
        .map(|p| p.split(' ').next().unwrap_or_default().to_owned())
        .collect();
    assert_eq!(
        inside,
        vec![".redkiln/".to_owned(), ".redkiln/config.toml".to_owned()],
        "the two created paths inside the exempt directory are the enumerator's positive control"
    );

    // And the manifest itself is empty at 7d, because the scaffold is inside `.redkiln/` and the
    // store is in the common dir — listed separately and unlinted (§7d.9).
    assert!(
        CreatedNames::at_seven_d().lintable().is_empty(),
        "init's lintable manifest is non-empty at 7d; §7d.9 says it is empty, and a name in it \
         would have to match the regex"
    );
}

/// AC-531's AND: a pre-existing `rk-` name is reported, never overwritten. Red by `rk-notes/`
/// altered or by no report on stderr; a weaker red asserts exit `0` alone.
#[test]
fn ac531_preexisting_rk_name_reported_not_overwritten() {
    let repo = temp_repo();
    let notes = repo.path().join("rk-notes");
    std::fs::create_dir(&notes).expect("the pre-existing directory is created");
    std::fs::write(notes.join("keep.txt"), b"not init's\n").expect("its file writes");
    let before = tree_listing(&notes);

    let out = run_init(repo.path());
    assert_eq!(
        out.status.code(),
        Some(0),
        "a pre-existing name is not an error"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("rk-notes"),
        "init did not report the pre-existing rk- name; stderr read:\n{stderr}"
    );
    assert_eq!(
        tree_listing(&notes),
        before,
        "init altered a pre-existing rk- name it did not create"
    );
}

/// AC-412: `init` installs zero hooks. Red by any non-`*.sample` entry appearing under `.git/hooks`.
#[test]
fn ac412_init_installs_zero_hooks() {
    let repo = temp_repo();
    let hooks = common_dir(repo.path()).join("hooks");

    let before = hook_entries(&hooks);
    assert_eq!(run_init(repo.path()).status.code(), Some(0));
    let after = hook_entries(&hooks);

    assert_eq!(
        after, before,
        "init changed the hooks directory. git's own *.sample files are not init's, and init \
         installs nothing (AC-412; the doctor row is RK-D-HOOKS)"
    );
    assert!(
        after.iter().all(|e| e.ends_with(".sample")),
        "a non-sample hook is present after init: {after:?}"
    );
}

/// Every entry under a hooks directory, sorted. An absent directory is an empty listing, which is a
/// valid state for a repository that has none.
fn hook_entries(hooks: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(hooks) else {
        return Vec::new();
    };
    let mut out: Vec<String> = entries
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    out.sort();
    out
}
