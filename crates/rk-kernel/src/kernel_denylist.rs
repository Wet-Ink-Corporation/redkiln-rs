//! **AC-487** — the kernel denylist: `rk-kernel` holds no comparison against an adapter identity.
//!
//! # What the criterion says, and what its own note strikes out of it
//!
//! AC-487: *"GIVEN kernel code that must behave differently depending on what an adapter can do /
//! WHEN the kernel lint runs / THEN `rk-kernel` contains no comparison against an adapter identity
//! — no adapter name string, no `Any::downcast`, no adapter-discriminant enum / AND an introduced
//! identity check fails the lint naming the file and line."* *Verified by:* *"a kernel lint
//! (denylist: adapter crate names, `downcast_ref`, `downcast_mut` on port objects); a compile-fail
//! fixture."*
//!
//! 🛑 **The criterion's own note strikes a clause, and this module does not claim it.** Verbatim
//! (`references/2026-09-08-acceptance-criteria.md`:7137-7140): *"the lint proves the **absence** of
//! identity checks; it cannot prove the **presence** of a capability lookup, so the earlier clause
//! 'and the difference is expressed as a capability lookup' is struck as unverifiable by the named
//! instrument. That the lookup exists and is correct is carried by AC-494 and AC-495, which fail if
//! it does not."* design §3.3 P6 (design:284) says the same in one line. **Nothing here asserts that
//! a capability lookup exists**, and a reader who wants that proposition is pointed at AC-494 and
//! AC-495, both of which are `s7b`:1178's hand-forwards by their own names. **Corrected 2026-09-14:
//! this sentence read *"neither of which is built on this tree"* and both now are** —
//! `crate::resolver` and `crate::pack_validation`. Nothing in *this* module changes: the split it
//! records is the point, and the two rules live in two other modules precisely so that an absence
//! rule and a presence rule are never read off one instrument.
//!
//! # Why the lint is a source rule and not a `clippy` entry or a `trybuild` case
//!
//! Neither of the crate's two existing instruments can carry this. `clippy.toml`'s
//! `disallowed-methods` names **paths**, and `downcast_ref` is an inherent method on `dyn Any` whose
//! path is not nameable per-callee; it also has **no path or module scoping** (`clippy.toml`:3-4),
//! so an entry added for the kernel fires across twelve crates including the adapters that must
//! downcast. And `trybuild` observes a **compile failure**, which an identity check is not: a
//! `match adapter_name { "rk-adapter-sqlite" => … }` inside `rk-kernel` compiles perfectly. So the
//! instrument the register names — *"a kernel lint"* — is a source rule, and this is it.
//!
//! The compile-fail fixture the register names **beside** the lint is
//! `tests/ui/ac487_port_identity_is_not_askable.rs`, and it pins a different proposition: that the
//! kernel *cannot* ask a port object what it is, because none of the five port traits carries an
//! `Any` supertrait. The lint says nobody **did**; the fixture says the language **refuses**.
//! Neither is the other, and the test carries both.
//!
//! # 🛑 A NAIVE `contains` OVER THIS CRATE FIRES ON ITS OWN DOCUMENTATION
//!
//! Measured on this tree before a line of the rule was written: `crates/rk-kernel/src/lib.rs`:172
//! and `crates/rk-kernel/src/workflow/retry.rs`:493 both carry the literal `rk-adapter-`, and both
//! are `//!`/`///` prose explaining what the kernel must **not** depend on. A whole-file
//! `contains("rk-adapter-")` therefore reports two violations over a crate that has none — a lint
//! that fires on the sentence describing the ban. The same trap was hit earlier in this session and
//! `tests/it/recorded_log.rs`:199-201 records the fix in one line: *"over CODE lines only"*.
//!
//! So [`scan_source`] strips comments before matching, and [`strip_comments`] is a real scanner
//! rather than a `starts_with("//")` filter: it carries `/* … */` across lines, keeps `//` that sits
//! inside a string literal, and steps over char literals and raw strings so a line such as
//! `src/workflow/breaker.rs`:772 (`l.split('"')`) does not desync it. **Its limits are stated rather
//! than discovered**: string state is reset at every newline, so a string literal spanning lines has
//! its middle lines scanned as code (which can only produce a false **positive**, and the rule is
//! run over the whole crate and must return `Ok`, so such a positive is caught immediately rather
//! than silently weakening the lint).
//!
//! # The one exemption, why it exists, and what bounds it
//!
//! A denylist has to **spell** the tokens it bans, so [`DENYLIST`] contains `"downcast_ref"` and
//! `"rk-adapter-"` as ordinary string literals on ordinary code lines. Scanning this file like any
//! other therefore flags the table itself. Three ways out were open and two were refused:
//!
//! - **Exclude this file from the walk.** Refused: it opens a hole exactly the size of one source
//!   file, and nothing would notice an identity check written into it.
//! - **Spell the tokens as `concat!("downcast", "_ref")`.** Refused: it is the lint evading itself,
//!   the evasion is invisible to a reader, and the technique is available to anyone hiding anything.
//! - **A marked, shape-bounded region.** Taken. [`TABLE_BEGIN`] and [`TABLE_END`] bracket the table
//!   and **only** lines shaped like one of its rows ([`TABLE_ROW_SHAPES`]) are skipped inside them;
//!   `tests/it/kernel_denylist.rs` asserts that exactly one region exists in the whole of
//!   `crates/rk-kernel/src/`, that it is in this file, and — over a synthetic file — that a line
//!   which is **not** table-shaped is flagged even between the markers.
//!
//! 🛑 **The first draft of that exemption was a per-line trailing comment, and `cargo fmt` broke
//! it** [measured 2026-09-14]. Each entry had been one long line ending `// …-denylist-table`;
//! `rustfmt` split every one into five lines and left the marker on the closing brace, exposing
//! seven `pattern: "…"` lines — and the rule immediately reported **9 violations against itself**,
//! naming its own table. The marker count the test asserted was **unchanged** at 8, so the budget
//! assertion did not catch it; the rule's own green over `src/` did. That is why the exemption is
//! now a region whose membership is decided by line **shape** rather than by a comment a formatter
//! is free to move, and why the control in the test is a smuggled line rather than a count.
//!
//! # What the denylist covers, and the clause it can only partly reach
//!
//! The register fixes three entries — *"adapter crate names, `downcast_ref`, `downcast_mut`"* — and
//! [`DENYLIST`] carries seven. The four beyond the floor are each a hole the floor leaves open:
//! `rk_adapter_` (a `use rk_adapter_sqlite::…` carries **no hyphen**, so the crate-name entry alone
//! misses every Rust path form), `downcast::<` and `any::Any` (the THEN's *"no `Any::downcast`"*,
//! which `downcast_ref`/`downcast_mut` do not spell), and `enum Adapter` (the THEN's *"no
//! adapter-discriminant enum"*).
//!
//! 🛑 **The discriminant-enum clause is reached only in part, and that is reported rather than
//! claimed.** A substring rule sees `enum AdapterKind` and `enum AdapterId`; it does not see an
//! adapter-discriminant enum called `enum Backend` or `enum Vendor`. That residue is **ARMED, NOT
//! DISCHARGED** — no instrument in this crate closes it, and the register names none. What is
//! claimed here is the two clauses a denylist can carry (*"no adapter name string"*, *"no
//! `Any::downcast`"*) plus the prefix-shaped half of the third.
//!
//! # What is NOT in the denylist, and why
//!
//! The bare word `adapter` is not banned. This crate's own prose uses it in nearly every module
//! header and the runbook requires it to (`s7b`:1176-1178 names the hand-forwards by that word);
//! banning it would make the rule fire on identifiers such as `adapter_declared_timeout` that carry
//! no identity comparison at all. The rule bans **identity**, not the **topic**.

use std::path::{Path, PathBuf};

/// One `.rs` file as the rule sees it: a crate-relative path and its whole text.
///
/// The rule is pure over a slice of these and does its own IO nowhere, so a caller can drive it
/// over the real tree ([`read_rust_sources`]) or over a fixture string with no filesystem at all.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Relative to the walk root, `/`-separated on every platform.
    pub path: String,
    /// The file's whole text, line endings as they were read.
    pub text: String,
}

/// Which half of AC-487's THEN a denied pattern belongs to.
///
/// Carried on every violation so the report can say *which clause* an introduced check broke, not
/// merely that something matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyClause {
    /// *"no adapter name string"* — an `rk-adapter-*` crate named in kernel code, in either the
    /// manifest spelling or the Rust path spelling.
    AdapterCrateName,
    /// *"`downcast_ref`, `downcast_mut` **on port objects**"* — the register's own two tokens.
    PortDowncast,
    /// *"no `Any::downcast`"* — the trait that makes a downcast possible at all.
    AnyTrait,
    /// *"no adapter-discriminant enum"* — reached by prefix only; see this module's header.
    DiscriminantEnum,
}

impl DenyClause {
    /// The clause as it is quoted in a violation message.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdapterCrateName => "no adapter name string",
            Self::PortDowncast => "no downcast on a port object",
            Self::AnyTrait => "no Any::downcast",
            Self::DiscriminantEnum => "no adapter-discriminant enum",
        }
    }
}

/// One entry of the denylist: the literal to search for, the clause it serves, and why it is here.
#[derive(Debug, Clone, Copy)]
pub struct DeniedPattern {
    /// The literal substring searched for, over code lines only.
    pub pattern: &'static str,
    /// Which half of AC-487's THEN it carries.
    pub clause: DenyClause,
    /// Why this entry exists — quoted into the violation so a reader is not sent to this file.
    pub why: &'static str,
}

/// The line that opens the one exempt region, matched as a **whole trimmed line**.
///
/// Whole-line equality, not `contains`, and the difference is load-bearing: this very declaration
/// carries the marker's text, and a `contains` rule would make the declaration itself open the
/// region. Trimmed, the declaration reads `pub const TABLE_BEGIN: …` and the marker reads
/// `// rk-kernel-denylist-table: begin`, so the two never collide.
pub const TABLE_BEGIN: &str = "// rk-kernel-denylist-table: begin";

/// The line that closes it, on the same whole-line rule.
pub const TABLE_END: &str = "// rk-kernel-denylist-table: end";

/// The only line shapes the exempt region covers.
///
/// **The region does not exempt whatever is inside it.** A line between the two markers is skipped
/// only if it is shaped like a row of the table; anything else is scanned exactly as if the markers
/// were not there. So a forgotten [`TABLE_END`], or an identity check written between the markers,
/// is **not** a bypass — which a line-marker scheme could not say, and a region alone could not
/// either.
const TABLE_ROW_SHAPES: &[&str] = &["DeniedPattern {", "pattern:", "clause:", "why:", "},"];

/// The denylist. Its first three entries are the register's own; the other four are named in this
/// module's header with the hole each one closes.
pub const DENYLIST: &[DeniedPattern] = &[
    // rk-kernel-denylist-table: begin
    DeniedPattern {
        pattern: "rk-adapter-",
        clause: DenyClause::AdapterCrateName,
        why: "the register's own first entry: an adapter crate named in kernel code",
    },
    DeniedPattern {
        pattern: "downcast_ref",
        clause: DenyClause::PortDowncast,
        why: "the register's own second entry",
    },
    DeniedPattern {
        pattern: "downcast_mut",
        clause: DenyClause::PortDowncast,
        why: "the register's own third entry",
    },
    DeniedPattern {
        pattern: "rk_adapter_",
        clause: DenyClause::AdapterCrateName,
        why: "the Rust path spelling, which carries no hyphen (module header)",
    },
    DeniedPattern {
        pattern: "downcast::<",
        clause: DenyClause::AnyTrait,
        why: "the THEN's Any::downcast, which neither _ref nor _mut spells",
    },
    DeniedPattern {
        pattern: "any::Any",
        clause: DenyClause::AnyTrait,
        why: "importing the trait is the only route to the question (module header)",
    },
    DeniedPattern {
        pattern: "enum Adapter",
        clause: DenyClause::DiscriminantEnum,
        why: "the prefix-shaped half; an arbitrary name is ARMED (module header)",
    },
    // rk-kernel-denylist-table: end
];

/// One introduced identity check, **naming the file and the line** — AC-487's THEN, second clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenylistViolation {
    file: String,
    line: usize,
    pattern: &'static str,
    clause: DenyClause,
    why: &'static str,
    text: String,
}

impl DenylistViolation {
    /// The file, relative to the walk root, `/`-separated.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The **1-based** line number, which is what an operator's editor takes.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// The denylist entry that matched.
    #[must_use]
    pub const fn pattern(&self) -> &'static str {
        self.pattern
    }

    /// Which half of AC-487's THEN it broke.
    #[must_use]
    pub const fn clause(&self) -> DenyClause {
        self.clause
    }

    /// The offending source line, trimmed, as it was read.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for DenylistViolation {
    /// `file:line` **first**, because AC-487's THEN is *"fails the lint naming the file and line"*
    /// and an operator reads the location before the reason.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: AC-487 ({}): the denied token `{}` appears in kernel code -- {}; the line reads: {}",
            self.file,
            self.line,
            self.clause.label(),
            self.pattern,
            self.why,
            self.text
        )
    }
}

/// What a clean run walked, so a green cannot be a green over nothing.
///
/// The same premise [`crate::facet_registry::EntryShapeReport`] carries: a rule that returns `Ok`
/// having read zero files is indistinguishable from a rule that returns `Ok` having read the crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DenylistReport {
    /// `.rs` files handed to the rule.
    pub files_walked: usize,
    /// Lines that survived comment-stripping and were searched.
    pub code_lines_scanned: usize,
    /// Lines that were wholly comment (or blank after stripping) and were not searched.
    pub comment_lines_skipped: usize,
    /// Lines skipped because they carry [`TABLE_MARKER`].
    pub table_lines_skipped: usize,
    /// How many patterns the table held, recorded so a denylist emptied by accident shows up
    /// here as a zero rather than as a clean run. See [`DENYLIST`].
    pub patterns_checked: usize,
}

/// Strip comments from one line, carrying block-comment state across lines.
///
/// Returns the **code** of the line: what a compiler would still see. String and char literals are
/// preserved (a `"downcast_ref"` in a string IS code and the rule must see it); `//` and `/* … */`
/// are removed. See this module's header for the two stated limits.
#[must_use]
pub fn strip_comments(line: &str, in_block: &mut bool) -> String {
    let b = line.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0_usize;

    while i < b.len() {
        if *in_block {
            if b[i] == b'*' && i + 1 < b.len() && b[i + 1] == b'/' {
                *in_block = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        let c = b[i];
        let next = b.get(i + 1).copied();

        if c == b'/' && next == Some(b'/') {
            // A line comment, and everything after it. This is the whole point of the module.
            break;
        }
        if c == b'/' && next == Some(b'*') {
            *in_block = true;
            i += 2;
            continue;
        }
        if c == b'"' {
            i = copy_string_literal(b, i, &mut out);
            continue;
        }
        if c == b'r' {
            // A raw string opens as `r`, zero or more `#`, then a quote. Anything else is an
            // ordinary identifier that happens to start with `r`.
            let mut j = i + 1;
            while j < b.len() && b[j] == b'#' {
                j += 1;
            }
            if b.get(j) == Some(&b'"') {
                let hashes = j - i - 1;
                out.extend_from_slice(&b[i..=j]);
                i = copy_raw_string_literal(b, j + 1, hashes, &mut out);
                continue;
            }
        }
        if c == b'\'' {
            // A char literal, or a lifetime. `'\''` and `'\\'` are four bytes; `'"'` is three.
            // A lifetime has no closing quote and falls through to the byte copy below.
            let width = if b.get(i + 1) == Some(&b'\\') && b.get(i + 3) == Some(&b'\'') {
                4
            } else if b.get(i + 1) != Some(&b'\\') && b.get(i + 2) == Some(&b'\'') {
                3
            } else {
                0
            };
            if width > 0 {
                out.extend_from_slice(&b[i..i + width]);
                i += width;
                continue;
            }
        }

        out.push(c);
        i += 1;
    }

    // Lossy, and it costs nothing the rule reads: every denylist pattern is ASCII, so a multi-byte
    // character mangled here cannot move the position of an ASCII match. The violation's `text` is
    // taken from the ORIGINAL line, never from this, so nothing an operator reads is mangled.
    String::from_utf8_lossy(&out).into_owned()
}

/// Copy a `"…"` string literal, escapes and all, and return the index just past it.
///
/// String literals are **kept**: `id == "rk-adapter-sqlite"` is precisely AC-487's *"no adapter
/// name string"*, and a scanner that dropped string contents would be blind to the clause it
/// exists to enforce.
fn copy_string_literal(b: &[u8], start: usize, out: &mut Vec<u8>) -> usize {
    out.push(b'"');
    let mut i = start + 1;
    while i < b.len() {
        if b[i] == b'\\' {
            out.push(b[i]);
            if let Some(&esc) = b.get(i + 1) {
                out.push(esc);
            }
            i += 2;
            continue;
        }
        out.push(b[i]);
        i += 1;
        if b[i - 1] == b'"' {
            return i;
        }
    }
    i
}

/// Copy the body of an `r#"…"#` literal and return the index just past its terminator.
fn copy_raw_string_literal(b: &[u8], start: usize, hashes: usize, out: &mut Vec<u8>) -> usize {
    let mut i = start;
    while i < b.len() {
        if b[i] == b'"' {
            let mut k = 0;
            while k < hashes && b.get(i + 1 + k) == Some(&b'#') {
                k += 1;
            }
            if k == hashes {
                out.extend_from_slice(&b[i..=i + hashes]);
                return i + hashes + 1;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    i
}

/// What one file's scan walked, alongside what it found.
#[derive(Debug, Clone, Copy, Default)]
struct LineCounts {
    code: usize,
    comment: usize,
    table: usize,
}

/// The single pass every public entry point shares, so the counts a report carries are counted by
/// the same loop that finds the violations and cannot drift from them.
fn scan_counted(path: &str, text: &str) -> (Vec<DenylistViolation>, LineCounts) {
    let mut found = Vec::new();
    let mut counts = LineCounts::default();
    let mut in_block = false;
    let mut in_table = false;

    for (idx, raw) in text.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed == TABLE_BEGIN {
            in_table = true;
            counts.table += 1;
            continue;
        }
        if trimmed == TABLE_END {
            in_table = false;
            counts.table += 1;
            continue;
        }
        if in_table && TABLE_ROW_SHAPES.iter().any(|s| trimmed.starts_with(s)) {
            // The exemption, and it reaches table rows and nothing else. Block state is still
            // advanced, because skipping a line is not the same as pretending the compiler never
            // saw it.
            let _ = strip_comments(raw, &mut in_block);
            counts.table += 1;
            continue;
        }

        let code = strip_comments(raw, &mut in_block);
        if code.trim().is_empty() {
            counts.comment += 1;
            continue;
        }
        counts.code += 1;

        for denied in DENYLIST {
            if code.contains(denied.pattern) {
                found.push(DenylistViolation {
                    file: path.to_owned(),
                    line: idx + 1,
                    pattern: denied.pattern,
                    clause: denied.clause,
                    why: denied.why,
                    text: raw.trim().to_owned(),
                });
            }
        }
    }

    (found, counts)
}

/// Scan one file's text and return every denied token on a code line.
///
/// Pure: no IO, no clock, no ordering dependence. `path` is reported verbatim on every violation.
#[must_use]
pub fn scan_source(path: &str, text: &str) -> Vec<DenylistViolation> {
    scan_counted(path, text).0
}

/// Read every `.rs` file under `root`, recursively, sorted by path.
///
/// The one IO in this module, and it is **outside** the rule on purpose: the rule stays pure and a
/// caller can drive it over a fixture with no filesystem.
///
/// # Errors
/// Any read failure under `root`, unchanged.
pub fn read_rust_sources(root: &Path) -> std::io::Result<Vec<SourceFile>> {
    let mut out: Vec<SourceFile> = Vec::new();
    let mut pending: Vec<PathBuf> = vec![root.to_path_buf()];

    while let Some(dir) = pending.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.extension().and_then(std::ffi::OsStr::to_str) != Some("rs") {
                continue;
            }
            // `/`-separated whatever the platform: the crate is developed on Windows and the
            // checkpoint reads these paths, so a `\` here would put two spellings of one file in
            // the record.
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path.as_path())
                .to_string_lossy()
                .replace('\\', "/");
            out.push(SourceFile {
                path: rel,
                text: std::fs::read_to_string(&path)?,
            });
        }
    }

    // `read_dir` order is the filesystem's; a lint that reported its violations in a different
    // order on two boxes would be a lint whose output cannot be compared.
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

/// **AC-487's rule.** `Ok` with what it walked, or every violation it found.
///
/// # Errors
/// One [`DenylistViolation`] per denied token per code line, in file-then-line order — never the
/// first one only, because an operator repairing an introduced identity check needs the whole set.
pub fn kernel_denylist_rule(
    files: &[SourceFile],
) -> Result<DenylistReport, Vec<DenylistViolation>> {
    let mut violations: Vec<DenylistViolation> = Vec::new();
    let mut totals = LineCounts::default();

    for file in files {
        let (found, counts) = scan_counted(&file.path, &file.text);
        violations.extend(found);
        totals.code += counts.code;
        totals.comment += counts.comment;
        totals.table += counts.table;
    }

    if violations.is_empty() {
        Ok(DenylistReport {
            files_walked: files.len(),
            code_lines_scanned: totals.code,
            comment_lines_skipped: totals.comment,
            table_lines_skipped: totals.table,
            patterns_checked: DENYLIST.len(),
        })
    } else {
        Err(violations)
    }
}

/// The crate's own `src/` directory, as the rule's one real subject.
///
/// `CARGO_MANIFEST_DIR` is `crates/rk-kernel`, so this resolves the same from any working directory
/// — which matters because Checkpoint 7b runs `cargo test -p rk-kernel` from the workspace root.
#[must_use]
pub fn kernel_src_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The prefix every path in [`kernel_sources`] carries.
pub const KERNEL_SRC_PREFIX: &str = "crates/rk-kernel/src/";

/// This crate's own sources, labelled with **repository-relative** paths.
///
/// [`read_rust_sources`] reports paths relative to the root it was handed, which for `src/` yields
/// bare names like `runtime.rs`. AC-487's THEN is *"fails the lint **naming the file** and line"*,
/// and `runtime.rs:485` names two files in this repository — `src/runtime.rs` and
/// `tests/it/runtime.rs`. Prefixing here is what makes the violation a location an operator can
/// open, and it is done at the one call site that knows which root it walked rather than by
/// teaching the walk about this crate.
///
/// # Errors
/// Any read failure under [`kernel_src_root`], unchanged.
pub fn kernel_sources() -> std::io::Result<Vec<SourceFile>> {
    let mut files = read_rust_sources(&kernel_src_root())?;
    for file in &mut files {
        file.path.insert_str(0, KERNEL_SRC_PREFIX);
    }
    Ok(files)
}
