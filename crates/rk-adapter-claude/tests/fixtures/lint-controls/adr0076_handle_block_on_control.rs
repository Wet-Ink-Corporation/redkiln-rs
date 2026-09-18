//! POSITIVE CONTROL for the `Handle::block_on|current_thread` ban (ADR-0076; s7c §7c.21). Not compiled.
pub fn control(handle: &tokio::runtime::Handle) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    let _ = (rt, tokio::runtime::Handle::block_on(handle, async {}));
}
