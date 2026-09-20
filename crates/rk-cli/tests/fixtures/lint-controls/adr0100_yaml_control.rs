// POSITIVE CONTROL for ADR-0100 d1 — rk-cli touches no YAML (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. redkiln-migrate/src/legacy is the ONE permitted YAML reader in this workspace.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_yaml_reader(src: &str) -> Result<Value, Error> {
    yaml_serde::from_str(src)
}
