//! POSITIVE CONTROL for the `pnpm test` ban (DECISIONS-2026-09-09 #34; s7c §7c.21). Not compiled.
pub fn control() -> Vec<&'static str> {
    vec!["pnpm test", "--", "--reporter=json"]
}
