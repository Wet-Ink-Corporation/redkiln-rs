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
// [amended 2026-09-24, s8a, ADR-0165] The premise moved: the type is now an enum whose `Text` arm
// a Toolchain implementor may construct, so a value CAN exist. The tripwire's reason is unchanged
// and stronger: redkiln-migrate may not depend on rk-ports at all (AC-466), so the identifier can
// reach the migrator only by someone re-declaring it, which Checkpoint 5's ban now refuses outside
// rk-ports as well. The token line below is unchanged.
//
// 🛑 THE TOKEN IS A STRING LITERAL, NOT A TYPE POSITION, AND THAT IS LOAD-BEARING
//
// Checkpoint 5 bans the ChangeSummary TYPE in a CODE position anywhere under `crates/` outside
// `rk-ports` (ADR-0099 clause 5, blocked on S-BIN) and strips string literals before matching.
// [amended 2026-09-24, s8a, ADR-0165: the ban now names what the record forbids -- a definition,
// alias, impl or renaming import outside rk-ports -- and admits an implementor's import and
// signature; a string literal is still blanked, so the literal below still satisfies both.] This
// control first read `-> ChangeSummary` / `ChangeSummary::default()`, which made the two checkpoints
// mutually unsatisfiable: Checkpoint 7d demands the token on a non-comment line here, Checkpoint 5
// forbids a code-position use of it. A string literal satisfies both — and it is the FAITHFUL
// control either way, because the ban it exists for (`rk_absent '\bChangeSummary\b'` over the
// migrator's `src`) is a plain text search that strips nothing, so a literal exercises exactly the
// pattern that must fire. [Found 2026-09-21 by Checkpoint 5's re-run at Step 7d's close.]

fn a_synthesised_change_summary() -> String {
    String::from("ChangeSummary")
}
