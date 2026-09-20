//! **The composition root's two reflexes** — the halt it refuses to recover from, and the handler
//! that does nothing but fire a token (ADR-0118 clauses 1/3/5; ADR-0099 decision 2, **D18-2**;
//! `s7d` §7d.4, §7d.10).
//!
//! Both are in-process by design, and §7d.10 says why for each: the process-level `70` is not
//! observable at all (no fixture can poison a spawned process's lock without the seam AC-474
//! forbids), and no test may deliver a console signal (under `cargo test` the binary is not in its
//! own process group, so the event would reach `cargo`, libtest and the shell running the block).
//! The OS-delivery arm is **ARMED**, not run.

use std::sync::{Arc, Mutex};

use rk_ports::cancel::CancelToken;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::VecStore;

use rk_cli::exit::Halt;
use rk_cli::{signal, store};

use std::path::Path;

use crate::{code_lines, common_dir, lint_controls_dir, run_init, src_dir, temp_repo};

/// The root commit of a fixture repository, read with git directly.
///
/// Module level rather than inside the test that uses it: `clippy::items_after_statements` is
/// DENIED workspace-wide (s7d 7d.15's enumerated pedantic subset).
#[expect(
    clippy::disallowed_types,
    reason = "a test reading its own fixture repository's root commit to assert the premise;                   the product's git goes through rk-git's one GitExec wrapper"
)]
fn root_of(dir: &Path) -> String {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("git runs");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// A store behind the root's own lock, in the shape `rk_ports::root::Ports` holds it.
fn locked_store() -> Arc<Mutex<dyn EventStore>> {
    Arc::new(Mutex::new(VecStore::default()))
}

/// ADR-0118 clauses 1, 3 and 5; RB-M-01. The in-process twin of the clause-5 fixture; the arm at the
/// verb (`adr0118_poisoned_store_lock_halts_run_exit_70`) is Slice 3's, when a verb exists to reach
/// it.
///
/// **The sound lock is acquired FIRST, and that is not a courtesy.** An `acquire` whose body was
/// `Err(Halt::PoisonedLock { .. })` unconditionally would satisfy every assertion about the poisoned
/// case forever — the guard-that-cannot-fail shape. §7d.10's *"never a guard, never a `StepError`"*
/// is only a discriminating claim about the poisoned lock if the same function is shown to return a
/// guard over a sound one.
///
/// Red by `Ok(guard)` over the poisoned `Arc` (a recovery), and by a panic instead of a returned
/// `Err` (exit 101's cause rather than the table's `70`).
#[test]
fn adr0118_acquire_returns_halt_on_poisoned_lock() {
    let store = locked_store();

    // The positive control: a sound lock yields a guard.
    {
        let guard = store::acquire(&store, "status").expect("a sound lock yields a guard");
        drop(guard);
    }

    // Poison it the only way a lock is really poisoned: a panic while the guard is held.
    let poisoner = Arc::clone(&store);
    let handle = std::thread::spawn(move || {
        let _guard = poisoner
            .lock()
            .expect("the lock is sound when this thread takes it");
        panic!("a writer panicked while holding the store lock");
    });
    assert!(
        handle.join().is_err(),
        "the poisoning thread was expected to panic, and did not — nothing below would be testing a \
         poisoned lock"
    );
    assert!(
        store.is_poisoned(),
        "the lock is not poisoned after a panic while held, so this test would assert the halt over \
         a sound lock"
    );

    match store::acquire(&store, "status") {
        Ok(_guard) => panic!(
            "`store::acquire` returned a GUARD over a poisoned lock. That is a recovery, and \
             ADR-0118 clause 3 forbids every spelling of one: the writer panicked while holding \
             this lock, so the state behind it is of unknown shape"
        ),
        Err(halt) => {
            assert_eq!(
                halt,
                Halt::PoisonedLock {
                    lock: "store",
                    verb: "status"
                },
                "the halt does not name the lock and the verb the diagnostic renders"
            );
            let rendered = rk_cli::render_halt(&halt);
            assert!(
                rendered.contains("the store lock is poisoned (store)"),
                "the rendered halt does not name the lock; it read:\n{rendered}"
            );
            assert_eq!(
                rk_cli::code_for(&Err(halt)),
                70,
                "a poisoned-lock halt is exit 70 (ADR-0118 clause 6; §7d.4's table)"
            );
        }
    }
}

/// ADR-0099 decision 2, D18-2. **This IS the closure-level test** (§7d.10): the OS-delivery arm is
/// ARMED, and no test signals its own process group.
///
/// Two halves. The first calls the closure and watches the token flip — and asserts it was NOT
/// flipped beforehand, or a token that starts fired would pass. The second is a source scan over
/// `signal.rs` alone for the three things §7d.10 forbids the handler to do, self-controlled over its
/// `lint-controls/` file first (§7d.14's standing rule).
///
/// **The instrument's limit, stated rather than implied** — §7d.10 asks that the test also show the
/// closure "wrote nothing" and "printed nothing". Neither is buildable as an observation here: the
/// closure takes only a token, so a counting store fake it cannot reach is vacuous by construction,
/// and libtest exposes no handle to read back its own capture (`set_output_capture` is unstable).
/// The source scan is the honest form of both, and §7d.10's wording is RAISED.
#[test]
fn signal_handler_fires_cancel_token() {
    let token = CancelToken::new();
    assert!(
        !token.is_cancelled(),
        "a fresh token is already fired, so the flip below would prove nothing"
    );

    signal::on_signal(&token);

    assert!(
        token.is_cancelled(),
        "the handler did not fire the token, which is the one thing it exists to do"
    );

    // The handler must do NOTHING else. Scanned over signal.rs alone: a ban scoped to the whole
    // crate would refuse `main`'s own printing and exiting, which are not the handler's.
    let banned = [
        "println!",
        "eprintln!",
        "process::exit",
        "append(",
        ".lock(",
    ];
    let control = lint_controls_dir().join("signal_handler_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| banned.iter().any(|b| l.contains(b)))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line matching a banned spelling — the scan is vacuous",
        control.display()
    );

    let signal_rs = src_dir().join("signal.rs");
    let text = std::fs::read_to_string(&signal_rs).expect("signal.rs is readable");
    let hits: Vec<String> = code_lines(&text)
        .into_iter()
        .filter(|(_, l)| banned.iter().any(|b| l.contains(b)))
        .map(|(n, l)| format!("{n}: {}", l.trim()))
        .collect();
    assert!(
        hits.is_empty(),
        "the signal handler writes, prints or exits. It runs on a foreign thread at an arbitrary \
         point, possibly while the store lock is held; it fires the token and returns \
         (ADR-0099 d2):\n{}",
        hits.join("\n")
    );
}

/// **H-3, D-18 — the common dir is discovered from the INHERITED cwd, through the wrapper.**
///
/// §7d.3, verbatim: *"spawn `redkiln status` with `Command::current_dir` set to a **subdirectory**
/// of a fixture repository and no `-C`, assert exit `0` and that the store it opened is the
/// repository's (the identity row's `common_dir` equals the **absolute**
/// `git rev-parse --path-format=absolute --git-common-dir` run from the root); a second arm spawns
/// `redkiln -C <repo> status` from an unrelated temp cwd and asserts the same identity row."*
///
/// §7d.14's reds: *"Red by a `status` from a subdirectory that fails to find the repository, by an
/// identity row stamped relative (`./.git`) instead of the absolute common dir, by a `-C <repo>`
/// the parser accepts and the discovery ignores (the second arm), and separately by
/// `env::current_dir` in either crate (the block's ban); a weaker red runs from the root only."*
///
/// **The discriminating red §7d.3 fixes**: *"a `\".\"` handed on would stamp `./.git` at `init` and
/// refuse a `status` from a subdirectory as another repository's store — an AC-413 false positive."*
/// So the absoluteness of the recorded `common_dir` is asserted, not merely its presence.
#[test]
fn common_dir_discovered_from_inherited_cwd_via_wrapper() {
    let repo = temp_repo();
    let root = repo.path();
    let expected = common_dir(root);
    assert!(
        expected.is_absolute(),
        "PREMISE: the expected common dir {} is not absolute, so the assertion below could not \
         tell an absolute reading from a relative one",
        expected.display()
    );
    assert_eq!(
        run_init(root).status.code(),
        Some(0),
        "the fixture repository must be initialised before a status can read its store"
    );

    // ---- arm 1: an inherited cwd that is a SUBDIRECTORY, and no `-C`. --------------------------
    let nested = root.join("crates").join("deep");
    std::fs::create_dir_all(&nested).expect("the subdirectory is created");
    let from_sub = spawn_status(&nested, &[]);
    assert_eq!(
        from_sub.status.code(),
        Some(0),
        "`status` from a subdirectory exited {:?}; stderr:\n{}",
        from_sub.status.code(),
        String::from_utf8_lossy(&from_sub.stderr)
    );
    let rows = crate::support::identity_rows(&expected);
    assert_eq!(
        Path::new(&rows.common_dir),
        expected,
        "the identity row's common_dir is not the repository's absolute common dir. A `\".\"` \
         handed on stamps a RELATIVE `./.git` here and then refuses a status from a subdirectory \
         as another repository's store — an AC-413 false positive"
    );
    assert!(
        Path::new(&rows.common_dir).is_absolute(),
        "the identity row's common_dir `{}` is relative",
        rows.common_dir
    );

    // ---- arm 2: `-C <repo>` from an UNRELATED cwd. ---------------------------------------------
    //
    // Red by a `-C` the parser accepts and the discovery ignores: from this cwd there is no
    // repository at all, so an implementation that read the process cwd cannot reach one.
    let elsewhere = tempfile::tempdir().expect("an unrelated temp dir");
    let with_dash_c = spawn_status(elsewhere.path(), &["-C", &root.display().to_string()]);
    assert_eq!(
        with_dash_c.status.code(),
        Some(0),
        "`redkiln -C <repo> status` from an unrelated cwd exited {:?}; stderr:\n{}",
        with_dash_c.status.code(),
        String::from_utf8_lossy(&with_dash_c.stderr)
    );
    assert_eq!(
        crate::support::identity_rows(&expected),
        rows,
        "the `-C` arm opened a different store than the inherited-cwd arm"
    );
}

/// **AC-413 as amended 2026-09-18 (three fields; RA-M-07 dropped boot-id)** — a store belonging to
/// another repository is refused, and nothing is written to it.
///
/// §7d.9, verbatim: *"a store created in repository A, its file copied under repository B's common
/// dir; `redkiln status` in B exits `3` naming the identity mismatch, and the store file's sha256 is
/// unchanged across the refused `status` (zero writes — a spawned binary has no counting seam …) —
/// the read half is `rk-store`'s `open` refusing before any query."*
///
/// §7d.14's reds: *"Red by exit ≠ `3`, or by the store file's sha256 changing across the refused
/// `status` … a weaker red asserts the refusal only."*
///
/// **Two things in order** (§7d.14's standing rule for a refusal test): exit `3` with the mismatch
/// NAMED, then the sha256 unchanged. The exit code alone is green-by-accident against the Slice-1
/// stub, which also exited `3`; the two halves below are what make it the absent behaviour.
#[test]
fn ac413_store_opened_from_other_repo_refused_zero_rows() {
    let a = temp_repo();
    let b = temp_repo();

    // PREMISE, and it was false until `temp_repo` was fixed at this cycle: the two repositories must
    // have DIFFERENT root commits, because `rk_store::identity::check` compares `root_commit` first.
    // Two identically-built repositories share a SHA, and against those this whole test is vacuous —
    // the store is not foreign and is correctly accepted.
    assert_ne!(
        root_of(a.path()),
        root_of(b.path()),
        "PREMISE: the two fixture repositories share a root commit, so B's store is not FOREIGN and \
         the refusal below would be asserted over a store that is legitimately its own"
    );

    assert_eq!(run_init(a.path()).status.code(), Some(0), "A initialises");
    assert_eq!(run_init(b.path()).status.code(), Some(0), "B initialises");

    let common_a = common_dir(a.path());
    let common_b = common_dir(b.path());
    let store_a = common_a.join("redkiln").join("store.sqlite3");
    let store_b = common_b.join("redkiln").join("store.sqlite3");

    // A's store, under B's common dir. The two repositories have different root commits, which is
    // the first field `rk_store::identity::check` compares.
    std::fs::copy(&store_a, &store_b).expect("A's store copies over B's");
    let before = crate::support::store_digest(&common_b).expect("B now holds A's store file");

    let refused = spawn_status(b.path(), &[]);
    let stderr = String::from_utf8_lossy(&refused.stderr).to_lowercase();

    // (1) exit 3, and the mismatch NAMED.
    assert_eq!(
        refused.status.code(),
        Some(3),
        "`status` over a foreign store exited {:?}; stderr:\n{}",
        refused.status.code(),
        String::from_utf8_lossy(&refused.stderr)
    );
    assert!(
        stderr.contains("identity") || stderr.contains("root_commit") || stderr.contains("commit"),
        "the refusal does not NAME the identity mismatch, so exit 3 alone is indistinguishable \
         from the Slice-1 stub's 3. stderr:\n{}",
        String::from_utf8_lossy(&refused.stderr)
    );

    // (2) and then: the store file's bytes are unchanged across the refusal.
    assert_eq!(
        crate::support::store_digest(&common_b).as_deref(),
        Some(before.as_str()),
        "the refused `status` WROTE to the foreign store — `open` must refuse before any query"
    );
}

/// Spawn `redkiln [args…] status` with stdin closed (D-15: no verb reads stdin), from `cwd`.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS — the inherited cwd and the -C flag are process \
              facts a library call cannot observe (s7d §7d.3's own instrument)"
)]
fn spawn_status(cwd: &Path, args: &[&str]) -> std::process::Output {
    std::process::Command::new(crate::redkiln())
        .args(args)
        .arg("status")
        .current_dir(cwd)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("the redkiln binary runs")
}

/// Steps (1)-(7) over a `pack-min` fixture repository, for the two in-process cases below.
///
/// Module level rather than inside each test: `clippy::items_after_statements` is DENIED
/// workspace-wide.
fn resolved_over_pack_min(root: &Path) -> rk_cli::verbs::run::Resolved {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::RunArgs {
        item: "bug-1".to_owned(),
        lane: None,
    };
    let env = rk_cli::env::Env::from_map(std::collections::BTreeMap::new());
    rk_cli::verbs::run::through_step_seven(&cli, &args, &env, rk_cli::verbs::Startup::real())
        .unwrap_or_else(|e| panic!("steps (1)-(7) resolve over the fixture repository: {e:?}"))
}

/// **H-2, RA-M-04 — a token fired BEFORE the run starts exits 130 and writes nothing partial.**
///
/// §7d.10, verbatim: *"the verb function called with a token fired **before** `run` starts (no env
/// toggle — `RK_TEST_PREFIRE_CANCEL` is **not** used; AC-474 forbids it; the token is a function
/// argument); returns `Ok(Exit::SignalOutsideDispatch)`; the store gained at most the
/// `reconcile_started` row; no commit."*
///
/// §7d.14's reds: *"Red by the verb function returning anything but
/// `Ok(Exit::SignalOutsideDispatch)` on a pre-fired token, by a second row, or by a commit; a weaker
/// red asserts the exit only."*
///
/// **No process is spawned**, so there is no 30 s bound and no `Stdio::null()` — the token is handed
/// straight to the function, which is the only form AC-474 admits.
#[test]
fn signal_outside_dispatch_exits_130_writes_nothing_partial() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let porcelain_before = crate::porcelain(root);
    let resolved = resolved_over_pack_min(root);

    let token = CancelToken::new();
    token.cancel();
    assert!(
        token.is_cancelled(),
        "PREMISE: the token is not fired, so the exit below would be the ordinary path's"
    );

    let store = locked_store();
    let exit = rk_cli::verbs::run::run_with(&resolved, &store, &token)
        .expect("a pre-fired token is an EXIT, not a halt");

    // (1) the return, THEN (2) the absences — §7d.14's "two things in order".
    assert_eq!(
        exit,
        rk_cli::exit::Exit::SignalOutsideDispatch,
        "a pre-fired token must return Exit::SignalOutsideDispatch (130), not {exit:?}"
    );
    assert_eq!(exit.code(), 130);

    let rows = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("bug-1"))
        .expect("the fixture store queries");
    assert!(
        rows.len() <= 1,
        "the interrupted run wrote {} rows; §7d.10 admits at most the `reconcile_started` one, \
         because every store write is ONE row and nothing partial may survive: {:?}",
        rows.len(),
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
    if let Some(row) = rows.first() {
        assert_eq!(
            row.kind,
            rk_types::event_kind::EventKind::ReconcileStarted,
            "the one row an interrupted run may leave is `reconcile_started`"
        );
    }
    assert_eq!(
        crate::porcelain(root),
        porcelain_before,
        "the interrupted run changed the working tree; every git write is ONE commit and an \
         interrupted run makes none"
    );
    assert!(
        crate::refs(root).iter().all(|r| !r.contains("rk-run/")),
        "the interrupted run created an `rk-run/*` ref"
    );
}

/// **ADR-0118 clauses 1, 3, 5, 6; RB-M-01 — the verb HALTS on a poisoned store lock, and
/// `is_lock_halt` is asserted against the payload a REAL kernel-side poison produces.**
///
/// This is `is_lock_halt`'s first and only instrument; Slice 2 shipped the function untested.
///
/// # 🛑 PART B IS THE CLAUSE THAT MATTERS, AND THE VACUOUS FORM IS NAMED SO IT IS NOT WRITTEN
///
/// §7d.10: *"a test that built its own `String` around `rk_kernel::runtime::LOCK_HALTS` (the `pub
/// const`) would never observe the format the kernel's `.lock().expect(LOCK_HALTS)` actually
/// produces, and a shipped payload of a different shape would leave the hook at 101 with every test
/// green."* So the payload here comes out of `catch_unwind` around a REAL kernel call, unmodified.
///
/// `Runtime::store_guard` is **private** (verified), so §7d.10's literal instruction cannot compile
/// from an integration test. The nearest public method whose first statement is `self.store_guard()`
/// is `Runtime::journal`, and that is what is called.
///
/// `AssertUnwindSafe` is required (`dyn EventStore` carries no `RefUnwindSafe` bound) and is **not** a
/// poison recovery: the block's recovery ban is scoped to `crates/rk-cli/src` and
/// `crates/redkiln-migrate/src`, and `tests/it/` is outside it.
///
/// The default panic hook is suppressed around the `catch_unwind` calls, which is process-global
/// under `cargo test`'s thread-per-test harness. That is an implementation caution, not a decision;
/// §7d.10 does not address it.
#[test]
fn adr0118_poisoned_store_lock_halts_run_exit_70() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved = resolved_over_pack_min(repo.path());

    // ---- Part A: the verb halts. ----------------------------------------------------------------
    let poisoned = crate::support::poisoned_store();
    let token = CancelToken::new();
    let result = rk_cli::verbs::run::run_with(&resolved, &poisoned, &token);

    match &result {
        Ok(exit) => panic!(
            "`run` returned Ok({exit:?}) over a poisoned lock. ADR-0118 clause 3 forbids every \
             spelling of a recovery: the writer panicked while holding this lock, so the state \
             behind it is of unknown shape"
        ),
        Err(halt) => {
            assert_eq!(
                halt,
                &Halt::PoisonedLock {
                    lock: "store",
                    verb: "the run"
                },
                "the halt does not name the lock and the verb"
            );
            let rendered = rk_cli::render_halt(halt);
            assert!(
                rendered.contains("the store lock is poisoned (store)"),
                "the rendered halt does not name the lock; it read:\n{rendered}"
            );
        }
    }
    assert_eq!(
        rk_cli::code_for(&result),
        70,
        "a poisoned-lock halt is exit 70 (ADR-0118 clause 6; §7d.4's table)"
    );

    // ---- Part B: `is_lock_halt` against the payload the KERNEL actually produces. ---------------
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let ports = crate::support::ports(
        Arc::new(rk_ports::fakes::FakeHarness::scripted(vec![])),
        &crate::support::poisoned_store(),
    );
    let runtime = rk_kernel::runtime::Runtime::new(ports, "run-id", "implement");
    let real = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = runtime.journal("any-item");
    }))
    .expect_err("a poisoned store lock panics at the kernel's own .expect(LOCK_HALTS)");

    let other = std::panic::catch_unwind(|| panic!("something else entirely"))
        .expect_err("the control panic panics");

    std::panic::set_hook(previous);

    assert!(
        rk_cli::is_lock_halt(real.as_ref()),
        "`is_lock_halt` is FALSE on the payload a real kernel-side poison produces. The kernel's \
         payload is the String formed by `.expect(LOCK_HALTS)` — the const FOLLOWED BY the \
         PoisonError's Debug rendering — not the const itself, and the hook substring-matches the \
         exported const. A hook that is false here leaves the process at 101 with every other test \
         green"
    );
    assert!(
        !rk_cli::is_lock_halt(other.as_ref()),
        "`is_lock_halt` is TRUE on an unrelated panic payload, so it is not discriminating"
    );
    assert!(
        !rk_cli::is_lock_halt(&42_i32),
        "`is_lock_halt` is TRUE on a non-string payload"
    );

    // ---- Part C: no recovered guard anywhere on the product path. -------------------------------
    //
    // The seam-level assertions above cannot see an `into_inner` in a file the run did not reach, so
    // the ban is a source scan — self-controlled first, on §7d.14's standing rule.
    let control = lint_controls_dir().join("adr0118_poison_recovery_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let recovery = ["into_inner", "catch_unwind"];
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| recovery.iter().any(|r| l.contains(r)))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT recovery spelling — the scan is vacuous",
        control.display()
    );
    let hits: Vec<String> = crate::rs_files(&src_dir())
        .into_iter()
        .flat_map(|path| {
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            code_lines(&text)
                .into_iter()
                .filter(|(_, l)| recovery.iter().any(|r| l.contains(r)))
                .map(|(n, l)| format!("{}:{n}: {}", path.display(), l.trim()))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        hits.is_empty(),
        "a poison RECOVERY is on the product path — ADR-0118's rejected third option:\n{}",
        hits.join("\n")
    );
}
