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

use crate::{code_lines, lint_controls_dir, src_dir};

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
