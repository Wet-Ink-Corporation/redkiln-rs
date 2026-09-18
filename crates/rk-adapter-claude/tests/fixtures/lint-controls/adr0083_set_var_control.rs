//! POSITIVE CONTROL for the `set_var|remove_var` ban (ADR-0083; s7c §7c.21). Not compiled.
pub fn control() {
    std::env::set_var("RK_CONTROL", "1");
}
