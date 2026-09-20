// POSITIVE CONTROL for ADR-0102 d3 — no #[allow( or #![allow( in either new crate (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Every override is #[expect(..., reason)], which goes red when it stops being needed; an #[allow]
// is silent forever.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_silenced_lint() {
    #[allow(clippy::disallowed_types)]
    let _command = std::process::Command::new("git");
}
