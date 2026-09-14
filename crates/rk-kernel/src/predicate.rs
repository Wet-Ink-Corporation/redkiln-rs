//! The predicate grammar — **ADR-0103's**, and no other.
//!
//! Precedence, tightest first: `not` (unary) > comparison and membership (`== != < <= > >= in`) >
//! `and` > `or`; `and` and `or` are left-associative and parentheses override. Comparisons do not
//! chain. In **operand** position an unquoted identifier is a field; in **value** position it is a
//! value literal — the clause that is load-bearing at **191 sites** in the live packs [measured,
//! F42]. Field-to-field comparison requires the `${…}` reference form (ADR-0040). `null` arithmetic
//! is total and the connectives are three-valued; `any` over `null` or an empty list is `false`
//! while `all` and `none` are `true`. `on_false` is a closed form set.
//!
//! # The reference implementation, and what "agrees with it" means here
//!
//! `spikes/p9/grammar-probe.py` is S-GRAMMAR's throwaway recursive-descent parser, run by **F42**
//! over every `applicable_if`, `when` and `on_false` string in `spikes/p9` — 386 strings, 287 of
//! them predicates. [`parser`] is written **from that probe's EBNF**, and
//! `grammar_agrees_with_the_s_grammar_probe` is a differential over the probe's own per-string
//! verdicts: parse-or-fail, the failure's kind and byte position, the feature census, and the
//! mixed-connective verdict, for all 287. The probe's verdicts are captured by importing the
//! probe's own `Parser`, `collect()` and `check_on_false` — never by restating them.
//!
//! **One deliberate divergence, and it is on strings the probe REJECTS rather than on strings that
//! parse.** ADR-0103 decision 3 accepts the prefix quantifier form `any(x in list, body)` as an
//! alias of `<list>.any(body)`. The probe's own operand rule admits any identifier outside
//! `{and, or, not, in, true, false, null}` as a path *before* it reaches its prefix-form branch, so
//! `len`, `any`, `all` and `none` are taken as paths and that branch is **unreachable** — which is
//! why its `quant_prefix` census key is absent from a run over a corpus that contains the form's
//! zero live uses. [`parser`] implements the alias, because under ADR-0092 the accepted record
//! outranks the probe. The differential is unaffected: the corpus carries **0** quantifiers of
//! either form [measured, F42], and `exists(ProblemRecord)` — one of the two live parse failures —
//! still fails at the same byte position, because `exists` is not one of the four call names.
//!
//! # Two forms the packs use that ADR-0103 does not yet admit
//!
//! **Reported as IMPLEMENTED-PENDING-RECORD, not as settled grammar** (`s7b-kernel.md` §7b.6). The
//! set-membership form `<string literal> in <set-valued facet>` (**11** live sites; ADR-0050's
//! *"same facet disjoins"*) sits outside ADR-0103 decision 2's operand model, which puts *"the
//! subject of `in`"* in operand position where an unquoted identifier is a field; and a hyphen
//! inside a `${…}` ref segment (step ids such as `green-verify`, **62** refs, 11 optional) sits
//! against decision 2's `[a-z_][a-z0-9_]*` class. Both are stated in exactly one place, the probe's
//! docstring EBNF, and **neither is in ADR-0103 or in the design's grammar section**. The parser
//! accepts them because 11 live predicates and every `${…}` step reference depend on them; **an
//! amendment or erratum against ADR-0103 adding both is OWED and is not written by this crate.**
//!
//! # Where each piece lives
//!
//! - [`lexer`] — the token set, mirroring the probe's alternation order (a `-` outside a quoted
//!   string or a `${…}` segment is a **lex error**, which is the second of F42's five pack defects).
//! - [`ast`] — the tree, plus [`ast::FeatureCensus`], the probe's own counter set.
//! - [`parser`] — precedence, associativity, the literal-versus-field rule, no chaining.
//! - [`eval`] — three-valued evaluation, quantifier null/empty rules, and the read-facet accounting
//!   **AC-079** needs (*"counted once per read facet, never per abstaining facet"*).
//! - [`lint`] — the load-time rules that are decided before evaluation: ADR-0103 decision 4's one
//!   narrowing pattern (**AC-069**), which is **syntactic, not semantic**.
//! - [`on_false`] — ADR-0103 clause 6's closed form set, whose reference behaviour is
//!   `spikes/p9/lint.py`'s `on_false_of`.

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod lint;
pub mod on_false;
pub mod parser;

pub use ast::{CmpOp, Expr, FeatureCensus, Literal, Quantifier};
pub use eval::{Context, EvalError, Outcome, Tri, Value, evaluate};
pub use lexer::{LexError, Token, TokenKind};
pub use lint::{LintError, lint_predicate};
pub use on_false::{OnFalse, OnFalseError, classify_on_false};
pub use parser::{ParseError, ParseErrorKind, Parsed, mixed_unparenthesised, parse};
