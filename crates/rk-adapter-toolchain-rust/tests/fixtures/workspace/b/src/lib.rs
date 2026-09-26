//! Package `b` of the workspace fixture (s8c §8c.3): depends on `a`.

/// Quadruple a number through `a`.
#[must_use]
pub const fn quadruple(x: u32) -> u32 {
    a::double(a::double(x))
}

#[cfg(test)]
mod tests {
    #[test]
    fn quadruple_quadruples() {
        assert_eq!(super::quadruple(2), 8);
    }
}
