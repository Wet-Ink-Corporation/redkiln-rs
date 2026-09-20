// POSITIVE CONTROL for ADR-0083 — set_var / remove_var forbidden workspace-wide, tests included (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. A test that needs a different environment spawns the binary with a CHILD env; mutating the
// parent's is a hidden input shared by every test in the binary.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_test_that_mutates_its_own_environment() {
    unsafe { std::env::set_var("TZ", "UTC") };
}
