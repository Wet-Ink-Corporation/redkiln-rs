//! POSITIVE CONTROL for the AC-329 prose-parse ban (s7c §7c.14, §7c.21): the summary-line spelling. Not compiled.
pub fn control(stdout: &str) -> bool {
    stdout.contains("test result: ok. 99 passed; 0 failed")
}
