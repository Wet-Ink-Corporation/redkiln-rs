// POSITIVE CONTROL for ADR-0083 — the process reads its environment ONCE, in rk-cli's env module (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Shared by two bans: rk-cli (with the env module excluded) and redkiln-migrate (with none).
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_second_environment_read() -> Option<String> {
    std::env::var("RK_SOMETHING").ok()
}
