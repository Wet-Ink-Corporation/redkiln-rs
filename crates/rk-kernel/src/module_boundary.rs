//! **AC-294's third instrument** — *"a module-boundary architecture lint asserting zero
//! construction sites outside"* the module that owns the type.
//!
//! > **AC-294**, `THEN`: *"the build fails if a block value is constructed anywhere outside
//! > `rk-kernel::policy`, and every block value that does exist originated in a
//! > `(class, stage_weight)` lookup."*
//! > *Verified by:* *"a private constructor plus a **module-boundary architecture lint asserting
//! > zero construction sites** outside `rk-kernel::policy`; a compile-fail test (`trybuild`) that
//! > adds one."*
//!
//! # 🛑 WHAT THIS RULE'S SUBJECT IS, AND WHAT IT IS NOT
//!
//! **It is not a block value, because this tree has none** [measured 2026-09-15, code lines only:
//! `grep -riE '\bBlockValue\b|\bstage_weight\b|mod policy|ReviewWeight'` over `crates/**/*.rs`
//! returns **0**]. There is no `policy` module either; `lib.rs`:298 quotes `s7b-ii`:534-539 on one
//! that does not exist yet. A lint over a type this tree does not have would return `Ok` over
//! nothing, which is the *"green that pins nothing"* this programme pays for.
//!
//! **Its subject is the `--stay` class state the criterion is extended to carry**: `RunState`, the
//! type `tests/ui/stay_class_state_is_unconstructible.rs` names, whose owning module is
//! `crate::workflow` — exactly the scope `RunState::initial`'s `pub(in crate::workflow)` declares.
//! The block-value half is **ARMED, NOT DISCHARGED**, and `tests/it/module_boundary.rs`'s
//! WHAT-IS-MEASURED table states it by clause. When `rk-kernel::policy` exists, its block value
//! takes a second entry beside this one; it does not take this one's green.
//!
//! # Why a source rule, when a `trybuild` case already exists
//!
//! The `trybuild` case proves the type cannot be constructed **from outside the crate**. It is
//! structurally silent about `crates/rk-kernel/src/runtime.rs` — inside the crate, outside
//! `crate::workflow`, and where a widening from `pub(in crate::workflow)` to `pub(crate)` would
//! land without a single compile error anywhere. `RequiredSet`'s doctrine one module over
//! (`src/select.rs`:97-105) is exact: *"A `pub(crate)` constructor, or a `#[cfg(test)]` public one,
//! silently converts it from a type fact into a convention."* This rule reads the in-crate half.
//!
//! # 🛑 IT SCANS CODE LINES ONLY
//!
//! Three guards in this codebase were once satisfied by the comments explaining them, and this file
//! family documents its own bans by name — a rule that counted a construction site mentioned in a
//! doc comment would be that defect a fourth time. Every line goes through
//! [`crate::kernel_denylist::strip_comments`] first, and
//! `tests/fixtures/lint-controls/ac294_runstate_construction_control.rs` carries every banned
//! spelling in a doc comment, a line comment **and** a block comment so the test can assert none of
//! them is reported — and can show the naive comment-blind scan over-reporting on the same bytes.
//!
//! # The one marked, counted exemption, and why a region alone would not do
//!
//! This module's own [`CONSTRUCTION_PATTERNS`] table carries the banned spellings as **string
//! literals on code lines**, and `strip_comments` preserves string literals deliberately. Without
//! an exemption the rule would fire on its own implementation — `s7b-ii`:534-539's general case.
//! The exemption is [`TABLE_BEGIN`]/[`TABLE_END`] on [`crate::kernel_denylist`]'s exact discipline:
//! **whole-line equality** for the markers (a `contains` rule would make the marker *declarations*
//! open the region), and inside the region only lines shaped like a table **row** are skipped. A
//! forgotten [`TABLE_END`], or a construction site written between the markers, is therefore not a
//! bypass. [`BoundaryReport::table_lines_skipped`] counts what was exempted, and the test asserts
//! its size.
//!
//! **The exemption is deliberately narrow, and the rule proved it on itself.** On the first green
//! run [measured 2026-09-15] this module was its own only violation:
//! `crates/rk-kernel/src/module_boundary.rs:221 `RunState {`` — the matcher had compared against
//! the braced spelling by writing it out, on a code line outside the region. The repair was to
//! discriminate on the spelling's **shape** (`pattern.ends_with('{')`) rather than to widen the
//! exemption, because an exemption wide enough to cover the matcher is an exemption wide enough to
//! cover a construction site.
//!
//! # A boundary, not a ban
//!
//! `fsm.rs` constructs a `RunState` and must: [`crate::workflow::fsm::fold`] is the only public
//! route to a value. The rule refuses construction **outside** [`OWNER_PREFIX`], and the test scans
//! the control fixture's identical bytes at a path *inside* it and asserts zero violations, so the
//! two criteria cannot be confused.
//!
//! # What this rule cannot see, stated rather than left to be discovered
//!
//! It is a **substring** rule over stripped code, so it reaches the three spellings the `trybuild`
//! case names and no others: a construction routed through a type alias (`type S = RunState;` then
//! `S::initial(…)`), through a generic parameter, or through a macro expansion is outside it. Each
//! would also need a widening of `initial`'s visibility to compile, which is the property the
//! `trybuild` case pins from the other side — but the rule is a **denylist of spellings** and is
//! reported as one, never as a proof that no route exists.

pub use crate::kernel_denylist::{
    KERNEL_SRC_PREFIX, SourceFile, kernel_src_root, read_rust_sources, strip_comments,
};

/// One construction site outside the owning module.
///
/// AC-294's THEN is *"the build fails"*, so a violation carries the location an operator opens —
/// file, line, and the spelling that matched — and never merely a count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryViolation {
    path: String,
    line: usize,
    pattern: &'static str,
    text: String,
}

impl BoundaryViolation {
    /// The file, repository-relative.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.path
    }

    /// The 1-based line within it.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// The construction spelling that matched.
    #[must_use]
    pub const fn pattern(&self) -> &'static str {
        self.pattern
    }

    /// The offending line, trimmed, so a report is readable without opening the file.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for BoundaryViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: `{}` constructs a `RunState` outside `crate::workflow` (AC-294) — {}",
            self.path, self.line, self.pattern, self.text
        )
    }
}

impl std::error::Error for BoundaryViolation {}

/// What a clean run walked, so a green cannot be a green over nothing.
///
/// The same premise [`crate::kernel_denylist::DenylistReport`] carries: a rule that returns `Ok`
/// having read zero files is indistinguishable from a rule that returns `Ok` having read the crate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BoundaryReport {
    /// `.rs` files handed to the rule.
    pub files_walked: usize,
    /// Lines that survived comment-stripping and were searched.
    pub code_lines_scanned: usize,
    /// Lines that were wholly comment (or blank after stripping) and were not searched.
    pub comment_lines_skipped: usize,
    /// Lines skipped by the one marked exemption — the markers and the table rows between them.
    pub table_lines_skipped: usize,
    /// Construction sites found **inside** the owning module. A floor on this is what stops a rule
    /// that matches nothing from reporting a clean crate.
    pub sites_inside_owner: usize,
    /// How many spellings the table held, so a table emptied by accident shows up as a zero rather
    /// than as a clean run.
    pub patterns_checked: usize,
}

/// The line that opens the one exempt region, matched as a **whole trimmed line**.
///
/// Whole-line equality, not `contains`, on [`crate::kernel_denylist::TABLE_BEGIN`]'s reasoning:
/// this very declaration carries the marker's text, and a `contains` rule would make the
/// declaration itself open the region.
pub const TABLE_BEGIN: &str = "// rk-kernel-runstate-table: begin";

/// The line that closes it, on the same whole-line rule.
pub const TABLE_END: &str = "// rk-kernel-runstate-table: end";

/// The only line shape the exempt region covers.
///
/// **The region does not exempt whatever is inside it.** A line between the markers is skipped only
/// if it is shaped like a row of [`CONSTRUCTION_PATTERNS`]; anything else is scanned exactly as if
/// the markers were not there.
const TABLE_ROW_PREFIX: &str = "\"RunState";

/// The construction spellings the rule bans outside the owning module.
///
/// **All three routes the `trybuild` case names**, and the count is not incidental:
/// `tests/ui/stay_class_state_is_unconstructible.rs` closes the constructor, `Default` and the
/// struct literal, and a source rule watching fewer than three would leave the unwatched one as the
/// route a later widening takes.
pub const CONSTRUCTION_PATTERNS: &[&str] = &[
    // rk-kernel-runstate-table: begin
    "RunState::initial(",
    "RunState {",
    "RunState::default()",
    // rk-kernel-runstate-table: end
];

/// The owning module's directory, `/`-separated and **with** its trailing slash.
///
/// The slash is load-bearing: without it a sibling `workflow_helpers.rs` would read as inside the
/// boundary. `tests/it/module_boundary.rs` asserts that case by name.
pub const OWNER_PREFIX: &str = "crates/rk-kernel/src/workflow/";

/// The owning module's own file, which is inside `crate::workflow` and outside [`OWNER_PREFIX`].
pub const OWNER_FILE: &str = "crates/rk-kernel/src/workflow.rs";

/// Whether `path` is inside the module that owns `RunState` — the scope
/// `pub(in crate::workflow)` names, and nothing wider.
#[must_use]
pub fn is_owner(path: &str) -> bool {
    path.starts_with(OWNER_PREFIX) || path == OWNER_FILE
}

/// Whether a `RunState {` match at `offset` is a **declaration or a return type** rather than a
/// struct-literal construction.
///
/// `pub fn f() -> RunState {`, `pub struct RunState {`, `impl RunState {` and
/// `impl Trait for RunState {` all carry the banned spelling and none constructs anything, so a
/// bare substring rule would report four false positives in `fsm.rs` alone. The test's control
/// fixture carries every one of these shapes.
///
/// The discrimination is over the **last whitespace-separated token before the match**, not over a
/// `ends_with`, so an identifier merely ending in `for` or `impl` does not exempt a real literal.
fn is_declaration_context(code: &str, offset: usize) -> bool {
    const KEYWORDS: &[&str] = &["->", "struct", "impl", "for", "enum", "union", "as", "dyn"];
    code.get(..offset)
        .map(str::trim_end)
        .and_then(|before| before.split_whitespace().next_back())
        .is_some_and(|last| KEYWORDS.contains(&last))
}

/// Every construction site on one already-stripped code line, as `(pattern, byte offset)`.
fn matches_on_code_line(code: &str) -> Vec<&'static str> {
    let mut hits = Vec::new();
    for pattern in CONSTRUCTION_PATTERNS {
        let mut from = 0_usize;
        while let Some(rel) = code[from..].find(pattern) {
            let at = from + rel;
            // THE DISCRIMINATOR IS THE SPELLING'S SHAPE, NOT THE SPELLING. Writing
            // `*pattern == <the braced token>` here would put the banned token on a code line
            // OUTSIDE the exempt region, and this rule would fire on its own matcher -- which it
            // did, at this exact line, on the first green run [measured 2026-09-15]. Only a
            // brace-terminated spelling can be a struct literal, so only it needs the check.
            if !(pattern.ends_with('{') && is_declaration_context(code, at)) {
                hits.push(*pattern);
                break;
            }
            from = at + pattern.len();
        }
    }
    hits
}

/// The line counts one pass produced.
#[derive(Debug, Default, Clone, Copy)]
struct LineCounts {
    code: usize,
    comment: usize,
    table: usize,
    sites: usize,
}

/// The single pass every public entry point shares, so the counts a report carries are counted by
/// the same loop that finds the sites and cannot drift from them.
///
/// `owner` decides whether a site is a **violation** or a lawful construction inside the boundary;
/// both are found by the same matcher, so the two readings cannot disagree.
fn scan_counted(path: &str, text: &str) -> (Vec<BoundaryViolation>, LineCounts) {
    let owner = is_owner(path);
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
        if in_table && trimmed.starts_with(TABLE_ROW_PREFIX) {
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

        for pattern in matches_on_code_line(&code) {
            if owner {
                counts.sites += 1;
            } else {
                found.push(BoundaryViolation {
                    path: path.to_owned(),
                    line: idx + 1,
                    pattern,
                    text: trimmed.to_owned(),
                });
            }
        }
    }

    (found, counts)
}

/// Scan one file's text and return every construction site on a code line **outside** the owning
/// module.
///
/// Pure: no IO, no clock, no ordering dependence. `path` decides the boundary and is reported
/// verbatim on every violation.
#[must_use]
pub fn scan_source(path: &str, text: &str) -> Vec<BoundaryViolation> {
    scan_counted(path, text).0
}

/// **AC-294's module-boundary rule.** `Ok` with what it walked, or every violation it found.
///
/// # Errors
/// One [`BoundaryViolation`] per construction spelling per code line outside [`OWNER_PREFIX`], in
/// file-then-line order — never the first one only, because an operator repairing an introduced
/// construction site needs the whole set.
pub fn run_state_boundary_rule(
    files: &[SourceFile],
) -> Result<BoundaryReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut totals = LineCounts::default();

    for file in files {
        let (found, counts) = scan_counted(&file.path, &file.text);
        violations.extend(found);
        totals.code += counts.code;
        totals.comment += counts.comment;
        totals.table += counts.table;
        totals.sites += counts.sites;
    }

    if violations.is_empty() {
        Ok(BoundaryReport {
            files_walked: files.len(),
            code_lines_scanned: totals.code,
            comment_lines_skipped: totals.comment,
            table_lines_skipped: totals.table,
            sites_inside_owner: totals.sites,
            patterns_checked: CONSTRUCTION_PATTERNS.len(),
        })
    } else {
        Err(violations)
    }
}

/// This crate's own sources, labelled with **repository-relative** paths.
///
/// [`crate::kernel_denylist::kernel_sources`]' walk and its prefixing, re-used rather than written
/// a second time: two walkers over one tree are two answers that can disagree.
///
/// # Errors
/// Any read failure under [`kernel_src_root`], unchanged.
pub fn kernel_sources() -> std::io::Result<Vec<SourceFile>> {
    crate::kernel_denylist::kernel_sources()
}
