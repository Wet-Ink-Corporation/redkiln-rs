//! Package `c`'s integration test: reaches `b` through the dev-dependency edge, and `a` through `b`.

#[test]
fn c_sees_b_through_its_dev_edge() {
    assert_eq!(b::quadruple(1), 4);
}
