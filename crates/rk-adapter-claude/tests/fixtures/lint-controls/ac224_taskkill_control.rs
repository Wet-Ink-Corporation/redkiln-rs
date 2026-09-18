//! POSITIVE CONTROL for the `taskkill|kill_tree` ban (AC-224; ADR-0099 d3; s7c §7c.21). Not compiled.
pub fn kill_tree(pid: u32) -> String {
    format!("taskkill /T /F /PID {pid}")
}
