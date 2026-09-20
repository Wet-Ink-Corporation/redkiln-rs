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
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.args(args).arg("status").current_dir(cwd);
    crate::support::output_bounded(&mut cmd)
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
    let resolved = crate::support::resolved_over_pack_min(root, "bug-1");

    let token = CancelToken::new();
    token.cancel();
    assert!(
        token.is_cancelled(),
        "PREMISE: the token is not fired, so the exit below would be the ordinary path's"
    );

    let store = locked_store();
    let inputs = crate::support::inputs_over_pack_min(&resolved);
    let ports = crate::support::ports(
        Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new())),
        &store,
    );
    let mut supply = crate::support::no_spec();
    let exit = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token)
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
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");

    // ---- Part A: the verb halts. ----------------------------------------------------------------
    let poisoned = crate::support::poisoned_store();
    let token = CancelToken::new();
    let inputs = crate::support::inputs_over_pack_min(&resolved);
    let ports = crate::support::ports(
        Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new())),
        &poisoned,
    );
    let mut supply = crate::support::no_spec();
    let result = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token);

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

/// The environment-read spellings the ADR-0083 ban covers — Checkpoint 7d's own pattern.
const ENV_SPELLINGS: [&str; 4] = ["std::env::var", "env::var(", "env::vars(", "env::var_os("];

/// Every `(path, line, text)` in `dir`'s Rust files whose CODE (never its prose) carries `needle`.
fn code_hits(dir: &Path, needles: &[&str]) -> Vec<String> {
    let mut hits = Vec::new();
    for file in crate::rs_files(dir) {
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (line_no, line) in code_lines(&text) {
            if needles.iter().any(|n| line.contains(n)) {
                hits.push(format!("{}:{line_no}: {}", file.display(), line.trim()));
            }
        }
    }
    hits
}

/// **ADR-0083, RB-M-05 — the `Config` is built ONCE at the composition root and threaded.**
///
/// §7d.14: *"The `rk_kernel::Config` value the run carries has the digest the first row recorded;
/// `RunInputs::with_config` called **exactly once**; no `std::env::var` outside
/// `crates/rk-cli/src/env.rs` (source scan, self-controlled against
/// `lint-controls/adr0083_env_var_control.rs`)."*
///
/// # 🛑 HOW THE THREADED VALUE IS READ BACK, AND WHY IT IS THE `run_id`
///
/// `RunInputs` exposes **no** `config()` accessor and its `to_json` is private, so the threaded
/// member cannot be read directly. It is observable in exactly one place: `run_id()` is
/// `sha256_lp([SCHEMA, stage, canonical(inputs)])`, and `to_json` inserts a `config` key **only
/// when a config was threaded in**. So re-threading the value the fixture declares must leave the
/// `run_id` UNCHANGED (it is already there), and threading a DIFFERENT one must move it. Both
/// halves are asserted: without the second, the first would pass forever against a `to_json` that
/// had stopped inserting the key at all.
///
/// # What this test does NOT claim
///
/// **The `config_digest` on the `reconcile_started` row.** §7d.5 puts that member on the row RT-41
/// describes, and **nothing in either binary appends a `ReconcileStarted` row yet** — measured:
/// `grep -rn ReconcileStarted crates/*/src/` matches only the kernel's three *classification*
/// arms. §7d.5 writes that row in its four-member form *"once (8) has re-created the worktree"*,
/// which is cycle 6's. So the digest half is cycle 6's, and the `adapter_set` half is
/// `ac485_…`'s. Claimed here: the value is constructed once and threaded, and the run reached the
/// port with it.
#[test]
fn adr0083_config_constructed_once_at_root_and_threaded() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
    let inputs = crate::support::inputs_over_pack_min(&resolved).with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::supplied(
            "emit-report@1",
            crate::support::any_activity_spec(),
        ),
    );

    // The EXPECTED value, derived from the INSTALLED fixture file rather than from the product's
    // own threading path — a value read back out of the thing under test would assert nothing.
    let file = rk_cli::config::parse(
        &std::fs::read_to_string(repo.path().join(".redkiln").join("config.toml"))
            .expect("the installed configuration is readable"),
    )
    .expect("the fixture configuration parses");
    let expected = rk_kernel::config::Config::new(
        &file.adapters.harness,
        &file.adapters.store,
        &file.adapters.toolchains,
        file.doctor.unread_window_runs,
    );

    // ---- A1: the value IS threaded. Re-threading the same one moves nothing. -------------------
    let threaded = inputs
        .clone()
        .with_config(expected.clone())
        .run_id()
        .expect("the input set hashes");
    let as_built = inputs.run_id().expect("the input set hashes");
    assert_eq!(
        threaded, as_built,
        "the run's input set does not carry the `Config` the repository declares: re-threading it \
         moved the run_id, so `base_inputs` chained no `.with_config(..)`"
    );

    // ---- A2: A1's CONTROL. A DIFFERENT config must move it, or A1 is blind. --------------------
    let wrong = rk_kernel::config::Config::new(
        &file.adapters.harness,
        &file.adapters.store,
        &file.adapters.toolchains,
        file.doctor.unread_window_runs ^ 1,
    );
    let moved = inputs
        .clone()
        .with_config(wrong)
        .run_id()
        .expect("the input set hashes");
    assert_ne!(
        moved, as_built,
        "a DIFFERENT `Config` left the run_id unchanged, so the run_id is blind to the config \
         member and A1 above would pass over a run that threaded nothing"
    );

    // ---- A3: EXACTLY ONE `with_config(` in the crate's source, over CODE LINES ONLY. -----------
    let threading = code_hits(&src_dir(), &["with_config("]);
    assert_eq!(
        threading.len(),
        1,
        "ADR-0083: the composition root threads the `Config` exactly once. Found {}:\n{}",
        threading.len(),
        threading.join("\n")
    );

    // ---- A4: THE COMMENT FILTER IS ITSELF CONTROLLED. ------------------------------------------
    //
    // Three guards on this tree have passed on their own prose. A scan whose filter is asserted
    // only by the absence it reports is one of them waiting to happen.
    for needle in ["with_config(", "std::env::var"] {
        let prose = format!("//! a header naming {needle}\n// and a line comment: {needle}\n");
        assert_eq!(
            code_lines(&prose)
                .into_iter()
                .filter(|(_, l)| l.contains(needle))
                .count(),
            0,
            "`code_lines` returned a COMMENT line for {needle}, so every scan below counts prose"
        );
        let code = format!("{prose}let _ = x.{needle});\n");
        assert_eq!(
            code_lines(&code)
                .into_iter()
                .filter(|(_, l)| l.contains(needle))
                .count(),
            1,
            "`code_lines` dropped the CODE line for {needle}, so every scan below sees nothing"
        );
    }

    // ---- A5: THE ENV CONTROL FIRES FIRST (§7d.14's standing rule). -----------------------------
    let control = lint_controls_dir().join("adr0083_env_var_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    assert!(
        code_lines(&control_text)
            .into_iter()
            .any(|(_, l)| ENV_SPELLINGS.iter().any(|s| l.contains(s))),
        "the control {} carries no NON-COMMENT environment read — the ban below is vacuous",
        control.display()
    );

    // ---- A6: THE EXCLUDED FILE STILL DOES THE THING IT IS EXCLUDED FOR. ------------------------
    let env_rs = src_dir().join("env.rs");
    let env_text = std::fs::read_to_string(&env_rs).expect("env.rs is readable");
    assert!(
        code_lines(&env_text)
            .into_iter()
            .any(|(_, l)| ENV_SPELLINGS.iter().any(|s| l.contains(s))),
        "`env.rs` reads no environment variable, so excluding it from A7 excludes nothing and the \
         zero there is a ban over a program that never does the thing"
    );

    // ---- A7: ZERO ENV READS OUTSIDE `env.rs`. --------------------------------------------------
    let stray: Vec<String> = code_hits(&src_dir(), &ENV_SPELLINGS)
        .into_iter()
        .filter(|h| !h.starts_with(&format!("{}:", env_rs.display())))
        .collect();
    assert!(
        stray.is_empty(),
        "ADR-0083: the process reads its environment ONCE, in `env.rs`, while building `Config`:\n{}",
        stray.join("\n")
    );

    // ---- A8: THE RUN ACTUALLY RAN, so the assertions above are about a live path. --------------
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    let ports = crate::support::ports(
        Arc::clone(&harness) as Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let token = CancelToken::new();
    let mut supply = crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());
    let exit = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token)
        .expect("a sound lock is not a halt");
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "the run did not complete, so nothing above was asserted about a run that threaded anything"
    );
    assert_eq!(
        harness.dispatches(),
        1,
        "the run did not reach the port exactly once ({} dispatches)",
        harness.dispatches()
    );
}

/// **AC-404 — a rate-limited dispatch writes the retry mark and EXITS.** It does not wait.
///
/// §7d.5: *"In-process: the verb function with a `Ports` whose scripted harness returns
/// `Err(RateLimited { resets_at: Some(now + 3600) })`."*
///
/// # 🛑 TWO PRODUCERS WRITE `EventKind::RateLimited`, AND ONLY ONE OF THEM IS THE MARK
///
/// The fake appends a bare failure row on **every** `Err` (`{}` payload), and the kernel appends
/// the retry MARK (`{ "kind": "retry_at", "resets_at": … }`). A test asserting only *"a
/// `rate_limited` row exists"* would be green against a kernel that wrote no mark at all. So the
/// count, both payloads, their ORDER, and `read_retry_at_marks` — which filters on the `kind` tag
/// and cannot miscount the fake's row — are all asserted.
///
/// # The 2 s bound catches a wait that FINISHED, and nothing more
///
/// Stated rather than implied: a verb that slept for the scripted hour would hang this suite for an
/// hour rather than fail at two seconds. `std::thread::sleep` is clippy-banned workspace-wide, and
/// golden constraint 2 (*"`run` never waits"*) is what actually holds it; this assertion is the
/// cheap corroboration, not the guarantee.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — s7d §7d.15's form"
)]
fn ac404_rate_limited_writes_retry_mark_and_exits() {
    const RESET: i64 = crate::support::FIXED_NOW + 3600;

    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
    let inputs = crate::support::inputs_over_pack_min(&resolved).with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::supplied(
            "emit-report@1",
            crate::support::any_activity_spec(),
        ),
    );

    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Err(
        rk_types::error::StepError::RateLimited {
            resets_at: Some(rk_types::time::Timestamp::from_unix_seconds(RESET)),
        },
    )]));
    let ports = crate::support::ports(
        Arc::clone(&harness) as Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let token = CancelToken::new();
    let mut supply = crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());

    let started = std::time::Instant::now();
    let result = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token);
    let elapsed = started.elapsed();

    // (1) the return, THEN (2) the rest — root.rs's own order.
    let exit = result.expect("a rate limit is an EXIT, not a halt");
    assert_eq!(
        exit,
        rk_cli::exit::Exit::RateLimited,
        "a rate-limited dispatch is exit 15, not {exit:?}"
    );
    assert_eq!(exit.code(), 15);

    // (2) the port was reached exactly once — a verb that looped back to the reset would dispatch
    // twice, and a verb that returned 15 without dispatching at all would dispatch zero times.
    assert_eq!(
        harness.dispatches(),
        1,
        "the run reached the harness {} times, so the exit above is not a reading of a dispatch",
        harness.dispatches()
    );

    // (3)(4) TWO rows of the one kind, in order, with their two distinct payloads.
    let rows = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("bug-1"))
        .expect("the journal reads");
    let rate: Vec<&rk_types::event::Event> = rows
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::RateLimited)
        .collect();
    assert_eq!(
        rate.len(),
        2,
        "expected the fake's failure row AND the kernel's retry mark; found {} `rate_limited` \
         rows: {:?}",
        rate.len(),
        rate.iter().map(|e| &e.payload).collect::<Vec<_>>()
    );
    assert_eq!(
        rate[0].payload,
        serde_json::json!({}),
        "the FIRST `rate_limited` row is the harness's bare failure row"
    );
    assert_eq!(
        rate[1].payload,
        serde_json::json!({ "kind": "retry_at", "resets_at": RESET }),
        "the SECOND is the kernel's MARK, and it carries the reset the failure named"
    );

    // (5) and as a VALUE, through the reader that filters on the `kind` tag.
    let marks = rk_kernel::runtime::marks::read_retry_at_marks(&rows);
    assert_eq!(
        marks.len(),
        1,
        "AC-404 wants ONE retry_at mark carrying the failure's reset; found {marks:?}"
    );

    // (6) THE ORDER, which two counters cannot express: the dispatch, then the failure, then the
    //     mark. A mark written before the dispatch is not a mark about it.
    let appends: Vec<rk_types::event_kind::EventKind> = trace
        .steps()
        .into_iter()
        .filter_map(|s| match s {
            crate::support::Step::Append(k) => Some(k),
            crate::support::Step::Query | crate::support::Step::Reconcile(_) => None,
        })
        .collect();
    let tail = &appends[appends.len().saturating_sub(3)..];
    assert_eq!(
        tail,
        [
            rk_types::event_kind::EventKind::Dispatch,
            rk_types::event_kind::EventKind::RateLimited,
            rk_types::event_kind::EventKind::RateLimited,
        ],
        "the last three appends are not dispatch → failure → mark; the whole sequence was \
         {appends:?}"
    );

    // (7) it EXITED rather than waiting out the scripted hour.
    assert!(
        elapsed < std::time::Duration::from_secs(2),
        "the verb took {elapsed:?} and the scripted reset is an hour out, so `run` waited for it"
    );

    // ---- (8) THE CONTROL ARM, and it is not optional. -------------------------------------------
    //
    // Everything above is satisfied by a step (10) that returned 15 for EVERY error. This arm
    // scripts a different class over the same fixture and asserts the verb does NOT say 15 — so
    // the row above is a mapping rather than a constant.
    let repo2 = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved2 = crate::support::resolved_over_pack_min(repo2.path(), "bug-1");
    let inputs2 = crate::support::inputs_over_pack_min(&resolved2).with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::supplied(
            "emit-report@1",
            crate::support::any_activity_spec(),
        ),
    );
    let trace2 = crate::support::Trace::new();
    let store2 = crate::support::counting_store(&trace2);
    let harness2 = Arc::new(crate::support::ScriptedHarness::scripted(vec![Err(
        rk_types::error::StepError::NonZero {
            code: Some(1),
            stderr_digest: rk_types::digest::sha256(b""),
        },
    )]));
    let ports2 = crate::support::ports(
        Arc::clone(&harness2) as Arc<dyn rk_ports::harness::Harness>,
        &store2,
    );
    let mut supply2 =
        crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());
    let control_exit =
        rk_cli::verbs::run::run_with(&resolved2, &inputs2, ports2, &mut supply2, &token)
            .expect("a NonZero failure is an EXIT, not a halt");
    assert_ne!(
        control_exit,
        rk_cli::exit::Exit::RateLimited,
        "the verb returned 15 for a NonZero script too, so the 15 above is a constant rather than \
         a mapping"
    );
    let control_rows = store2
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("bug-1"))
        .expect("the journal reads");
    assert!(
        rk_kernel::runtime::marks::read_retry_at_marks(&control_rows).is_empty(),
        "a NonZero failure wrote a retry_at mark, so the mark above is not about the rate limit"
    );
}

/// **AC-485 — the adapter set is resolved ONCE and is RUN-SCOPED.** A configuration rewritten
/// mid-run does not reach the run that is already going.
///
/// §7d.5: *"Two-activity pack; `tests/it/support.rs::MutatingHarness` rewrites `config.toml`'s
/// `harness` to `rk-adapter-linear` on its second `dispatch` before delegating to
/// `FakeHarness::scripted`. **The discriminating assertions are the seams**: the `ConfigFile`
/// reader called **exactly once**, `AdapterSet::resolve` **exactly once**."*
///
/// # 🛑 WHY THE SEAMS AND NOT THE OUTCOME
///
/// `rk-adapter-linear` is not a real adapter, so a run that DID re-read the file mid-flight would
/// refuse on the unknown name — and a test asserting only *"the run succeeded"* would pass against
/// a root that re-read the file **once more after the run**, or that never read it at all. The
/// counters are what make "exactly once" a reading. They are injected through `Startup`, which is
/// the seam §7d.5 calls *"counting seams the verb function takes"*.
///
/// # 🛑 WHICH HALF IS THE INSTRUMENT, AND WHICH IS A FLOOR AT THIS CYCLE
///
/// **The PREMISE is the instrument.** Measured against this test's declared mutant (a second
/// `startup.config.read(..)` inside `through_step_seven`): the assertion that fires is the
/// premise — *"they read 2 and resolved 1"* — and not either post-run count.
///
/// That is because the counting seams travel on `Startup`, and **`Startup` does not travel past
/// `through_step_seven`**: `run_with` never receives it. So at this cycle no implementation
/// reachable from `run_with` could re-read the configuration, and the two post-run counts cannot
/// fail. They are written because §7d.5 names them, and because they become instruments the moment
/// cycle 6 wires the outer path — but they are FLOORS here, and their messages say so rather than
/// leaving a reader to assume they discriminate.
///
/// What does discriminate beside the premise: the run COMPLETING over an adapter set resolved
/// before the rewrite. `rk-adapter-linear` is not a real adapter, so a root that re-read the file
/// anywhere in the loop would refuse with exit 3.
///
/// # 🛑 THE REWRITE IS ASSERTED TO HAVE HAPPENED
///
/// The whole test is an absence — *the new bytes did not reach this run* — and an absence over a
/// rewrite that never occurred is a guard that cannot fail. So the file's bytes are read back at
/// the end and asserted to carry `rk-adapter-linear`.
#[test]
fn ac485_adapter_set_resolved_once_and_run_scoped() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::TwoActivities);
    let root = repo.path();
    let config_path = root.join(".redkiln").join("config.toml");

    let trace = crate::support::Trace::new();
    let reader = crate::support::CountingConfigReader::new();
    let resolver = crate::support::CountingResolver::new();
    let reconciler = crate::support::RecordingReconciler::new(&trace);

    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::RunArgs {
        item: "story-1".to_owned(),
        lane: None,
    };
    let env = rk_cli::env::Env::from_map(std::collections::BTreeMap::new());
    let startup = rk_cli::verbs::Startup {
        reconciler: &reconciler,
        config: &reader,
        adapters: &resolver,
    };
    let resolved = rk_cli::verbs::run::through_step_seven(&cli, &args, &env, startup)
        .expect("steps (1)-(7) resolve over the two-activity fixture");

    // PREMISE: steps (1)-(7) read the file once and resolved once. Without this the post-run
    // counts below could not distinguish "read once, before the run" from "never read".
    assert_eq!(
        (reader.reads(), resolver.resolves()),
        (1, 1),
        "steps (1)-(7) did not read the configuration exactly once and resolve exactly once; they \
         read {} and resolved {}",
        reader.reads(),
        resolver.resolves()
    );

    let inputs = crate::support::inputs_over_pack_min(&resolved)
        .with_spec(
            "emit-report",
            rk_kernel::replay::RecordedSpec::supplied(
                "emit-report@1",
                crate::support::any_activity_spec(),
            ),
        )
        .with_spec(
            "second-activity",
            rk_kernel::replay::RecordedSpec::supplied(
                "second-activity@1",
                crate::support::any_activity_spec(),
            ),
        );

    let store = crate::support::counting_store(&trace);
    // The harness rewrites `config.toml`'s `harness` to `rk-adapter-linear` on its SECOND dispatch,
    // i.e. WHILE this run is still going. Two script elements, one per node.
    let harness = std::sync::Arc::new(crate::support::MutatingHarness::new(
        &config_path,
        vec![
            Ok(crate::support::impl_report()),
            Ok(crate::support::impl_report()),
        ],
    ));
    let ports = crate::support::ports(
        std::sync::Arc::clone(&harness) as std::sync::Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let token = rk_ports::cancel::CancelToken::new();
    let mut supply = crate::support::no_spec();

    let exit = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token)
        .expect("a sound lock is not a halt");

    // The run COMPLETED, over an adapter set resolved before the rewrite. A run that re-read the
    // file would have met `rk-adapter-linear` and refused with exit 3.
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "the run did not complete ({exit:?}); if it refused on `rk-adapter-linear` then the \
         rewritten configuration reached a run already in flight"
    );
    assert_eq!(
        harness.dispatches(),
        2,
        "the harness was reached {} times; the rewrite happens on the SECOND dispatch, so fewer \
         than two means the rewrite never occurred and the absence below is vacuous",
        harness.dispatches()
    );

    // THE DISCRIMINATING ASSERTIONS: still once each, AFTER the rewrite.
    assert_eq!(
        reader.reads(),
        1,
        "FLOOR (see this test's own note): the `ConfigFile` reader was called {} times AFTER the \
         run. `Startup` does not travel past `through_step_seven`, so nothing in the loop could \
         have called it; this becomes an instrument when cycle 6 wires the outer path",
        reader.reads()
    );
    assert_eq!(
        resolver.resolves(),
        1,
        "FLOOR (see this test's own note): `AdapterSet::resolve` was called {} times AFTER the run, \
         and the resolver seam is likewise out of the loop's reach at this cycle",
        resolver.resolves()
    );

    // AND THE REWRITE REALLY HAPPENED — otherwise everything above is an absence over nothing.
    let after = std::fs::read_to_string(&config_path).expect("the configuration is readable");
    assert!(
        after.contains("rk-adapter-linear"),
        "the harness did not rewrite the configuration, so the two counts above are a guard that \
         cannot fail. config.toml after the run:\n{after}"
    );
}

/// **H-2 / AC-225 — a signal DURING a dispatch cancels it, and the run exits 13.**
///
/// §7d.10: *"in-process, real `ClaudeHarness` with `harness_binary` → `rk-fixture-child-claude` in
/// `hold` mode; the test fires the token from a thread **after the child's marker file appears**
/// under `RK_FIXTURE_MARKER_DIR` (a store poll would block — the kernel holds the store `Mutex`
/// across `harness.dispatch`)."*
///
/// # 🛑 IN-PROCESS WITH A **REAL** ADAPTER, WHICH IS NOT A CONTRADICTION
///
/// No test may deliver a console signal at 7d — under `cargo test` the binary is not in its own
/// process group, so the event would reach `cargo`, libtest and the shell. The OS-delivery arm is
/// ARMED, not run. What IS run is everything downstream of the token: the harness the test builds
/// is the real one, the child it spawns is the real fixture child, and the only thing standing in
/// for the signal is `token.cancel()` — which is exactly what `signal::on_signal` does and nothing
/// more (`signal_handler_fires_cancel_token` pins that).
///
/// # 🛑 THE TOKEN IS FIRED ON THE MARKER FILE, NEVER ON A STORE POLL
///
/// `Runtime::run_activity` holds the store `Mutex` across `harness.dispatch`, so a watcher thread
/// that polled the store would block until the dispatch it is trying to interrupt had finished.
/// The child's own marker file is outside that lock: `rk-fixture-child` writes `<pid>.marker`
/// before it parks, so its appearance is proof the child is ALIVE AND HOLDING — which is the
/// state AC-225 is about.
///
/// `timeout_secs` stays the pack's 900. A small one would let the adapter's own timer arm fire
/// first and the row would read `cancelled_by: timeout` — §7d.4's named wrong-reason red.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — s7d §7d.15's form for §7d.14's bounds"
)]
fn signal_during_dispatch_is_cancelled_by_signal_exit_13() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let markers = tempfile::tempdir().expect("a marker directory");
    let wt = tempfile::tempdir().expect("the spec's worktree");
    let child = crate::support::fixture_child();

    let resolved = crate::support::resolved_over_pack_min(root, "bug-1");

    // The captured map the adapter hands the child. `RK_FIXTURE_MODE=hold` is what makes the child
    // park instead of returning, which is the only way a dispatch is still running when the token
    // fires. Neither name is on the adapter's env denylist.
    let mut captured = std::collections::BTreeMap::new();
    captured.insert("RK_FIXTURE_MODE".to_owned(), "hold".to_owned());
    captured.insert(
        "RK_FIXTURE_MARKER_DIR".to_owned(),
        markers.path().to_string_lossy().into_owned(),
    );
    let cfg = rk_adapter_claude::config::Config::new(rk_adapter_claude::config::ConfigValues {
        captured_env: captured,
        harness_binary: child,
        prompt_cache_ttl: Some(resolved.extras.cache.prompt_ttl.clone()),
        subagent_prompt_cache_ttl: Some(resolved.extras.cache.subagent_prompt_ttl.clone()),
        margins: rk_adapter_claude::config::Margins {
            five_hour: resolved.extras.margins.five_hour,
            seven_day: resolved.extras.margins.seven_day,
        },
        on_version_mismatch: rk_adapter_claude::config::MismatchPolicy::Refuse,
    })
    .expect("the fixture harness configuration constructs");

    // Bound through the REAL binder: it is what fills the card store `dispatch` reads, and an
    // unbound `card_digest` is a pre-spawn `Transport { Spawn }` rather than a cancellation.
    // Read off the PACK rather than written as literals, so a pack edit moves this test with it.
    let card_text =
        std::fs::read_to_string(root.join(".redkiln").join("cards").join("implementer.json"))
            .expect("the fixture card is installed");
    let card = rk_adapter_claude::card::AuthoredCard::from_file("implementer", &card_text)
        .expect("the fixture card loads");
    let schema_json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            root.join(".redkiln")
                .join("schemas")
                .join("ImplReport.json"),
        )
        .expect("the fixture schema is installed"),
    )
    .expect("the schema is JSON");
    let declared = resolved
        .extras
        .role
        .get("implementer")
        .expect("pack-min declares [role.implementer]");
    let role = rk_adapter_claude::binder::Role {
        model: declared.model,
        effort: declared.effort,
    };
    let timeout_secs = resolved
        .extras
        .activity
        .get("emit-report")
        .expect("pack-min declares [activity.emit-report]")
        .timeout_secs;
    let mut binder = rk_adapter_claude::binder::Binder::new(cfg);
    let spec = binder
        .bind(rk_adapter_claude::binder::BindRequest {
            role_name: "implementer",
            role: &role,
            card: &card,
            schema: &rk_adapter_claude::binder::Schema {
                id: rk_types::typed::SchemaId::new("ImplReport"),
                json: schema_json,
            },
            timeout_secs,
            worktree: wt.path(),
            inputs: vec![("item".to_owned(), serde_json::json!("bug-1"))],
        })
        .expect("pack-min's one card binds");

    let token = CancelToken::new();
    let harness = rk_adapter_claude::ClaudeHarness::new(binder.into_config(), token.clone())
        .expect("the harness constructs");
    let store = crate::support::counting_store(&crate::support::Trace::new());
    let ports = crate::support::ports(
        Arc::new(harness) as Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let inputs = crate::support::inputs_over_pack_min(&resolved).with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::supplied("emit-report@1", spec),
    );

    // The watcher. A channel nobody sends on is the wait — `std::thread::sleep` is clippy-banned
    // workspace-wide, and `recv_timeout` reads no clock in our code.
    let marker_dir = markers.path().to_path_buf();
    let firing = token.clone();
    let watcher = std::thread::spawn(move || -> Option<u32> {
        let (_tx, rx) = std::sync::mpsc::channel::<()>();
        let started = std::time::Instant::now();
        while started.elapsed() < std::time::Duration::from_secs(30) {
            if let Some(pid) = crate::support::marker_pid(&marker_dir) {
                firing.cancel();
                return Some(pid);
            }
            let _ = rx.recv_timeout(std::time::Duration::from_millis(10));
        }
        None
    });

    let mut supply = crate::support::no_spec();
    let exit = rk_cli::verbs::run::run_with(&resolved, &inputs, ports, &mut supply, &token)
        .expect("a sound lock is not a halt");
    let child_pid = watcher.join().expect("the watcher thread does not panic");

    // (0) THE PREMISE: the child really did start and park. Without a marker the token fired on a
    //     timeout rather than on a live dispatch, and everything below is about nothing.
    let child_pid = child_pid.expect(
        "the fixture child wrote no marker within 30 s, so the token was never fired ON a running \
         dispatch and this test measured a bound rather than a cancellation",
    );

    // (1) THE EXIT.
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Cancelled,
        "a dispatch cancelled by a signal is exit 13, not {exit:?}"
    );
    assert_eq!(exit.code(), 13);

    // (2) THE ROWS, read off the store the run wrote.
    let rows = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("bug-1"))
        .expect("the journal reads");
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::Dispatch)
            .count(),
        1,
        "exactly one `dispatch` row is owed — the cancellation is OF a dispatch. Rows: {:?}",
        rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    let completed: Vec<&rk_types::event::Event> = rows
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::DispatchCompleted)
        .collect();
    assert_eq!(
        completed.len(),
        1,
        "the adapter records every path (`finish()`), so exactly one `dispatch_completed` row is \
         owed. Rows: {:?}",
        rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert_eq!(
        completed[0].payload.get("cancelled_by"),
        Some(&serde_json::json!("signal")),
        "the `dispatch_completed` row does not carry `cancelled_by: signal`. A `timeout` here \
         means the adapter's own timer arm fired first and the cancellation was not the token's \
         (§7d.4's named wrong-reason red); a null means nothing cancelled it at all"
    );

    // (3) THE CHILD IS DEAD. Read through the pid the CHILD wrote into its own marker, so this is
    //     a reading of the process the run killed rather than a number the run reported.
    assert!(
        !crate::support::pid_is_alive(child_pid),
        "the fixture child (pid {child_pid}) survived the cancellation. ADR-0099's containment is \
         what makes a cancelled dispatch leave nothing behind"
    );
}
