//! POSITIVE CONTROL for the process-environment ban (AC-217; s7c §7c.21). Not compiled.
pub fn control() -> Option<String> {
    std::env::var("ANTHROPIC_API_KEY").ok()
}
