//! Package `a` of the workspace fixture (s8c §8c.3). One function, one doctest, one passing unit
//! test and one failing unit test that runs only by selection.

/// Double a number.
///
/// ```
/// assert_eq!(a::double(2), 4);
/// ```
#[must_use]
pub const fn double(x: u32) -> u32 {
    x * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn double_doubles() {
        assert_eq!(super::double(3), 6);
    }

    /// Reachable only by selection (`-- --include-ignored` or `-- --ignored red_in_a`): the workspace's
    /// red, in package `a`'s unit-test binary, the first binary cargo runs.
    #[test]
    #[ignore = "reachable only by selection: the workspace's red"]
    fn red_in_a() {
        assert_eq!(super::double(2), 5, "this test is the workspace's red");
    }
}
