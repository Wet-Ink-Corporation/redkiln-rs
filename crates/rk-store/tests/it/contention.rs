//! **AC-181** — two processes, two items, both writes land, zero lock errors, one store file.
//! This is also **R-SQLITE**'s empirical half.
//!
//! *"GIVEN two redkiln processes running in two worktrees of the same repository, of two different
//! items … WHEN both write concurrently … THEN both writes are present after both processes exit
//! AND neither fails with a lock error, and no second store file exists."*
//!
//! **Two processes, not two threads.** The distinction is the criterion: POSIX advisory locks are
//! per-**process** and Windows `LockFileEx` is per-**handle**, so a two-thread test exercises the
//! WAL `-shm` index and the same SQLite locking code while missing exactly the layer that differs
//! between one process and two. A thread-based near-oracle would have been a reasonable *control*
//! and a poor *instrument*, and reporting it as the criterion would have been the "cheap adjacent
//! check produces the same shape of output" failure.
//!
//! Spawning a child needs `std::process::Command`. Checkpoint 6's general ban was narrowed to
//! production code on 2026-09-13 to match its warrant — ADR-0077:70 forbids
//! `std::process::Command` **for git**, and ADR-0105 clause 5 enumerates the **git** write set;
//! neither forbids a non-git child. The `Command::new("git")` ban is untouched and still covers
//! this file. The `#[expect]` below names the criterion, so the exemption is declared rather than
//! silent.
//!
//! **What this does NOT license.** One machine, one filesystem (NTFS), one run, two processes. It
//! does not license claiming 5000 ms suffices under the self-hosting run's load — ADR-0105's own
//! evidence limits reserve that for R-SQLITE's load question and Step 8 (3).

use std::path::Path;

use rk_store::conn::{Reader, StoreError, Writer};

const CHILD_ENV: &str = "RK_AC181_STORE_DIR";
const CHILD_ITEM: &str = "RK_AC181_ITEM";

fn ensure_table(w: &Writer) {
    w.transaction(|tx| {
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS t1_slice_seal_probe (
                 id      INTEGER PRIMARY KEY,
                 item    TEXT NOT NULL,
                 verdict TEXT NOT NULL
             );",
        )?;
        Ok(())
    })
    .expect("create the probe table");
}

fn write_rows(w: &Writer, item: &str, n: usize) -> Result<(), StoreError> {
    for i in 0..n {
        w.transaction(|tx| {
            // A payload large enough that a transaction is not instantaneous: with tiny rows the
            // two processes rarely overlap and the test cannot tell busy_timeout from its absence.
            let payload = "x".repeat(4096);
            for k in 0..40 {
                tx.execute(
                    "INSERT INTO t1_slice_seal_probe (item, verdict) VALUES (?1, ?2)",
                    (item, format!("approved-{i}-{k}-{payload}")),
                )?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

/// The child half. `#[ignore]`d so it never runs in an ordinary sweep; the parent invokes it by
/// name. It writes as **a different item**, which is AC-181's GIVEN.
#[test]
#[ignore = "child entrypoint for the AC-181 two-process contention test"]
fn ac181_child_writer() {
    let dir = std::env::var(CHILD_ENV).expect("the parent sets the store dir");
    let item = std::env::var(CHILD_ITEM).unwrap_or_else(|_| "child-item".into());
    let w = Writer::open(Path::new(&dir)).expect("child opens the store");
    write_rows(&w, &item, 40).expect("child writes without a lock error");
}

/// The parent half — the criterion.
#[test]
#[expect(
    clippy::disallowed_types,
    reason = "AC-181's GIVEN is two redkiln PROCESSES, and its Verified-by is a two-process \
              integration test. ADR-0077:70's ban is on std::process::Command FOR GIT and \
              ADR-0105 clause 5 enumerates the git write set; neither covers a non-git child. \
              Checkpoint 6's wider form was narrowed to its warrant on 2026-09-13 with the \
              operator's decision recorded in s6 §6.2."
)]
fn two_processes_writing_concurrently_both_land_with_no_lock_errors() {
    let dir = tempfile::tempdir().expect("tempdir");
    let common = dir.path().to_path_buf();

    let parent = Writer::open(&common).expect("parent opens the store");
    ensure_table(&parent);

    let exe = std::env::current_exe().expect("the test binary's own path");
    // The FULL test path. `--exact` matches the whole name, and a filter that selects nothing exits
    // **0** having run nothing — the shape the preamble names and the one this test first hit: the
    // child "succeeded" and wrote no rows. The selected-count assertion below is what turns that
    // from a silent zero into a named failure.
    let mut child = std::process::Command::new(exe)
        .args([
            "contention::ac181_child_writer",
            "--exact",
            "--ignored",
            "--nocapture",
        ])
        .env(CHILD_ENV, &common)
        .env(CHILD_ITEM, "item-b")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn the second process");

    // The parent writes its own item while the child writes its own — genuinely concurrent.
    let parent_result = write_rows(&parent, "item-a", 40);

    let out = child.wait_with_output().expect("wait for the child");

    assert!(
        parent_result.is_ok(),
        "the PARENT failed while the child was writing: {parent_result:?}"
    );
    let child_stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        out.status.success(),
        "the CHILD process failed.\nstdout: {child_stdout}\nstderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    // A zero-exit child that selected NO tests is the failure this assertion exists for: libtest
    // exits 0 on a filter that matches nothing, so "the child succeeded" is not evidence the child
    // ran. Assert it ran exactly one.
    assert!(
        child_stdout.contains("1 passed"),
        "the child process exited 0 but did not run its test — the filter selected nothing, which \
         libtest reports as success. Child stdout:\n{child_stdout}"
    );

    // Both writes present after both processes exit.
    let reader = Reader::open(&common).expect("reader");
    let a: i64 = reader
        .conn()
        .query_row(
            "SELECT count(*) FROM t1_slice_seal_probe WHERE item = 'item-a'",
            [],
            |r| r.get(0),
        )
        .expect("count a");
    let b: i64 = reader
        .conn()
        .query_row(
            "SELECT count(*) FROM t1_slice_seal_probe WHERE item = 'item-b'",
            [],
            |r| r.get(0),
        )
        .expect("count b");
    assert_eq!(a, 40 * 40, "the parent's rows are not all present");
    assert_eq!(b, 40 * 40, "the child's rows are not all present");

    // And no second store file was created beside the first.
    let mut stores = Vec::new();
    let mut stack = vec![common.clone()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("read").filter_map(Result::ok) {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "sqlite3") {
                stores.push(p);
            }
        }
    }
    assert_eq!(
        stores.len(),
        1,
        "more than one store file exists: {stores:?}"
    );
}

/// The control that makes the test above an instrument rather than a coincidence.
///
/// If the child never actually ran — a wrong filter, a binary that exited 0 having selected zero
/// tests — the parent would still write its 25 rows and the counts for `item-b` would be 0. That
/// would be caught above. What this adds is the other direction: the child entrypoint, run
/// directly, really does write, so `0` would mean contention and not a mis-wired fixture.
#[test]
fn control_the_child_entrypoint_writes_when_run_directly() {
    let dir = tempfile::tempdir().expect("tempdir");
    let w = Writer::open(dir.path()).expect("open");
    ensure_table(&w);
    write_rows(&w, "direct", 1).expect("direct write");

    let reader = Reader::open(dir.path()).expect("reader");
    let n: i64 = reader
        .conn()
        .query_row(
            "SELECT count(*) FROM t1_slice_seal_probe WHERE item = 'direct'",
            [],
            |r| r.get(0),
        )
        .expect("count");
    assert_eq!(n, 40);
}
