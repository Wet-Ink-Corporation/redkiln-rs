//! The four grammar rules ADR-0103 decisions 1 and 2 fix, each under its own name.
//!
//! Each is red against a different plausible parser: one that reads `and`/`or` at equal
//! precedence, one that puts `not` above the comparison instead of below it, one that reads an
//! unquoted identifier in value position as a field, one that lets comparisons chain, and one that
//! admits a bare identifier where ADR-0040's `${…}` form is required.
//!
//! **Two of those are invisible to the corpus and one of them was invisible to this file.** F42
//! measured mixed unparenthesised connectives at **0/287** and the feature census carries no `not`
//! key at all, so the differential in [`crate::probe_differential`] is green under either
//! precedence for either operator. Every case below is therefore authored, and the two that carry
//! the whole rule are written out: **`or` before `and`** (a one-level left-associative parser
//! renders `and`-before-`or` identically and passed this test until 2026-09-13), and **`not`
//! before a comparison** (ADR-0103 decision 1 puts `not` tightest, so `not a == b` is
//! `(not a) == b`).

use rk_kernel::predicate::{CmpOp, Expr, Literal, ParseErrorKind, mixed_unparenthesised, parse};

use crate::support::{ok_parse, rhs};

/// ADR-0103 decision 1: `not` > comparison/`in` > `and` > `or`, left-associative, parentheses
/// override.
///
/// **0 of 287 live predicates depend on this** [measured, F42], so every case here is authored.
/// That is the finding, not a gap: the rule settles Phase-1 authoring, and a parser with the wrong
/// precedence would have passed a corpus-only check.
#[test]
fn grammar_precedence_and_associativity() {
    // `not a and b or c` parses as `((not a) and b) or c` — the ADR's own worked example.
    let p = ok_parse("not a and b or c");
    let Expr::Or(left, right) = &p.expr else {
        panic!("`or` must be the loosest operator, got {:?}", p.expr);
    };
    assert_eq!(**right, Expr::Path("c".to_owned()));
    let Expr::And(and_l, and_r) = left.as_ref() else {
        panic!("`and` must bind tighter than `or`, got {left:?}");
    };
    assert_eq!(**and_r, Expr::Path("b".to_owned()));
    assert_eq!(**and_l, Expr::Not(Box::new(Expr::Path("a".to_owned()))));

    // THE REVERSE ORDER, which the two cases above cannot see. Both of them write `and` BEFORE
    // `or`, and for those two strings a parser reading both connectives at ONE level,
    // left-associative, produces the identical tree — so neither discriminates the precedence this
    // test is named for. The corpus cannot arbitrate either: F42's mixed-unparenthesised census is
    // 0/287. Writing `or` first is what separates them.
    let p = ok_parse("a or b and c");
    let Expr::Or(l, r) = &p.expr else {
        panic!(
            "`or` must be the LOOSEST operator: `a or b and c` is `a or (b and c)`, got {:?}",
            p.expr
        );
    };
    assert_eq!(**l, Expr::Path("a".to_owned()), "the left of `or` is bare");
    assert!(
        matches!(r.as_ref(), Expr::And(..)),
        "`and` binds tighter, so it is the RIGHT child of the `or`, got {r:?}"
    );
    // And with both connectives twice, which a one-level parser renders as a left spine.
    let p = ok_parse("a and b or c and d");
    let Expr::Or(l, r) = &p.expr else {
        panic!(
            "`or` is the spine of `a and b or c and d`, got {:?}",
            p.expr
        );
    };
    assert!(matches!(l.as_ref(), Expr::And(..)), "left conjunct: {l:?}");
    assert!(matches!(r.as_ref(), Expr::And(..)), "right conjunct: {r:?}");

    // `not` is the TIGHTEST operator — tighter than the comparison, which is ADR-0103 decision 1's
    // own ordering (*"tightest first: `not` (unary) > comparison and membership … > `and` > `or`"*)
    // and whose forbids clause is *"Forbids a parser or lint implementing any precedence but clause
    // 1"*. So `not a == b` is `(not a) == b`: the unary binds its OPERAND, never the whole
    // comparison. A descent spelled `not_expr := "not" not_expr | comparison` — Python's
    // precedence, and `spikes/p9/grammar-probe.py`:95-98's — gives `Not(Cmp(..))` and is the
    // reverse of the record. **The corpus cannot arbitrate**: its feature census carries no `not`
    // key at all, 0 of 287 [measured 2026-09-13], so the differential is green either way and
    // ADR-0092 decision 2 rule 1 makes the accepted ADR the authority over the probe.
    let p = ok_parse("not a == b");
    let Expr::Cmp { op, left, right } = &p.expr else {
        panic!(
            "`not` binds TIGHTER than `==` (ADR-0103 decision 1), so the comparison is the root: \
             got {:?}",
            p.expr
        );
    };
    assert_eq!(*op, CmpOp::Eq);
    assert_eq!(
        **left,
        Expr::Not(Box::new(Expr::Path("a".to_owned()))),
        "the unary took its operand and nothing more"
    );
    assert_eq!(**right, Expr::Lit(Literal::Bare("b".to_owned())));
    // Parenthesising is how the other reading is written, and it still parses.
    let p = ok_parse("not (a == b)");
    let Expr::Not(inner) = &p.expr else {
        panic!("got {:?}", p.expr);
    };
    assert!(matches!(inner.as_ref(), Expr::Group(_)));

    // LEFT associativity, which a right-associative parser gets wrong while still producing a tree
    // that evaluates identically for `and`/`or` — so the shape is asserted, not the truth value.
    let p = ok_parse("a and b and c");
    let Expr::And(l, r) = &p.expr else {
        panic!("expected an `and` spine");
    };
    assert_eq!(
        **r,
        Expr::Path("c".to_owned()),
        "the LAST conjunct is the right child"
    );
    assert!(
        matches!(l.as_ref(), Expr::And(..)),
        "left-associative: `(a and b) and c`, got {l:?}"
    );
    let p = ok_parse("a or b or c");
    let Expr::Or(l, _) = &p.expr else {
        panic!("expected an `or` spine");
    };
    assert!(matches!(l.as_ref(), Expr::Or(..)), "left-associative `or`");

    // Parentheses override, and the group node SURVIVES — F42's precedence-dependence census is
    // defined over it, so a parser that flattens parentheses erases the measurement.
    let p = ok_parse("not (a and b) or c");
    assert!(
        !mixed_unparenthesised(&p.expr),
        "a parenthesised mix is not a mixed one"
    );
    let unparenthesised = ok_parse("a and b or c");
    assert!(
        mixed_unparenthesised(&unparenthesised.expr),
        "an `and` directly under an `or` IS precedence-dependent — the census F42 measured at 0/287"
    );

    // The census counts the connectives the way the probe does.
    assert_eq!(unparenthesised.census.get("and"), 1);
    assert_eq!(unparenthesised.census.get("or"), 1);
    assert_eq!(ok_parse("not a and b or c").census.get("not"), 1);
    assert_eq!(ok_parse("not (a and b) or c").census.get("parens"), 1);
}

/// ADR-0103 decision 2: in **value** position an unquoted identifier is a **literal**.
///
/// This is the load-bearing clause — **191** live sites read one way only because of it [measured,
/// F42] — and it is forbidden to read it the other way: *"Forbids an unquoted identifier in value
/// position being read as a field."*
#[test]
fn grammar_bare_identifier_is_a_literal() {
    let p = ok_parse("kind == project");
    assert_eq!(
        *rhs(&p.expr),
        Expr::Lit(Literal::Bare("project".to_owned())),
        "`project` on the RIGHT of a comparison is a value, never a field"
    );
    assert_eq!(p.census.get("bare_literal"), 1);

    // The same identifier on the LEFT is a field. One rule, two positions, and the census
    // distinguishes them: a parser that read both as literals reports bare_literal 2.
    let Expr::Cmp { left, .. } = &p.expr else {
        panic!("expected a comparison");
    };
    assert_eq!(**left, Expr::Path("kind".to_owned()));

    // Every element of an `in` list is value position too.
    let p = ok_parse("kind in [incident, bug, project]");
    assert_eq!(p.census.get("bare_literal"), 3);
    assert_eq!(p.census.get("in"), 1);
    let Expr::In { list, .. } = &p.expr else {
        panic!("expected a membership test");
    };
    let Expr::List(items) = list.as_ref() else {
        panic!("expected a literal list");
    };
    assert!(
        items
            .iter()
            .all(|i| matches!(i, Expr::Lit(Literal::Bare(_)))),
        "every element of an `in` list is a value literal, got {items:?}"
    );

    // `true`, `false` and `null` are NOT bare literals: they are the three keywords, and the
    // census keeps them apart. A parser that swept them into `bare_literal` disagrees with the
    // probe on the corpus's 2 `null` sites.
    let p = ok_parse("flag_exists == true");
    assert_eq!(*rhs(&p.expr), Expr::Lit(Literal::True));
    assert_eq!(p.census.get("bare_literal"), 0);
    let p = ok_parse("flag_exists == null");
    assert_eq!(*rhs(&p.expr), Expr::Lit(Literal::Null));
    assert_eq!(p.census.get("null"), 1);
    assert_eq!(p.census.get("bare_literal"), 0);

    // A reserved word in value position is a parse error, never a literal.
    let e = parse("kind == and").expect_err("`and` is reserved");
    assert_eq!(e.kind, ParseErrorKind::KeywordInValuePosition);
}

/// ADR-0103 decision 1's last sentence: *"Comparison operators do not chain (`a < b < c` is a parse
/// error)."*
#[test]
fn grammar_comparisons_do_not_chain() {
    let e = parse("a < b < c").expect_err("`a < b < c` must not parse");
    assert_eq!(
        e.kind,
        ParseErrorKind::Chain,
        "the failure class names chaining"
    );

    for src in ["a == b == c", "a >= b >= c", "a != b == c"] {
        let e = parse(src).expect_err("a chained comparison must not parse");
        assert_eq!(e.kind, ParseErrorKind::Chain, "{src:?}");
    }

    // The parenthesised form IS the way to write it, and it parses — without this the rule is
    // satisfied by a parser that rejects the operator outright.
    let p = ok_parse("(a < b) and (b < c)");
    assert_eq!(p.census.get("parens"), 2);
}

/// ADR-0103 decision 2's last clause: *"Field-to-field comparison requires the `${…}` reference
/// form (ADR-0040)."*
///
/// The rule has teeth only because of the literal rule above it: `size == blast_radius` is not a
/// field-to-field comparison that happens to be spelled loosely — it is a comparison against the
/// **string** `blast_radius`, and an author who meant the facet must write `${…}`.
#[test]
fn grammar_field_to_field_needs_a_ref() {
    let p = ok_parse("size == blast_radius");
    assert_eq!(
        *rhs(&p.expr),
        Expr::Lit(Literal::Bare("blast_radius".to_owned())),
        "a bare identifier on the right is a LITERAL, so this is not a field-to-field comparison"
    );

    let p = ok_parse("size == ${item.blast_radius}");
    assert_eq!(
        *rhs(&p.expr),
        Expr::Ref("${item.blast_radius}".to_owned()),
        "the reference form is the ONLY field-to-field comparison"
    );
    assert_eq!(p.census.get("ref"), 1);
    assert_eq!(p.census.get("bare_literal"), 0);

    // The optional marker and the hyphen-bearing ref segment, both live in the packs: 62 refs, 11
    // of them optional, with step ids such as `green-verify` inside them. The hyphen is one of the
    // two productions ADR-0103 does not yet admit and is IMPLEMENTED-PENDING-RECORD (§7b.6).
    let p = ok_parse("${green-verify.outputs.status?} == pass");
    assert_eq!(p.census.get("ref"), 1);
    assert_eq!(p.census.get("ref_optional"), 1);
    let Expr::Cmp { left, .. } = &p.expr else {
        panic!("expected a comparison");
    };
    assert_eq!(
        **left,
        Expr::Ref("${green-verify.outputs.status?}".to_owned())
    );

    // And the same hyphen OUTSIDE a reference or a quoted string is a lex error — F42's second
    // pack defect, which ADR-0048 requires spelled `'security-vuln'`.
    let e = parse("kind in [incident, security-vuln]").expect_err("a bare hyphen must not lex");
    assert_eq!(e.kind, ParseErrorKind::Lex);
    assert_eq!(e.pos, 27, "the failure's byte position is the probe's");
}
