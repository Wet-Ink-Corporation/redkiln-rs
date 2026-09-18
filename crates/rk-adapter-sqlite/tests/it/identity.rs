//! **AC-369 (sqlite half)** — *"GIVEN a store whose identity rows … do not match the repository
//! it is opened against / WHEN the store is opened / THEN the open is refused with a typed error
//! and nothing is written / AND no "adopt this store" fallback path exists."* *Verified by:* *"a
//! unit test opening a store against a second fixture repo"*; the `doctor` row is ARMED to 7d.
//!
//! Step 6's `rk_identity` table is the surface and `rk_store::identity::{initialise, read, check}`
//! are the three calls (s7c §7c.13); what 7c adds is the two git answers the row needs — the root
//! commit from `git log --max-parents=0` and the common dir from `git rev-parse --git-common-dir`,
//! both through `GitExec` — and *nothing written* on a refusal, which the copied store with a
//! table dropped makes observable: an open that committed the schema before checking the stamp
//! would re-create the table (red, measured 2026-09-18); one transaction that rolls back does not.

use rk_ports::event_store::{EventStore, Query};
use rk_store::conn::Writer;
use rk_store::identity::SCHEMA_VERSION;

use rk_adapter_sqlite::{SqliteStore, StoreOpenError};

use crate::support::{Fixture, event, open, store_bytes, wt};

/// RED (observed 2026-09-18) against the skeleton that neither stamped nor checked an identity:
/// repository B opened A's copied store and appended to it. The root-commit case was watched red
/// separately, via a temporary mutant reading `rev-parse HEAD` instead of `log --max-parents=0`:
/// A's own store refused A after A's second commit. Run and reverted; both outputs are quoted in
/// the slice.
#[test]
fn ac369_identity_mismatch_refuses_open() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo_a = fx.repo_with_one_commit("a");
    let repo_b = fx.repo_with_one_commit("b");
    // Two repositories with DIFFERENT roots. The fixture pins author, committer and dates for
    // reproducibility, so two repositories with identical first commits would share one root SHA
    // and this test would measure nothing — the builder writes the repository's name into its
    // first commit, and the premise is asserted rather than assumed.
    let root_b = fx
        .git(&repo_b, &["rev-list", "--max-parents=0", "HEAD"])
        .stdout_trimmed();

    // ---- A stamps its own identity on first open, and it is the ROOT commit, not HEAD --------
    let root_a = fx
        .git(&repo_a, &["rev-list", "--max-parents=0", "HEAD"])
        .stdout_trimmed();
    assert_ne!(
        root_a, root_b,
        "PREMISE: the two fixture repositories have different roots"
    );
    let (store_path_a, identity_a) = {
        let mut store = open(&exec, &repo_a);
        store.append(event(1)).expect("append in A");
        (store.path().to_path_buf(), store.identity().clone())
    };
    assert_eq!(
        identity_a.root_commit, root_a,
        "the stamp is the root commit"
    );
    assert_eq!(identity_a.schema_version, SCHEMA_VERSION);
    assert!(
        identity_a
            .common_dir
            .replace('\\', "/")
            .ends_with("/a/.git"),
        "the stamp names A's common dir: {}",
        identity_a.common_dir
    );

    // A second commit moves HEAD and not the root: A's store still opens for A.
    fx.commit(&repo_a, "b.txt", b"two\n", "two");
    let head_a = fx.git(&repo_a, &["rev-parse", "HEAD"]).stdout_trimmed();
    assert_ne!(head_a, root_a, "PREMISE: HEAD moved");
    {
        let store = open(&exec, &repo_a);
        assert_eq!(
            store.identity().root_commit,
            root_a,
            "an identity keyed on HEAD would have refused A its own store after one more commit"
        );
        assert_eq!(
            store
                .query(Query::ByItem("x".to_owned()))
                .expect("query")
                .len(),
            1,
            "A's row survived the reopen"
        );
    }

    // ---- B opens A's store: refused, typed, nothing written, no adopt path -------------------
    let store_path_b = rk_store::conn::store_path(&repo_b.join(".git"));
    std::fs::create_dir_all(store_path_b.parent().expect("redkiln/")).expect("mkdir");
    std::fs::copy(&store_path_a, &store_path_b).expect("copy A's store into B's common dir");
    // A copy of the main file only: A's writer was dropped, which checkpointed and removed the
    // WAL, so the main file IS the store. Assert that premise rather than assume it.
    assert!(
        !store_path_a.with_extension("sqlite3-wal").exists(),
        "PREMISE: no WAL left behind A's closed store"
    );
    // Make the ORDER observable: drop a table the schema recreates from B's copy, so an open that
    // applied `0001_initial.sql` before checking the stamp would have something to write. A refused
    // open must leave this store exactly as it is, `t2_usage` still absent.
    {
        let writer = Writer::open(&repo_b.join(".git")).expect("a writer on B's copy");
        writer
            .transaction(|tx| {
                tx.execute_batch("DROP TABLE t2_usage;")?;
                Ok(())
            })
            .expect("the table is dropped");
    }
    let before = store_bytes(&store_path_b);

    let refusal =
        SqliteStore::open(&exec, &wt(&repo_b)).expect_err("B must be refused A's store (AC-369)");
    match &refusal {
        StoreOpenError::Refused(detail) => {
            assert!(
                detail.contains(&root_a),
                "the refusal names the stored root commit: {detail}"
            );
            assert!(
                detail.contains(&root_b),
                "the refusal names the opening root commit: {detail}"
            );
        }
        other => panic!("a mismatch is StoreOpenError::Refused, got {other}"),
    }

    let after = store_bytes(&store_path_b);
    assert!(
        before == after,
        "the refused open wrote to the store file or its WAL (main {} -> {} bytes, WAL {} -> {}          bytes)",
        before.0.len(),
        after.0.len(),
        before.1.len(),
        after.1.len()
    );

    // No adopt path: a SECOND open from B is refused the same way (a fallback that re-stamped on
    // the first refusal would let the second open through), and the stamp still names A.
    assert!(
        matches!(
            SqliteStore::open(&exec, &wt(&repo_b)),
            Err(StoreOpenError::Refused(_))
        ),
        "the second open from B must be refused too — there is no adopt-this-store path"
    );
    let stamped = {
        let reader = rk_store::conn::Reader::open(&repo_b.join(".git")).expect("reader");
        rk_store::identity::read(reader.conn())
            .expect("read")
            .expect("stamped")
    };
    assert_eq!(
        stamped.root_commit, root_a,
        "the stamp was not rewritten to B's"
    );
    assert!(
        store_bytes(&store_path_b) == before,
        "still nothing written after the second refusal"
    );
}

/// A repository with **no commit** has no root commit and so no identity to stamp: the open is
/// refused typed, on the `git log` read, and **no store file is created** — a `.git/redkiln/`
/// minted before the identity is known would be a store nobody's identity owns. s7c §7c.13 says
/// `Log` "prints nothing" here; measured, git 2.55 exits non-zero on an unborn branch (`fatal:
/// your current branch 'main' does not have any commits yet`), so the observed variant is
/// `StoreOpenError::Git`, and `NoRootCommit` (an exit-0 empty answer) has no observed producer.
/// The rendering is asserted readable: the retained stderr as text, not `Debug`'s byte vector.
///
/// RED (observed 2026-09-18) via a temporary mutant of `SqliteStore::open` that called
/// `Writer::open` before `identity::opening`: the refusal was still typed, and the store file
/// existed after it. Run and reverted; quoted in the slice.
#[test]
fn open_refuses_a_repository_with_no_commit() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.sub("unborn");
    std::fs::create_dir_all(&repo).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    let store_path = rk_store::conn::store_path(&repo.join(".git"));

    let refusal = SqliteStore::open(&exec, &wt(&repo))
        .expect_err("a repository with no commit has no identity to stamp");
    match &refusal {
        StoreOpenError::Git { op, .. } => assert_eq!(*op, "log --max-parents=0"),
        StoreOpenError::NoRootCommit { .. } => {
            panic!(
                "git printed nothing and exited 0 — a producer for NoRootCommit exists: {refusal}"
            )
        }
        other => panic!("the refusal is on the git log read, got {other}"),
    }
    let text = refusal.to_string();
    assert!(
        text.contains("log --max-parents=0") && !text.contains("stderr: ["),
        "the refusal renders git's stderr as text, not a byte vector: {text}"
    );
    assert!(
        !store_path.exists() && !store_path.parent().expect("redkiln/").exists(),
        "no store file or directory was created for a repository with no identity"
    );
}
