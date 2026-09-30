//! **`init` — non-interactive, eager, idempotent** (AC-437 as amended by D-14, AC-531, AC-412;
//! ADR-0107 decision 2; `s7d` §7d.9).
//!
//! `redkiln init` does exactly ~~two~~ three things: writes `.redkiln/config.toml` **if and only if
//! the file is absent**, ~~and~~ opens the store, which creates it in the common dir and stamps its
//! identity rows on first open, *and appends the `# redkiln-runtime` block to
//! `<commondir>/info/exclude` when its marker is absent* *(dated 2026-09-27, `s8d-ii` build,
//! package P5: D21-45 W4, ADR-0180, proposed)*. It installs no hook and creates no git ref.
//!
//! **Every test here mints its own temp repository** (D18-9): `tempfile::tempdir()`, `git init`, one
//! commit — the commit because `SqliteStore::open` needs a root commit for its identity row. **The
//! product repository is never touched by this stage**, which is why nothing below resolves a path
//! relative to the working directory.

use std::path::Path;

use rk_cli::verbs::init::{CreatedNames, INIT_UNREAD_WINDOW_RUNS, NAME_REGEX, name_is_admissible};

use crate::{common_dir, porcelain, refs, run_init, temp_repo};

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

// `identity_rows` and `store_digest` were DEFINED here until Slice 3, cycle 3, and are now
// `crate::support`'s: `ac413_store_opened_from_other_repo_refused_zero_rows` and
// `common_dir_discovered_from_inherited_cwd_via_wrapper` make the identical readings, and two
// definitions of "the store's identity" can drift while both suites stay green against different
// notions of the same fact. Moved, never copied.

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
    let store_1 =
        crate::support::store_digest(&common).expect("init creates the store eagerly (D-14)");
    let identity_1 = crate::support::identity_rows(&common);

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
    // AC-437's second conjunct, "identical store identity rows", read through the instrument §7d.9
    // names. The rows are DETERMINISTIC — root commit, common dir, schema version — so they compare
    // equal whether or not the second `init` re-stamped them; that is why the file digest below is
    // also compared, and why neither assertion replaces the other. What THIS one catches, and the
    // digest cannot, is an `init` that leaves a store the rows are not in at all.
    let identity_2 = crate::support::identity_rows(&common);
    assert_eq!(
        identity_2, identity_1,
        "the store's identity rows moved across the second init"
    );

    assert_eq!(
        crate::support::store_digest(&common).as_deref(),
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
    let refs_before = refs(repo.path());
    assert!(
        !refs_before.is_empty(),
        "the fixture repository has no refs at all, so the refs comparison below would read nothing"
    );
    assert_eq!(run_init(repo.path()).status.code(), Some(0));

    // **The `refs/` surface.** §7d.9 spells the enumerator as "the real tree diff after `init`
    // (working tree, `refs/`, `.git/hooks`)" and AC-531 as amended is "scoped to the working tree,
    // refs and hooks". The hooks third is `ac412_init_installs_zero_hooks`; this is the refs third,
    // and it was missing when this slice first shipped — an `init` that created a ref whose name
    // violated the regex passed the whole suite. §7d.9 also says `init` "creates no git ref", so the
    // assertion is equality rather than a name check; the name check below still governs anything a
    // later slice adds.
    let refs_after = refs(repo.path());
    assert_eq!(
        refs_after, refs_before,
        "`init` created or moved a git ref; §7d.9 says it creates none"
    );
    for name in &refs_after {
        if let Some(leaf) = name.rsplit('/').next() {
            assert!(
                !leaf.starts_with("rk-") || name_is_admissible(leaf),
                "a ref named `{name}` carries the rk- prefix and does not match {NAME_REGEX}"
            );
        }
    }

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
    // The control that the scan root RESOLVES. `hook_entries` returns an empty vector for an
    // unreadable directory by design — a repository may legitimately have none — so without this a
    // mis-resolved common dir would make the comparison below pass having read nothing, the Halt 8
    // shape `rs_files` refuses for source scans. `git init` always seeds `*.sample`, so the control
    // costs nothing and turns a silently wrong path into a loud red.
    assert!(
        !before.is_empty(),
        "the hooks directory {} is empty or unresolved — this absence assertion would read nothing",
        hooks.display()
    );
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

/// The block `init` appends to `<commondir>/info/exclude`, as `s8d-ii` §8d-ii.5 prints it (D21-45
/// W4; ADR-0180, proposed). Its middle row is taken from `rk_git::worktree`, the root's one home
/// (D21-44), so this file never spells the run-tree root itself.
fn expected_block() -> Vec<u8> {
    format!(
        "# redkiln-runtime\n{}\n# end redkiln-runtime\n",
        rk_git::worktree::RUN_ROOT_IGNORE_ROW
    )
    .into_bytes()
}

/// A path inside a run tree, relative to the main worktree, derived through `rk_git::worktree`'s
/// one derivation — so the ignore assertions below judge the row against the real root, not a
/// second spelling of it.
fn run_tree_probe(root: &Path) -> String {
    let item = rk_git::worktree::ItemId::parse("exclude-probe").expect("a valid item id");
    let tree = rk_git::worktree::derive(root, &item).path;
    tree.strip_prefix(root)
        .expect("the run tree lies under the main worktree")
        .join("probe.txt")
        .to_string_lossy()
        .replace('\\', "/")
}

/// Does git in `root` report `rel` as ignored? `check-ignore -q` exits 0 for ignored and 1 for not;
/// anything else is a broken probe, never a "not ignored". Through `rk-git`'s one process-exec site
/// (`support::git_run`; ADR-0077; D21-128 A1): the exit code is read off
/// `GitError::NonZeroExit`, which carries it (A2).
fn git_ignores(root: &Path, rel: &str) -> bool {
    match crate::support::git_run(root, &["check-ignore", "-q", "--no-index", rel]) {
        Ok(_) => true,
        Err(rk_git::error::GitError::NonZeroExit { code: Some(1), .. }) => false,
        Err(other) => panic!("git check-ignore did not answer 0 or 1: {other:?}"),
    }
}

/// Lines of `bytes` equal to the marker, a trailing CR dropped — the count the idempotence pins.
fn marker_lines(bytes: &[u8]) -> usize {
    bytes
        .split(|b| *b == b'\n')
        .filter(|l| l.strip_suffix(b"\r").unwrap_or(l) == b"# redkiln-runtime")
        .count()
}

/// D21-45 W4 / ADR-0180 (proposed): `init` appends the `# redkiln-runtime` block to
/// `<commondir>/info/exclude` once, and a later `init` leaves the file byte-identical (ADR-0107
/// decision 2 for this file once the block exists; `s8d-ii` §8d-ii.5).
///
/// It starts from the **pre-stage initialised shape** — `init` has run, `.redkiln/config.toml` and
/// the store exist, and `info/exclude` holds no block — because that is the one case in which a
/// post-stage `init` changes something, and the record supersedes decision 2 in part for exactly
/// it. Red by an `init` that writes no exclude, by one that appends on every run (S8D-II-M6: the
/// second `init` must leave the bytes identical), by a block whose row does not ignore the derived
/// run tree, and by an `init` that does not create the file when git's template did not.
#[test]
fn init_writes_the_redkiln_runtime_exclude_block_once() {
    let repo = temp_repo();
    let root = repo.path();
    let info = common_dir(root).join("info");
    let exclude = info.join("exclude");
    let probe = run_tree_probe(root);

    // The pre-stage initialised shape: an `init` has run, and the exclude file is what it was
    // before that `init` — whatever git's template left, with no block.
    let template = std::fs::read(&exclude).ok();
    assert_eq!(run_init(root).status.code(), Some(0), "the seeding init");
    match &template {
        Some(bytes) => std::fs::write(&exclude, bytes).expect("the template exclude restores"),
        None => {
            let _ = std::fs::remove_file(&exclude);
        }
    }
    assert!(
        root.join(".redkiln").join("config.toml").is_file(),
        "the pre-stage shape has .redkiln/config.toml"
    );
    let before = template.unwrap_or_default();
    assert_eq!(
        marker_lines(&before),
        0,
        "the pre-stage exclude already carries the block"
    );
    // CONTROL: before the block, nothing ignores the run tree — so the ignore assertion below can
    // only pass because of the row `init` writes.
    assert!(
        !git_ignores(root, &probe),
        "the run tree `{probe}` is ignored before init wrote any block; the ignore assertion below \
         would read nothing"
    );

    let first = run_init(root);
    assert_eq!(
        first.status.code(),
        Some(0),
        "the first post-stage init exited {:?}; stderr:\n{}",
        first.status.code(),
        String::from_utf8_lossy(&first.stderr)
    );
    let after_first = std::fs::read(&exclude).unwrap_or_default();
    let mut expected = before;
    if !expected.is_empty() && !expected.ends_with(b"\n") {
        expected.push(b'\n');
    }
    expected.extend_from_slice(&expected_block());
    assert_eq!(
        String::from_utf8_lossy(&after_first),
        String::from_utf8_lossy(&expected),
        "init writes no exclude block, or not exactly one append of it after the template's bytes \
         (D21-45 W4; s8d-ii section 8d-ii.5)"
    );
    assert!(
        git_ignores(root, &probe),
        "the block init wrote does not make git ignore the derived run tree `{probe}`"
    );
    assert!(
        !root.join(".gitignore").exists(),
        "init wrote a .gitignore; the block goes to info/exclude only (D21-45: never to .gitignore)"
    );

    // S8D-II-M6: a second `init` finds the marker and leaves the file byte-identical.
    assert_eq!(run_init(root).status.code(), Some(0), "the second init");
    assert_eq!(
        std::fs::read(&exclude).expect("the exclude file is readable"),
        after_first,
        "a second init changed info/exclude; it appends only when the marker is absent \
         (ADR-0107 decision 2, for this file once the block exists)"
    );
    assert_eq!(marker_lines(&after_first), 1, "exactly one block");

    // The file is created when git's template did not leave one, and `info/` with it.
    std::fs::remove_dir_all(&info).expect("the info directory is removable");
    assert_eq!(run_init(root).status.code(), Some(0), "init with no info/");
    assert_eq!(
        std::fs::read(&exclude).ok(),
        Some(expected_block()),
        "init writes no exclude when info/exclude is absent; it creates the file with the block"
    );
    assert_eq!(run_init(root).status.code(), Some(0), "init again");
    assert_eq!(
        std::fs::read(&exclude).ok(),
        Some(expected_block()),
        "a second init changed the exclude file it created"
    );
}

/// D21-45 W4 / ADR-0180 (proposed): lines the operator wrote in `info/exclude` are never touched,
/// the block is appended after them, and `.gitignore` is never written (`s8d-ii` §8d-ii.5).
///
/// The operator's file has CRLF lines, no trailing newline, and a line that merely **starts
/// with** the marker text. Red by an `init` that writes no exclude, by one that rewrites or reorders
/// the operator's bytes, by one that glues the marker onto the operator's unterminated last line,
/// and by a marker match on a prefix (the look-alike line would then suppress the block).
#[test]
fn init_leaves_the_operators_exclude_lines_untouched() {
    let repo = temp_repo();
    let root = repo.path();
    let info = common_dir(root).join("info");
    std::fs::create_dir_all(&info).expect("the info directory exists");
    let exclude = info.join("exclude");
    let operator: &[u8] = b"# the operator's own rules\r\n*.log\r\n\
                            # redkiln-runtime-notes: the operator's, not init's\r\n/scratch/";
    std::fs::write(&exclude, operator).expect("the operator's exclude writes");

    let out = run_init(root);
    assert_eq!(
        out.status.code(),
        Some(0),
        "init exited {:?}; stderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let after = std::fs::read(&exclude).expect("the exclude file is readable");
    let mut expected = operator.to_vec();
    expected.push(b'\n');
    expected.extend_from_slice(&expected_block());
    assert_eq!(
        String::from_utf8_lossy(&after),
        String::from_utf8_lossy(&expected),
        "init writes no exclude block after the operator's lines, or it touched them: the \
         operator's bytes stand as written and the block follows on a line of its own"
    );
    assert!(
        after.starts_with(operator),
        "the operator's bytes are not a prefix of the file"
    );
    assert!(
        git_ignores(root, &run_tree_probe(root)),
        "the block after the operator's lines does not ignore the derived run tree"
    );
    assert!(
        !root.join(".gitignore").exists(),
        "init wrote a .gitignore (D21-45: never to .gitignore)"
    );

    assert_eq!(run_init(root).status.code(), Some(0), "the second init");
    assert_eq!(
        std::fs::read(&exclude).expect("the exclude file is readable"),
        after,
        "a second init changed the operator's exclude file"
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
