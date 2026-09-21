// POSITIVE CONTROL for AC-475's tripwire — no ChangeSummary value or identifier in the migrator (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. The ban it controls is scoped to `crates/redkiln-migrate/src`, and `ac475_zero_change_summary_values_in_output` runs its in-process scanner over this file first.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.
//
// AC-475's second conjunct is VACUOUS while the type does not exist: `rk_ports::toolchain::
// ChangeSummary` has no decided contents and no `Ok` is constructible until S-BIN runs. This
// control is the tripwire against the type appearing in the migration path before then.

fn a_synthesised_change_summary() -> ChangeSummary {
    ChangeSummary::default()
}
