//! **The `redkiln` binary** — a thin caller, and deliberately nothing else.
//!
//! Parse, dispatch, exit through [`rk_cli::code_for`]. Everything the tests assert on lives in the
//! library beside this file, because an integration test cannot link a `[[bin]]` (ADR-0083) and a
//! surface asserted in one place and shipped from another agrees only by inspection.
//!
//! **One `std::process::exit` site at this slice, and it is below.** §7d.4 allows exactly two, both
//! in this file: the table exit here, and the panic hook's, which arrives at Slice 2 when the
//! kernel's `LOCK_HALTS` becomes reachable and a poison first observed inside the kernel has to be
//! turned into an explicit `70` rather than the unhandled panic's `101`. Checkpoint 7d counts two,
//! so its count is first satisfiable at Slice 2 and is absent-shaped here.
//!
//! **The table exit and the halt arm are ONE call.** Reaching for a second `std::process::exit` to
//! render the halt is what the count exists to refuse (§7d.4), so the diagnostic is printed and the
//! same `code_for` carries both outcomes out.

fn main() {
    let cli = rk_cli::cli::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit());
    let result = rk_cli::verbs::dispatch(&cli);
    if let Err(halt) = &result {
        eprintln!("{}", rk_cli::render_halt(halt));
    }
    std::process::exit(rk_cli::code_for(&result));
}
