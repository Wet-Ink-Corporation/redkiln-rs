//! Shared helpers: the captured S-GRAMMAR verdicts, and the contexts the evaluator cases run over.

use rk_kernel::predicate::{Context, Expr, Outcome, Parsed, Tri, Value, evaluate, parse};
use serde_json::Value as Json;

/// The probe's per-string verdicts, **compiled in** rather than read at run time.
///
/// `include_str!` is deliberate. A fixture loaded from a path is a `Result` an implementer can
/// `unwrap_or_default()` into an empty corpus, and a differential over an empty corpus passes —
/// which is Halt 8's *"a filter that matches nothing exits 0"*, one level up. A missing file here
/// is a **compile** error.
const PROBE_VERDICTS: &str = include_str!("../fixtures/grammar/s-grammar-probe-verdicts.json");

/// The captured verdicts, parsed.
///
/// # Panics
///
/// If the committed fixture is not the object the capture script writes.
#[must_use]
pub fn probe_verdicts() -> Json {
    serde_json::from_str(PROBE_VERDICTS).expect("the committed S-GRAMMAR verdicts parse as JSON")
}

/// Parse a predicate that is expected to parse.
///
/// # Panics
///
/// With the parser's own error, so a fixture that stops parsing names its reason rather than a
/// bare `unwrap` site.
#[must_use]
pub fn ok_parse(src: &str) -> Parsed {
    match parse(src) {
        Ok(p) => p,
        Err(e) => panic!("expected {src:?} to parse, got {e}"),
    }
}

/// Parse and evaluate in one step.
///
/// # Panics
///
/// If the source does not parse, or if evaluation errors.
#[must_use]
pub fn eval(src: &str, ctx: &Context) -> Outcome {
    let parsed = ok_parse(src);
    match evaluate(&parsed.expr, ctx) {
        Ok(o) => o,
        Err(e) => panic!("expected {src:?} to evaluate, got {e}"),
    }
}

/// The truth value alone.
///
/// # Panics
///
/// As [`eval`].
#[must_use]
pub fn truth(src: &str, ctx: &Context) -> Tri {
    eval(src, ctx).value
}

/// The right-hand operand of a top-level comparison, for the literal-versus-field cases.
///
/// # Panics
///
/// If the expression is not a comparison.
#[must_use]
pub fn rhs(expr: &Expr) -> &Expr {
    match expr {
        Expr::Cmp { right, .. } => right,
        other => panic!("expected a comparison at the top level, got {other:?}"),
    }
}

/// A context in which the four ordinal axes and the three booleans are bound to `null` unless a
/// case rebinds them — the shape F39 measured, where 4 of 20 items changed no code at all and the
/// panel returned `unknown` on all 16 of their semantic-facet cells.
#[must_use]
pub fn abstaining_context() -> Context {
    Context::new()
        .with("novelty", Value::Null)
        .with("blast_radius", Value::Null)
        .with("has_repro", Value::Null)
        .with("market_facing", Value::Null)
}

/// The four-band ordinal domain the packs use.
pub const BANDS: [&str; 4] = ["low", "med", "high", "critical"];
