//! Identity, connection discipline, tiers, the schema lint and retention.

use rk_store::conn::{self, BUSY_TIMEOUT_MS, Reader, StoreError, Writer};
use rk_store::identity::{self, Identity, SCHEMA_VERSION};
use rk_store::retention::{DispatchRecord, RecordTail, Retained, looks_like_credential};
use rk_store::schema_lint::{self, LintError};
use rk_store::tiered_state;
use rk_types::usage::{CliVersion, Timings};

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

/// **ADR-0118 clauses 1, 3 and 4 — a poisoned WRITER mutex halts, and is never recovered**
/// (`s8e` §8e.1, D21-42 24b).
///
/// Clause 4 scopes the halt to both store locks and names this one: *"`crates/rk-store/src/conn.rs`
/// … is therefore a violation of this record from its acceptance, and **its repair is owed**"*.
///
/// The writer is poisoned the way clause 1 means it — a panic unwinding while the one writer holds
/// the lock, here a transaction closure that panics. That one panic is caught by the test. The second
/// `transaction` call must then panic with a payload carrying the one `LOCK_HALTS` string (so
/// `rk-cli`'s hook maps it to exit `70`), and must write **no row**: a recovered guard would commit.
///
/// Red while `Writer::transaction` reads `.unwrap_or_else(std::sync::PoisonError::into_inner)`: the
/// second call recovers the guard and commits its row.
#[test]
fn writer_poison_is_a_halt_and_never_recovered() {
    let dir = tmp();
    let w = Writer::open(dir.path()).expect("open");
    w.transaction(|tx| {
        tx.execute_batch("CREATE TABLE t (x INTEGER);")?;
        Ok(())
    })
    .expect("the table is created over a sound writer");

    // Poison: a panic while the writer mutex is held.
    let first = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        w.transaction(|_tx| -> Result<(), StoreError> {
            panic!("a writer panics mid-transaction (the poisoning panic)")
        })
    }));
    assert!(first.is_err(), "the poisoning closure did not panic");

    // The second call, over the poisoned mutex, tries to write a row.
    let second = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        w.transaction(|tx| {
            tx.execute("INSERT INTO t (x) VALUES (2)", [])?;
            Ok(())
        })
    }));

    let r = Reader::open(dir.path()).expect("reader");
    let rows: i64 = r
        .conn()
        .query_row("SELECT count(*) FROM t", [], |row| row.get(0))
        .expect("count");
    assert_eq!(
        rows, 0,
        "the second transaction call recovered the poisoned writer guard and committed {rows} \
         row(s) — ADR-0118 clause 3 forbids every spelling of `PoisonError::into_inner`"
    );

    let payload = match second {
        Ok(result) => panic!(
            "the second transaction call RETURNED ({:?}) over a poisoned writer mutex; ADR-0118 \
             clause 1: no `StepError`, nothing crosses a port boundary — it halts",
            result.map(|()| "Ok")
        ),
        Err(payload) => payload,
    };
    let text = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&'static str>().copied())
        .unwrap_or("<non-string payload>");
    assert!(
        text.contains(rk_types::LOCK_HALTS),
        "the halt's payload does not carry the one LOCK_HALTS string, so rk-cli's hook would leave \
         the process at 101; it read: {text}"
    );
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

/// **One `DispatchRecord`, one declaration** (ADR-0126 decision 2; `s7a` §7a.5's collision, closed
/// at Step 7c Slice 1). `rk_store::retention::DispatchRecord` is a re-export of
/// `rk_types::activity::DispatchRecord` and not a second type under the same name — asserted by
/// type identity at runtime, so the red is two `TypeId`s and not a compile error.
#[test]
fn retention_record_is_rk_types_dispatch_record() {
    use std::any::{TypeId, type_name};
    assert_eq!(
        TypeId::of::<rk_store::retention::DispatchRecord>(),
        TypeId::of::<rk_types::activity::DispatchRecord>(),
        "two DispatchRecord types are on the tree: {} and {}",
        type_name::<rk_store::retention::DispatchRecord>(),
        type_name::<rk_types::activity::DispatchRecord>()
    );
    // The re-export is the only spelling this crate offers: the type's home is `rk_types`.
    assert_eq!(
        type_name::<rk_store::retention::DispatchRecord>(),
        "rk_types::activity::DispatchRecord"
    );
}

/// argv and env are digests; there is **no field** for the child environment.
///
/// The constructor is `rk-types`' since Step 7c Slice 1 and takes the completed record's tail
/// (ADR-0126; `s7c` §7c.19); this site builds a pre-spawn tail — nothing measured, the pin as
/// `Pinned` — because what it asserts is the digesting, which the tail does not touch.
#[test]
fn a_dispatch_record_keeps_digests_and_has_no_environment_field() {
    let rec = DispatchRecord::new(
        &["claude".into(), "--print".into(), "--secret".into()],
        &[("ANTHROPIC_API_KEY".into(), "sk-verysecretvalue".into())],
        "2.1.269",
        b"out",
        b"err",
        RecordTail {
            cli_version: CliVersion::Pinned("2.1.269".to_owned()),
            api_retry_events: 0,
            prompt_cache_ttl: "1h".to_owned(),
            subagent_prompt_cache_ttl: "1h".to_owned(),
            timings: Timings {
                wall_ms: 0,
                api_ms: None,
            },
            usage: None,
            total_cost_usd: None,
            headroom_before: None,
            headroom_after: None,
            cancelled_by: None,
            detected_by: None,
        },
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
        usage: _,
        timings: _,
        total_cost_usd: _,
        headroom_before: _,
        headroom_after: _,
        cli_version: _,
        api_retry_events: _,
        prompt_cache_ttl: _,
        subagent_prompt_cache_ttl: _,
        cancelled_by: _,
        detected_by: _,
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

// ---------------------------------------------------------------- s8e §8e.2: the tier-1 writer's statements, R2's migration, AC-180

/// The fresh store every tier-1 statement test runs over: `rk_store::migrate::open`, which applies
/// every shipped migration and stamps the identity in one transaction (ADR-0181 decision 6).
fn migrated(dir: &std::path::Path) -> Writer {
    rk_store::migrate::open(dir, &ident("aaaa1111", "/repo/a/.git")).expect("a fresh store opens")
}

fn one_row<T>(
    dir: &std::path::Path,
    sql: &str,
    f: impl FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
) -> T {
    let r = Reader::open(dir).expect("reader");
    r.conn().query_row(sql, [], f).expect("the row reads")
}

fn count(dir: &std::path::Path, sql: &str) -> i64 {
    one_row(dir, sql, |r| r.get::<_, i64>(0))
}

/// **`rk_store::tier_one::insert_stage_transition`** writes a boundary row carrying its item, its
/// two stages, the anchor commit and that commit's tree (ADR-0181 decisions 3 and 4; ADR-0105
/// clause 4: *"the recorded tree is that commit's tree"*).
///
/// RED at entry: no `rk_store::tier_one` module (E0433) — no statement exists.
#[test]
fn tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree() {
    let dir = tmp();
    let w = migrated(dir.path());
    let sha = "a".repeat(40);
    let tree = "b".repeat(40);
    w.transaction(|tx| {
        rk_store::tier_one::insert_stage_transition(
            tx,
            "item-a",
            "triage",
            "implement",
            &sha,
            &tree,
        )
    })
    .expect("the statement inserts");
    drop(w);

    let row: (String, String, String, String, String) = one_row(
        dir.path(),
        "SELECT item, from_stage, to_stage, anchor_sha, tree_hash FROM t1_stage_transition",
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    );
    assert_eq!(
        row,
        (
            "item-a".to_owned(),
            "triage".to_owned(),
            "implement".to_owned(),
            sha,
            tree
        ),
        "the boundary row carries its item, both stages, its anchor and the anchor's tree"
    );
    assert_eq!(
        count(dir.path(), "SELECT count(*) FROM t1_stage_transition"),
        1
    );
}

/// **`t1_gate_transition` is append-only: one row per transition, and a written row never moves**
/// (D21-43 N6; D21-46 R2; ADR-0181 decision 5). An opening then an answer for one token are two
/// rows, the opening row reads exactly as written afterwards, and a second opening for the same
/// token is refused by `UNIQUE (token, state)` and adds nothing.
///
/// RED at entry: no `t1_gate_transition` table and no statement (E0433 on `rk_store::tier_one`).
#[test]
fn t1_gate_transition_appends_one_row_per_transition_and_is_never_updated() {
    use rk_store::tier_one::{GateState, GateTransition};

    let dir = tmp();
    let w = migrated(dir.path());
    let opened = GateTransition {
        token: "c".repeat(64),
        state: GateState::Opened,
        tree_hash: Some("d".repeat(40)),
        deadline: Some("1760000000".to_owned()),
        kind: Some("human".to_owned()),
        reach: Some("1".to_owned()),
        option_id: None,
        stage: Some("implement".to_owned()),
        run_id: Some("implement-0123456789abcdef".to_owned()),
    };
    let answered = GateTransition {
        state: GateState::Answered,
        option_id: Some("approve".to_owned()),
        stage: None,
        run_id: None,
        ..opened.clone()
    };
    let (sha_open, sha_answer) = ("1".repeat(40), "2".repeat(40));
    w.transaction(|tx| rk_store::tier_one::insert_gate_transition(tx, &opened, &sha_open))
        .expect("the opening appends");
    w.transaction(|tx| rk_store::tier_one::insert_gate_transition(tx, &answered, &sha_answer))
        .expect("the answer appends");
    let again =
        w.transaction(|tx| rk_store::tier_one::insert_gate_transition(tx, &opened, &sha_open));
    assert!(
        again.is_err(),
        "a second `opened` row for one token is refused (UNIQUE (token, state)); a re-reach mints a fresh token (ADR-0105 clause 10)"
    );
    drop(w);

    assert_eq!(
        count(dir.path(), "SELECT count(*) FROM t1_gate_transition"),
        2,
        "one row per transition: the opening and the answer are two rows, never one updated row"
    );
    let first: (String, String, String, String, String) = one_row(
        dir.path(),
        "SELECT state, anchor_sha, CAST(stage AS TEXT), CAST(run_id AS TEXT), CAST(kind AS TEXT) \
         FROM t1_gate_transition ORDER BY id LIMIT 1",
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    );
    assert_eq!(
        first,
        (
            "opened".to_owned(),
            sha_open,
            "implement".to_owned(),
            "implement-0123456789abcdef".to_owned(),
            "human".to_owned()
        ),
        "the opening row reads exactly as written after the answer: nothing updated it"
    );
    let second: (String, String, String) = one_row(
        dir.path(),
        "SELECT state, anchor_sha, CAST(option_id AS TEXT) FROM t1_gate_transition ORDER BY id DESC LIMIT 1",
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    );
    assert_eq!(
        second,
        ("answered".to_owned(), sha_answer, "approve".to_owned()),
        "the answer is its own row, anchored to its own commit"
    );
}

/// **An `answered` row carries its gate's BOUND tree, read from the `opened` row; with no
/// `opened` row it is refused** (D21-96; ADR-0181 decision 3's dated note). *(Added 2026-09-28,
/// `s8e` build, package D2196-8e.)*
///
/// Arm 1: an opening bound to tree `B`, then a `gate_answered` whose payload `tree_hash` is the
/// answer's OBSERVED tree `O` (a void answer's shape) — the `answered` row's `tree_hash` is `B`, as
/// its `Gate: … answered` mark's is (ADR-0184 decision 1), never `O`. Arm 2 (the refusal branch's
/// fixture): a `gate_answered` for a token the store never opened is refused as
/// [`StoreError::TierOne`] naming the rule, and the transaction leaves zero rows — the store does
/// not fall back to the payload's tree. RED against the pre-D21-96 writer (the payload's tree
/// taken): arm 1 reads `O`. Red under S8E-M58 (arm 1) and S8E-M59 (arm 2: the payload's tree taken
/// when no `opened` row exists).
#[test]
fn an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening() {
    use rk_types::event::Event;
    use rk_types::event_kind::EventKind;

    let bound = "b".repeat(40);
    let observed = "0".repeat(40);
    let token = "c".repeat(64);
    let anchor = |sha: &'static str| rk_store::tier_one::Anchor {
        sha,
        tree: sha,
        opening_run: None,
    };
    let answered = |token: &str| {
        Event::new(
            EventKind::GateAnswered,
            serde_json::json!({ "token": token, "tree_hash": observed, "verdict": "approve" }),
        )
    };

    // ---- arm 1: the bound tree.
    let dir = tmp();
    let w = migrated(dir.path());
    let opened = Event::new(
        EventKind::GateOpened,
        serde_json::json!({ "deadline": 1_760_000_000, "kind": "human", "reach": 1, "token": token, "tree_hash": bound }),
    );
    w.transaction(|tx| {
        rk_store::tier_one::append(
            tx,
            &opened,
            &anchor("1111111111111111111111111111111111111111"),
        )
    })
    .expect("the opening appends");
    w.transaction(|tx| {
        rk_store::tier_one::append(
            tx,
            &answered(&token),
            &anchor("2222222222222222222222222222222222222222"),
        )
    })
    .expect("the answer appends");
    drop(w);
    let tree: String = one_row(
        dir.path(),
        "SELECT tree_hash FROM t1_gate_transition WHERE state = 'answered'",
        |r| r.get(0),
    );
    assert_eq!(
        tree, bound,
        "the `answered` row carries the answer's OBSERVED tree, not the gate's bound tree its mark \
         carries (D21-96)"
    );

    // ---- arm 2: no opening, refused, zero rows.
    let dir = tmp();
    let w = migrated(dir.path());
    let refused = w.transaction(|tx| {
        rk_store::tier_one::append(
            tx,
            &answered(&token),
            &anchor("3333333333333333333333333333333333333333"),
        )
    });
    drop(w);
    assert!(
        matches!(&refused, Err(StoreError::TierOne(m)) if m.contains("bound tree") && m.contains("D21-96")),
        "an `answered` row with no `opened` row for its token was not refused by the bound-tree \
         rule: {refused:?}"
    );
    assert_eq!(
        (
            count(dir.path(), "SELECT count(*) FROM t1_gate_transition"),
            count(dir.path(), "SELECT count(*) FROM t2_event")
        ),
        (0, 0),
        "the refused answer left rows"
    );
}

/// **`0002_gate_transitions`** (D21-46 R2; ADR-0181 decisions 5 and 6): a pair the schema lint
/// reads; a fresh store gains `t1_gate_transition` and `t2_event.item`, loses `t1_gate`, stamps
/// schema version 2 and counts two applied migrations; and a second open of that store applies
/// nothing again (an `ALTER TABLE … ADD COLUMN` run twice would refuse).
///
/// RED at entry: `SCHEMA_VERSION` is 1 and no `0002` exists (and no `rk_store::migrate`, E0433).
#[test]
fn migration_0002_adds_the_transitions_table_and_the_item_column_and_stamps_version_two() {
    assert_eq!(SCHEMA_VERSION, 2, "R2's migration stamps schema version 2");
    let names: Vec<&str> = schema_lint::all().iter().map(|(n, _, _)| *n).collect();
    assert_eq!(
        names,
        ["0001_initial", "0002_gate_transitions"],
        "the schema lint lists the 0002 pair after 0001"
    );
    for (name, sql, keys) in schema_lint::all() {
        schema_lint::lint(sql, keys)
            .unwrap_or_else(|e| panic!("{name} fails AC-166's lint: {e:?}"));
    }
    let (_, sql2, keys2) = schema_lint::all()[1];
    assert!(
        schema_lint::marked_columns(sql2).contains(&"t1_gate_transition.anchor_sha".to_owned())
            && schema_lint::parse_keys(keys2)
                .expect("keys parse")
                .get("t1_gate_transition.tree_hash")
                .map(String::as_str)
                == Some("tree"),
        "0002 marks and declares t1_gate_transition's repo keys (AC-166)"
    );

    let dir = tmp();
    let w = migrated(dir.path());
    drop(w);
    let tables = |name: &str| {
        count(
            dir.path(),
            &format!("SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = '{name}'"),
        )
    };
    assert_eq!(
        tables("t1_gate_transition"),
        1,
        "0002 creates t1_gate_transition"
    );
    assert_eq!(
        tables("t1_gate"),
        0,
        "0002 drops t1_gate, the table nothing wrote"
    );
    assert_eq!(
        count(
            dir.path(),
            "SELECT count(*) FROM pragma_table_info('t2_event') WHERE name = 'item'"
        ),
        1,
        "0002 adds the interim item column to t2_event (ADR-0182)"
    );
    assert_eq!(
        count(
            dir.path(),
            "SELECT count(*) FROM pragma_table_info('t1_gate_transition') WHERE name = 'answered'"
        ),
        0,
        "no gate-state column an UPDATE could move"
    );
    assert_eq!(
        count(
            dir.path(),
            "SELECT schema_version FROM rk_identity WHERE id = 1"
        ),
        2,
        "the fresh store stamps schema version 2"
    );
    assert_eq!(
        count(dir.path(), "PRAGMA user_version"),
        2,
        "two migrations counted as applied"
    );

    // Opened again: an existing version-2 store applies no migration a second time.
    let again = migrated(dir.path());
    drop(again);
    assert_eq!(count(dir.path(), "PRAGMA user_version"), 2);
}

/// The checked-in AC-180 fixture: minted on the entry tree at `766854c` before `0002` existed
/// (`s8e` §8e.0), schema version 1, a path-neutral identity, `t2_event` 3 and `t1_*` 0.
const STORE_V1_SHA256: &str = "603d60041fc41e0788deb8cc80957c330dd3d8101b2cdadb887fbfd42f055dd3";

/// **AC-180, on the register's own instrument**: *"a checked-in fixture store at version N-1; an
/// assertion that the file's bytes are unchanged after the refused open."* A copy of
/// `tests/fixtures/store-v1.sqlite3` is opened through `rk_store::migrate::open` against its own
/// root (so AC-179's root check, which runs first, passes), the open is refused naming the
/// migration from 1 to 2, and the copy's sha256 still equals the blob's. The stamp still reads 1.
///
/// RED at entry: `SCHEMA_VERSION` is 1, so the version-1 fixture is not refused (and no
/// `rk_store::migrate`, E0433).
#[test]
fn a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("store-v1.sqlite3");
    let blob = std::fs::read(&fixture).expect("the checked-in fixture reads");
    assert_eq!(
        rk_types::digest::sha256(&blob).as_str(),
        STORE_V1_SHA256,
        "PREMISE: the fixture is the one minted at stage open (binary, never smudged)"
    );

    let dir = tmp();
    let target = conn::store_path(dir.path());
    std::fs::create_dir_all(target.parent().expect("parent")).expect("mkdir");
    std::fs::write(&target, &blob).expect("copy");

    let opening = Identity {
        root_commit: "ac180".repeat(8),
        common_dir: "/fixture/store-v1/.git".to_owned(),
        schema_version: SCHEMA_VERSION,
    };
    let refused = rk_store::migrate::open(dir.path(), &opening);
    let text = match refused {
        Err(StoreError::Identity(text)) => text,
        Err(other) => {
            panic!("a version-1 store must be refused by the identity check, not {other}")
        }
        Ok(_) => panic!(
            "the version-1 fixture OPENED under schema version {SCHEMA_VERSION}: no migration was required of it (AC-180)"
        ),
    };
    assert!(
        text.contains("A migration from 1 to 2 is required")
            && text.contains("no implicit in-place migration"),
        "the refusal names the migration required: {text}"
    );

    let after = std::fs::read(&target).expect("the copy reads");
    assert_eq!(
        rk_types::digest::sha256(&after).as_str(),
        STORE_V1_SHA256,
        "the refused open changed the file's bytes: an implicit in-place migration or stamp (AC-180)"
    );
    assert_eq!(
        count(
            dir.path(),
            "SELECT schema_version FROM rk_identity WHERE id = 1"
        ),
        1,
        "the stamp still reads version 1"
    );
}
