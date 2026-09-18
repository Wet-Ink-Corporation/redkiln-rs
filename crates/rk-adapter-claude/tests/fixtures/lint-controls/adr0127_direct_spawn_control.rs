//! POSITIVE CONTROL for the direct-spawn ban (ADR-0127 forbids; s7c §7c.21): the zero-argument call form. Not compiled.
pub fn control(cmd: &mut std::process::Command) {
    let _ = cmd.spawn();
}
