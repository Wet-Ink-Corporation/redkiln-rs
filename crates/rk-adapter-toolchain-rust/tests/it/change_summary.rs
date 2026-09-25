//! `RustToolchain::change_summary` — the `Text` arm from a real range read (ADR-0165; `s8a`
//! §8a.2, §8a.9). The body calls `rk_git::diff::range_diff`, a gix read (ADR-0077), and:
//!
//! - returns `Ok(ChangeSummary::Text(..))` when every changed path classifies `Text` or
//!   `Generated`, with one manifest row per changed path;
//! - refuses a range carrying a `Binary` path, or an LFS pointer path, through the re-dated
//!   `change_summary` registry row, NAMING the path -- never a `Text` that silently omits it
//!   (AC-497), never a `Text` whose `diff` is a pointer's `oid`/`size` delta (§8a.2 item 2a);
//! - refuses an empty, unresolvable or change-less range by the capability's name. The kernel
//!   calls `change_summary("")` today (`crates/rk-kernel/src/runtime/step.rs`), so an `Ok` there
//!   would turn eleven refusals into eleven vacuous greens.
//!
//! Every test runs over a throwaway two-commit repository (`support::FixtureRepo`), never over the
//! workspace running the suite.

use rk_ports::toolchain::{ChangeKind, ChangeSummary, Classification, Toolchain};
use rk_types::error::{StepError, TransportKind};

use crate::support::{
    FixtureRepo, binary_range_repo, deleted_binary_range_repo, lfs_range_repo, text_range_repo,
    unchecked_out_binary_range_repo,
};

/// Over a two-commit repository whose range touches only text and generated paths, the body
/// returns `Text`: the head commit as `sha`, one manifest row per changed path (path, `classify`
/// result, change kind), and a diff that carries every path's header and the changed line.
#[test]
fn change_summary_text_variant_from_git_diff() {
    let repo = text_range_repo();
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = repo.adapter(scratch.path());

    let summary = tc.change_summary("HEAD~1..HEAD");
    let answer = format!("{summary:?}");
    let text = match summary {
        Ok(ChangeSummary::Text(text)) => text,
        other => panic!("a text range yields Ok(ChangeSummary::Text(..)); got {other:?}"),
    };

    assert_eq!(
        text.sha,
        repo.git(&["rev-parse", "HEAD"]),
        "`sha` is the range's head commit (`artifact:ChangeSummary.sha`)"
    );
    let rows: Vec<(&str, Classification, ChangeKind)> = text
        .manifest
        .iter()
        .map(|m| (m.path.as_str(), m.classification, m.change))
        .collect();
    assert_eq!(
        rows,
        vec![
            (
                "Cargo.lock",
                Classification::Generated,
                ChangeKind::Modified
            ),
            ("README.md", Classification::Text, ChangeKind::Deleted),
            ("notes.md", Classification::Text, ChangeKind::Added),
            ("src/lib.rs", Classification::Text, ChangeKind::Modified),
        ],
        "one manifest row per changed path, each with the adapter's own classify result and the \
         range's change kind"
    );
    for row in &text.manifest {
        assert!(
            text.diff
                .contains(&format!("diff --git a/{0} b/{0}\n", row.path)),
            "the diff carries {}'s header: {:?}",
            row.path,
            text.diff
        );
    }
    assert!(
        text.diff.contains("\n-    1\n") && text.diff.contains("\n+    2\n"),
        "the diff is read from the two blobs of src/lib.rs: {:?}",
        text.diff
    );
    // The whole value, not its pieces (added 2026-09-25, s8a fix round 11): `diff` is what
    // `${summarise.outputs.diff}` hands the lint step, so its content, path order and separators
    // are pinned against git's own answer for the range, `index` lines dropped (`range_diff`
    // writes none).
    let git: String = repo
        .git(&["diff", "--no-color", "--no-renames", "HEAD~1", "HEAD"])
        .lines()
        .filter(|l| !l.starts_with("index "))
        .map(|l| format!("{l}\n"))
        .collect();
    assert_eq!(
        text.diff, git,
        "`diff` is the range's whole diff, every path's hunks in manifest order"
    );

    // git is the authority for the `Text` arm (added 2026-09-25, s8a, D21-51; ADR-0165 decisions
    // 4 and 9): the arm is a function of the range, read from the object store, and reads neither
    // a `Cargo.toml` (no `cargo metadata`) nor a changed path's working-tree bytes. The fixture's
    // `Cargo.toml` is in both commits and outside the range; `src/lib.rs` is a changed path.
    // Mutated, then deleted, in the working tree: the answer is byte-identical each time.
    assert!(
        repo.root().join("Cargo.toml").is_file() && answer.starts_with("Ok(Text("),
        "PREMISE: the working tree holds the committed Cargo.toml and the range answers Text"
    );
    repo.write("Cargo.toml", b"[package\0 this is not a manifest\n");
    repo.write("src/lib.rs", b"pub fn one() -> u32 {\n    99\n}\n\0");
    assert_eq!(
        format!("{:?}", tc.change_summary("HEAD~1..HEAD")),
        answer,
        "a mutated working-tree Cargo.toml and changed path never move the range's answer"
    );
    repo.remove("Cargo.toml");
    repo.remove("src/lib.rs");
    assert_eq!(
        format!("{:?}", tc.change_summary("HEAD~1..HEAD")),
        answer,
        "a deleted working-tree Cargo.toml and changed path never move the range's answer"
    );
}

/// A `.uasset` in the range refuses through the re-dated row, naming the path; it never yields a
/// `Text` that silently omits the path (AC-497).
#[test]
fn change_summary_refuses_a_range_carrying_a_binary_path() {
    let repo = binary_range_repo();
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = repo.adapter(scratch.path());

    assert_eq!(
        tc.classify("Content/Hero.uasset").expect("classify"),
        Classification::Binary,
        "PREMISE: the adapter's own classify answers Binary for the .uasset"
    );
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.starts_with("change_summary") && capability.contains("Content/Hero.uasset"),
            "the refusal carries the capability token and names the Binary path: {capability:?}"
        ),
        other => panic!(
            "a range carrying a Binary path refuses by name; a Text (even one listing only the \
             text path) would omit the binary change (AC-497). Got {other:?}"
        ),
    }
}

/// An empty, blank, unresolvable or change-less range refuses by the capability's name -- the
/// value the kernel receives for its `change_summary("")` today -- and never answers an empty
/// `Text`.
///
/// **The control comes first**: the same adapter over the same repository answers `Text` for the
/// real range, so each refusal below is the body's decision, not an adapter that cannot read.
/// Last, a tree missing from the store must NOT take that refusal: it is `Transport { Io }`
/// *(added 2026-09-25, s8a fix round 4)*.
#[test]
fn change_summary_refuses_an_empty_range() {
    let repo = text_range_repo();
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = repo.adapter(scratch.path());

    assert!(
        matches!(
            tc.change_summary("HEAD~1..HEAD"),
            Ok(ChangeSummary::Text(_))
        ),
        "CONTROL: the real range answers Text, or the refusals below prove nothing"
    );
    for range in ["", "   ", "HEAD..HEAD", "HEAD~1..no-such-rev"] {
        match tc.change_summary(range) {
            Err(StepError::Unsupported { capability }) => assert_eq!(
                capability, "change_summary",
                "range {range:?} is refused by the capability's name, from the registry row"
            ),
            other => panic!(
                "range {range:?} carries nothing to summarise and must be refused by name, never \
                 an empty Text; got {other:?}"
            ),
        }
    }

    // A single revision is not a range (added 2026-09-25, s8a, D21-51; ADR-0165 decisions 4 and
    // 6): the shipped catalog binds `range = "artifact:Changeset.sha"`, one sha, and under
    // step-grain commits a changeset spans several commits, so reading `R^..R` would answer a
    // `Text` omitting the earlier ones. It refuses NAMING the reason -- never the bare token
    // `""` gets, and never a one-commit summary. `HEAD` was in the loop above until this round.
    let head = repo.git(&["rev-parse", "HEAD"]);
    for rev in ["HEAD", head.as_str()] {
        match tc.change_summary(rev) {
            Err(StepError::Unsupported { capability }) => assert_eq!(
                capability,
                format!("change_summary: `{rev}` is a single revision, not a <base>..<head> range"),
                "a single revision refuses by name, naming the revision and the grammar"
            ),
            other => panic!(
                "a single revision {rev:?} must refuse naming why, never summarise one commit; \
                 got {other:?}"
            ),
        }
    }

    // A repository whose config does not parse is no capability answer either (added 2026-09-25,
    // s8a fix round 7): it was `Open`, answered with the same bare refusal as `""`.
    let config = repo.root().join(".git/config");
    let saved_config = std::fs::read(&config).expect("PREMISE: the fixture has a config");
    let mut broken = saved_config.clone();
    broken.extend_from_slice(b"[core\n  broken = ");
    std::fs::write(&config, &broken).expect("break the config");
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail,
        }) => assert!(
            detail.contains("config"),
            "the store fault names the configuration: {detail:?}"
        ),
        other => panic!("a broken config is Transport {{ Io }}, not a refusal: {other:?}"),
    }
    std::fs::write(&config, &saved_config).expect("restore the config");

    // A store fault is not a capability answer (added 2026-09-25, s8a fix round 4): `HEAD~1`
    // resolves, but its tree is gone from the store, so the read is `Transport { Io }` naming the
    // tree, never the refusal an unresolvable range gets.
    let tree = repo.git(&["show", "-s", "--format=%T", "HEAD~1"]);
    let commit = repo.git(&["rev-parse", "HEAD~1"]);
    let delete_loose = |id: &str| {
        let loose = repo
            .root()
            .join(".git/objects")
            .join(&id[..2])
            .join(&id[2..]);
        let mut perms = std::fs::metadata(&loose)
            .unwrap_or_else(|e| panic!("PREMISE: {id} is a loose object: {e}"))
            .permissions();
        #[expect(
            clippy::permissions_set_readonly_false,
            reason = "git writes loose objects read-only; the store-fault fixture must delete one"
        )]
        perms.set_readonly(false);
        std::fs::set_permissions(&loose, perms).expect("chmod");
        std::fs::remove_file(&loose).expect("rm object");
    };
    // A changed path's blob gone from the store is the same store fault (added 2026-09-25, s8a
    // fix round 13): `Transport { Io }` naming the blob and its path, never a `Text` whose diff
    // reads the missing side as empty, and never the refusal. It goes before the tree, which
    // fails the read before any blob is reached.
    let blob = repo.git(&["rev-parse", "HEAD~1:src/lib.rs"]);
    delete_loose(&blob);
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail,
        }) => assert!(
            detail.contains(&blob) && detail.contains("src/lib.rs"),
            "the store fault names the missing blob and its path: {detail:?}"
        ),
        other => {
            panic!("a blob missing from the store is Transport {{ Io }}, not a refusal: {other:?}")
        }
    }
    delete_loose(&tree);
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail,
        }) => assert!(
            detail.contains(&tree),
            "the store fault names the missing tree: {detail:?}"
        ),
        other => {
            panic!("a tree missing from the store is Transport {{ Io }}, not a refusal: {other:?}")
        }
    }
    // The same for `HEAD~1`'s commit (added 2026-09-25, s8a fix round 6): the end is reached by
    // walking a parent, and the commit it reaches is gone, so it is Transport naming the commit.
    delete_loose(&commit);
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail,
        }) => assert!(
            detail.contains(&commit),
            "the store fault names the missing commit: {detail:?}"
        ),
        other => panic!(
            "a commit missing from the store is Transport {{ Io }}, not a refusal: {other:?}"
        ),
    }
}

/// `classify` answers `Text` for `data/table.json` (its extension is in the text table and a
/// pointer file has no NUL), but its blob is a git-lfs pointer: the range refuses naming the path,
/// and never yields a `Text` whose `diff` is the pointer's `oid`/`size` delta (§8a.2 item 2a).
#[test]
fn change_summary_refuses_a_range_carrying_an_lfs_pointer_path() {
    let repo = lfs_range_repo();
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = repo.adapter(scratch.path());

    assert_eq!(
        tc.classify("data/table.json").expect("classify"),
        Classification::Text,
        "PREMISE: classify cannot see a pointer -- it answers Text for data/table.json"
    );
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Unsupported { capability }) => {
            assert!(
                capability.starts_with("change_summary") && capability.contains("data/table.json"),
                "the refusal carries the capability token and names the pointer path: \
                 {capability:?}"
            );
            assert!(
                !capability.contains("data/plain.json"),
                "the ordinary json beside it is not the reason: {capability:?}"
            );
        }
        other => panic!(
            "a range carrying an LFS pointer path refuses by name; a Text here would carry a \
             pointer delta as the diff. Got {other:?}"
        ),
    }
}

/// A binary blob the working tree does not hold is still `Binary`: an extensionless NUL-bearing
/// blob DELETED in the range, and one ADDED at a head the checkout is not on, each refuse naming
/// the path. `classify` reads the working tree for an extension in none of its tables and answers
/// `Text` for an absent file, so the decision is taken on the range's blobs (`RangeEntry::binary`)
/// as well. *(Added 2026-09-25, s8a fix round 1: both ranges answered `Ok(Text)` with the path's
/// row marked `Text`, the AC-497 failure.)* *(Amended 2026-09-25, s8a fix round 12: a third case
/// puts the first NUL at byte 8100 and reads the same range with the checkout at each end;
/// `classify` read 8192 bytes where the range read reads git's 8000, so the answer moved.)*
#[test]
fn change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold() {
    let scratch = tempfile::tempdir().expect("scratch");

    let deleted = deleted_binary_range_repo();
    let tc = deleted.adapter(scratch.path());
    assert_eq!(
        tc.classify("assets/blob").expect("classify"),
        Classification::Text,
        "PREMISE: the path is gone from the working tree and classify answers Text"
    );
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.starts_with("change_summary") && capability.contains("assets/blob"),
            "the refusal names the deleted binary path: {capability:?}"
        ),
        other => panic!("a range deleting a binary blob refuses by name; got {other:?}"),
    }

    let (unchecked, range) = unchecked_out_binary_range_repo();
    let tc = unchecked.adapter(scratch.path());
    assert_eq!(
        tc.classify("tool").expect("classify"),
        Classification::Text,
        "PREMISE: the checkout is on the base end and classify answers Text"
    );
    match tc.change_summary(&range) {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.starts_with("change_summary") && capability.contains("`tool`"),
            "the refusal names the binary path added at the head end: {capability:?}"
        ),
        other => panic!("a range adding a binary blob refuses by name; got {other:?}"),
    }

    // (Added 2026-09-25, s8a fix round 12.) A blob whose first NUL sits between byte 8000 and
    // byte 8191 is text to git (its window is 8000 bytes), so the answer must not move with the
    // checkout: classify's byte window and the range read's are one window.
    let late = FixtureRepo::new();
    late.write("a.txt", b"one\n");
    late.commit("one");
    let base = late.git(&["rev-parse", "HEAD"]);
    let mut bytes = vec![b'a'; 8100];
    bytes.push(0);
    bytes.extend_from_slice(b"tail\n");
    late.write("data", &bytes);
    late.write("a.txt", b"two\n");
    late.commit("two");
    let head = late.git(&["rev-parse", "HEAD"]);
    let range = format!("{base}..{head}");
    let tc = late.adapter(scratch.path());
    let at_head = format!("{:?}", tc.change_summary(&range));
    late.git(&["checkout", "--quiet", &base]);
    let at_base = format!("{:?}", tc.change_summary(&range));
    assert_eq!(
        at_head, at_base,
        "the same committed range answers the same whichever end is checked out"
    );
    assert!(
        at_head.starts_with("Ok(Text("),
        "a NUL past git's 8000-byte window is text, as `git diff` reads it: {at_head}"
    );

    // (Added 2026-09-25, s8a fix round 14.) The NOT-UTF-8 half: a `.txt` blob with no NUL whose
    // bytes are Latin-1. `classify` answers `Text` by extension, so only the range's blob mark
    // can refuse it; a NUL-only mark answered `Ok(Text)` with a `Binary files … differ` diff.
    let latin = FixtureRepo::new();
    latin.write("notes.txt", b"caf\xe9\n");
    latin.commit("latin-1");
    latin.write("notes.txt", b"caf\xe8\n");
    latin.commit("latin-1 again");
    let tc = latin.adapter(scratch.path());
    assert_eq!(
        tc.classify("notes.txt").expect("classify"),
        Classification::Text,
        "PREMISE: classify answers Text by the `.txt` extension"
    );
    match tc.change_summary("HEAD~1..HEAD") {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.starts_with("change_summary") && capability.contains("`notes.txt`"),
            "the refusal names the non-UTF-8 path: {capability:?}"
        ),
        other => panic!("a range changing a non-UTF-8 blob refuses by name; got {other:?}"),
    }

    // (Added 2026-09-25, s8a fix round 14.) A committed range is answered on its blobs alone: an
    // extensionless path that is text at both ends answers `Ok(Text)` with its row `Text`, when
    // the checkout sits at a THIRD commit whose blob is binary, and when the working tree is
    // dirty with NUL bytes at that path. `classify`'s working-tree byte read decided it before.
    let third = FixtureRepo::new();
    third.write("run", b"#!/bin/sh\necho 1\n");
    third.commit("one");
    let base = third.git(&["rev-parse", "HEAD"]);
    third.write("run", b"#!/bin/sh\necho 2\n");
    third.commit("two");
    let head = third.git(&["rev-parse", "HEAD"]);
    let range = format!("{base}..{head}");
    let tc = third.adapter(scratch.path());
    let clean = format!("{:?}", tc.change_summary(&range));
    assert!(
        clean.starts_with("Ok(Text(") && clean.contains("classification: Text"),
        "PREMISE: the range is text at both ends: {clean}"
    );
    third.write("run", b"x\0y");
    let dirty = format!("{:?}", tc.change_summary(&range));
    assert_eq!(
        dirty, clean,
        "a dirty working tree never moves a committed range's answer"
    );
    third.write("run", b"bin\0ary");
    third.commit("three");
    let later = format!("{:?}", tc.change_summary(&range));
    assert_eq!(
        later, clean,
        "a checkout at a third commit never moves a committed range's answer"
    );
}
