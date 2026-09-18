//! POSITIVE CONTROL for the AC-211 append-flag ban (s7c §7c.21): BOTH flags on non-comment lines. Not compiled.
pub fn control(argv: &mut Vec<String>) {
    argv.push("--append-system-prompt".to_owned());
    argv.push("--exclude-dynamic-system-prompt-sections".to_owned());
}
