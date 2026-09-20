// POSITIVE CONTROL for AC-474 — no dual-write / compat-write route in either binary (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. There is one write path. A second one behind a flag is a path nobody tests and everybody trusts.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_route_that_must_not_exist(dual_write: bool) -> &'static str {
    if dual_write { "both" } else { "one" }
}
