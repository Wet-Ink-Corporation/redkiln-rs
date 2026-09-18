//! POSITIVE CONTROL for the poison-recovery ban (ADR-0118; s7c §7c.21): the canonical spelling. Not compiled.
pub fn control(m: &std::sync::Mutex<u32>) -> u32 {
    // The call sits on a line that does not BEGIN with `*`: the ban's comment filter drops lines whose first
    // non-blank character is `*` (a block-comment continuation), and a deref there would hide the control.
    let guard = m.lock().unwrap_or_else(PoisonError::into_inner);
    *guard
}
