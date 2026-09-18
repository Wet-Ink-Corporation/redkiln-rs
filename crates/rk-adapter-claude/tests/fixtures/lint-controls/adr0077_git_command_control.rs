//! POSITIVE CONTROL for the `Command::new("git")` ban (ADR-0077; s7c §7c.21). Not compiled; ci.yml's git
//! rule excludes this directory by the anchored lint-controls exclusion (RT-13).
pub fn control() -> std::process::Command {
    std::process::Command::new("git")
}
