//! `grammar_agrees_with_the_s_grammar_probe` — the differential against S-GRAMMAR's own parser.
//!
//! F42 ran `spikes/p9/grammar-probe.py` over every `applicable_if`, `when` and `on_false` string in
//! `spikes/p9/{packs,catalog,catalog-b}`: **386 strings**, of which **287** are predicates.
//! `crates/rk-kernel/tests/fixtures/grammar/s-grammar-probe-verdicts.json` is that probe's
//! **per-string** verdict, captured by importing the probe's own `Parser`, `collect()` and
//! `check_on_false` — never by restating a number.
//!
//! # Four things are compared, not one
//!
//! Two parsers that agree on *whether* a string parses can still disagree about *what it is*, so
//! the differential compares, per string: parse-or-fail; the failure's class and **byte position**;
//! the **feature census**, key for key and count for count; and the mixed-connective verdict. The
//! census is where the literal-versus-field rule shows up — `bare_literal` is **191** across the
//! corpus, and a parser that read those identifiers as fields would parse every one of the 285
//! strings and disagree on all 191.
//!
//! # Five pack defects, not two, and this test names all five
//!
//! F42's title names both halves and the probe reproduces them read-only on this tree: **two parse
//! failures** — `exists(ProblemRecord)` in `packs/incident.toml` (a user function, which ADR-0033
//! admits none of) and a bare hyphenated value in `catalog-b/blameless-postmortem.toml` (which
//! ADR-0048 requires quoting) — **and three `on_false` values outside ADR-0103 clause 6's closed
//! set**, in `catalog-b/api-contract-spec.toml` step\[5\] and
//! `catalog-b/blameless-postmortem.toml` steps \[2\] and \[4\]. **Five.** The `on_false` half is in
//! this file because it is the same probe run and the same corpus; `on_false_closed_set` asserts
//! the rule those three rows violate.
//!
//! # The totals are asserted before the rows
//!
//! A differential over a corpus that moved is not a differential, and the three numbers below are
//! valid **until `s7e` §7e.2 repairs the five defects** (`s7e`:117 expects 287/287 afterwards). If
//! one of them moves, the fixture is re-captured from the probe and the new reading recorded —
//! never widened (golden constraint 9).
//!
//! *(amended 2026-09-24, s8a, ADR-0166: this test is a **historical capture**, and it stays one.
//! `s7e` repaired only `packs/incident.toml`; the other four defects stayed in `spikes/p9/catalog-b/`
//! until `s8a` §8a.4 repaired them, after which the live probe prints `predicates parsed 279/279`
//! (never 287/287: `s7e` had dropped eight `when` predicates). The fixture and the literals here
//! pin F42's measurement at its capture, over the captured verdict file and never over the live
//! corpus, so no assertion moves; the live reading is Checkpoint 7b's.)*

use rk_kernel::predicate::on_false::form_tag;
use rk_kernel::predicate::{classify_on_false, mixed_unparenthesised, parse};
use serde_json::Value as Json;

use crate::support::probe_verdicts;

#[test]
fn grammar_agrees_with_the_s_grammar_probe() {
    let v = probe_verdicts();
    let totals = &v["totals"];

    // F42's pinned denominators, asserted before any row is read.
    assert_eq!(totals["strings"], 386, "S-GRAMMAR's string count moved");
    assert_eq!(totals["predicates"], 287, "the predicate denominator moved");
    assert_eq!(
        totals["parsed"], 285,
        "the probe's parse count moved off F42's 285/287"
    );
    assert_eq!(
        totals["failed"], 2,
        "the probe's failure count moved off F42's 2/287"
    );
    assert_eq!(
        totals["mixed_unparenthesised"], 0,
        "a precedence-dependent predicate now exists; F42 measured 0/287"
    );
    assert_eq!(totals["on_false"], 99, "the on_false denominator moved");
    assert_eq!(
        totals["on_false_in_closed_set"], 96,
        "the on_false count moved off F42's 96/99"
    );
    assert_eq!(
        totals["feature_census"]["bare_literal"], 191,
        "the literal-versus-field rule's site count moved off F42's 191"
    );
    assert_eq!(
        totals["feature_census"]["in_membership"], 11,
        "ADR-0050's set-membership form's site count moved off F42's 11"
    );

    let rows = v["predicates"].as_array().expect("predicates is an array");
    let mut compared = 0usize;
    let mut agreed_parse = 0usize;
    let mut agreed_fail = 0usize;

    for row in rows {
        let site = row["site"].as_str().expect("every row names its site");
        let text = row["text"].as_str().expect("every row carries its string");
        let probe_parses = row["parses"]
            .as_bool()
            .expect("every row carries a verdict");
        let ours = parse(text);
        match (probe_parses, &ours) {
            (true, Ok(parsed)) => {
                // The census, key for key. A missing key and a zero count are different things:
                // the probe's Counter only materialises a key it has incremented.
                let expected = row["features"]
                    .as_object()
                    .expect("a parsed row has features");
                let mut ours_keys: Vec<String> =
                    parsed.census.keys().map(ToOwned::to_owned).collect();
                ours_keys.sort();
                let mut probe_keys: Vec<String> = expected.keys().cloned().collect();
                probe_keys.sort();
                assert_eq!(
                    ours_keys, probe_keys,
                    "feature KEY SET disagrees at {site}: {text:?}"
                );
                for (k, n) in expected {
                    let want = u32::try_from(n.as_u64().expect("a census count is a number"))
                        .expect("a census count fits in u32");
                    assert_eq!(
                        parsed.census.get(k),
                        want,
                        "feature {k:?} disagrees at {site}: {text:?}"
                    );
                }
                let probe_mixed = row["mixed_unparenthesised"]
                    .as_bool()
                    .expect("a parsed row carries the mixed verdict");
                assert_eq!(
                    mixed_unparenthesised(&parsed.expr),
                    probe_mixed,
                    "the mixed-connective verdict disagrees at {site}: {text:?}"
                );
                agreed_parse += 1;
            }
            (false, Err(e)) => {
                let kind = row["error_kind"]
                    .as_str()
                    .expect("a failed row names its class");
                assert_eq!(
                    e.kind.tag(),
                    kind,
                    "the failure CLASS disagrees at {site}: {text:?} (probe said {:?})",
                    row["probe_message"]
                );
                if let Some(pos) = row["error_pos"].as_u64() {
                    assert_eq!(
                        e.pos as u64, pos,
                        "the failure POSITION disagrees at {site}: {text:?}"
                    );
                }
                agreed_fail += 1;
            }
            (true, Err(e)) => {
                panic!("the probe parses {text:?} at {site} and this parser does not: {e}")
            }
            (false, Ok(_)) => panic!(
                "the probe REJECTS {text:?} at {site} and this parser accepts it — that string is \
                 one of F42's pack defects and accepting it is how the lint stops catching them"
            ),
        }
        compared += 1;
    }

    // The `on_false` half of the same probe run: form tag and closed-set verdict, per value.
    let of_rows = v["on_false"].as_array().expect("on_false is an array");
    let mut of_compared = 0usize;
    let mut of_rejected = 0usize;
    for row in of_rows {
        let site = row["site"]
            .as_str()
            .expect("every on_false row names its site");
        let value: &Json = &row["value"];
        let probe_ok = row["in_closed_set"].as_bool().expect("a verdict");
        let probe_form = row["form"].as_str().expect("a form tag");
        assert_eq!(
            form_tag(value),
            probe_form,
            "the on_false FORM disagrees at {site}: {value}"
        );
        assert_eq!(
            classify_on_false(value).is_ok(),
            probe_ok,
            "the on_false CLOSED-SET verdict disagrees at {site}: {value}"
        );
        if !probe_ok {
            of_rejected += 1;
        }
        of_compared += 1;
    }

    // A comparison that compared nothing is a green that asserts nothing.
    assert_eq!(
        compared, 287,
        "the differential did not read all 287 predicates"
    );
    assert_eq!(agreed_parse, 285, "agreed on 285 parses");
    assert_eq!(agreed_fail, 2, "agreed on both parse failures");
    assert_eq!(
        of_compared, 99,
        "the differential did not read all 99 on_false values"
    );
    assert_eq!(of_rejected, 3, "the three closed-set violations");
    println!(
        "s_grammar_differential: predicates={compared} parsed={agreed_parse} failed={agreed_fail} \
         on_false={of_compared} outside_closed_set={of_rejected} pack_defects={}",
        agreed_fail + of_rejected
    );
}
