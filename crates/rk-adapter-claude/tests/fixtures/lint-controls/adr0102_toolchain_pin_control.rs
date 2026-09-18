//! POSITIVE CONTROL for the `+1.98.1` third-copy ban (ADR-0102/ADR-0083; s7c §7c.21). Not compiled.
pub fn control() -> Vec<&'static str> {
    vec!["cargo", "+1.98.1", "test"]
}
