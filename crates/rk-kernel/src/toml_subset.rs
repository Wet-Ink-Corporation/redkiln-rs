//! A **subset** TOML reader, and the measurement that says the subset is the whole corpus.
//!
//! The registries this crate's lints read (`facets.toml`), the packs whose `require` rows they
//! read, and the activity catalog the rigor rule walks are all TOML, and this workspace has no TOML
//! dependency. Adding one moves `Cargo.lock`, re-derives the licence set `cargo deny check licenses`
//! reads, and lands a new 0.x crate under ADR-0100 decision 2's exact-pin rule — a dependency
//! decision, and `00-preamble.md`:55 says a stage stops where it is tempted to take one. So this
//! module reads the subset the corpus actually uses, and **refuses everything else with a typed
//! error rather than a best effort**.
//!
//! # What the subset is, and how it was chosen
//!
//! Measured 2026-09-13 over `spikes/p9/`'s **102** catalog files, **12** packs, `catalog-b/`,
//! `facets.toml` and `artifacts.toml`: **no multi-line value, no multi-line array, no multi-line
//! inline table, and no triple-quoted string of either kind occurs anywhere.** Every value sits on
//! one line. The scan counted brackets outside string context per line and asserted the depth
//! returns to zero on every non-header line; it reported nothing.
//!
//! So the reader is line-oriented, and that is a *measured* simplification rather than a hopeful
//! one. What it accepts:
//!
//! - `[a.b]` table headers and `[[a.b]]` array-of-table headers, dotted, unquoted keys only;
//! - `key = value` where the value is a basic string (`"…"`, with the four escapes the corpus
//!   uses), a literal string (`'…'`), an integer (underscores allowed), a float, a boolean, an
//!   array, or an inline table — arrays and inline tables nesting freely **within one line**;
//! - `#` comments, honoured outside string context only.
//!
//! What it refuses, loudly and by line number: a value that does not close on its line, a dotted or
//! quoted bare key, a date, and a duplicate key. **A reader that silently ignores what it cannot
//! parse turns a malformed registry into a passing lint**, which is the failure both facet lints
//! exist to make impossible, so every path out of here is a `Result`.

use std::collections::BTreeMap;
use std::fmt;

/// A table: keys in byte order, which is also the order the lints report in.
pub type Table = BTreeMap<String, Value>;

/// A TOML value, in the subset above.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Self>),
    Table(Table),
}

impl Value {
    /// The string, if this is one.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    /// The boolean, if this is one.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// The integer, if this is one.
    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// The number as an `f64`, whether it was written as an integer or as a float.
    ///
    /// `identity = 1` and `identity = 1.0` are the same reading, and a registry that spells the
    /// stability bar without a decimal point must not read as an absent clause.
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            #[expect(
                clippy::cast_precision_loss,
                reason = "a registry integer that exceeds 2^53 is not a number this reader will \
                          ever meet; the alternative is refusing `n_items = 20`"
            )]
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// The array, if this is one.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }

    /// The table, if this is one.
    #[must_use]
    pub const fn as_table(&self) -> Option<&Table> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }
}

/// What stopped the read, and on which line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TomlError {
    pub line: usize,
    pub detail: String,
}

impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.detail)
    }
}

impl std::error::Error for TomlError {}

/// Where every header and every key was read from: a canonical dotted path to the 1-based line.
///
/// An array-of-tables element is spelled with its index — `stage.review.require[0]` — which is the
/// **same** vocabulary `crate::select::pack`'s `RequireRow::site` is built from, so a caller holding
/// a loaded `Pack` can look a row's span up without a second path convention to keep in sync.
///
/// The header line and the key lines of one table are separate entries: `stage.review.require[0]` is
/// the `[[stage.review.require]]` line and `stage.review.require[0].capability` is the `capability =`
/// line inside it.
pub type Spans = BTreeMap<String, usize>;

/// Read one document.
///
/// # Errors
///
/// [`TomlError`] naming the line, for anything outside the subset this module documents.
pub fn parse(src: &str) -> Result<Table, TomlError> {
    // ONE PARSER. `parse_spans` is the loop and this is a projection of it, rather than two loops
    // that could come to disagree about what a line is — which would put a span on a line the value
    // did not come from, and AC-495's whole THEN is a span location.
    parse_spans(src).map(|(table, _)| table)
}

/// Read one document, **and** the line each header and each key was read at.
///
/// **AC-495's instrument needs a span** — *"validation fails with a span pointing at the requiring
/// row"* — and a line is the grain this reader has: its subset is measured line-oriented (see the
/// module header), so every value in the corpus sits on exactly one line and a line number is an
/// exact location rather than an approximation of one.
///
/// # Errors
///
/// [`TomlError`] naming the line, for anything outside the subset this module documents.
pub fn parse_spans(src: &str) -> Result<(Table, Spans), TomlError> {
    let mut root = Table::new();
    let mut spans = Spans::new();
    let mut prefix = String::new();
    let mut path: Vec<String> = Vec::new();
    for (n, raw) in src.lines().enumerate() {
        let line_no = n + 1;
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        let err = |detail: String| TomlError {
            line: line_no,
            detail,
        };
        if let Some(rest) = line.strip_prefix("[[") {
            let name = rest
                .strip_suffix("]]")
                .ok_or_else(|| err("unterminated [[array-of-tables]] header".to_owned()))?;
            path = split_header(name, line_no)?;
            push_array_table(&mut root, &path).map_err(err)?;
            prefix = canonical_path(&root, &path);
            spans.insert(prefix.clone(), line_no);
        } else if let Some(rest) = line.strip_prefix('[') {
            let name = rest
                .strip_suffix(']')
                .ok_or_else(|| err("unterminated [table] header".to_owned()))?;
            path = split_header(name, line_no)?;
            open_table(&mut root, &path).map_err(err)?;
            prefix = canonical_path(&root, &path);
            spans.insert(prefix.clone(), line_no);
        } else {
            let (key, value_src) = line
                .split_once('=')
                .ok_or_else(|| err(format!("not a key/value line: {line:?}")))?;
            let key = key.trim();
            if key.is_empty() || !key.chars().all(is_bare_key_char) {
                return Err(err(format!(
                    "bare unquoted keys only, and this one is {key:?}"
                )));
            }
            let mut scan = Scan::new(value_src);
            let value = scan.value().map_err(err)?;
            scan.end().map_err(err)?;
            let table = open_table(&mut root, &path).map_err(err)?;
            if table.insert(key.to_owned(), value).is_some() {
                return Err(err(format!("duplicate key {key:?}")));
            }
            let at = if prefix.is_empty() {
                key.to_owned()
            } else {
                format!("{prefix}.{key}")
            };
            spans.insert(at, line_no);
        }
    }
    Ok((root, spans))
}

/// The dotted path a header resolves to, with `[i]` on every array-of-tables segment.
///
/// It walks the **already-built** tree by [`open_table`]'s own rule — descending through an
/// array-of-tables lands in its last element — so the index it writes is the index the value was
/// actually stored at, rather than a count kept in parallel that could drift from it.
fn canonical_path(root: &Table, path: &[String]) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let mut cursor = Some(root);
    for seg in path {
        if !out.is_empty() {
            out.push('.');
        }
        let Some(table) = cursor else {
            // Unreachable through `parse_spans`, which calls this only after the descent succeeded.
            // It degrades to the bare segment rather than panicking, because a span is a diagnostic
            // and a diagnostic that aborts the read is worse than one that is coarse.
            out.push_str(seg);
            continue;
        };
        match table.get(seg) {
            Some(Value::Array(items)) => {
                let _ = write!(out, "{seg}[{}]", items.len().saturating_sub(1));
                cursor = items.last().and_then(Value::as_table);
            }
            Some(Value::Table(t)) => {
                out.push_str(seg);
                cursor = Some(t);
            }
            _ => {
                out.push_str(seg);
                cursor = None;
            }
        }
    }
    out
}

const fn is_bare_key_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn split_header(name: &str, line: usize) -> Result<Vec<String>, TomlError> {
    let parts: Vec<String> = name.split('.').map(|p| p.trim().to_owned()).collect();
    if parts.is_empty() || parts.iter().any(String::is_empty) {
        return Err(TomlError {
            line,
            detail: format!("empty segment in header {name:?}"),
        });
    }
    if !parts.iter().all(|p| p.chars().all(is_bare_key_char)) {
        return Err(TomlError {
            line,
            detail: format!("bare unquoted header segments only, and this one is {name:?}"),
        });
    }
    Ok(parts)
}

/// Descend to the table `path` names, creating tables as it goes. Descending through an
/// array-of-tables lands in its **last** element, which is TOML's own rule.
fn open_table<'a>(root: &'a mut Table, path: &[String]) -> Result<&'a mut Table, String> {
    let mut cursor = root;
    for seg in path {
        let entry = cursor
            .entry(seg.clone())
            .or_insert_with(|| Value::Table(Table::new()));
        cursor = match entry {
            Value::Table(t) => t,
            Value::Array(items) => match items.last_mut() {
                Some(Value::Table(t)) => t,
                _ => return Err(format!("{seg:?} is an array of non-tables")),
            },
            _ => return Err(format!("{seg:?} is a value, not a table")),
        };
    }
    Ok(cursor)
}

fn push_array_table(root: &mut Table, path: &[String]) -> Result<(), String> {
    let (last, parents) = path
        .split_last()
        .ok_or_else(|| "empty [[header]]".to_owned())?;
    let parent = open_table(root, parents)?;
    let entry = parent
        .entry(last.clone())
        .or_insert_with(|| Value::Array(Vec::new()));
    match entry {
        Value::Array(items) => {
            items.push(Value::Table(Table::new()));
            Ok(())
        }
        _ => Err(format!("{last:?} is not an array of tables")),
    }
}

/// Drop a `#` comment, honouring string context so a `#` inside a quoted value survives.
fn strip_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut quote: Option<u8> = None;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        match quote {
            Some(q) => {
                if c == b'\\' && q == b'"' {
                    i += 2;
                    continue;
                }
                if c == q {
                    quote = None;
                }
            }
            None => {
                if c == b'#' {
                    return &line[..i];
                }
                if c == b'"' || c == b'\'' {
                    quote = Some(c);
                }
            }
        }
        i += 1;
    }
    line
}

/// A one-line value scanner.
struct Scan<'a> {
    src: &'a str,
    at: usize,
}

impl<'a> Scan<'a> {
    const fn new(src: &'a str) -> Self {
        Self { src, at: 0 }
    }

    fn rest(&self) -> &'a str {
        &self.src[self.at..]
    }

    fn skip_ws(&mut self) {
        let trimmed = self.rest().trim_start();
        self.at = self.src.len() - trimmed.len();
    }

    fn end(&mut self) -> Result<(), String> {
        self.skip_ws();
        if self.rest().is_empty() {
            Ok(())
        } else {
            Err(format!("trailing input {:?}", self.rest()))
        }
    }

    fn eat(&mut self, c: char) -> bool {
        self.skip_ws();
        if self.rest().starts_with(c) {
            self.at += c.len_utf8();
            true
        } else {
            false
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        self.skip_ws();
        let rest = self.rest();
        let first = rest
            .chars()
            .next()
            .ok_or_else(|| "empty value".to_owned())?;
        match first {
            '"' | '\'' => self.string(first),
            '[' => self.array(),
            '{' => self.inline_table(),
            _ => self.atom(),
        }
    }

    fn string(&mut self, quote: char) -> Result<Value, String> {
        self.at += quote.len_utf8();
        let mut out = String::new();
        let mut chars = self.rest().char_indices();
        while let Some((i, c)) = chars.next() {
            if c == quote {
                self.at += i + c.len_utf8();
                return Ok(Value::Str(out));
            }
            if c == '\\' && quote == '"' {
                let (_, esc) = chars
                    .next()
                    .ok_or_else(|| "string ends inside an escape".to_owned())?;
                out.push(match esc {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    other => return Err(format!("unsupported escape \\{other}")),
                });
            } else {
                out.push(c);
            }
        }
        Err("string does not close on its line".to_owned())
    }

    fn array(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut items = Vec::new();
        loop {
            if self.eat(']') {
                return Ok(Value::Array(items));
            }
            items.push(self.value()?);
            if self.eat(',') {
                continue;
            }
            if self.eat(']') {
                return Ok(Value::Array(items));
            }
            self.skip_ws();
            if self.rest().is_empty() {
                return Err("array does not close on its line".to_owned());
            }
            return Err(format!("expected ',' or ']' at {:?}", self.rest()));
        }
    }

    fn inline_table(&mut self) -> Result<Value, String> {
        self.at += 1;
        let mut table = Table::new();
        loop {
            if self.eat('}') {
                return Ok(Value::Table(table));
            }
            self.skip_ws();
            let rest = self.rest();
            let key_len = rest
                .find(|c: char| !is_bare_key_char(c))
                .unwrap_or(rest.len());
            if key_len == 0 {
                return Err(format!("expected a bare key at {rest:?}"));
            }
            let key = rest[..key_len].to_owned();
            self.at += key_len;
            if !self.eat('=') {
                return Err(format!("expected '=' after {key:?}"));
            }
            let value = self.value()?;
            if table.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate inline key {key:?}"));
            }
            if self.eat(',') {
                continue;
            }
            if self.eat('}') {
                return Ok(Value::Table(table));
            }
            self.skip_ws();
            if self.rest().is_empty() {
                return Err("inline table does not close on its line".to_owned());
            }
            return Err(format!("expected ',' or '}}' at {:?}", self.rest()));
        }
    }

    /// A bare atom: `true`, `false`, an integer or a float.
    fn atom(&mut self) -> Result<Value, String> {
        let rest = self.rest();
        let len = rest
            .find(|c: char| c == ',' || c == ']' || c == '}' || c.is_whitespace())
            .unwrap_or(rest.len());
        let token = &rest[..len];
        self.at += len;
        match token {
            "true" => return Ok(Value::Bool(true)),
            "false" => return Ok(Value::Bool(false)),
            "" => return Err("empty value".to_owned()),
            _ => {}
        }
        let cleaned: String = token.chars().filter(|c| *c != '_').collect();
        if let Ok(i) = cleaned.parse::<i64>() {
            return Ok(Value::Int(i));
        }
        if (cleaned.contains('.') || cleaned.contains('e') || cleaned.contains('E'))
            && let Ok(f) = cleaned.parse::<f64>()
        {
            return Ok(Value::Float(f));
        }
        Err(format!(
            "{token:?} is outside this reader's value subset (no dates, no multi-line values)"
        ))
    }
}

#[cfg(test)]
mod tests {
    /// The duplicate-key rejection this module's header argues for at length, asserted.
    ///
    /// *"A reader that silently skipped what it cannot parse turns a malformed registry into a
    /// passing lint."* Deleting the guard changed nothing any test observed until 2026-09-13.
    #[test]
    fn a_duplicated_key_is_refused_rather_than_last_one_wins() {
        let ok =
            super::parse("[facet.toolchain]\nsource = \"derived\"\n").expect("the control parses");
        assert!(ok.contains_key("facet"));

        let err = super::parse("[facet.toolchain]\nsource = \"derived\"\nsource = \"kernel\"\n")
            .expect_err("a duplicated key is a load failure, never last-one-wins");
        assert!(
            err.to_string().contains("duplicate key"),
            "the message names the defect: {err}"
        );
    }
}
