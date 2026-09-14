//! Recursive descent over ADR-0103's precedence.

use super::ast::{CmpOp, Expr, FeatureCensus, Literal, Quantifier};
use super::lexer::{self, CALL_NAMES, KEYWORDS, Token, TokenKind};

/// Why a string did not parse.
///
/// The variants are the probe's own failure classes: the differential compares this discriminant
/// and the byte position, not a message, because the probe's messages embed Python `repr`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// No token could start at this position.
    Lex,
    /// The predicate parsed and input remained.
    Trailing,
    /// `a < b < c` — ADR-0103 decision 1.
    Chain,
    /// An operand was required and the token could not begin one.
    Operand,
    /// A value was required and the token could not begin one.
    Value,
    /// A reserved word appeared in value position.
    KeywordInValuePosition,
    /// A specific token was required (a `)`, a `,`, an `in`).
    Expected,
}

impl ParseErrorKind {
    /// The stable tag the captured probe verdicts are keyed on.
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Lex => "lex",
            Self::Trailing => "trailing",
            Self::Chain => "chain",
            Self::Operand => "operand",
            Self::Value => "value",
            Self::KeywordInValuePosition => "keyword_in_value_position",
            Self::Expected => "expected",
        }
    }
}

/// A parse failure, with the byte position the probe reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub pos: usize,
    pub detail: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}: {}", self.kind.tag(), self.pos, self.detail)
    }
}

impl std::error::Error for ParseError {}

/// A parsed predicate and the feature census taken while parsing it.
#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub expr: Expr,
    pub census: FeatureCensus,
}

/// Parse one predicate string.
///
/// # Errors
///
/// [`ParseError`] with the failure's class and byte position.
pub fn parse(src: &str) -> Result<Parsed, ParseError> {
    let tokens = lexer::lex(src).map_err(|e| ParseError {
        kind: ParseErrorKind::Lex,
        pos: e.pos,
        detail: format!("{:?}", e.snippet),
    })?;
    let mut p = Parser {
        tokens,
        i: 0,
        census: FeatureCensus::new(),
    };
    let expr = p.or_expr()?;
    if p.peek().kind != TokenKind::Eof {
        let tok = p.peek();
        return Err(ParseError {
            kind: ParseErrorKind::Trailing,
            pos: tok.pos,
            detail: format!("{:?}", tok.text),
        });
    }
    Ok(Parsed {
        expr,
        census: p.census,
    })
}

/// `true` when an `and` sits directly under an `or` (or the reverse) with no group node between.
///
/// This is F42's precedence-dependence census — **0/287** on the live corpus — and it is why
/// [`Expr::Group`] survives parsing.
#[must_use]
pub fn mixed_unparenthesised(node: &Expr) -> bool {
    const fn is_connective(e: &Expr) -> Option<bool> {
        match e {
            Expr::And(..) => Some(true),
            Expr::Or(..) => Some(false),
            _ => None,
        }
    }
    match node {
        Expr::And(l, r) | Expr::Or(l, r) => {
            let me = is_connective(node);
            for child in [l.as_ref(), r.as_ref()] {
                if let Some(k) = is_connective(child)
                    && Some(k) != me
                {
                    return true;
                }
                if mixed_unparenthesised(child) {
                    return true;
                }
            }
            false
        }
        Expr::Group(inner) | Expr::Not(inner) => mixed_unparenthesised(inner),
        _ => false,
    }
}

struct Parser {
    tokens: Vec<Token>,
    i: usize,
    census: FeatureCensus,
}

impl Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn peek_at(&self, k: usize) -> &Token {
        let idx = (self.i + k).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    fn at(&self, kind: TokenKind, text: Option<&str>) -> bool {
        let t = self.peek();
        t.kind == kind && text.is_none_or(|v| t.text == v)
    }

    fn bump(&mut self) -> Token {
        let t = self.tokens[self.i].clone();
        self.i += 1;
        t
    }

    fn expect(&mut self, kind: TokenKind, text: &str) -> Result<Token, ParseError> {
        if self.at(kind, Some(text)) {
            Ok(self.bump())
        } else {
            let t = self.peek();
            Err(ParseError {
                kind: ParseErrorKind::Expected,
                pos: t.pos,
                detail: format!("expected {text:?}, got {:?}", t.text),
            })
        }
    }

    fn or_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.and_expr()?;
        let mut n = 0;
        while self.at(TokenKind::Ident, Some("or")) {
            self.bump();
            let right = self.and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
            n += 1;
        }
        self.census.bump("or", n);
        Ok(left)
    }

    fn and_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.comparison()?;
        let mut n = 0;
        while self.at(TokenKind::Ident, Some("and")) {
            self.bump();
            let right = self.comparison()?;
            left = Expr::And(Box::new(left), Box::new(right));
            n += 1;
        }
        self.census.bump("and", n);
        Ok(left)
    }

    /// `not` is the **tightest** operator, which is what ADR-0103 decision 1 says and not what the
    /// S-GRAMMAR probe does.
    ///
    /// Decision 1 is *"tightest first: `not` (unary) > comparison and membership (`== != < <= > >=
    /// in`) > `and` > `or`"*, and its forbids clause is *"Forbids a parser or lint implementing any
    /// precedence but clause 1"*. So `not a == b` is **`(not a) == b`**, and a descent in which
    /// `not` sits *above* [`Parser::comparison`] — `not_expr := "not" not_expr | comparison`, which
    /// is Python's precedence and which `spikes/p9/grammar-probe.py`:95-98 implements — is the
    /// reverse of the record. Under **ADR-0092 decision 2 rule 1** the accepted ADR governs the
    /// probe, which is the same warrant [`Parser::prefix_call`] uses for decision 3's alias; the
    /// two readings are indistinguishable on the corpus, whose feature census carries **no `not`
    /// key at all** (0 of 287 strings) [measured 2026-09-13], so the differential cannot arbitrate
    /// and the record is all there is.
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.at(TokenKind::Ident, Some("not")) {
            self.bump();
            self.census.bump("not", 1);
            return Ok(Expr::Not(Box::new(self.unary()?)));
        }
        self.primary()
    }

    /// A parenthesised sub-expression, or an operand. `not`'s operand and a comparison's left side.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.at(TokenKind::Punct, Some("(")) {
            self.bump();
            self.census.bump("parens", 1);
            let inner = self.or_expr()?;
            self.expect(TokenKind::Punct, ")")?;
            return Ok(Expr::Group(Box::new(inner)));
        }
        self.operand()
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        // `<string literal> in <set-valued facet>` — ADR-0050's form, and one of the two
        // productions ADR-0103 does not yet admit (§7b.6, IMPLEMENTED-PENDING-RECORD).
        if self.peek().kind == TokenKind::Str
            && self.peek_at(1).kind == TokenKind::Ident
            && self.peek_at(1).text == "in"
        {
            let lit = self.bump();
            self.expect(TokenKind::Ident, "in")?;
            let set = self.operand()?;
            self.census.bump("in_membership", 1);
            self.census.bump("quoted", 1);
            return Ok(Expr::Member {
                literal: Box::new(Expr::Lit(Literal::Str(lexer::unquote(&lit.text)))),
                set: Box::new(set),
            });
        }
        let left = self.unary()?;
        if self.peek().kind == TokenKind::Op {
            let op_tok = self.bump();
            let op =
                CmpOp::from_token(&op_tok.text).expect("the lexer emits only the six operators");
            let right = self.value()?;
            if self.peek().kind == TokenKind::Op {
                let t = self.peek();
                return Err(ParseError {
                    kind: ParseErrorKind::Chain,
                    pos: t.pos,
                    detail: format!("comparison chain at {:?}", t.text),
                });
            }
            return Ok(Expr::Cmp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        if self.at(TokenKind::Ident, Some("in")) {
            self.bump();
            let list = self.list()?;
            self.census.bump("in", 1);
            return Ok(Expr::In {
                left: Box::new(left),
                list: Box::new(list),
            });
        }
        Ok(left)
    }

    fn operand(&mut self) -> Result<Expr, ParseError> {
        let mut node = if self.peek().kind == TokenKind::Ref {
            let tok = self.bump();
            self.census.bump("ref", 1);
            if tok.text.ends_with("?}") {
                self.census.bump("ref_optional", 1);
            }
            Expr::Ref(tok.text)
        } else if self.peek().kind == TokenKind::Ident
            && CALL_NAMES.contains(&self.peek().text.as_str())
            && self.peek_at(1).kind == TokenKind::Punct
            && self.peek_at(1).text == "("
        {
            // The prefix forms `len(x)` and `any(x in list, body)` (ADR-0103 decision 3's alias).
            return self.prefix_call();
        } else if self.peek().kind == TokenKind::Ident
            && !matches!(
                self.peek().text.as_str(),
                "and" | "or" | "not" | "in" | "true" | "false" | "null"
            )
        {
            let mut parts = vec![self.bump().text];
            while self.at(TokenKind::Punct, Some("."))
                && self.peek_at(1).kind == TokenKind::Ident
                && !CALL_NAMES.contains(&self.peek_at(1).text.as_str())
            {
                self.bump();
                parts.push(self.bump().text);
            }
            Expr::Path(parts.join("."))
        } else {
            let t = self.peek();
            return Err(ParseError {
                kind: ParseErrorKind::Operand,
                pos: t.pos,
                detail: format!("operand expected, got {:?}", t.text),
            });
        };
        // Method calls: `.len()`, `.any(body)`, `.all(body)`, `.none(body)`.
        while self.at(TokenKind::Punct, Some("."))
            && self.peek_at(1).kind == TokenKind::Ident
            && CALL_NAMES.contains(&self.peek_at(1).text.as_str())
        {
            self.bump();
            let name = self.bump().text;
            self.expect(TokenKind::Punct, "(")?;
            if name == "len" {
                self.expect(TokenKind::Punct, ")")?;
                self.census.bump("len", 1);
                node = Expr::Len(Box::new(node));
            } else {
                let body = self.or_expr()?;
                self.expect(TokenKind::Punct, ")")?;
                let q = Quantifier::from_token(&name).expect("CALL_NAMES minus len is the three");
                self.census.bump(&format!("quant_{name}"), 1);
                self.census.bump("quant_method", 1);
                node = Expr::Quant {
                    quantifier: q,
                    var: None,
                    source: Box::new(node),
                    body: Box::new(body),
                };
            }
        }
        Ok(node)
    }

    fn prefix_call(&mut self) -> Result<Expr, ParseError> {
        let name = self.bump().text;
        self.expect(TokenKind::Punct, "(")?;
        if name == "len" {
            let inner = self.operand()?;
            self.expect(TokenKind::Punct, ")")?;
            self.census.bump("len", 1);
            return Ok(Expr::Len(Box::new(inner)));
        }
        let var = if self.peek().kind == TokenKind::Ident {
            self.bump().text
        } else {
            let t = self.peek();
            return Err(ParseError {
                kind: ParseErrorKind::Expected,
                pos: t.pos,
                detail: format!("expected a bound variable, got {:?}", t.text),
            });
        };
        self.expect(TokenKind::Ident, "in")?;
        let source = self.operand()?;
        self.expect(TokenKind::Punct, ",")?;
        let body = self.or_expr()?;
        self.expect(TokenKind::Punct, ")")?;
        let q = Quantifier::from_token(&name).expect("CALL_NAMES minus len is the three");
        self.census.bump(&format!("quant_{name}"), 1);
        self.census.bump("quant_prefix", 1);
        Ok(Expr::Quant {
            quantifier: q,
            var: Some(var),
            source: Box::new(source),
            body: Box::new(body),
        })
    }

    fn value(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::Ref => {
                let tok = self.bump();
                self.census.bump("ref", 1);
                let mut node = Expr::Ref(tok.text);
                while self.at(TokenKind::Punct, Some(".")) && self.peek_at(1).text == "len" {
                    self.bump();
                    self.bump();
                    self.expect(TokenKind::Punct, "(")?;
                    self.expect(TokenKind::Punct, ")")?;
                    self.census.bump("len", 1);
                    node = Expr::Len(Box::new(node));
                }
                Ok(node)
            }
            TokenKind::Str => {
                self.census.bump("quoted", 1);
                let tok = self.bump();
                Ok(Expr::Lit(Literal::Str(lexer::unquote(&tok.text))))
            }
            TokenKind::Num => Ok(Expr::Lit(Literal::Num(self.bump().text))),
            TokenKind::Ident => {
                let tok = self.bump();
                match tok.text.as_str() {
                    "true" => Ok(Expr::Lit(Literal::True)),
                    "false" => Ok(Expr::Lit(Literal::False)),
                    "null" => {
                        self.census.bump("null", 1);
                        Ok(Expr::Lit(Literal::Null))
                    }
                    other if KEYWORDS.contains(&other) => Err(ParseError {
                        kind: ParseErrorKind::KeywordInValuePosition,
                        pos: tok.pos,
                        detail: format!("keyword {other:?} in value position"),
                    }),
                    // ADR-0103 decision 2: an unquoted identifier in VALUE position is a literal,
                    // never a field. 191 live sites read one way only because of this line.
                    other => {
                        self.census.bump("bare_literal", 1);
                        Ok(Expr::Lit(Literal::Bare(other.to_owned())))
                    }
                }
            }
            TokenKind::Punct if self.peek().text == "[" => self.list(),
            _ => {
                let t = self.peek();
                Err(ParseError {
                    kind: ParseErrorKind::Value,
                    pos: t.pos,
                    detail: format!("value expected, got {:?}", t.text),
                })
            }
        }
    }

    fn list(&mut self) -> Result<Expr, ParseError> {
        self.expect(TokenKind::Punct, "[")?;
        let mut items = Vec::new();
        while !self.at(TokenKind::Punct, Some("]")) {
            items.push(self.value()?);
            if self.at(TokenKind::Punct, Some(",")) {
                self.bump();
            }
        }
        self.expect(TokenKind::Punct, "]")?;
        Ok(Expr::List(items))
    }
}
