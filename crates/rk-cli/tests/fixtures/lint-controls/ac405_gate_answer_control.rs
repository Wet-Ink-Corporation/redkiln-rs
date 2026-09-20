// POSITIVE CONTROL for AC-405 as amended (D-13) — exactly ONE gate::answer call site in rk-cli (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Checked with the COUNT's own pattern, import alternation included, so a broken import branch is
// caught: the control carries a call AND a use line.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

use rk_kernel::workflow::gate::answer;

fn the_one_call_site(state: &State) -> Result<Outcome, GateError> {
    gate::answer(state, gate, presented, tree_hash, verdict)
}
