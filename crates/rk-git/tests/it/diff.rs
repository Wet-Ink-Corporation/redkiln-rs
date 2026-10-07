//! `rk_git::diff::range_diff` — the range read `ChangeSummary`'s `Text` arm is built from
//! (ADR-0165; `s8a` §8a.2, §8a.9). A **gix** read: gix-diff's tree diff for the changed
//! paths and their change kinds, and each changed blob read by object id for the text diff and the
//! LFS pointer check. No `GitOp` is added and nothing is spawned (ADR-0077: *"gix owns the read
//! path"*). The fixture repositories are BUILT through the one wrapper (`support::Fixture::git`),
//! as every other test in this binary builds them; the read under test spawns nothing.

use std::path::Path;

use rk_git::diff::{ChangeKind, RangeError, range_diff};

use crate::support::Fixture;

/// A repository with two commits: the first adds `a.txt`, `b.txt` and `dir/c.txt`; the second
/// modifies `a.txt`, deletes `b.txt` and adds `dir/d.txt`. `dir/c.txt` is untouched.
fn two_commit_repo(fx: &Fixture) -> std::path::PathBuf {
    let repo = fx.sub("repo");
    std::fs::create_dir_all(repo.join("dir")).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    std::fs::write(repo.join("a.txt"), b"one\nkeep\n").expect("write");
    std::fs::write(repo.join("b.txt"), b"doomed\n").expect("write");
    std::fs::write(repo.join("dir/c.txt"), b"still\n").expect("write");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "one"]);
    std::fs::write(repo.join("a.txt"), b"two\nkeep\n").expect("write");
    std::fs::remove_file(repo.join("b.txt")).expect("rm");
    std::fs::write(repo.join("dir/d.txt"), b"new\n").expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "two"]);
    repo
}

fn rev(fx: &Fixture, repo: &Path, spec: &str) -> String {
    fx.git(repo, &["rev-parse", spec]).stdout_trimmed()
}

/// Every changed path comes back, once, with its change kind, and nothing unchanged does; the two
/// ends are the resolved commits; each path's diff is the blob-level text diff.
#[test]
fn range_diff_lists_every_changed_path_with_its_change_kind() {
    let fx = Fixture::new();
    let repo = two_commit_repo(&fx);

    let got = range_diff(&repo, "HEAD~1..HEAD").expect("a two-commit range resolves and reads");

    let rows: Vec<(&str, ChangeKind)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("a.txt", ChangeKind::Modified),
            ("b.txt", ChangeKind::Deleted),
            ("dir/d.txt", ChangeKind::Added),
        ],
        "one row per changed FILE (no tree rows, no unchanged dir/c.txt), sorted by path"
    );
    assert_eq!(
        got.base,
        rev(&fx, &repo, "HEAD~1"),
        "the base end is resolved"
    );
    assert_eq!(
        got.head,
        rev(&fx, &repo, "HEAD"),
        "the head end is resolved"
    );
    assert!(
        got.entries.iter().all(|e| !e.lfs_pointer),
        "no path in this range is an LFS pointer"
    );

    let a = &got.entries[0].diff;
    assert!(
        a.contains("--- a/a.txt\n") && a.contains("+++ b/a.txt\n"),
        "a.txt's diff names both ends: {a:?}"
    );
    assert!(
        a.contains("\n-one\n") && a.contains("\n+two\n"),
        "a.txt's diff is read from the two blobs: {a:?}"
    );
    let b = &got.entries[1].diff;
    assert!(
        b.contains("--- a/b.txt\n") && b.contains("+++ /dev/null\n") && b.contains("\n-doomed\n"),
        "a deletion diffs against /dev/null: {b:?}"
    );
    let d = &got.entries[2].diff;
    assert!(
        d.contains("--- /dev/null\n") && d.contains("+++ b/dir/d.txt\n") && d.contains("\n+new\n"),
        "an addition diffs from /dev/null: {d:?}"
    );

    // A MODE-ONLY change is a Modified path whose diff says how it changed: git's `old mode` /
    // `new mode` lines, and no `---`/`+++` pair, since there is no hunk. The whole diff equals
    // `git diff`'s, which writes no `index` line when the blob is unchanged. *(Added 2026-09-25,
    // s8a fix round 2: the diff was the three header lines alone and never named the modes.)*
    fx.git(&repo, &["update-index", "--chmod=+x", "dir/c.txt"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "mode"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the mode-only range reads");
    let rows: Vec<(&str, ChangeKind)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change))
        .collect();
    assert_eq!(rows, vec![("dir/c.txt", ChangeKind::Modified)]);
    let out = fx.git(
        &repo,
        &["diff", "--no-color", "HEAD~1", "HEAD", "--", "dir/c.txt"],
    );
    let git = String::from_utf8(out.stdout).expect("git's diff is UTF-8");
    assert_eq!(
        got.entries[0].diff,
        "diff --git a/dir/c.txt b/dir/c.txt\nold mode 100644\nnew mode 100755\n",
        "a mode-only change names both modes and carries no ---/+++ pair"
    );
    assert_eq!(got.entries[0].diff, git, "the mode-only diff is git's");

    // An ADDED or DELETED path carries git's `new file mode <m>` / `deleted file mode <m>` line,
    // so an added executable keeps its mode, an added or deleted EMPTY file still says what
    // happened, and the concatenated diffs are a patch `git apply --index` takes from the base end
    // to exactly the head end's tree. Each path's WHOLE diff equals `git diff`'s with its `index`
    // lines dropped (none is written). *(Added 2026-09-25, s8a fix round 3: an addition was
    // rejected by `git apply` as "inconsistent old filename", a deletion was applied as a new file
    // named `dev/null`, and an added empty file was the bare `diff --git` line.)*
    std::fs::write(repo.join("gone-empty.txt"), b"").expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "prep"]);
    std::fs::remove_file(repo.join("gone-empty.txt")).expect("rm");
    std::fs::remove_file(repo.join("dir/d.txt")).expect("rm");
    std::fs::write(repo.join("empty.txt"), b"").expect("write");
    std::fs::write(repo.join("run.sh"), b"#!/bin/sh\necho hi\n").expect("write");
    // The working file carries the mode too where the filesystem has one: with `core.filemode`
    // true (POSIX), an index-only `+x` leaves a 0644 file that reads as a local modification and
    // the `checkout HEAD~1` below refuses it. Windows has no exec bit; the index flip is its mode.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(repo.join("run.sh"), std::fs::Permissions::from_mode(0o755))
            .expect("chmod");
    }
    std::fs::write(repo.join("a.txt"), b"three\nkeep\n").expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["update-index", "--chmod=+x", "run.sh"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "adds and deletes"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the add/delete range reads");
    let rows: Vec<(&str, ChangeKind)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("a.txt", ChangeKind::Modified),
            ("dir/d.txt", ChangeKind::Deleted),
            ("empty.txt", ChangeKind::Added),
            ("gone-empty.txt", ChangeKind::Deleted),
            ("run.sh", ChangeKind::Added),
        ]
    );
    for entry in &got.entries {
        assert_eq!(
            entry.diff,
            git_diff_without_index(&fx, &repo, "HEAD~1", "HEAD", &entry.path),
            "{}: the whole diff is git's",
            entry.path
        );
    }
    let patch = fx.sub("adds-deletes.patch");
    std::fs::write(
        &patch,
        got.entries
            .iter()
            .map(|e| e.diff.as_str())
            .collect::<String>(),
    )
    .expect("write the patch");
    let head_tree = rev(&fx, &repo, "HEAD^{tree}");
    fx.git(&repo, &["checkout", "--quiet", "--detach", "HEAD~1"]);
    fx.git(
        &repo,
        &["apply", "--index", patch.to_str().expect("a UTF-8 path")],
    );
    assert_eq!(
        fx.git(&repo, &["write-tree"]).stdout_trimmed(),
        head_tree,
        "the concatenated diffs apply from the base end to exactly the head end's tree"
    );
    fx.git(&repo, &["reset", "--quiet", "--hard", &got.head]);

    // A TYPE change (a blob that becomes a symlink) is git's deletion-then-addition pair, and an
    // added symlink carries `new file mode 120000`. Built in the index, so no filesystem symlink
    // is needed.
    let target = fx.sub("link-target");
    std::fs::write(&target, b"run.sh").expect("write");
    let blob = fx
        .git(
            &repo,
            &["hash-object", "-w", target.to_str().expect("a UTF-8 path")],
        )
        .stdout_trimmed();
    fx.git(
        &repo,
        &[
            "update-index",
            "--cacheinfo",
            &format!("120000,{blob},a.txt"),
        ],
    );
    fx.git(
        &repo,
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("120000,{blob},link"),
        ],
    );
    fx.git(&repo, &["commit", "--quiet", "-m", "symlinks"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the symlink range reads");
    let paths: Vec<&str> = got.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(paths, vec!["a.txt", "link"]);
    for entry in &got.entries {
        assert_eq!(
            entry.diff,
            git_diff_without_index(&fx, &repo, "HEAD~1", "HEAD", &entry.path),
            "{}: the whole diff is git's",
            entry.path
        );
    }
    assert!(
        got.entries[0].diff.contains("\ndeleted file mode 100644\n")
            && got.entries[0].diff.contains("\nnew file mode 120000\n"),
        "a type change is a deletion and an addition: {:?}",
        got.entries[0].diff
    );

    // A path git QUOTES: a tab, a double quote, a backslash or a byte >= 0x80 is written C-style
    // inside `"…"` on the `diff --git`, `---` and `+++` lines, and a `---`/`+++` name holding a
    // space is followed by a tab, as git writes them. The whole diff equals `git diff`'s, and the
    // patch applies at the base end's index to exactly the head end's tree. Built in the index
    // with `core.protectNTFS=false`, so no such file is ever written to this filesystem. *(Added
    // 2026-09-25, s8a fix round 5: the raw path made `git apply` refuse the patch with
    // "inconsistent new filename".)*
    let body = fx.sub("quoted-body");
    std::fs::write(&body, b"x\n").expect("write");
    let blob = fx
        .git(
            &repo,
            &["hash-object", "-w", body.to_str().expect("a UTF-8 path")],
        )
        .stdout_trimmed();
    let quoted = [
        "a\tb.txt",
        "q\"uote.txt",
        "back\\slash.txt",
        "sp ace.txt",
        "\u{e9}.txt",
    ];
    for path in quoted {
        fx.git(
            &repo,
            &[
                "-c",
                "core.protectNTFS=false",
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("100644,{blob},{path}"),
            ],
        );
    }
    fx.git(&repo, &["commit", "--quiet", "-m", "quoted paths"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the quoted-path range reads");
    assert_eq!(got.entries.len(), quoted.len(), "{:?}", got.entries);
    let ours: String = got.entries.iter().map(|e| e.diff.as_str()).collect();
    let out = fx.git(
        &repo,
        &["diff", "--no-color", "--no-renames", "HEAD~1", "HEAD"],
    );
    let git: String = String::from_utf8(out.stdout)
        .expect("git's diff is UTF-8")
        .lines()
        .filter(|l| !l.starts_with("index "))
        .map(|l| format!("{l}\n"))
        .collect();
    assert!(
        git.contains("diff --git \"a/a\\tb.txt\" \"b/a\\tb.txt\"\n"),
        "PREMISE: git quotes the tab path: {git:?}"
    );
    assert_eq!(ours, git, "the quoted paths' whole diff is git's");
    let patch = fx.sub("quoted.patch");
    std::fs::write(&patch, &ours).expect("write the patch");
    let head_tree = rev(&fx, &repo, "HEAD^{tree}");
    fx.git(&repo, &["read-tree", "HEAD~1"]);
    fx.git(
        &repo,
        &[
            "-c",
            "core.protectNTFS=false",
            "apply",
            "--cached",
            patch.to_str().expect("a UTF-8 path"),
        ],
    );
    assert_eq!(
        fx.git(&repo, &["write-tree"]).stdout_trimmed(),
        head_tree,
        "the quoted-path patch applies at the base end's index to exactly the head end's tree"
    );
    fx.git(
        &repo,
        &["-c", "core.protectNTFS=false", "read-tree", "HEAD"],
    );

    // A path that is NOT UTF-8 is refused by name (`Read`), never carried lossily: two paths that
    // differ only in a non-UTF-8 byte (`caf\xe8.txt` added, `caf\xe9.txt` modified) collapsed to
    // one `caf\u{FFFD}.txt` row twice, under a header naming neither file. The trees are written
    // as raw tree objects, so no such name ever reaches this filesystem or a process argument.
    // *(Added 2026-09-25, s8a fix round 8.)*
    let other = fx.sub("non-utf8-body");
    std::fs::write(&other, b"y\n").expect("write");
    let changed = fx
        .git(
            &repo,
            &["hash-object", "-w", other.to_str().expect("a UTF-8 path")],
        )
        .stdout_trimmed();
    let raw_tree = |name: &str, rows: &[(&[u8], &str)]| -> String {
        let mut bytes = Vec::new();
        for (path, id) in rows {
            bytes.extend_from_slice(b"100644 ");
            bytes.extend_from_slice(path);
            bytes.push(0);
            bytes.extend(
                (0..id.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&id[i..i + 2], 16).expect("a hex object id")),
            );
        }
        let file = fx.sub(name);
        std::fs::write(&file, bytes).expect("write the raw tree");
        fx.git(
            &repo,
            &[
                "hash-object",
                "-t",
                "tree",
                "-w",
                file.to_str().expect("a UTF-8 path"),
            ],
        )
        .stdout_trimmed()
    };
    let base_tree = raw_tree("base-tree", &[(b"caf\xe9.txt", &blob)]);
    let head_tree = raw_tree(
        "head-tree",
        &[(b"caf\xe8.txt", &blob), (b"caf\xe9.txt", &changed)],
    );
    let base = fx
        .git(&repo, &["commit-tree", &base_tree, "-m", "base"])
        .stdout_trimmed();
    let head = fx
        .git(
            &repo,
            &["commit-tree", &head_tree, "-p", &base, "-m", "head"],
        )
        .stdout_trimmed();
    let err = range_diff(&repo, &format!("{base}..{head}"))
        .expect_err("a non-UTF-8 path is refused, never read lossily");
    let RangeError::Read { detail } = &err else {
        panic!("a non-UTF-8 path is `Read`, naming it: {err:?}");
    };
    assert!(
        detail.contains("caf\\xe8.txt") && detail.contains("not UTF-8"),
        "the refusal names the path's raw bytes: {detail:?}"
    );

    // A DIVERGENT range is read tree to tree, the two-dot meaning: the main line adds `y.txt`
    // after the fork point and a side branch modifies `x.txt`, so `main..side` lists `x.txt`
    // Modified AND `y.txt` Deleted. A merge-base (three-dot) read, or a read of the head against
    // its own first parent, would drop `y.txt`; every other range in this binary has its base as
    // an ancestor of its head, where the three readings agree. *(Added 2026-09-25, s8a fix round
    // 9.)*
    let div = fx.sub("diverge");
    std::fs::create_dir_all(&div).expect("mkdir");
    fx.git(&div, &["init", "--quiet"]);
    std::fs::write(div.join("x.txt"), b"x\n").expect("write");
    fx.git(&div, &["add", "."]);
    fx.git(&div, &["commit", "--quiet", "-m", "fork point"]);
    let fork = rev(&fx, &div, "HEAD");
    std::fs::write(div.join("y.txt"), b"y\n").expect("write");
    fx.git(&div, &["add", "."]);
    fx.git(&div, &["commit", "--quiet", "-m", "main line"]);
    let main = rev(&fx, &div, "HEAD");
    fx.git(&div, &["checkout", "--quiet", "-b", "side", &fork]);
    std::fs::write(div.join("x.txt"), b"x\nside\n").expect("write");
    fx.git(&div, &["add", "."]);
    fx.git(&div, &["commit", "--quiet", "-m", "side"]);
    let side = rev(&fx, &div, "HEAD");
    assert_ne!(
        fx.git(&div, &["merge-base", &main, &side]).stdout_trimmed(),
        main,
        "PREMISE: the base end is not an ancestor of the head end"
    );
    let got = range_diff(&div, &format!("{main}..{side}")).expect("the divergent range reads");
    let rows: Vec<(&str, ChangeKind)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("x.txt", ChangeKind::Modified),
            ("y.txt", ChangeKind::Deleted)
        ],
        "`main..side` is the tree-to-tree answer, never the merge-base one"
    );
    assert_eq!(
        (got.base.as_str(), got.head.as_str()),
        (main.as_str(), side.as_str())
    );
    let out = fx.git(&div, &["diff", "--no-color", "--no-renames", &main, &side]);
    let git: String = String::from_utf8(out.stdout)
        .expect("git's diff is UTF-8")
        .lines()
        .filter(|l| !l.starts_with("index "))
        .map(|l| format!("{l}\n"))
        .collect();
    let ours: String = got.entries.iter().map(|e| e.diff.as_str()).collect();
    assert_eq!(
        ours, git,
        "the divergent range's whole diff is `git diff main side`'s"
    );
}

/// `git diff --no-color <base> <head> -- <path>` with its `index` lines dropped: `range_diff`
/// writes no `index` line. *(Added 2026-09-25, s8a fix round 3.)*
fn git_diff_without_index(fx: &Fixture, repo: &Path, base: &str, head: &str, path: &str) -> String {
    let out = fx.git(repo, &["diff", "--no-color", base, head, "--", path]);
    String::from_utf8(out.stdout)
        .expect("git's diff is UTF-8")
        .lines()
        .filter(|l| !l.starts_with("index "))
        .map(|l| format!("{l}\n"))
        .collect()
}

/// An empty range, a range that is not `<base>..<head>`, an end that resolves to nothing and a
/// directory that is no repository are each refused BY NAME, never read as an empty diff. An end
/// that resolves but whose commit or tree is missing from the store is `Read`, not `Resolve`: a
/// store fault is not an unresolvable range *(added 2026-09-25, s8a fix round 4)*.
///
/// **The control comes first**: the same repository's real range must read, or every refusal
/// below would pass on a function that refuses everything.
#[test]
fn range_diff_refuses_an_unresolvable_range() {
    let fx = Fixture::new();
    let repo = two_commit_repo(&fx);

    let control = range_diff(&repo, "HEAD~1..HEAD").expect("CONTROL: the real range reads");
    assert_eq!(
        control.entries.len(),
        3,
        "CONTROL: the real range carries three paths"
    );

    assert_eq!(
        range_diff(&repo, ""),
        Err(RangeError::Empty),
        "an empty range"
    );
    assert_eq!(
        range_diff(&repo, "  "),
        Err(RangeError::Empty),
        "a blank range"
    );
    match range_diff(&repo, "HEAD") {
        Err(RangeError::Malformed { range }) => assert_eq!(range, "HEAD"),
        other => panic!("a single revision is not a range: {other:?}"),
    }
    match range_diff(&repo, "HEAD~1...HEAD") {
        Err(RangeError::Malformed { range }) => assert_eq!(range, "HEAD~1...HEAD"),
        other => panic!("a symmetric-difference range is not read as a two-dot one: {other:?}"),
    }
    match range_diff(&repo, "HEAD~1..no-such-rev") {
        Err(RangeError::Resolve { rev, .. }) => assert_eq!(rev, "no-such-rev"),
        other => panic!("an unresolvable head end is refused naming it: {other:?}"),
    }
    match range_diff(&repo, "HEAD~5..HEAD") {
        Err(RangeError::Resolve { rev, .. }) => assert_eq!(rev, "HEAD~5"),
        other => panic!("a base end beyond the history is refused naming it: {other:?}"),
    }
    let not_a_repo = fx.sub("plain-dir");
    std::fs::create_dir_all(&not_a_repo).expect("mkdir");
    match range_diff(&not_a_repo, "HEAD~1..HEAD") {
        Err(RangeError::Open { .. }) => {}
        other => panic!(
            "a directory that is no repository is refused, never discovered upward: {other:?}"
        ),
    }
    match range_diff(&repo, "HEAD~1^{tree}..HEAD") {
        Err(RangeError::Resolve { rev, .. }) => assert_eq!(rev, "HEAD~1^{tree}"),
        other => panic!("an end naming a tree, not a commit, is refused naming it: {other:?}"),
    }

    // The range is read from the object store, never from the checkout's index (added 2026-09-25,
    // s8a fix round 7): gix's `diff_tree_to_tree` loaded `.git/index` for a blob resource cache,
    // so an index under 20 bytes PANICKED inside gix-index and a damaged one failed a committed
    // range. A 0-byte and a garbage index must both leave the range readable.
    let index = repo.join(".git/index");
    let saved_index = std::fs::read(&index).expect("PREMISE: the fixture has an index");
    for (what, bytes) in [("a 0-byte", &b""[..]), ("a garbage", &b"DIRC garbage"[..])] {
        std::fs::write(&index, bytes).expect("damage the index");
        match range_diff(&repo, "HEAD~1..HEAD") {
            Ok(read) => assert_eq!(
                read, control,
                "{what} index leaves the committed range whole"
            ),
            other => panic!("{what} index must not fail a committed range: {other:?}"),
        }
    }
    std::fs::write(&index, &saved_index).expect("restore the index");
    // A repository whose config does not parse is a store fault, `Read` with gix's source chain,
    // never the `Open` a directory that is no repository gets (added 2026-09-25, s8a fix round 7).
    let config = repo.join(".git/config");
    let saved_config = std::fs::read(&config).expect("PREMISE: the fixture has a config");
    let mut broken = saved_config.clone();
    broken.extend_from_slice(b"[core\n  broken = ");
    std::fs::write(&config, &broken).expect("break the config");
    match range_diff(&repo, "HEAD~1..HEAD") {
        Err(RangeError::Read { detail }) => assert!(
            detail.contains("configuration") && detail.contains("unexpected token on line"),
            "a broken config is Read carrying gix's source chain: {detail:?}"
        ),
        other => panic!("a repository whose config does not parse is Read, not Open: {other:?}"),
    }
    std::fs::write(&config, &saved_config).expect("restore the config");

    // A store fault is not an unresolvable range (added 2026-09-25, s8a fix round 4). Each end
    // below RESOLVES to an id; the object behind it is gone from the store, so the read is
    // `Read` naming the id, never the `Resolve` a consumer answers as "nothing to summarise".
    // A walk past a shallow boundary is no store fault: the parent is absent by design, and the
    // end stays `Resolve` (added 2026-09-25, s8a fix round 6).
    let shallow = fx.sub("shallow");
    let url = format!("file://{}", repo.display().to_string().replace('\\', "/"));
    fx.git(
        &fx.sub(""),
        &[
            "clone",
            "--quiet",
            "--depth",
            "1",
            &url,
            shallow.to_str().expect("utf-8 path"),
        ],
    );
    assert!(
        shallow.join(".git/shallow").is_file(),
        "PREMISE: the clone is shallow at HEAD"
    );
    match range_diff(&shallow, "HEAD~1..HEAD") {
        Err(RangeError::Resolve { rev, .. }) => assert_eq!(rev, "HEAD~1"),
        other => panic!("a walk past a shallow boundary is Resolve, not a store fault: {other:?}"),
    }
    // A changed path's blob, and a changed subtree, gone from the store are the same store fault
    // past the two ends (added 2026-09-25, s8a fix round 13): `Read` naming the object, never a
    // `Text` built from bytes that were never read, and never `Resolve`. The blob goes first: a
    // missing subtree fails the tree diff before any blob is read.
    let base_blob = rev(&fx, &repo, "HEAD~1:a.txt");
    delete_loose_object(&repo, &base_blob);
    match range_diff(&repo, "HEAD~1..HEAD") {
        Err(RangeError::Read { detail }) => assert!(
            detail.contains(&base_blob) && detail.contains("a.txt"),
            "a missing blob is Read naming the blob and its path: {detail:?}"
        ),
        other => panic!("a changed path's blob missing from the store is a store fault: {other:?}"),
    }
    let base_subtree = rev(&fx, &repo, "HEAD~1:dir");
    delete_loose_object(&repo, &base_subtree);
    match range_diff(&repo, "HEAD~1..HEAD") {
        Err(RangeError::Read { detail }) => assert!(
            detail.contains(&base_subtree),
            "a missing subtree is Read naming the subtree: {detail:?}"
        ),
        other => panic!("a changed subtree missing from the store is a store fault: {other:?}"),
    }
    let base_tree = rev(&fx, &repo, "HEAD~1^{tree}");
    let base_commit = rev(&fx, &repo, "HEAD~1");
    fx.git(&repo, &["branch", "base", "HEAD~1"]);
    delete_loose_object(&repo, &base_tree);
    match range_diff(&repo, "HEAD~1..HEAD") {
        Err(RangeError::Read { detail }) => assert!(
            detail.contains(&base_tree),
            "a missing tree at a resolved end is Read naming the tree: {detail:?}"
        ),
        other => panic!("a missing tree is a store fault, not an unresolvable end: {other:?}"),
    }
    delete_loose_object(&repo, &base_commit);
    match range_diff(&repo, "base..HEAD") {
        Err(RangeError::Read { detail }) => assert!(
            detail.contains(&base_commit),
            "a ref whose commit is missing is Read naming the commit: {detail:?}"
        ),
        other => panic!("a missing commit behind a ref is a store fault: {other:?}"),
    }
    // An end reached by walking parents (`~N`, `^N`) to a commit gone from the store is the same
    // store fault, `Read` naming the commit, never `Resolve` (added 2026-09-25, s8a fix round 6:
    // gix's own ancestor walk fails before an id is returned, and that failure read as `Resolve`).
    for walked in ["HEAD~1", "HEAD^", "HEAD^1", "HEAD~", "HEAD^^", "HEAD~^"] {
        match range_diff(&repo, &format!("{walked}..HEAD")) {
            Err(RangeError::Read { detail }) => assert!(
                detail.contains(&base_commit),
                "`{walked}` reaching a missing commit is Read naming the commit: {detail:?}"
            ),
            other => panic!(
                "`{walked}` walks to a commit missing from the store, a store fault: {other:?}"
            ),
        }
    }
    match range_diff(&repo, "HEAD^2..HEAD") {
        Err(RangeError::Resolve { rev, .. }) => assert_eq!(rev, "HEAD^2"),
        other => panic!("a parent number the commit does not have is still Resolve: {other:?}"),
    }
}

/// Remove one loose object from `repo`'s store (git writes them read-only).
fn delete_loose_object(repo: &Path, id: &str) {
    let path = repo.join(".git/objects").join(&id[..2]).join(&id[2..]);
    let mut perms = std::fs::metadata(&path)
        .unwrap_or_else(|e| panic!("PREMISE: {id} is a loose object at {path:?}: {e}"))
        .permissions();
    #[expect(
        clippy::permissions_set_readonly_false,
        reason = "git writes loose objects read-only; the store-fault fixture must delete one"
    )]
    perms.set_readonly(false);
    std::fs::set_permissions(&path, perms).expect("chmod");
    std::fs::remove_file(&path).expect("rm object");
}

/// The git-lfs pointer form, as git-lfs writes it.
const POINTER: &[u8] = b"version https://git-lfs.github.com/spec/v1\noid sha256:4d7a214614ab2935c943f9e0ff69d22eadbb8f32b1258daaa5e2ca24d17e2393\nsize 12345\n";

/// A path whose blob at EITHER end has the git-lfs pointer form (under 1024 bytes, first line
/// `version https://git-lfs.github.com/spec/v1`) is marked, decided on the blob and not on
/// `.gitattributes`; an ordinary `.json` beside it is not, and neither are two near-misses.
#[test]
fn range_diff_marks_an_lfs_pointer_path() {
    let fx = Fixture::new();
    let repo = fx.sub("repo");
    std::fs::create_dir_all(repo.join("data")).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    std::fs::write(repo.join("README.md"), b"base\n").expect("write");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "base"]);

    // The commit under test: a pointer blob, an ordinary json, and two near-misses.
    std::fs::write(repo.join("data/table.json"), POINTER).expect("write");
    std::fs::write(repo.join("data/plain.json"), b"{\"rows\": [1, 2, 3]}\n").expect("write");
    let mut big = POINTER.to_vec();
    big.extend(std::iter::repeat_n(b'x', 1024));
    std::fs::write(repo.join("data/too-big.json"), &big).expect("write");
    let mut second_line = b"{}\n".to_vec();
    second_line.extend_from_slice(POINTER);
    std::fs::write(repo.join("data/second-line.json"), &second_line).expect("write");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "lfs"]);

    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the range reads");
    let marks: Vec<(&str, bool)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.lfs_pointer))
        .collect();
    assert_eq!(
        marks,
        vec![
            ("data/plain.json", false),
            ("data/second-line.json", false),
            ("data/table.json", true),
            ("data/too-big.json", false),
        ],
        "only the pointer blob is marked; the ordinary json and the two near-misses are not"
    );

    // AT EITHER END: deleting the pointer leaves it only at the base end, and it is still marked.
    std::fs::remove_file(repo.join("data/table.json")).expect("rm");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "drop the pointer"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the range reads");
    assert_eq!(got.entries.len(), 1, "one path changed: {:?}", got.entries);
    assert_eq!(got.entries[0].path, "data/table.json");
    assert_eq!(got.entries[0].change, ChangeKind::Deleted);
    assert!(
        got.entries[0].lfs_pointer,
        "a pointer at the BASE end marks the path as well as one at the head end"
    );
}

/// A path whose blob at EITHER end is binary (a NUL in its leading bytes, or not UTF-8) is marked
/// `binary`, decided on the range's blobs and never on the working tree: an extensionless
/// NUL-bearing blob deleted in the range, and one added at a head the checkout is not on, are both
/// marked; a text path beside them is not. *(Added 2026-09-25, s8a fix round 1: a consumer that
/// classified by the working tree answered `Text` for both, the AC-497 failure.)*
#[test]
fn range_diff_marks_a_binary_blob_at_either_end() {
    let fx = Fixture::new();
    let repo = fx.sub("repo");
    std::fs::create_dir_all(repo.join("assets")).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    std::fs::write(repo.join("a.txt"), b"one\n").expect("write");
    std::fs::write(repo.join("assets/blob"), b"\x00\x01\x02binary\x00payload").expect("write");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "base"]);
    let base = rev(&fx, &repo, "HEAD");

    // Deleted in the range: the blob exists at the base end only.
    std::fs::remove_file(repo.join("assets/blob")).expect("rm");
    std::fs::write(repo.join("a.txt"), b"two\n").expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "drop the blob"]);
    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the range reads");
    let marks: Vec<(&str, bool)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.binary))
        .collect();
    assert_eq!(
        marks,
        vec![("a.txt", false), ("assets/blob", true)],
        "a binary blob at the BASE end marks the path; the text path is not marked"
    );

    // Added at a head the checkout is not on: the working tree never holds the file.
    std::fs::write(repo.join("tool"), b"\x7fELF\x02\x01\x01\x00\x00\x00tool").expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "add a tool"]);
    let head = rev(&fx, &repo, "HEAD");
    fx.git(&repo, &["checkout", "--quiet", &base]);
    assert!(
        !repo.join("tool").exists(),
        "PREMISE: the checkout is not on the head end"
    );
    let got = range_diff(&repo, &format!("{base}..{head}")).expect("the range reads");
    let tool = got
        .entries
        .iter()
        .find(|e| e.path == "tool")
        .expect("the added tool is a changed path");
    assert!(
        tool.binary,
        "a binary blob at the HEAD end marks the path whatever the checkout holds"
    );
    // The range spans BOTH commits, so the earlier commit's deletion is listed beside the later
    // commit's addition: a read of the head against its own first parent lists `tool` alone.
    // *(Added 2026-09-25, s8a fix round 9.)*
    let rows: Vec<(&str, ChangeKind, bool)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change, e.binary))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("a.txt", ChangeKind::Modified, false),
            ("assets/blob", ChangeKind::Deleted, true),
            ("tool", ChangeKind::Added, true),
        ],
        "the two-commit range is read end to end"
    );
    assert!(
        got.entries
            .iter()
            .all(|e| e.path == "tool" || e.path == "assets/blob" || !e.binary),
        "only the binary blobs are marked: {:?}",
        got.entries
    );

    // The NOT-UTF-8 half: a `.txt` blob with no NUL whose bytes are Latin-1 is marked too, so no
    // consumer that answers `Text` by extension can carry it. *(Added 2026-09-25, s8a fix round
    // 14: every binary blob above carries a NUL, so a NUL-only mark passed this test.)*
    let latin = fx.sub("latin");
    std::fs::create_dir_all(&latin).expect("mkdir");
    fx.git(&latin, &["init", "--quiet"]);
    std::fs::write(latin.join("notes.txt"), b"caf\xe9\n").expect("write");
    fx.git(&latin, &["add", "."]);
    fx.git(&latin, &["commit", "--quiet", "-m", "latin-1"]);
    std::fs::write(latin.join("notes.txt"), b"caf\xe8\n").expect("write");
    fx.git(&latin, &["add", "-A"]);
    fx.git(&latin, &["commit", "--quiet", "-m", "latin-1 again"]);
    let got = range_diff(&latin, "HEAD~1..HEAD").expect("the range reads");
    let rows: Vec<(&str, ChangeKind, bool)> = got
        .entries
        .iter()
        .map(|e| (e.path.as_str(), e.change, e.binary))
        .collect();
    assert_eq!(
        rows,
        vec![("notes.txt", ChangeKind::Modified, true)],
        "a NUL-free blob that is not UTF-8 is marked binary"
    );
}

/// A side whose last line has no final newline carries git's `\ No newline at end of file` marker
/// after that line, so a change that only adds or drops a trailing newline is visible and never
/// reads as a `-X`/`+X` no-op. The hunk bodies are compared with `git diff`'s own over the same
/// range. *(Added 2026-09-25, s8a fix round 1.)* The comparison runs from the first `@@`, so it
/// covers the hunk HEADERS too, and the fixture carries a one-line modification, a one-line
/// addition, a one-line and a two-line deletion: git leaves out a length of 1 and writes a start of
/// 0 for a side with no lines, which a fixture of two-line modifications alone cannot tell from
/// `@@ -{},{} +{},{} @@`. *(Amended 2026-09-25, s8a fix round 2.)*
#[test]
fn range_diff_marks_a_missing_final_newline() {
    let fx = Fixture::new();
    let repo = fx.sub("repo");
    std::fs::create_dir_all(&repo).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    std::fs::write(repo.join("x.txt"), b"one\ntwo").expect("write");
    std::fs::write(repo.join("y.txt"), b"keep\ntail").expect("write");
    std::fs::write(repo.join("one.txt"), b"a\n").expect("write");
    std::fs::write(repo.join("gone1.txt"), b"p\n").expect("write");
    std::fs::write(repo.join("gone2.txt"), b"p\nq\n").expect("write");
    std::fs::write(repo.join("big.txt"), section_heading_file(&[])).expect("write");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "base"]);
    std::fs::write(repo.join("x.txt"), b"one\nthree").expect("write");
    std::fs::write(repo.join("y.txt"), b"keep\ntail\n").expect("write");
    std::fs::write(repo.join("one.txt"), b"b\n").expect("write");
    std::fs::write(repo.join("new.txt"), b"x\n").expect("write");
    std::fs::remove_file(repo.join("gone1.txt")).expect("rm");
    std::fs::remove_file(repo.join("gone2.txt")).expect("rm");
    std::fs::write(
        repo.join("big.txt"),
        section_heading_file(&[15, 35, 50, 58]),
    )
    .expect("write");
    fx.git(&repo, &["add", "-A"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "head"]);

    let got = range_diff(&repo, "HEAD~1..HEAD").expect("the range reads");
    let paths: Vec<&str> = got.entries.iter().map(|e| e.path.as_str()).collect();
    assert_eq!(
        paths,
        vec![
            "big.txt",
            "gone1.txt",
            "gone2.txt",
            "new.txt",
            "one.txt",
            "x.txt",
            "y.txt"
        ],
        "every changed path is read"
    );
    let by_path = |p: &str| {
        got.entries
            .iter()
            .find(|e| e.path == p)
            .expect("the path is an entry")
    };
    assert!(
        by_path("one.txt").diff.contains("\n@@ -1 +1 @@\n"),
        "a one-line side's length is left out, as git writes it: {:?}",
        by_path("one.txt").diff
    );
    assert!(
        by_path("new.txt").diff.contains("\n@@ -0,0 +1 @@\n"),
        "an added file's empty side starts at 0, as git writes it: {:?}",
        by_path("new.txt").diff
    );
    assert!(
        by_path("gone2.txt").diff.contains("\n@@ -1,2 +0,0 @@\n"),
        "a deleted file's empty side starts at 0, as git writes it: {:?}",
        by_path("gone2.txt").diff
    );
    let x = by_path("x.txt");
    assert_eq!(x.path, "x.txt");
    assert!(
        x.diff.ends_with(
            " one\n-two\n\\ No newline at end of file\n+three\n\\ No newline at end of file\n"
        ),
        "both sides lack a final newline, and each last line carries the marker: {:?}",
        x.diff
    );
    let y = by_path("y.txt");
    assert_eq!(y.path, "y.txt");
    assert!(
        y.diff
            .ends_with(" keep\n-tail\n\\ No newline at end of file\n+tail\n"),
        "a trailing-newline-only change is visible, not a -tail/+tail no-op: {:?}",
        y.diff
    );

    for entry in &got.entries {
        let out = fx.git(
            &repo,
            &["diff", "--no-color", "HEAD~1", "HEAD", "--", &entry.path],
        );
        let git = String::from_utf8(out.stdout).expect("git's diff is UTF-8");
        let git_hunks = &git[git.find("@@").expect("git's diff has a hunk")..];
        let ours = &entry.diff[entry.diff.find("@@").expect("our diff has a hunk")..];
        assert_eq!(ours, git_hunks, "{}: the hunk body is git's", entry.path);
    }

    // A hunk that does not start at line 1 carries git's default section heading after its
    // `@@`: the nearest base-side line before the hunk that starts with a letter, `_` or `$`,
    // cut to 80 bytes and stripped of trailing whitespace. The comparison above covers it; these
    // name each rule. *(Added 2026-09-25, s8a fix round 5: the heading was left out.)*
    let big = &by_path("big.txt").diff;
    for heading in [
        "@@ -12,7 +12,7 @@ fn a_long_function_name_that_runs_past_the_eighty_byte_heading_limit_xdiff_keeps\n",
        "@@ -32,7 +32,7 @@ impl Short\n",
        "@@ -47,7 +47,7 @@ $dollar\n",
        "@@ -55,6 +55,6 @@ _under\n",
    ] {
        assert!(
            big.contains(heading),
            "the section heading is git's: {heading:?} in {big:?}"
        );
    }
}

/// Sixty lines whose only heading candidates (a first byte that is an ASCII letter, `_` or `$`)
/// sit at lines 10 (longer than 80 bytes), 25 (trailing whitespace), 45 and 53; line 46 starts
/// with a non-ASCII letter, which git's heading rule does not take. Each line in `changed` is
/// rewritten. *(Added 2026-09-25, s8a fix round 5.)*
fn section_heading_file(changed: &[usize]) -> Vec<u8> {
    (1..=60)
        .map(|i| {
            let line = match i {
                _ if changed.contains(&i) => format!("changed {i}"),
                10 => "fn a_long_function_name_that_runs_past_the_eighty_byte_heading_limit_xdiff_keeps(x: u32) {"
                    .to_owned(),
                25 => "impl Short \t ".to_owned(),
                45 => "$dollar".to_owned(),
                46 => "\u{e9}lan".to_owned(),
                53 => "_under".to_owned(),
                _ if i % 2 == 0 => format!("    indented {i}"),
                _ => format!("{i} ( digit"),
            };
            format!("{line}\n")
        })
        .collect::<String>()
        .into_bytes()
}
