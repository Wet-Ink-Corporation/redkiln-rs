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

use rk_cli::exit::Halt;

fn main() {
    let cli = rk_cli::cli::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit());

    // The signal path (ADR-0099 decision 2; D18-2). `ctrlc::set_handler` may be called once per
    // process, so this is its only call site in the workspace — no test installs one.
    //
    // The root's own copy of the token is dropped here, and that is this slice's shape rather than
    // an oversight: the handler holds the clone that matters, and the flag's readers — the adapter
    // it is cloned into at construction, and the step-boundary check that turns it into exit `130`
    // — are Slice 3's, when there is a loop to check it. Carrying it into a verb that ignored it
    // would read as threading it through when nothing does.
    match rk_cli::signal::install() {
        Ok(token) => drop(token),
        Err(e) => eprintln!("redkiln: the signal handler could not be installed: {e}"),
    }

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
    let result = rk_cli::verbs::dispatch(&cli, &env);
    if let Err(halt) = &result {
        eprintln!("{}", rk_cli::render_halt(halt));
    }
    std::process::exit(rk_cli::code_for(&result));
}
