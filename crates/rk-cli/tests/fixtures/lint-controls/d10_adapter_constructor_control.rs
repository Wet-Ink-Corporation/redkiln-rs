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

// EXTENDED 2026-09-21 at Step 7d Slice 4 cycle 6. Until then this file carried ONE of the pattern's
// alternatives, `rk_adapter_claude::config::Config::new(` -- which also covers the bare
// `Config::new(` arm -- and the shell's `rk_control` was satisfied, because it needs the file to
// match the alternation ONCE. The in-process twin `ac414_doctor_module_names_no_adapter_constructor`
// asks the stronger question, one alternative at a time, and FIVE of the seven had never been seen
// to be findable at all: a ban on a token no control carries is a ban nothing has tested.
//
// Every banned token now appears below on a code line, so each arm of the pattern is shown to bite.

fn a_doctor_that_builds_a_harness() -> ClaudeHarness {
    ClaudeHarness::new(config, token).expect("built")
}

fn a_doctor_that_builds_toolchains() -> (NodeToolchain, RustToolchain) {
    (NodeToolchain::new(cfg), RustToolchain::new(cfg))
}

fn a_doctor_that_opens_the_store(exec: &GitExec, worktree: &WorktreePath) -> SqliteStore {
    SqliteStore::open(exec, worktree).expect("opened")
}

fn a_doctor_that_acquires_a_port(ports: &Ports) -> Arc<dyn Harness> {
    Arc::clone(&ports.harness)
}
