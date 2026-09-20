// POSITIVE CONTROL for ADR-0077 — no git spawn outside rk-git's one wrapper (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Every git operation goes through GitExec, which is what makes hooks-disabled, config-pruned
// invocations the only kind this workspace makes.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_bare_git_spawn() -> std::process::Command {
    std::process::Command::new("git")
}
