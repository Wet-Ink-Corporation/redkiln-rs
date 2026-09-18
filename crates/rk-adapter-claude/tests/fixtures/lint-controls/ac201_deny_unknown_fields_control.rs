//! POSITIVE CONTROL for the module-filtered `deny_unknown_fields` ban (AC-201; s7c §7c.21). Not compiled.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    pub kind: String,
}
