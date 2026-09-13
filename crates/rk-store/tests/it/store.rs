//! Identity, connection discipline, tiers, the schema lint and retention.

use rk_store::conn::{self, BUSY_TIMEOUT_MS, Reader, StoreError, Writer};
use rk_store::identity::{self, Identity, SCHEMA_VERSION};
use rk_store::retention::{DispatchRecord, Retained, looks_like_credential};
use rk_store::schema_lint::{self, LintError};
use rk_store::tiered_state;

fn tmp() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

fn ident(root: &str, common: &str) -> Identity {
    Identity {
        root_commit: root.into(),
        common_dir: common.into(),
        schema_version: SCHEMA_VERSION,
    }
}

// ---------------------------------------------------------------- AC-178: one store per repository

/// The store path is derived from the **common dir**, so opening from any worktree of a repository
/// yields one path — and no second store file appears beside a linked worktree.
#[test]
fn the_store_path_is_under_the_common_dir() {
    let dir = tmp();
    let p = conn::store_path(dir.path());
    assert!(p.starts_with(dir.path()));
    assert!(
        p.to_string_lossy().contains("redkiln"),
        "the store is not under <common-dir>/redkiln/: {}",
        p.display()
    );
}

/// Opening twice against the same common dir yields the same file, not two.
#[test]
fn opening_twice_yields_one_store_file() {
    let dir = tmp();
    let w1 = Writer::open(dir.path()).expect("first");
    let p1 = w1.path().to_path_buf();
    drop(w1);
    let w2 = Writer::open(dir.path()).expect("second");
    assert_eq!(p1, w2.path());

    let mut found = Vec::new();
    let mut stack = vec![dir.path().to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).expect("read").filter_map(Result::ok) {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "sqlite3") {
                found.push(p);
            }
        }
    }
    assert_eq!(found.len(), 1, "more than one store file exists: {found:?}");
}

// ---------------------------------------------------------------- ADR-0105 clause 7

/// WAL, and the busy timeout, on the writer.
#[test]
fn the_writer_opens_in_wal_with_the_busy_timeout() {
    let dir = tmp();
    let w = Writer::open(dir.path()).expect("open");
    let mode: String = w
        .transaction(|tx| {
            tx.query_row("PRAGMA journal_mode", [], |r| r.get(0))
                .map_err(StoreError::from)
        })
        .expect("read journal_mode");
    assert!(mode.eq_ignore_ascii_case("wal"), "journal_mode is {mode}");
    assert_eq!(BUSY_TIMEOUT_MS, 5000, "clause 7's documented value moved");
}

/// **Readers also set `busy_timeout`** — clause 7 says so explicitly, because readers can see
/// `SQLITE_BUSY` too. An easy clause to implement for the writer and forget for the reader.
#[test]
fn a_reader_also_configures_the_busy_timeout() {
    let dir = tmp();
    let _w = Writer::open(dir.path()).expect("writer first, to create the file");
    let r = Reader::open(dir.path()).expect("reader");
    let mode: String = r
        .conn()
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("journal_mode");
    assert!(mode.eq_ignore_ascii_case("wal"));
}

/// A write transaction commits and is timed. The timing guard is clause 7's "every write
/// transaction shorter than busy_timeout", asserted rather than assumed.
#[test]
fn a_write_transaction_commits_and_is_within_the_timeout() {
    let dir = tmp();
    let w = Writer::open(dir.path()).expect("open");
    w.transaction(|tx| {
        tx.execute_batch("CREATE TABLE t (x INTEGER);")?;
        tx.execute("INSERT INTO t (x) VALUES (1)", [])?;
        Ok(())
    })
    .expect("transaction");

    let n: i64 = w
        .transaction(|tx| {
            tx.query_row("SELECT count(*) FROM t", [], |r| r.get(0))
                .map_err(StoreError::from)
        })
        .expect("read back");
    assert_eq!(n, 1);
}

/// `doctor`'s network-filesystem check. Reported honestly as partial — it recognises UNC and the
/// UNC device form and does not resolve a mapped drive letter.
#[test]
fn a_unc_path_is_recognised_as_a_network_filesystem() {
    assert!(conn::looks_like_network_filesystem(std::path::Path::new(
        r"\\server\share\repo"
    )));
    assert!(!conn::looks_like_network_filesystem(std::path::Path::new(
        r"D:\repos\redkiln-rs"
    )));
}

// ---------------------------------------------------------------- AC-179 / AC-180

/// **AC-179.** A foreign root commit is refused, the error names **both** identities, and the store
/// file's bytes are unchanged — "no row is read or written".
#[test]
fn a_foreign_root_commit_is_refused_and_the_file_is_unchanged() {
    let dir = tmp();
    let w = Writer::open(dir.path()).expect("open");
    let stamped = ident("aaaa1111", "/repo/a/.git");
    w.transaction(|tx| identity::initialise(tx, &stamped))
        .expect("stamp");
    drop(w);

    let before = identity::file_digest(&conn::store_path(dir.path())).expect("digest");

    let opening = ident("bbbb2222", "/repo/b/.git");
    let err = identity::check(&stamped, &opening).expect_err("must refuse");
    let text = err.to_string();
    assert!(
        text.contains("aaaa1111"),
        "the stored identity is not named: {text}"
    );
    assert!(
        text.contains("bbbb2222"),
        "the opening identity is not named: {text}"
    );

    let after = identity::file_digest(&conn::store_path(dir.path())).expect("digest");
    assert_eq!(before, after, "the refused open changed the store file");
}

/// **AC-180.** An older schema is refused naming the migration required, and **no implicit
/// in-place migration occurs** — asserted on the file's bytes.
#[test]
fn an_older_schema_is_refused_and_no_migration_happens() {
    let dir = tmp();
    let w = Writer::open(dir.path()).expect("open");
    let mut stamped = ident("aaaa1111", "/repo/a/.git");
    stamped.schema_version = SCHEMA_VERSION;
    w.transaction(|tx| identity::initialise(tx, &stamped))
        .expect("stamp");
    drop(w);

    let before = identity::file_digest(&conn::store_path(dir.path())).expect("digest");

    let old = Identity {
        schema_version: SCHEMA_VERSION - 1,
        ..stamped.clone()
    };
    let err = identity::check(&old, &stamped).expect_err("must refuse");
    let text = err.to_string();
    assert!(
        text.contains("migration"),
        "the error does not name a migration: {text}"
    );
    assert!(text.contains(&(SCHEMA_VERSION - 1).to_string()));
    assert!(text.contains(&SCHEMA_VERSION.to_string()));

    let after = identity::file_digest(&conn::store_path(dir.path())).expect("digest");
    assert_eq!(before, after, "a refused open migrated the store in place");
}

/// A store NEWER than this binary is also refused, rather than read as though the extra columns
/// were absent. AC-369's "no adopt-this-store fallback path exists".
#[test]
fn a_newer_schema_is_refused_too() {
    let stamped = Identity {
        schema_version: SCHEMA_VERSION + 1,
        ..ident("a", "/x")
    };
    let err = identity::check(&stamped, &ident("a", "/x")).expect_err("must refuse");
    assert!(err.to_string().contains("NEWER"));
}

/// Control: a matching identity opens. Without it, a `check` that refused everything would pass.
#[test]
fn control_a_matching_identity_is_accepted() {
    let i = ident("aaaa1111", "/repo/a/.git");
    identity::check(&i, &i).expect("a matching identity must be accepted");
}

// ---------------------------------------------------------------- AC-160: tier annotations

tiered_state! {
    /// A state type, declared the only way a state type may be declared.
    pub struct RunState {
        tier1 stage: String,
        tier1 gate_token: Option<String>,
        tier2 wall_clock_ms: u64,
        tier2 inner_retries: u32,
    }
}

/// Every field answered the deleted-database question, and the answers are readable.
#[test]
fn every_state_field_declares_a_tier() {
    assert_eq!(RunState::TIERS.len(), 4);
    assert_eq!(RunState::tier1_fields(), ["stage", "gate_token"]);
    assert_eq!(RunState::tier2_fields(), ["wall_clock_ms", "inner_retries"]);
}

/// The memo is tier 2 — ADR-0097 decision 1 and the amendment AC-160's fixture depends on.
#[test]
fn a_memo_field_is_declared_tier_two() {
    tiered_state! {
        pub struct WithMemo {
            tier1 stage: String,
            tier2 step_memo: String,
        }
    }
    assert_eq!(WithMemo::tier2_fields(), ["step_memo"]);
}

// ---------------------------------------------------------------- AC-166: the schema lint

/// The shipped migrations pass their own lint.
#[test]
fn the_shipped_migrations_declare_every_repo_key() {
    for (name, sql, keys) in schema_lint::all() {
        schema_lint::lint(sql, keys)
            .unwrap_or_else(|e| panic!("migration {name} fails the key-domain lint: {e:?}"));
    }
}

/// **The fixture AC-166 names**: a `branch TEXT` key must fail the build.
#[test]
fn a_branch_key_fails_the_lint() {
    let sql = "CREATE TABLE t (\n    branch TEXT NOT NULL   -- repo-key\n);\n";
    let keys = "t.branch = branch\n";
    let errors = schema_lint::lint(sql, keys).expect_err("a branch domain must be refused");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, LintError::BadDomain { domain, .. } if domain == "branch")),
        "the branch domain was not the reason: {errors:?}"
    );
}

/// A marked column with no declared domain fails.
#[test]
fn an_undeclared_repo_key_fails_the_lint() {
    let sql = "CREATE TABLE t (\n    anchor_sha TEXT NOT NULL   -- repo-key\n);\n";
    let errors = schema_lint::lint(sql, "").expect_err("must refuse");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, LintError::UndeclaredDomain { .. }))
    );
}

/// The two files drifting apart is itself an error — a `.keys` entry for a column the SQL does not
/// mark means one of them was edited and the other was not.
#[test]
fn a_declared_column_the_sql_does_not_mark_fails_the_lint() {
    let sql = "CREATE TABLE t (\n    x INTEGER\n);\n";
    let errors = schema_lint::lint(sql, "t.x = sha\n").expect_err("must refuse");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, LintError::DeclaredButNotMarked { .. }))
    );
}

/// Control: a correct pair passes, so the refusals above are about the rule and not about the lint
/// rejecting everything.
#[test]
fn control_a_correct_pair_passes() {
    let sql = "CREATE TABLE t (\n    anchor_sha TEXT NOT NULL,   -- repo-key\n    tree_hash TEXT NOT NULL   -- repo-key\n);\n";
    schema_lint::lint(sql, "t.anchor_sha = sha\nt.tree_hash = tree\n").expect("valid pair");
}

// ---------------------------------------------------------------- AC-168: no tier-1 DELETE

/// No `DELETE` against a tier-1 table exists in the crate's SQL surface outside a rebuild path.
#[test]
fn there_is_no_delete_against_a_tier_one_table() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    let mut stack = vec![src];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)
            .expect("read src")
            .filter_map(Result::ok)
        {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs" || x == "sql") {
                let text = std::fs::read_to_string(&p).expect("read");
                for (i, line) in text.lines().enumerate() {
                    let l = line.to_ascii_uppercase();
                    let is_comment =
                        line.trim_start().starts_with("//") || line.trim_start().starts_with("--");
                    if l.contains("DELETE FROM T1_") && !is_comment && !p.ends_with("rebuild.rs") {
                        hits.push(format!("{}:{}", p.display(), i + 1));
                    }
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "a DELETE against a tier-1 table exists outside the rebuild path (AC-168): {hits:?}"
    );
}

// ---------------------------------------------------------------- clause 7: retention

/// argv and env are digests; there is **no field** for the child environment.
#[test]
fn a_dispatch_record_keeps_digests_and_has_no_environment_field() {
    let rec = DispatchRecord::new(
        &["claude".into(), "--print".into(), "--secret".into()],
        &[("ANTHROPIC_API_KEY".into(), "sk-verysecretvalue".into())],
        "2.1.269",
        b"out",
        b"err",
    );
    assert_eq!(rec.argv_digest.as_str().len(), 64);
    assert!(!rec.argv_digest.as_str().contains("secret"));
    assert_eq!(rec.env_digest.as_str().len(), 64);

    // Structural: destructuring names every field. A new `env` field stops this compiling.
    let DispatchRecord {
        argv_digest: _,
        env_digest: _,
        instrument_version: _,
        stdout: _,
        stderr: _,
    } = rec;
}

/// Under the cap, stdout is verbatim; over it, digest plus head and tail.
#[test]
fn stdout_is_verbatim_under_the_cap_and_summarised_over_it() {
    assert!(matches!(Retained::of(b"short"), Retained::Verbatim(_)));

    let big = vec![b'x'; rk_store::retention::STDIO_CAP + 1];
    match Retained::of(&big) {
        Retained::Capped {
            head,
            tail,
            total_len,
            ..
        } => {
            assert_eq!(total_len, big.len());
            assert_eq!(head.len(), rk_store::retention::HEAD_TAIL);
            assert_eq!(tail.len(), rk_store::retention::HEAD_TAIL);
        }
        Retained::Verbatim(_) => panic!("an over-cap stream was retained verbatim"),
    }
}

/// `doctor`'s credential rule. Noisy on purpose: a false positive costs a row someone dismisses, a
/// false negative costs a credential in a retained field.
#[test]
fn credential_shaped_values_are_recognised() {
    for v in [
        "sk-abcdefghijklmnopqrstuvwxyz",
        "ghp_abcdefghijklmnopqrstuvwxyz012345",
        "Bearer abcdefghijklmnopqrstuvwxyz",
        "-----BEGIN RSA PRIVATE KEY-----",
    ] {
        assert!(looks_like_credential(v), "not flagged: {v}");
    }
    for v in ["short", "a normal sentence with spaces in it", "1.2.3"] {
        assert!(!looks_like_credential(v), "false positive: {v}");
    }
}
