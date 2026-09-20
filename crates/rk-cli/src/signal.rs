//! **The signal path** — ADR-0099 decision 2; **D18-2**; `s7d` §7d.10.
//!
//! One dependency, one handler, one token. `ctrlc` is a dependency of this crate and of no other;
//! `main` installs the handler exactly once (`ctrlc::set_handler` returns `MultipleHandlers` on a
//! second call, so there is no second call anywhere, tests included); and the handler does nothing
//! but fire the token.
//!
//! **Nothing but fire the token, and each of the three exclusions has a reason.** A handler runs on
//! a foreign thread, at an arbitrary point, while the store lock may be held by the thread it
//! interrupted. So it does not write a row (it cannot take the lock without deadlocking, and taking
//! it is not its job); it does not print (stdout may be mid-line, and the diagnostic belongs to
//! whoever decides the exit); and it does not exit (an exit here would skip every `Drop`, including
//! the containment job that kills the child tree).
//!
//! **What the token means is decided elsewhere, and that is the design.** Fired DURING a dispatch,
//! the adapter's polled arm turns it into `Cancelled { by: Signal }` and the run exits `13`. Fired
//! outside one, the root sees it at the next step boundary and exits `130`. The handler knows
//! neither; it sets a flag.
//!
//! **This function is `pub` so a test can call it**, and that is the whole instrument: an
//! integration test cannot install a second handler, and no test signals its own process group —
//! under `cargo test` the test binary is not in its own group, so a console event would reach
//! `cargo`, libtest and the shell running the block (Halt 6's shape). The OS-delivery arm is
//! **ARMED**, not run, and `ctrlc`'s own contract is what stands in for it.

use rk_ports::cancel::CancelToken;

/// What the installed handler does, and all it does.
///
/// `main` wraps this in the closure it hands to `ctrlc::set_handler`; the test calls it directly.
/// Keeping it a named function rather than an inline closure is what makes those two the same code
/// rather than two closures that agree by inspection.
pub fn on_signal(token: &CancelToken) {
    token.cancel();
}

/// Install the handler and return the token it fires.
///
/// The one `ctrlc::set_handler` call site in the workspace. `ctrlc` refuses a second call with
/// `MultipleHandlers`, so a test that installed one would make whichever ran second fail — which is
/// the mechanical reason, on top of the process-group reason, that no test installs one and
/// [`on_signal`] is `pub` instead.
///
/// # Errors
/// `ctrlc::Error` if a handler is already installed or the OS refused.
pub fn install() -> Result<CancelToken, ctrlc::Error> {
    let token = CancelToken::new();
    let handler_token = token.clone();
    ctrlc::set_handler(move || on_signal(&handler_token))?;
    Ok(token)
}
