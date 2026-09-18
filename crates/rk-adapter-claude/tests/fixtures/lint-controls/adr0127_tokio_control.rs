//! POSITIVE CONTROL for the no-tokio-in-rk-containment ban (ADR-0127 d2; s7c §7c.21). Not compiled.
pub fn control() -> tokio::runtime::Handle {
    tokio::runtime::Handle::current()
}
