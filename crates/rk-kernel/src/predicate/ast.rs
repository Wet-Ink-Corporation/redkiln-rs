//! The predicate tree, and the feature census the differential compares.

use std::collections::BTreeMap;

/// The six comparison operators of ADR-0103 decision 1. They do not chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    /// The operator's source spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
        }
    }

    /// Parse an operator token. Returns `None` for anything else.
    ///
    /// Named `from_token` and not `from_str`: the latter is `std::str::FromStr`'s method name and
    /// clippy's `should_implement_trait` refuses the shadow, which is the right refusal here — this
    /// is a lexer-internal mapping over six fixed spellings, not a public parse.
    #[must_use]
    pub fn from_token(s: &str) -> Option<Self> {
        match s {
            "==" => Some(Self::Eq),
            "!=" => Some(Self::Ne),
            "<" => Some(Self::Lt),
            "<=" => Some(Self::Le),
            ">" => Some(Self::Gt),
            ">=" => Some(Self::Ge),
            _ => None,
        }
    }

    /// `true` for `<`, `<=`, `>`, `>=` — the four ADR-0069 makes lint errors against `null`.
    #[must_use]
    pub const fn is_ordering(self) -> bool {
        matches!(self, Self::Lt | Self::Le | Self::Gt | Self::Ge)
    }
}

/// The three quantifiers. Their `null`/empty behaviour is ADR-0103 decision 4's and is asymmetric:
/// `all` and `none` are `true`, `any` is `false`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    Any,
    All,
    None,
}

impl Quantifier {
    /// The quantifier's source spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::All => "all",
            Self::None => "none",
        }
    }

    /// Parse a call name. Returns `None` for anything else, including `len`.
    #[must_use]
    pub fn from_token(s: &str) -> Option<Self> {
        match s {
            "any" => Some(Self::Any),
            "all" => Some(Self::All),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

/// A literal in **value** position.
///
/// [`Literal::Bare`] is the one ADR-0103 decision 2 creates and that nothing else in the records
/// implies: an unquoted identifier matching `[a-z_][a-z0-9_]*` on the right of a comparison or
/// inside an `in` list is a **value**, never a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// An unquoted identifier read as a value (`kind == project`).
    Bare(String),
    /// A quoted string, with its quotes stripped and its escapes resolved.
    Str(String),
    /// A number, kept in its source spelling — the canonical serialiser refuses floats, so nothing
    /// here converts one.
    Num(String),
    True,
    False,
    Null,
}

/// The predicate tree.
///
/// [`Expr::Group`] is retained rather than flattened: `mixed_unparenthesised` — F42's
/// precedence-dependence census, measured at **0/287** — is defined over the presence of a group
/// node between an `and` and an `or`, so erasing parentheses at parse time erases the measurement.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Or(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Not(Box<Self>),
    /// A parenthesised sub-expression.
    Group(Box<Self>),
    Cmp {
        op: CmpOp,
        left: Box<Self>,
        right: Box<Self>,
    },
    /// `<operand> in [<value>, …]` — membership in a literal list.
    In {
        left: Box<Self>,
        list: Box<Self>,
    },
    /// `<string literal> in <set-valued facet>` — ADR-0050's *"same facet disjoins"* form, 11 live
    /// sites, and one of the two productions ADR-0103 does not yet admit (§7b.6).
    Member {
        literal: Box<Self>,
        set: Box<Self>,
    },
    /// A dotted field path from a root (`item`, `rigor`, a facet name).
    Path(String),
    /// A `${…}` reference, kept with its braces and its optional `?` marker.
    Ref(String),
    Lit(Literal),
    List(Vec<Self>),
    Len(Box<Self>),
    Quant {
        quantifier: Quantifier,
        /// The bound variable of the prefix form; `None` for the method form, whose element is in
        /// scope by its own fields.
        var: Option<String>,
        source: Box<Self>,
        body: Box<Self>,
    },
}

/// The probe's own feature counters, key for key.
///
/// This is the differential's payload: two parsers that agree on *whether* a string parses can
/// still disagree about *what it is*, and the census is where that shows. `bare_literal` is the
/// literal-versus-field rule's own count, `in_membership` the ADR-0050 form's, `ref_optional` the
/// `${…?}` marker's.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatureCensus(BTreeMap<String, u32>);

impl FeatureCensus {
    /// An empty census.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `n` to `key`. A key that would land at zero is never inserted, because the probe's
    /// `collections.Counter` only materialises a key it has incremented and the differential
    /// compares key sets as well as values.
    pub fn bump(&mut self, key: &str, n: u32) {
        if n == 0 {
            return;
        }
        *self.0.entry(key.to_owned()).or_insert(0) += n;
    }

    /// The count under `key`, or 0.
    #[must_use]
    pub fn get(&self, key: &str) -> u32 {
        self.0.get(key).copied().unwrap_or(0)
    }

    /// Every key with a non-zero count, in byte order.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    /// The census as `(key, count)` pairs, in byte order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u32)> {
        self.0.iter().map(|(k, v)| (k.as_str(), *v))
    }

    /// Fold another census into this one.
    pub fn merge(&mut self, other: &Self) {
        for (k, v) in other.iter() {
            self.bump(k, v);
        }
    }
}
