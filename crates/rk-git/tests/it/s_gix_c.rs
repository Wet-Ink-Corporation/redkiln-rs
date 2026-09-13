//! **S-GIX/c** — does `PrepareFetch::with_revision` + `with_shallow` at the pin produce a clone
//! whose HEAD tree equals `git clone --depth 1 --revision <sha>`'s?
//!
//! Registered by ADR-0098 decision 3. The review clone is the one gix **write** the hybrid retains
//! on purpose (decision 2), so a go keeps it there and a no-go moves it to the wrapper as
//! `git clone --depth 1 --revision` — the fallback ADR-0098's own Considered-options pre-authorised.
//! The verdict also moves the `git` floor ADR-0091 records: on a go the wrapper never invokes
//! `clone --revision` and the floor stays at 2.20; on a no-go it rises to 2.49, where that option
//! was introduced.
//!
//! Three things about the fixture are load-bearing, and each has a control:
//!
//! * The target SHA is `HEAD~1`, deliberately **not** a branch tip — a reviewed SHA need not be one.
//! * The transport is a `file://` **URL**, not a bare path. Given a bare local path git takes the
//!   hardlink/copy route and silently ignores `--depth`, and the arm would compare two full clones.
//! * `uploadpack.allowAnySHA1InWant` is set on the SOURCE repository. It is a server-side setting,
//!   so `-c` on the client does not reach it.

use std::path::{Path, PathBuf};

use crate::support::{Fixture, assert_agrees_with_record};

/// Build a source repo with three commits and return `(repo_path, file_url, target_sha)`.
fn source_repo(fx: &Fixture) -> (PathBuf, String, String) {
    let repo = fx.sub("src");
    std::fs::create_dir_all(&repo).expect("mkdir src");
    fx.git(&repo, &["init", "--quiet"]);
    for (n, body) in [("one", "1\n"), ("two", "2\n"), ("three", "3\n")] {
        std::fs::write(repo.join("f.txt"), body).expect("write f.txt");
        fx.git(&repo, &["add", "f.txt"]);
        fx.git(&repo, &["commit", "--quiet", "-m", n]);
    }
    // Server-side: permit fetching an arbitrary SHA that no ref points at.
    fx.git(&repo, &["config", "uploadpack.allowAnySHA1InWant", "true"]);

    let sha = fx.git(&repo, &["rev-parse", "HEAD~1"]).stdout_trimmed();
    let url = format!("file:///{}", repo.display().to_string().replace('\\', "/"));
    (repo, url, sha)
}

fn head_tree(fx: &Fixture, clone: &Path) -> String {
    fx.git(clone, &["rev-parse", "HEAD^{tree}"])
        .stdout_trimmed()
}

/// Is this clone genuinely shallow? A non-empty `shallow` file under the git dir is the evidence.
fn is_shallow(fx: &Fixture, clone: &Path) -> bool {
    let git_dir = fx
        .git(clone, &["rev-parse", "--absolute-git-dir"])
        .stdout_trimmed();
    let shallow = Path::new(&git_dir).join("shallow");
    std::fs::metadata(&shallow).is_ok_and(|m| m.len() > 0)
}

/// Control: the shallow assertion must be able to say "no". A full clone has no `shallow` file, so
/// if this ever reports true the depth check below is asserting nothing.
#[test]
fn control_a_full_clone_is_not_shallow() {
    let fx = Fixture::new();
    let (_repo, url, _sha) = source_repo(&fx);
    let full = fx.sub("full");
    fx.git(
        fx.path(),
        &["clone", "--quiet", &url, full.to_str().expect("utf-8")],
    );
    assert!(
        !is_shallow(&fx, &full),
        "a full clone reported as shallow — the depth evidence is not evidence"
    );
}

/// Control: two different trees must compare unequal, or the comparator below cannot fail.
#[test]
fn control_different_revisions_have_different_trees() {
    let fx = Fixture::new();
    let (repo, _url, sha) = source_repo(&fx);
    let tip = fx
        .git(&repo, &["rev-parse", "HEAD^{tree}"])
        .stdout_trimmed();
    let older = fx
        .git(&repo, &["rev-parse", &format!("{sha}^{{tree}}")])
        .stdout_trimmed();
    assert_ne!(tip, older, "two different revisions produced the same tree");
}

#[test]
#[ignore = "S-GIX/c: a spike arm against the gix pin; run explicitly with --ignored"]
fn s_gix_c_shallow_clone_by_sha_matches_git() {
    let fx = Fixture::new();
    let (_repo, url, sha) = source_repo(&fx);

    // The oracle: the wrapper invocation the no-go branch would use.
    let by_git = fx.sub("by_git");
    fx.git(
        fx.path(),
        &[
            "clone",
            "--quiet",
            "--depth",
            "1",
            "--revision",
            &sha,
            &url,
            by_git.to_str().expect("utf-8"),
        ],
    );
    assert!(
        is_shallow(&fx, &by_git),
        "`git clone --depth 1` produced a NON-shallow clone; --depth was ignored and this arm \
         would be comparing two full clones"
    );
    let oracle_tree = head_tree(&fx, &by_git);

    // The candidate: gix's retained write. Bare, per ADR-0098 decision 2 — the review clone is bare
    // unless the review activity declares it needs a working tree.
    let by_gix = fx.sub("by_gix");
    let depth = std::num::NonZeroU32::new(1).expect("1 is non-zero");
    let measured = match clone_with_gix(&url, &by_gix, &sha, depth) {
        Err(e) => {
            println!("S-GIX/c  gix error          : {e}");
            "no-go"
        }
        Ok(gix_tree) => {
            println!("S-GIX/c  oracle tree        : {oracle_tree}");
            println!("S-GIX/c  gix tree           : {gix_tree}");
            if gix_tree == oracle_tree {
                "go"
            } else {
                "no-go"
            }
        }
    };

    assert_agrees_with_record('c', measured);
}

/// Run the gix side and return its HEAD tree id, or an error string.
fn clone_with_gix(
    url: &str,
    dest: &Path,
    sha: &str,
    depth: std::num::NonZeroU32,
) -> Result<String, String> {
    let mut prep = gix::prepare_clone_bare(url, dest)
        .map_err(|e| format!("prepare_clone_bare: {e}"))?
        .with_revision(Some(sha))
        .map_err(|e| format!("with_revision: {e}"))?
        .with_shallow(gix::remote::fetch::Shallow::DepthAtRemote(depth));

    let (repo, _outcome) = prep
        .fetch_only(
            gix::progress::Discard,
            &std::sync::atomic::AtomicBool::default(),
        )
        .map_err(|e| format!("fetch_only: {e}"))?;

    let id = gix::ObjectId::from_hex(sha.as_bytes()).map_err(|e| format!("bad sha: {e}"))?;
    let commit = repo
        .find_object(id)
        .map_err(|e| format!("find_object: {e}"))?
        .try_into_commit()
        .map_err(|e| format!("not a commit: {e}"))?;
    let tree = commit.tree_id().map_err(|e| format!("tree_id: {e}"))?;
    Ok(tree.to_string())
}
