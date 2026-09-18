//! POSITIVE CONTROL for the `result\.rate_limit|rate_limit_from_result` ban (AC-219/AC-380; s7c §7c.21). Not compiled.
pub fn rate_limit_from_result(result: &serde_json::Value) -> Option<&serde_json::Value> {
    result.get("rate_limit")
}
pub fn control(result: &Frame) -> bool {
    result.rate_limit.is_some()
}
