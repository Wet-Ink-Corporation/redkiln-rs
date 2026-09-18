//! POSITIVE CONTROL for the AC-202 lint half (s7c §7c.4, §7c.21): an isolation reading of `build_args`. Not compiled.
pub fn isolated(argv: &[String]) -> bool {
    argv.iter().any(|a| a == "--strict-mcp-config") || build_args().is_empty()
}
