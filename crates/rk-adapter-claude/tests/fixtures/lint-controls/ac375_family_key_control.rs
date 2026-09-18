//! POSITIVE CONTROL for the family-key / cache-headroom ban (AC-375/AC-381; s7c §7c.21). Not compiled.
pub struct Control {
    pub seven_day_opus: u16,
    pub headroom_saved: u16,
}
