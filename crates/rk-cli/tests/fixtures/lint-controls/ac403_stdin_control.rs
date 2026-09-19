// POSITIVE CONTROL for AC-403's lint half (`s7d` §7d.13, §7d.14).
//
// This file is never compiled and never linked. It exists so that
// `ac403_no_blocking_stdin_read_in_rk_cli` — and Checkpoint 7d's `rk_absent 'io::stdin|stdin\(\)'`
// line, which runs the same pattern from the shell — can show that the scanner they share actually
// FIRES before either one asserts that `crates/rk-cli/src` is clean. An absence ban whose scanner
// has never been observed to find anything is indistinguishable from one that cannot.
//
// The token below is on a NON-COMMENT line, deliberately: both scanners drop comment lines before
// counting, so a control that carried the token only inside this header would prove nothing and the
// control's own assertion would fail.
//
// It is COMMITTED rather than written and deleted (the tests/fixtures/rk-clock-read/ precedent,
// DECISIONS-2026-09-12 E4): a reverted observation is not an assertion.

fn a_verb_that_would_prompt() {
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
}
