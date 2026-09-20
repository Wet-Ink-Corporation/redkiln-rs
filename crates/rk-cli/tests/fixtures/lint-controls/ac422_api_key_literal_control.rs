// POSITIVE CONTROL for AC-422 / AC-488's one-list rule (`s7d` §7d.13).
//
// Never compiled, never linked. It exists so `ac422_config_reads_the_one_denylist` and Checkpoint
// 7d's `rk_absent '"ANTHROPIC_API_KEY"'` line can show the scanner they share FIRES before either
// asserts that `crates/rk-cli/src` carries no such literal. There is exactly ONE list of credential
// names in this workspace, `rk_adapter_claude::env_denylist`, and the whole point of the ban is that
// a second one — even a correct second one — is a list that can drift.
//
// The literal below is on a NON-COMMENT line, deliberately: both scanners drop comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_second_list_that_must_not_exist() -> &'static [&'static str] {
    &["ANTHROPIC_API_KEY", "ANTHROPIC_AUTH_TOKEN"]
}
