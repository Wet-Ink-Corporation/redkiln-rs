//! **The `redkiln` binary** — a thin caller, and deliberately nothing else.
//!
//! Parse, install the two handlers, dispatch, exit through [`rk_cli::code_for`]. Everything the
//! tests assert on lives in the library beside this file, because an integration test cannot link a
//! `[[bin]]` (ADR-0083) and a surface asserted in one place and shipped from another agrees only by
//! inspection.
//!
//! **The two `std::process::exit` sites are both here, and as of Slice 2 there are two.** The table
//! exit at the end of `main`, and the panic hook's `70`. Checkpoint 7d counts exactly two and
//! requires both to be in this file; a third — written, say, to render the halt separately — is what
//! that count exists to refuse (§7d.4).
//!
//! **Why the hook exists at all.** `rk-cli` acquires the store lock through one function that
//! returns `Err(Halt)`, so a poison IT observes never panics. But the kernel acquires the same
//! `Arc<Mutex<…>>` itself, with `.lock().expect(LOCK_HALTS)` — so a poison first observed inside
//! `rk-kernel` unwinds out through a panic this binary did not raise. Without the hook that leaves
//! the process at Rust's default `101`, a code the exit table does not describe, for the one
//! condition ADR-0118 is entirely about. The hook turns exactly that payload into `70` and leaves
//! every other panic alone.

use std::io::Write as _;

use rk_cli::exit::Halt;

fn main() {
    let cli = rk_cli::cli::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit());

    // The signal path (ADR-0099 decision 2; D18-2). `ctrlc::set_handler` may be called once per
    // process, so this is its only call site in the workspace — no test installs one.
    //
    // ~~The root's own copy of the token is dropped here … the flag's readers … are Slice 3's.~~
    // *(dated 2026-10-17, s8j build, package S8J-P1-CANCEL; issue #5.)* Slice 3 built the readers
    // but `run` built a fresh token for them, so the handler fired a token nobody read. The token
    // `install()` returns is now handed to `verbs::dispatch` below, which hands it to `run`: the
    // adapter each stage builds clones it (exit `13` during a dispatch) and the step-boundary check
    // reads it (exit `130` outside one). If the handler cannot be installed, the run gets a token
    // no handler holds, which is what every run had before this fix, and the OS default applies.
    let token = match rk_cli::signal::install() {
        Ok(token) => token,
        Err(e) => {
            let warning = writeln!(
                std::io::stderr().lock(),
                "redkiln: the signal handler could not be installed: {e}"
            );
            if warning.is_err() {
                // stderr is closed, so the warning has no channel left. The run proceeds on the
                // same unwired token either way; there is nothing else to tell.
            }
            rk_ports::cancel::CancelToken::new()
        }
    };

    // The panic hook. It CHAINS to the previous hook rather than replacing it, so a panic that is
    // not a poisoned store lock keeps Rust's own message and its `101` — the hook narrows one case
    // and changes nothing else. It is not `catch_unwind` and it recovers nothing (clause 3).
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if rk_cli::is_lock_halt(info.payload()) {
            eprintln!(
                "{}",
                rk_cli::render_halt(&Halt::PoisonedLock {
                    lock: "store",
                    verb: "the run",
                })
            );
            std::process::exit(rk_cli::code_for(&Err(Halt::PoisonedLock {
                lock: "store",
                verb: "the run",
            })));
        }
        previous(info);
    }));

    // §7d.5 step (3): THE ONE ENVIRONMENT READ, performed here at the composition root and handed
    // down as a value. No verb calls `Env::capture()` — §7d.5's AC-486 bullet: "no test calls
    // `Env::capture()`; the captured-map inputs of every `resolve` … are values the test builds".
    let env = rk_cli::env::Env::capture();
    let result = rk_cli::verbs::dispatch(&cli, &env, &token);
    if let Err(halt) = &result {
        eprintln!("{}", rk_cli::render_halt(halt));
    }
    std::process::exit(rk_cli::code_for(&result));
}
