// POSITIVE CONTROL for AC-442 — the legacy signature literals live ONLY under redkiln-migrate/src/legacy (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. It is ALSO AC-442's first must-fail instrument: `ac442_lint_flags_legacy_reader_outside_legacy_module` copies this file to a synthetic `crates/rk-store/src/` path in a temp tree and asserts the scan flags it there.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.
//
// This is what "a legacy-format handling site anywhere else fails the build" looks like: a reader
// of the predecessor's frontmatter, in a crate that is not the migrator.

fn a_legacy_reader_outside_the_migrator(frontmatter: &str) -> Option<String> {
    for line in frontmatter.lines() {
        if let Some(value) = line.strip_prefix("template_sig:") {
            return Some(value.trim().to_owned());
        }
        if let Some(value) = line.strip_prefix("rendered_sig:") {
            return Some(value.trim().to_owned());
        }
    }
    None
}
