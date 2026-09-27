//! Three-valued evaluation — ADR-0103 decision 4, and the first implementation of it.
//!
//! F42 says so in terms: *"the probe parses; it does not evaluate. ADR-0103's three-valued
//! connectives and quantifier null rules are unexercised (**0** quantifiers in the corpus). Step
//! 7b's evaluator is the first implementation."* Every fixture here is therefore **authored**, not
//! lifted from the packs.
//!
//! # The three readings this module takes, and why each is forced rather than chosen
//!
//! ADR-0103 decision 4 is complete for `==` and for the quantifiers and leaves three things to a
//! reading. Each is stated here because a reading taken silently is a stage deciding a clause an
//! accepted record owns.
//!
//! 1. **`!=` is the negation of `==`.** The record fixes `null == null` as true and says `!=`
//!    *"against `null`"* is `null`; it does not state `null != null`. Negation is the only reading
//!    under which decision 4's own blessed narrowing pattern is safe: with `x` absent,
//!    `x != null and <ordering on x>` needs its guard to be **false** so the conjunction
//!    short-circuits and the ordering — a lint error against `null` under ADR-0069 — is never
//!    evaluated. Under the alternative (`null != null` is `null`) the guard cannot guard.
//!
//!    *(dated 2026-09-26, `s8d` build; `spikes/DECISIONS-2026-09-21.md` D21-67 ruling 2.)*
//!    **ADR-0172** (written `proposed`) makes a comparison against the LITERAL `null` a definite
//!    presence test and supersedes ADR-0103 decision 4 and ADR-0121 decision 3's first bullet in
//!    part, for `==` and `!=` only. Its `x != null` over an absent `x` is `false`, which is what
//!    this reading gives, so the reading is subsumed there and stands unchanged for every other
//!    `!=` (a null field against a non-null value is still `null`, negated). Over a PRESENT `x`,
//!    `x != null` is now `true`, not `null`, so the narrowing pattern also lets the ordering run on
//!    a present axis. `compare`'s literal-null arm is where it is built; `in` against `null` is
//!    not the record's and stays `null`.
//! 2. **The connectives are Kleene's strong three-valued logic, and both absorbing positions
//!    absorb.** ✅ **SETTLED BY RECORD — this is no longer a reading.** **ADR-0121** decision 1
//!    (accepted 2026-09-13) states the two full truth tables and makes `and` and `or` commutative
//!    and associative, and it **supersedes ADR-0103 decision 4's connective sentence in part**:
//!    exactly two cells move — `null and false` from `null` to `false`, `null or true` from `null`
//!    to `true`. ADR-0103's *"otherwise `null`"* keeps full force everywhere else, and the rest of
//!    decision 4 (the `==`/`!=`/`in` null rules, `not null`, the quantifier rules, the abstention
//!    case, the narrowing pattern) and decisions 1, 2, 3, 5 and 6 all stand, so ADR-0103 stays
//!    `accepted` carrying `superseded_by: ["0121"]`.
//!
//!    **It was taken on the merits, not as a clarification**: read literally, decision 4 made
//!    `a and b` differ from `b and a`, and ADR-0121's Context measures four live `spikes/p9`
//!    predicates that break under it. The behaviour below is unchanged — what changed is that it is
//!    now warranted by an accepted record rather than pinned as a divergence — and
//!    `evaluator::kleene_absorption_is_commutative_per_adr_0121` is where it is asserted.
//!
//!    **What ADR-0121 does NOT decide, and this module must still guarantee:** evaluation order.
//!    Its table is about VALUES. The short-circuit below is what makes decision 4's narrowing
//!    pattern safe and what keeps AC-079's read-facet count honest; see the `Expr::And` arm.
//!    ADR-0121 clause 5 states the obligation and declines to supply it. Nor does it decide
//!    `null != null` — that stays reading 1 above, and it stays a residual against ADR-0103.
//! 3. **`<`, `<=`, `>`, `>=` against `null` are an ERROR at evaluation, never a value.** ADR-0069's
//!    forbids clause is *"lint must reject it, **not evaluate it with some default ordering**"*,
//!    and returning `null` for it would be a default. The lint ([`super::lint`]) is the primary
//!    defence and the short-circuit above is the second; this error is the third, so a guarded row
//!    that still reaches an absent operand fails loudly instead of abstaining.
//!
//! # The read-facet accounting AC-079 needs
//!
//! [`Outcome::read_facets`] is *"counted once per read facet, never per abstaining facet"* — the
//! register's own words, and AC-079's *Verified by* sharpens it to *"one `gate_opened` **per facet
//! a firing row read**; an abstaining facet no row reads opens nothing"*. A facet read twice by one
//! predicate is therefore one entry, a facet behind a short-circuited conjunct is **no** entry, and
//! a quantifier's bound variable is never one. **Minting the `gate_opened` event from this set is
//! selection step 2's and therefore Slice 3's**; what is here is the accounting it consumes.

use std::collections::{BTreeMap, BTreeSet};

use super::ast::{CmpOp, Expr, Literal, Quantifier};

/// The three truth values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tri {
    True,
    False,
    /// Abstention — no evidence was computable, which is never the same as `false`.
    Null,
}

impl Tri {
    /// `Tri::True` for `true`, `Tri::False` for `false`.
    #[must_use]
    pub const fn from_bool(b: bool) -> Self {
        if b { Self::True } else { Self::False }
    }

    /// ADR-0103 decision 4: `not null` is `null`.
    #[must_use]
    pub const fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Null => Self::Null,
        }
    }
}

/// A value a predicate can read.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Abstention. ADR-0089 made this load-bearing: absence of evidence never resolves to `false`.
    Null,
    Bool(bool),
    Str(String),
    Int(i64),
    /// A member of an ordered domain, carried **with** its domain so an ordering never has to guess
    /// a rank. `criticality >= high` compares positions in `["low", "med", "high", …]`, and a
    /// literal outside the domain is an error rather than a silent `false`.
    Ordinal {
        domain: Vec<String>,
        index: usize,
    },
    /// A list or a set-valued facet — the right-hand side of ADR-0050's `'text' in change`.
    List(Vec<Self>),
    /// A record, whose fields come into scope inside a method-form quantifier body.
    Record(BTreeMap<String, Self>),
}

impl Value {
    /// A set-valued facet, from string members.
    #[must_use]
    pub fn set<I: IntoIterator<Item = S>, S: Into<String>>(items: I) -> Self {
        Self::List(items.into_iter().map(|s| Self::Str(s.into())).collect())
    }

    /// An ordinal from its domain and the member it currently holds.
    ///
    /// # Errors
    ///
    /// [`EvalError::NotInDomain`] when `member` is not one of `domain`.
    pub fn ordinal(domain: &[&str], member: &str) -> Result<Self, EvalError> {
        let index =
            domain
                .iter()
                .position(|d| *d == member)
                .ok_or_else(|| EvalError::NotInDomain {
                    literal: member.to_owned(),
                    domain: domain.iter().map(|s| (*s).to_owned()).collect(),
                })?;
        Ok(Self::Ordinal {
            domain: domain.iter().map(|s| (*s).to_owned()).collect(),
            index,
        })
    }

    const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

/// What stopped evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// A path the context does not bind. A facet outside the registry is a load-time error, so
    /// reaching one here is a fault and not an abstention.
    UnknownField(String),
    /// `<`, `<=`, `>`, `>=` reached an absent operand — ADR-0069's forbids clause.
    OrderingAgainstNull(String),
    /// Two operands an ordering is not defined over.
    NotOrderable(String),
    /// A quantifier or a membership test over something that is not a list.
    NotAList(String),
    /// A literal compared against an ordinal that does not carry it.
    NotInDomain {
        literal: String,
        domain: Vec<String>,
    },
    /// A `${…}` reference reached the evaluator. Reference resolution is the selection function's
    /// step 2 and is Slice 3's; nothing in this slice resolves one, and returning a default here
    /// would be the silent coercion ADR-0069 forbids one level up.
    UnresolvedRef(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownField(p) => write!(f, "unknown field {p:?}"),
            Self::OrderingAgainstNull(p) => write!(f, "ordering against null at {p:?}"),
            Self::NotOrderable(d) => write!(f, "operands are not orderable: {d}"),
            Self::NotAList(d) => write!(f, "not a list: {d}"),
            Self::NotInDomain { literal, domain } => {
                write!(f, "literal {literal:?} is not in the domain {domain:?}")
            }
            Self::UnresolvedRef(r) => write!(f, "unresolved reference {r}"),
        }
    }
}

impl std::error::Error for EvalError {}

/// The item's context vector, as a predicate sees it.
#[derive(Debug, Clone, Default)]
pub struct Context {
    bindings: BTreeMap<String, Value>,
}

impl Context {
    /// An empty context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind one path.
    #[must_use]
    pub fn with(mut self, path: &str, value: Value) -> Self {
        self.bindings.insert(path.to_owned(), value);
        self
    }

    /// Bind one path in place.
    pub fn bind(&mut self, path: &str, value: Value) {
        self.bindings.insert(path.to_owned(), value);
    }

    /// The value at `path`, if the context binds it.
    #[must_use]
    pub fn get(&self, path: &str) -> Option<&Value> {
        self.bindings.get(path)
    }
}

/// A predicate's truth value and the facets that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub value: Tri,
    /// Every context-bound path the evaluation actually read, once each, in byte order.
    pub read_facets: BTreeSet<String>,
}

impl Outcome {
    /// The axes a `gate_opened { reason: unknown_facet, axis }` is owed for, per AC-079.
    ///
    /// Empty unless the predicate's **top-level** value is `null`: the register's rule is *"one per
    /// facet a firing row read"*, and a row that resolved to `true` or `false` opens nothing.
    #[must_use]
    pub fn abstention_axes(&self) -> Vec<&str> {
        if self.value == Tri::Null {
            self.read_facets.iter().map(String::as_str).collect()
        } else {
            Vec::new()
        }
    }
}

/// Evaluate `expr` over `ctx`.
///
/// # Errors
///
/// [`EvalError`], including [`EvalError::OrderingAgainstNull`] for the case ADR-0069 forbids being
/// evaluated with a default ordering.
pub fn evaluate(expr: &Expr, ctx: &Context) -> Result<Outcome, EvalError> {
    let mut ev = Eval {
        ctx,
        read: BTreeSet::new(),
    };
    let value = ev.truth(expr, &[])?;
    Ok(Outcome {
        value,
        read_facets: ev.read,
    })
}

type Frame = BTreeMap<String, Value>;

struct Eval<'a> {
    ctx: &'a Context,
    read: BTreeSet<String>,
}

impl Eval<'_> {
    fn truth(&mut self, expr: &Expr, scope: &[Frame]) -> Result<Tri, EvalError> {
        match expr {
            Expr::Group(inner) => self.truth(inner, scope),
            Expr::Not(inner) => Ok(self.truth(inner, scope)?.not()),
            Expr::And(l, r) => {
                // The short-circuit is load-bearing, not an optimisation: it is what makes
                // `x != null and <ordering on x>` safe when `x` is absent.
                //
                // THE ABSORPTION BELOW IS COMMUTATIVE, AND ADR-0121 DECISION 1 IS THE WARRANT.
                // `null and false` is `false` and `null or true` is `true`; ADR-0121 supersedes
                // ADR-0103 decision 4's connective sentence in part, moving exactly those two cells
                // and leaving the rest of decision 4 standing. The full argument and the four live
                // predicates that forced it are in ADR-0121 and summarised on `or3` below. THIS is
                // the site the test `evaluator::kleene_absorption_is_commutative_per_adr_0121`
                // actually pins — `or3`/`and3` serve `in`-membership and the quantifiers, not the
                // connectives.
                //
                // The SHORT-CIRCUIT above is NOT ADR-0121's — its table is about values, not order.
                // ADR-0121 clause 5(a) states the obligation and leaves it to the implementation:
                // `and` must return `false` on a `false` left operand WITHOUT evaluating the right,
                // or decision 4's narrowing pattern errors and AC-079's read-facet count is wrong.
                let left = self.truth(l, scope)?;
                if left == Tri::False {
                    return Ok(Tri::False);
                }
                let right = self.truth(r, scope)?;
                Ok(match (left, right) {
                    (_, Tri::False) => Tri::False,
                    (Tri::True, other) | (other, Tri::True) => other,
                    _ => Tri::Null,
                })
            }
            Expr::Or(l, r) => {
                let left = self.truth(l, scope)?;
                if left == Tri::True {
                    return Ok(Tri::True);
                }
                let right = self.truth(r, scope)?;
                Ok(match (left, right) {
                    (_, Tri::True) => Tri::True,
                    (Tri::False, other) | (other, Tri::False) => other,
                    _ => Tri::Null,
                })
            }
            Expr::Cmp { op, left, right } => self.compare(*op, left, right, scope),
            Expr::In { left, list } => {
                let subject = self.value_of(left, scope)?;
                let Expr::List(items) = list.as_ref() else {
                    return Err(EvalError::NotAList(format!("{list:?}")));
                };
                if subject.is_null() {
                    return Ok(Tri::Null);
                }
                let mut acc = Tri::False;
                for item in items {
                    let member = self.value_of(item, scope)?;
                    acc = or3(acc, equality(&subject, &member)?);
                }
                Ok(acc)
            }
            Expr::Member { literal, set } => {
                let needle = self.value_of(literal, scope)?;
                let haystack = self.value_of(set, scope)?;
                match haystack {
                    Value::Null => Ok(Tri::Null),
                    Value::List(items) => {
                        let mut acc = Tri::False;
                        for item in &items {
                            acc = or3(acc, equality(item, &needle)?);
                        }
                        Ok(acc)
                    }
                    other => Err(EvalError::NotAList(format!("{other:?}"))),
                }
            }
            Expr::Quant {
                quantifier,
                var,
                source,
                body,
            } => self.quantify(*quantifier, var.as_deref(), source, body, scope),
            // A bare boolean facet, a bare literal, a `${…}`: read its value and coerce.
            other => {
                let v = self.value_of(other, scope)?;
                match v {
                    Value::Null => Ok(Tri::Null),
                    Value::Bool(b) => Ok(Tri::from_bool(b)),
                    ref x => Ok(Tri::from_bool(!matches!(
                        x,
                        Value::List(l) if l.is_empty()
                    ))),
                }
            }
        }
    }

    fn compare(
        &mut self,
        op: CmpOp,
        left: &Expr,
        right: &Expr,
        scope: &[Frame],
    ) -> Result<Tri, EvalError> {
        let l = self.value_of(left, scope)?;
        let r = self.value_of(right, scope)?;
        // *(dated 2026-09-26, `s8d` build; D21-67 ruling 2; ADR-0172, written `proposed`)* A
        // comparison against the LITERAL `null` is a definite presence test: `x == null` is true
        // when `x` is absent and false when it is present, `x != null` the reverse. The arm keys on
        // the operand EXPRESSION being the literal as written, never on a value that happens to be
        // null, so a null field against a non-null value still falls through to `equality` and
        // stays `null` (ADR-0103 decision 4, standing). `in` against `null` is not this arm's.
        let literal_null = |e: &Expr| matches!(e, Expr::Lit(Literal::Null));
        let presence = if literal_null(right) {
            Some(&l)
        } else if literal_null(left) {
            Some(&r)
        } else {
            None
        };
        if let Some(other) = presence {
            match op {
                CmpOp::Eq => return Ok(Tri::from_bool(other.is_null())),
                CmpOp::Ne => return Ok(Tri::from_bool(!other.is_null())),
                _ => {}
            }
        }
        match op {
            CmpOp::Eq => equality(&l, &r),
            // `!=` is the negation of `==`; see this module's reading 1.
            CmpOp::Ne => Ok(equality(&l, &r)?.not()),
            _ => {
                if l.is_null() {
                    return Err(EvalError::OrderingAgainstNull(render(left)));
                }
                if r.is_null() {
                    return Err(EvalError::OrderingAgainstNull(render(right)));
                }
                let ord = order(&l, &r)?;
                Ok(Tri::from_bool(match op {
                    CmpOp::Lt => ord.is_lt(),
                    CmpOp::Le => ord.is_le(),
                    CmpOp::Gt => ord.is_gt(),
                    CmpOp::Ge => ord.is_ge(),
                    CmpOp::Eq | CmpOp::Ne => unreachable!("handled above"),
                }))
            }
        }
    }

    fn quantify(
        &mut self,
        q: Quantifier,
        var: Option<&str>,
        source: &Expr,
        body: &Expr,
        scope: &[Frame],
    ) -> Result<Tri, EvalError> {
        let src = self.value_of(source, scope)?;
        let items = match src {
            // ADR-0103 decision 4, and the clause that is load-bearing for AC-527: `all` and `none`
            // over `null` or an EMPTY list are true; `any` is false. ADR-0032's "vacuously true"
            // is withdrawn for `any`, because `any(s in surfaces, s == "auth")` over an absent
            // `surfaces` must not fire the activity.
            // *(dated 2026-09-25, s8b refuting round 5)* Since AC-527's 2026-09-25 amendment
            // (D21-55), AC-527 binds `findings` only to non-empty one-finding witnesses, never to
            // `null` or an empty list, so this clause is no longer load-bearing for it. It still
            // governs selection (the `any(s in surfaces, …)` sentence above).
            Value::Null => Vec::new(),
            Value::List(items) => items,
            other => return Err(EvalError::NotAList(format!("{other:?}"))),
        };
        if items.is_empty() {
            return Ok(match q {
                Quantifier::Any => Tri::False,
                Quantifier::All | Quantifier::None => Tri::True,
            });
        }
        let mut any = Tri::False;
        let mut all = Tri::True;
        for item in &items {
            let mut frame = Frame::new();
            if let Some(name) = var {
                frame.insert(name.to_owned(), item.clone());
            } else if let Value::Record(fields) = item {
                // The method form puts the element in scope by its own fields.
                for (k, v) in fields {
                    frame.insert(k.clone(), v.clone());
                }
            }
            let mut nested: Vec<Frame> = scope.to_vec();
            nested.push(frame);
            let t = self.truth(body, &nested)?;
            any = or3(any, t);
            all = and3(all, t);
        }
        Ok(match q {
            Quantifier::Any => any,
            Quantifier::All => all,
            Quantifier::None => any.not(),
        })
    }

    fn value_of(&mut self, expr: &Expr, scope: &[Frame]) -> Result<Value, EvalError> {
        match expr {
            Expr::Lit(lit) => Ok(literal_value(lit)),
            Expr::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.value_of(item, scope)?);
                }
                Ok(Value::List(out))
            }
            Expr::Path(path) => {
                for frame in scope.iter().rev() {
                    if let Some(v) = frame.get(path) {
                        return Ok(v.clone());
                    }
                }
                let v = self
                    .ctx
                    .get(path)
                    .ok_or_else(|| EvalError::UnknownField(path.clone()))?;
                self.read.insert(path.clone());
                Ok(v.clone())
            }
            Expr::Ref(r) => Err(EvalError::UnresolvedRef(r.clone())),
            Expr::Len(inner) => match self.value_of(inner, scope)? {
                Value::Null => Ok(Value::Null),
                Value::List(items) => {
                    Ok(Value::Int(i64::try_from(items.len()).unwrap_or(i64::MAX)))
                }
                Value::Str(s) => Ok(Value::Int(i64::try_from(s.len()).unwrap_or(i64::MAX))),
                other => Err(EvalError::NotAList(format!("{other:?}"))),
            },
            Expr::Group(inner) => self.value_of(inner, scope),
            other => Ok(match self.truth(other, scope)? {
                Tri::True => Value::Bool(true),
                Tri::False => Value::Bool(false),
                Tri::Null => Value::Null,
            }),
        }
    }
}

fn literal_value(lit: &Literal) -> Value {
    match lit {
        Literal::Null => Value::Null,
        Literal::True => Value::Bool(true),
        Literal::False => Value::Bool(false),
        Literal::Str(s) | Literal::Bare(s) => Value::Str(s.clone()),
        Literal::Num(n) => n
            .parse::<i64>()
            .map_or_else(|_| Value::Str(n.clone()), Value::Int),
    }
}

fn render(e: &Expr) -> String {
    match e {
        Expr::Path(p) => p.clone(),
        Expr::Ref(r) => r.clone(),
        other => format!("{other:?}"),
    }
}

/// ADR-0103 decision 4's equality: `null == null` is true, `null == v` is `null`.
fn equality(l: &Value, r: &Value) -> Result<Tri, EvalError> {
    match (l, r) {
        (Value::Null, Value::Null) => Ok(Tri::True),
        (Value::Null, _) | (_, Value::Null) => Ok(Tri::Null),
        (Value::Ordinal { domain, index }, other) | (other, Value::Ordinal { domain, index }) => {
            let name = &domain[*index];
            match other {
                Value::Str(s) => Ok(Tri::from_bool(s == name)),
                Value::Ordinal {
                    domain: d2,
                    index: i2,
                } => Ok(Tri::from_bool(&d2[*i2] == name)),
                _ => Ok(Tri::False),
            }
        }
        (a, b) => Ok(Tri::from_bool(a == b)),
    }
}

fn order(l: &Value, r: &Value) -> Result<std::cmp::Ordering, EvalError> {
    match (l, r) {
        (Value::Int(a), Value::Int(b)) => Ok(a.cmp(b)),
        (Value::Str(a), Value::Str(b)) => Ok(a.cmp(b)),
        (Value::Ordinal { domain, index }, Value::Str(s)) => {
            let other = rank(domain, s)?;
            Ok(index.cmp(&other))
        }
        (Value::Str(s), Value::Ordinal { domain, index }) => {
            let other = rank(domain, s)?;
            Ok(other.cmp(index))
        }
        (
            Value::Ordinal { domain, index },
            Value::Ordinal {
                domain: d2,
                index: i2,
            },
        ) if domain == d2 => Ok(index.cmp(i2)),
        (a, b) => Err(EvalError::NotOrderable(format!("{a:?} vs {b:?}"))),
    }
}

fn rank(domain: &[String], member: &str) -> Result<usize, EvalError> {
    domain
        .iter()
        .position(|d| d == member)
        .ok_or_else(|| EvalError::NotInDomain {
            literal: member.to_owned(),
            domain: domain.to_vec(),
        })
}

/// Kleene `or`: `true` absorbs from either side.
///
/// **The commutativity is ADR-0121 decision 1**, which states both truth tables in full and makes
/// `and` and `or` commutative and associative over `{true, false, null}`. It **supersedes ADR-0103
/// decision 4's connective sentence in part**: decision 4 (`0103`:70-71) read *"`and`/`or` are
/// three-valued: `false and null` = false, `true or null` = true, otherwise `null`"* -- each
/// absorbing case in ONE order, with a total catch-all, while the same sentence spells out BOTH
/// orders for `==` two clauses earlier. Exactly **two cells** move: `null and false` `null` ->
/// `false`, `null or true` `null` -> `true`. Everything else of decision 4 stands.
///
/// **It was taken on the merits, on four live predicates, not as a clarification.** The swapped pair
/// `catalog/api-contract-spec.toml`:10 (`"external_dep or kind == project"`) and
/// `catalog/integration-tests.toml`:10 (`"kind == project or external_dep"`) is one guard written
/// both ways that would select differently on operand order alone; `catalog-b/feature-flag-kill.toml`
/// :43 could never be true once `answer` existed; and
/// `"${rollback.outputs?} == null and (${proceed.answer?} == 'proceed' or rigor < mandatory)"`
/// (`catalog/canary-release.toml`:73) has a left operand that is `null` whenever `rollback.outputs` is
/// PRESENT, so the cell decides abstain versus `Unsatisfiable` -- and those are not the same answer
/// [measured 2026-09-13, grammar-probe census: `and` 23, `or` 25, `null` 2 over 287 live predicates].
///
/// Pinned by `evaluator::kleene_absorption_is_commutative_per_adr_0121`, which is also the site that
/// goes red and names itself if ADR-0121 is ever superseded in turn.
const fn or3(a: Tri, b: Tri) -> Tri {
    match (a, b) {
        (Tri::True, _) | (_, Tri::True) => Tri::True,
        (Tri::False, other) | (other, Tri::False) => other,
        _ => Tri::Null,
    }
}

/// Kleene `and`: `false` absorbs from either side.
const fn and3(a: Tri, b: Tri) -> Tri {
    match (a, b) {
        (Tri::False, _) | (_, Tri::False) => Tri::False,
        (Tri::True, other) | (other, Tri::True) => other,
        _ => Tri::Null,
    }
}
