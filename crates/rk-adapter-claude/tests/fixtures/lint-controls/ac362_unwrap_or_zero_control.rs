//! POSITIVE CONTROL for the AC-362 laundering ban (s7c §7c.21). Not compiled.
pub fn control(v: Option<u64>) -> u64 {
    v.unwrap_or(0)
}
