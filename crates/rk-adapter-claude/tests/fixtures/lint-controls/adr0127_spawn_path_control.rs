//! POSITIVE CONTROL for the `tokio::process::(Command|Child)` type ban (RT-13 as amended; s7c §7c.21). Not compiled.
pub fn control() -> tokio::process::Command {
    tokio::process::Command::new("claude")
}
