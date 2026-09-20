// POSITIVE CONTROL for AC-529 / H-3 / D-18 — neither binary reads the process cwd (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. The common dir is discovered through rk-git's wrapper with a literal ".", which the CHILD git
// resolves against the inherited cwd, so no Rust path names current_dir.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_root_that_reads_its_own_cwd() -> std::path::PathBuf {
    std::env::current_dir().expect("a cwd")
}
