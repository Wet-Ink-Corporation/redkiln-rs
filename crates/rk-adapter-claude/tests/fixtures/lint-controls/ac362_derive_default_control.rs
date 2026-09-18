//! POSITIVE CONTROL for the derive-Default walk (`dd_hits`, s7c §7c.21; AC-362/AC-377). Not compiled.
//! Two structs, so the walk is controlled on both layouts it must read: the derive on its own line directly
//! above `pub struct Usage {}`, and a derive separated from `pub struct WindowReading {}` by a plain `//`
//! line (a walk that stopped at an ordinary comment missed the second — the third review). Plus the impl form.
#[derive(Debug, Default)]
pub struct Usage {}

#[derive(Debug, Default)]
// SYNTHETIC: absent stays None
pub struct WindowReading {}

pub struct CacheCreation {}

impl Default for CacheCreation {
    fn default() -> Self {
        Self {}
    }
}
