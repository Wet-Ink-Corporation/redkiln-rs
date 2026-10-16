//! Which paths a range's textual diff carries a TEXT patch of (ADR-0199 decision 2; AC-497's
//! AND), read once, the way git writes a diff.
//!
//! The diff carries git's default `a/` and `b/` prefixes. git C-quotes a name holding a `"`, a
//! backslash, a control byte or (under `core.quotePath`, on by default) a non-ASCII byte:
//! `"a/Caf\303\251.uasset"`. It writes a TAB after an unquoted `---`/`+++` name holding a space.
//! A rename's or copy's `diff --git` line names both of its paths, a-side and b-side, and either
//! one counts. Inside a hunk, a removed line `-- a/x` reads `--- a/x`, so a `---`/`+++` line is a
//! header only outside a hunk, and a hunk's extent is its header's counts. A quoted name that
//! cannot be read (its quote never closes, or it holds an escape git does not write) might name
//! any path, so it counts as naming every one.

use std::collections::BTreeSet;

/// The paths a diff patches as text: each `--- a/<path>` or `+++ b/<path>` header outside a
/// hunk, and each path a `diff --git` section names on its a-side or b-side (a rename's or
/// copy's two paths both) when the section holds a hunk (`@@`) or a `GIT binary patch`.
///
/// A hunk header whose counts do not parse leaves the lines after it read as headers, a hunk
/// that ends before its counts are spent marks the diff unreadable, and a quoted name that
/// cannot be read covers every path, so a malformed diff can only make the check refuse more,
/// never less.
pub(super) struct TextPatched {
    paths: BTreeSet<Vec<u8>>,
    /// A header or section that patches as text named a path the reading could not recover.
    unreadable: bool,
}

impl TextPatched {
    /// Reads `diff` once.
    pub(super) fn read(diff: &str) -> Self {
        let mut patched = Self {
            paths: BTreeSet::new(),
            unreadable: false,
        };
        let mut section = Names::default();
        let mut hunk: Option<HunkLeft> = None;
        // `lines` splits on `\n` and on `\r\n`, so a CRLF diff reads as an LF one.
        for line in diff.lines() {
            if let Some(left) = hunk.as_mut() {
                if left.consume(line) {
                    continue;
                }
                // A hunk that ends before its counts are spent overstated them, and may have
                // swallowed a header as a hunk line.
                patched.unreadable |= !left.is_spent();
            }
            hunk = None;
            match Line::of(line) {
                Line::Section(names) => section = section_names(names),
                Line::File { name, prefix } => match file_name(name, prefix) {
                    Ok(Some(path)) => {
                        patched.paths.insert(path);
                    }
                    Ok(None) => {}
                    Err(Unreadable) => patched.unreadable = true,
                },
                Line::Hunk => {
                    patched.absorb(&mut section);
                    hunk = HunkLeft::parse(line);
                }
                Line::BinaryPatch => patched.absorb(&mut section),
                Line::Other => {}
            }
        }
        patched.unreadable |= hunk.is_some_and(|left| !left.is_spent());
        patched
    }

    /// Whether the diff patches `path` as text.
    pub(super) fn covers(&self, path: &str) -> bool {
        self.unreadable || self.paths.contains(path.as_bytes())
    }

    /// The section patches as text: every path it names is patched. Its names move into the set,
    /// so a later hunk of the same section adds nothing it has not already added.
    fn absorb(&mut self, section: &mut Names) {
        let Names { paths, unreadable } = std::mem::take(section);
        self.paths.extend(paths);
        self.unreadable |= unreadable;
    }
}

/// The paths a `diff --git` line names, and whether one of its quoted names could not be read.
#[derive(Default)]
struct Names {
    paths: Vec<Vec<u8>>,
    unreadable: bool,
}

/// A quoted name that cannot be read: its quote never closes, or it holds an escape git does
/// not write.
struct Unreadable;

/// What a line outside a hunk is.
enum Line<'a> {
    /// `diff --git <a> <b>`: the text after the prefix.
    Section(&'a str),
    /// `--- <name>` (prefix `a/`) or `+++ <name>` (prefix `b/`).
    File { name: &'a str, prefix: &'static str },
    /// `@@ …`: a hunk header.
    Hunk,
    /// `GIT binary patch`: git's textual encoding of a binary blob.
    BinaryPatch,
    /// Anything else (`index`, `similarity index`, `rename from`, `Binary files … differ`, …).
    Other,
}

impl<'a> Line<'a> {
    fn of(line: &'a str) -> Self {
        let file = |header: &str, prefix: &'static str| {
            line.strip_prefix(header)
                .map(|name| Self::File { name, prefix })
        };
        line.strip_prefix("diff --git ")
            .map(Self::Section)
            .or_else(|| file("--- ", "a/"))
            .or_else(|| file("+++ ", "b/"))
            .unwrap_or_else(|| {
                if line.starts_with("@@") {
                    Self::Hunk
                } else if line == "GIT binary patch" {
                    Self::BinaryPatch
                } else {
                    Self::Other
                }
            })
    }
}

/// The paths a `diff --git` line names, prefixes stripped, unquoted.
///
/// A quoted a-side ends at its closing quote. An unquoted a-side holding a space makes the split
/// ambiguous (git's own reader says so), so every split before a ` b/` or ` "b/` contributes both
/// of its sides: a candidate too many can only make the check refuse more. A quoted side that
/// cannot be read marks the line unreadable.
fn section_names(names: &str) -> Names {
    let mut found = Names::default();
    if names.starts_with('"') {
        match take_quoted(names) {
            Ok((a, rest)) => {
                push_stripped(&mut found.paths, a, b"a/");
                if let Some(b) = rest.strip_prefix(' ') {
                    found.take(name_bytes(b), b"b/");
                }
            }
            Err(Unreadable) => found.unreadable = true,
        }
        return found;
    }
    for (at, _) in names.match_indices(' ') {
        let Some((a, b)) = names.split_at_checked(at) else {
            continue;
        };
        let Some(b) = b.strip_prefix(' ') else {
            continue;
        };
        if b.starts_with("b/") || b.starts_with("\"b/") {
            push_stripped(&mut found.paths, a.as_bytes().to_vec(), b"a/");
            found.take(name_bytes(b), b"b/");
        }
    }
    found
}

impl Names {
    /// Adds a side's name, its `prefix` stripped, or marks the line unreadable.
    fn take(&mut self, name: Result<Vec<u8>, Unreadable>, prefix: &[u8]) {
        match name {
            Ok(name) => push_stripped(&mut self.paths, name, prefix),
            Err(Unreadable) => self.unreadable = true,
        }
    }
}

/// A `---`/`+++` header's path, its `prefix` stripped: a quoted name ends at its closing quote,
/// an unquoted one at the first TAB (git's marker after a name holding a space, or a plain diff's
/// timestamp). `Ok(None)` for `/dev/null` and any other name without the prefix.
fn file_name(name: &str, prefix: &'static str) -> Result<Option<Vec<u8>>, Unreadable> {
    let bytes = if name.starts_with('"') {
        take_quoted(name)?.0
    } else {
        name.split_once('\t')
            .map_or(name, |(n, _)| n)
            .as_bytes()
            .to_vec()
    };
    Ok(bytes.strip_prefix(prefix.as_bytes()).map(<[u8]>::to_vec))
}

/// A name as git writes it, unquoted when quoted.
fn name_bytes(name: &str) -> Result<Vec<u8>, Unreadable> {
    if name.starts_with('"') {
        take_quoted(name).map(|(bytes, _)| bytes)
    } else {
        Ok(name.as_bytes().to_vec())
    }
}

fn push_stripped(found: &mut Vec<Vec<u8>>, name: Vec<u8>, prefix: &[u8]) {
    if let Some(path) = name.strip_prefix(prefix) {
        found.push(path.to_vec());
    }
}

/// git's C-unquoting of a name that opens with `"`: the bytes up to the closing quote, with
/// `\"`, `\\`, `\a`, `\b`, `\t`, `\n`, `\v`, `\f`, `\r` and three-digit octal escapes undone, and
/// the text after the closing quote. [`Unreadable`] when the quote never closes or an escape is
/// not one git writes.
fn take_quoted(name: &str) -> Result<(Vec<u8>, &str), Unreadable> {
    let mut rest = name.strip_prefix('"').ok_or(Unreadable)?.as_bytes();
    let mut out = Vec::with_capacity(rest.len());
    loop {
        rest = match rest {
            [b'"', tail @ ..] => {
                return std::str::from_utf8(tail)
                    .ok()
                    .map(|tail| (out, tail))
                    .ok_or(Unreadable);
            }
            [
                b'\\',
                d1 @ b'0'..=b'3',
                d2 @ b'0'..=b'7',
                d3 @ b'0'..=b'7',
                tail @ ..,
            ] => {
                out.push(((d1 - b'0') << 6) | ((d2 - b'0') << 3) | (d3 - b'0'));
                tail
            }
            [b'\\', escape, tail @ ..] => {
                out.push(unescape(*escape).ok_or(Unreadable)?);
                tail
            }
            [byte, tail @ ..] => {
                out.push(*byte);
                tail
            }
            [] => return Err(Unreadable),
        };
    }
}

/// The byte a one-character C escape stands for, as git's `quote_c_style` writes them.
const fn unescape(escape: u8) -> Option<u8> {
    Some(match escape {
        b'a' => 0x07,
        b'b' => 0x08,
        b't' => b'\t',
        b'n' => b'\n',
        b'v' => 0x0b,
        b'f' => 0x0c,
        b'r' => b'\r',
        b'"' | b'\\' => escape,
        _ => return None,
    })
}

/// The lines a hunk has left on each side, from its header `@@ -<l>[,<n>] +<l>[,<m>] @@`.
struct HunkLeft {
    old: u64,
    new: u64,
}

impl HunkLeft {
    /// The counts of a hunk header; `None` when they do not parse.
    fn parse(header: &str) -> Option<Self> {
        let ranges = header.strip_prefix("@@ -")?;
        let (old, ranges) = ranges.split_once(" +")?;
        let (new, _) = ranges.split_once(" @@")?;
        Some(Self {
            old: range_count(old)?,
            new: range_count(new)?,
        })
    }

    /// Whether every line the header counted has been read.
    const fn is_spent(&self) -> bool {
        self.old == 0 && self.new == 0
    }

    /// Whether `line` is the hunk's next line, counting it off. A line the counts cannot hold,
    /// or one that is not a hunk line, ends the hunk and is read as a header.
    const fn consume(&mut self, line: &str) -> bool {
        match line.as_bytes().first() {
            // An empty line is a context line whose leading space was stripped.
            Some(b' ') | None => match (self.old.checked_sub(1), self.new.checked_sub(1)) {
                (Some(old), Some(new)) => (self.old, self.new) = (old, new),
                _ => return false,
            },
            Some(b'-') => match self.old.checked_sub(1) {
                Some(old) => self.old = old,
                None => return false,
            },
            Some(b'+') => match self.new.checked_sub(1) {
                Some(new) => self.new = new,
                None => return false,
            },
            // `\ No newline at end of file` belongs to the line before it.
            Some(b'\\') => {}
            Some(_) => return false,
        }
        true
    }
}

/// A unified range's line count: `<l>,<n>` is `n`, a bare `<l>` is 1.
fn range_count(range: &str) -> Option<u64> {
    let (start, count) = range.split_once(',').unwrap_or((range, "1"));
    if decimal(start).is_some() {
        decimal(count)
    } else {
        None
    }
}

fn decimal(digits: &str) -> Option<u64> {
    if !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
        digits.parse().ok()
    } else {
        None
    }
}
