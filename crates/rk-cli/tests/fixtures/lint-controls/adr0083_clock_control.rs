// POSITIVE CONTROL for AC-001 / ADR-0083 — the clock is read in rk-cli's clock module only (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Shared by two bans, as the env control is. A decision that reads the clock twice is a function
// of something nobody passed it.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_second_clock_read() -> std::time::SystemTime {
    SystemTime::now()
}
