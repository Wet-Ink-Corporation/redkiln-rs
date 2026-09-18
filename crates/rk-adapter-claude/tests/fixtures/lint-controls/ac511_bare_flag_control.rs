//! POSITIVE CONTROL for the `"--bare"` ban (AC-511/AC-216; s7c §7c.21): a call site that adds the flag,
//! which `ac511_bare_never_in_argv_lint_with_control` must NAME (file:line). Not compiled.
pub fn control(argv: &mut Vec<String>) {
    argv.push("--bare".to_owned());
}
