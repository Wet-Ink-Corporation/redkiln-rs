// POSITIVE CONTROL for AC-414 as amended (D-10) / D18-7 — the doctor module constructs no port implementor (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. doctor reports; it does not dispatch. A Harness it constructed would be a live adapter in a
// read-only path.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_doctor_that_constructs(cfg: &Value) -> ClaudeHarness {
    rk_adapter_claude::config::Config::new(cfg).expect("built")
}
