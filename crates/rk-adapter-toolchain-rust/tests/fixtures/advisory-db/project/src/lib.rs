//! The scan fixture's library (s8c §8c.4): it exists so the manifest has a target. It uses `itoa`,
//! the crate the fixture advisories name.

/// Format a number through `itoa`.
#[must_use]
pub fn render(n: u32) -> String {
    itoa::Buffer::new().format(n).to_owned()
}
