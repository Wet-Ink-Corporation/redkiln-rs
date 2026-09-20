// POSITIVE CONTROL for D-10 / D18-7 — the doctor module imports no port-implementor constructor type (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. The import spelling is banned beside the call spelling, or `use ...::Config;` then `Config::new(`
// would evade a fully-qualified pattern.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

use rk_adapter_claude::config::Config;

fn a_doctor_that_imports(c: Config) -> Config {
    c
}
