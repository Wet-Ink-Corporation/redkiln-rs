//! POSITIVE CONTROL for the AC-374 pattern (s7c §7c.8, §7c.21): `f64` AND the rounding path's `as_f64(`. Not compiled.
//! Its path is under lint-controls/, which the module-path filter must EXCLUDE (the filter's own control).
pub fn control(v: &serde_json::Value) -> u16 {
    let x: f64 = v.as_f64().unwrap();
    (x * 100.0).round() as u16
}
