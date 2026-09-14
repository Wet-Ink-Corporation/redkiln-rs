//! The token set, in the probe's own alternation order.
//!
//! Order is behaviour, not style. `spikes/p9/grammar-probe.py`'s `TOKEN` regex tries
//! `ws | ref | str | num | op | punct | ident` at each position and fails if none matches, so `-5`
//! lexes as one number while a bare `-` inside an unquoted identifier lexes as **nothing** and the
//! whole string is a lex error. That is not a corner case: it is F42's second pack defect —
//! `catalog-b/blameless-postmortem.toml`'s `kind in [incident, security-vuln]`, which ADR-0048
//! requires spelled `'security-vuln'` — and the byte position of the failure is pinned by the
//! differential.

/// What a token is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// `${a.b}` or `${a.b?}`; segments admit `-` (step ids such as `green-verify`).
    Ref,
    /// A double- or single-quoted string, retained with its quotes.
    Str,
    /// `-?\d+(\.\d+)?`.
    Num,
    /// One of `== != <= >= < >`.
    Op,
    /// One of `( ) [ ] , .`.
    Punct,
    /// `[a-zA-Z_][a-zA-Z0-9_]*`, keywords included.
    Ident,
    /// End of input.
    Eof,
}

/// A token and the byte offset it starts at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub pos: usize,
}

/// A position at which no token could be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub pos: usize,
    /// Up to twelve bytes of the offending input, as the probe reports them.
    pub snippet: String,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lex error at {}: {:?}", self.pos, self.snippet)
    }
}

impl std::error::Error for LexError {}

/// The eleven words the grammar reserves.
///
/// `len`, `any`, `all` and `none` are here because [`crate::predicate::parser`] refuses them in
/// **value** position — `x == len` is a parse error, not a bare literal — which is the probe's own
/// `KEYWORDS` set and is what keeps the literal-versus-field rule from swallowing a call name.
pub const KEYWORDS: [&str; 11] = [
    "and", "or", "not", "in", "true", "false", "null", "len", "any", "all", "none",
];

/// The four words that may open a call, and that therefore are **not** paths when followed by `(`.
pub const CALL_NAMES: [&str; 4] = ["len", "any", "all", "none"];

const OPS: [&str; 6] = ["==", "!=", "<=", ">=", "<", ">"];

const fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

const fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

const fn is_refseg_continue(b: u8) -> bool {
    is_ident_continue(b) || b == b'-'
}

/// Read a `${…}` reference starting at `start`, returning its end offset.
///
/// `${` `refseg` { `.` `refseg` } [`?`] `}`, where a refseg is `[a-zA-Z_][a-zA-Z0-9_-]*`. A
/// malformed reference is not an error of its own: the alternative simply fails to match, the
/// remaining alternatives are tried at the `$`, and none of them can start there — so the string
/// is a lex error at the `$`, exactly as the probe reports it.
const fn scan_ref(b: &[u8], start: usize) -> Option<usize> {
    let mut i = start + 2;
    loop {
        if i >= b.len() || !is_ident_start(b[i]) {
            return None;
        }
        i += 1;
        while i < b.len() && is_refseg_continue(b[i]) {
            i += 1;
        }
        if i < b.len() && b[i] == b'.' {
            i += 1;
            continue;
        }
        break;
    }
    if i < b.len() && b[i] == b'?' {
        i += 1;
    }
    if i < b.len() && b[i] == b'}' {
        Some(i + 1)
    } else {
        None
    }
}

/// Read a quoted string starting at `start`, returning its end offset. `\\.` is the escape.
const fn scan_string(b: &[u8], start: usize) -> Option<usize> {
    let quote = b[start];
    let mut i = start + 1;
    while i < b.len() {
        match b[i] {
            b'\\' if i + 1 < b.len() => i += 2,
            c if c == quote => return Some(i + 1),
            b'\\' => return None,
            _ => i += 1,
        }
    }
    None
}

/// Read a number starting at `start`, returning its end offset.
const fn scan_number(b: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    if b[i] == b'-' {
        i += 1;
    }
    if i >= b.len() || !b[i].is_ascii_digit() {
        return None;
    }
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i + 1 < b.len() && b[i] == b'.' && b[i + 1].is_ascii_digit() {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
    }
    Some(i)
}

/// Tokenise `src`, appending a single [`TokenKind::Eof`].
///
/// # Errors
///
/// [`LexError`] at the first byte offset no alternative can start at.
pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < b.len() {
        let c = b[pos];
        if c.is_ascii_whitespace() {
            pos += 1;
            continue;
        }
        // Each alternative is a let-chain rather than a nested `if`: a malformed `${…}`, an
        // unterminated string or a lone `-` must FALL THROUGH to the alternatives after it and
        // ultimately to the lex error, exactly as the probe's single regex alternation does.
        if c == b'$'
            && pos + 1 < b.len()
            && b[pos + 1] == b'{'
            && let Some(end) = scan_ref(b, pos)
        {
            out.push(Token {
                kind: TokenKind::Ref,
                text: src[pos..end].to_owned(),
                pos,
            });
            pos = end;
            continue;
        }
        if (c == b'"' || c == b'\'')
            && let Some(end) = scan_string(b, pos)
        {
            out.push(Token {
                kind: TokenKind::Str,
                text: src[pos..end].to_owned(),
                pos,
            });
            pos = end;
            continue;
        }
        if (c == b'-' || c.is_ascii_digit())
            && let Some(end) = scan_number(b, pos)
        {
            out.push(Token {
                kind: TokenKind::Num,
                text: src[pos..end].to_owned(),
                pos,
            });
            pos = end;
            continue;
        }
        if let Some(op) = OPS.iter().find(|o| src[pos..].starts_with(**o)) {
            out.push(Token {
                kind: TokenKind::Op,
                text: (*op).to_owned(),
                pos,
            });
            pos += op.len();
            continue;
        }
        if matches!(c, b'(' | b')' | b'[' | b']' | b',' | b'.') {
            out.push(Token {
                kind: TokenKind::Punct,
                text: (c as char).to_string(),
                pos,
            });
            pos += 1;
            continue;
        }
        if is_ident_start(c) {
            let mut end = pos + 1;
            while end < b.len() && is_ident_continue(b[end]) {
                end += 1;
            }
            out.push(Token {
                kind: TokenKind::Ident,
                text: src[pos..end].to_owned(),
                pos,
            });
            pos = end;
            continue;
        }
        let stop = src
            .char_indices()
            .map(|(i, _)| i)
            .chain(std::iter::once(src.len()))
            .find(|&i| i >= pos + 12)
            .unwrap_or(src.len());
        return Err(LexError {
            pos,
            snippet: src[pos..stop].to_owned(),
        });
    }
    out.push(Token {
        kind: TokenKind::Eof,
        text: String::new(),
        pos: b.len(),
    });
    Ok(out)
}

/// Strip a quoted string's delimiters and resolve its escapes (`\"`, `\'`, `\\`).
///
/// ADR-0103 decision 3 fixes the escape set at exactly those; any other `\x` is kept verbatim
/// rather than silently reinterpreted.
#[must_use]
pub fn unquote(raw: &str) -> String {
    let inner = &raw[1..raw.len().saturating_sub(1)];
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some(e @ ('"' | '\'' | '\\')) => out.push(e),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}
