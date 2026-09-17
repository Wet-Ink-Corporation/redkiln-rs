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

// ------------------------------------------------------------ AC-250: no consumer of gate_answer_void writes a transition

/// **AC-250's architecture rule** — *"the codebase contains no merge, re-application or
/// deferred-reconciliation path that consumes a `gate_answer_void` event as an input to a state
/// transition"*, verified by *"an architecture test that no consumer of `gate_answer_void` writes
/// a state transition"*.
///
/// **What a consumer is here.** Any `match` arm, `if let`/`if` branch or `|`-grouped arm whose
/// head names `GateAnswerVoid` (the enum variant) or `gate_answer_void` (the wire name), together
/// with the block it selects. The rule extracts that block by brace matching from the first `=>`
/// or `{` after the naming line and scans its **code** (comments stripped) for the transition
/// writes below. The universal half of AC-250 is this rule; the existential half is the generated
/// interleavings in `tests/it/gates.rs`.
///
/// **Why the rule lives here and not in `workflow::fsm` or `workflow::gate`.** It must contain the
/// very tokens it searches for, so a rule written inside the module it polices would fire on its
/// own implementation (`s7b-ii` §7b-ii.12; `s7a`:1032-1039's self-referential shape). This module
/// is the house source-walk lint, outside both paths.
///
/// **Its positive control is committed**: `tests/fixtures/gates/ac250_reconcile_branch.rs` is
/// ADR-0017's rejected option written as a fold arm — a consumer of `gate_answer_void` that
/// re-applies the answer and advances the stage — and `ac250_no_reconciliation_path` asserts the
/// rule reports it. A lint with no fixture is green over a gate path nobody has written.
pub const TRANSITION_WRITES: &[&str] = &[
    "state.stage = ",
    ".stage = ",
    "stages_entered.push",
    "last_transition = Some",
    "committed.insert",
    "stage_transition_event",
    "AdvanceStage",
    "EventKind::StageTransition",
];

/// What [`void_consumer_rule`] walked, so a green can be read against a floor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoidConsumerReport {
    pub files_walked: usize,
    /// Consumer blocks found and scanned. A rule that found none over `src/` walked the wrong tree:
    /// the fold's own `GateAnswerVoid` arm exists and must be among them.
    pub consumers_scanned: usize,
    pub code_lines_scanned: usize,
}

const VOID_NAMES: &[&str] = &["GateAnswerVoid", "gate_answer_void"];

/// Find the block a consumer head selects: from `start` (the byte offset of the naming line),
/// skip to the first `=>` or `{`, then brace-match. Returns the block's byte range, or `None` if
/// the head is not followed by a block within the window (a bare `| EventKind::GateAnswerVoid`
/// line inside a grouped arm is followed by more heads and then the arm's own `=>`).
fn consumer_block(text: &str, start: usize) -> Option<(usize, usize)> {
    let window_end = text.len().min(start + 4_000);
    let window = &text[start..window_end];
    let arrow = window.find("=>");
    let brace = window.find('{');
    let open = match (arrow, brace) {
        (Some(a), Some(b)) if a < b => {
            // `=> {` or `=> expr,` — an expression arm has no braces; take it to the line end.
            let after = &window[a + 2..];
            let nb = after.find(|c: char| !c.is_whitespace())?;
            if after[nb..].starts_with('{') {
                start + a + 2 + nb
            } else {
                let end = after.find('\n').map_or(window.len(), |e| a + 2 + e);
                return Some((start + a + 2, start + end));
            }
        }
        (_, Some(b)) => start + b,
        (Some(a), None) => {
            let end = window[a..].find('\n').map_or(window.len(), |e| a + e);
            return Some((start + a + 2, start + end));
        }
        (None, None) => return None,
    };
    let bytes = text.as_bytes();
    let mut depth = 0_usize;
    let mut i = open;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some((open, i + 1));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// The rule. `Ok` carries what was walked; `Err` carries one violation per transition write
/// found inside a consumer block, naming the file, the line and the write.
///
/// # Errors
///
/// The violations, when any consumer of `gate_answer_void` writes a state transition.
pub fn void_consumer_rule(
    files: &[SourceFile],
) -> Result<VoidConsumerReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut consumers_scanned = 0_usize;
    let mut code_lines_scanned = 0_usize;

    for file in files {
        // Heads are located over comment-stripped text, so a doc comment naming the variant is
        // not a consumer. Stripping does not preserve byte length, so blocks are cut from the
        // ORIGINAL text at the line's original offset and line numbers are re-derived from it.
        let mut in_block = false;
        let mut seen_blocks: Vec<(usize, usize)> = Vec::new();
        let mut offset = 0_usize;
        for raw in file.text.split_inclusive('\n') {
            let code = strip_comments(raw, &mut in_block);
            let names_void = VOID_NAMES.iter().any(|n| code.contains(n));
            // A naming line INSIDE a block already collected is the consumer's own body (the
            // fold's arm reads its payload by wire name), not a second consumer — brace-matching
            // from it would land in the NEXT arm and attribute that arm's writes to this one
            // [measured 2026-09-17: the first cut reported `committed.insert` from the
            // `ActivityCommitted` arm against the `GateAnswerVoid` one].
            let inside_seen = seen_blocks
                .iter()
                .any(|(b0, b1)| offset >= *b0 && offset < *b1);
            if names_void
                && !inside_seen
                && let Some(range) = consumer_block(&file.text, offset)
                && !seen_blocks.contains(&range)
            {
                seen_blocks.push(range);
            }
            offset += raw.len();
        }

        for (b0, b1) in seen_blocks {
            consumers_scanned += 1;
            let block = &file.text[b0..b1];
            // the line number of the block's first byte, 1-based, from the original text
            let first_line = file.text[..b0].matches('\n').count() + 1;
            let mut in_block = false;
            for (i, raw) in block.split_inclusive('\n').enumerate() {
                let code = strip_comments(raw, &mut in_block);
                if code.trim().is_empty() {
                    continue;
                }
                code_lines_scanned += 1;
                for pattern in TRANSITION_WRITES {
                    if code.contains(pattern) {
                        violations.push(BoundaryViolation {
                            path: file.path.clone(),
                            line: first_line + i,
                            pattern,
                            text: code.trim().to_owned(),
                        });
                    }
                }
            }
        }
    }

    if violations.is_empty() {
        Ok(VoidConsumerReport {
            files_walked: files.len(),
            consumers_scanned,
            code_lines_scanned,
        })
    } else {
        Err(violations)
    }
}

// ------------------------------------------------------------ AC-242 / AC-258 / AC-253: the gate path is pure

/// The constructs the gate path may not contain (AC-242's *"thread::sleep, a blocking channel
/// receive, or a timer/scheduler spawn"*; AC-258's *"reads no ambient host state"*; AC-253's
/// *"reads no clock"*), matched on code lines only. `workflow.rs`'s
/// `#![deny(clippy::disallowed_methods)]` already fails the BUILD on `thread::sleep`,
/// `SystemTime::now` and `Instant::now`; this rule is the syntactic net over what that denylist
/// does not name, and its positive control is `tests/fixtures/lint-controls/ac242_gate_path_blocks.rs`.
pub const GATE_PATH_BANS: &[&str] = &[
    "thread::sleep",
    "::sleep(",
    ".recv(",
    ".recv_timeout(",
    "thread::spawn",
    "tokio::",
    "SystemTime",
    "Instant::",
    "std::fs",
    "std::env",
    "process::Command",
];

/// The modules that ARE the gate path: the fold and the answer/expiry module.
pub const GATE_PATH_MODULES: &[&str] = &["workflow/fsm.rs", "workflow/gate.rs"];

/// What [`gate_path_purity_rule`] walked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatePathReport {
    pub files_scanned: usize,
    pub code_lines_scanned: usize,
}

/// The rule. Files whose path ends in one of [`GATE_PATH_MODULES`] are scanned on their code
/// lines (comments stripped) for every [`GATE_PATH_BANS`] token.
///
/// # Errors
///
/// One violation per banned token per code line.
pub fn gate_path_purity_rule(
    files: &[SourceFile],
) -> Result<GatePathReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut files_scanned = 0_usize;
    let mut code_lines_scanned = 0_usize;
    for file in files {
        if !GATE_PATH_MODULES.iter().any(|m| file.path.ends_with(m)) {
            continue;
        }
        files_scanned += 1;
        let mut in_block = false;
        // THE ONE EXEMPTION, scoped structurally and not by name: an item under
        // `#[cfg(feature = "…-reintroduce")]` is a committed positive control for the clippy
        // determinism gate (`workflow-fsm-clock-reintroduce`; DECISIONS-2026-09-12 E4's
        // "committed, not reverted" form) — it EXISTS to carry a banned call, is never compiled
        // by default, and is the reason the gate can be shown to bite. The exemption runs from
        // that attribute to the item's closing `}` at column 0 and nowhere else.
        let mut in_reintroduce_control = false;
        for (i, raw) in file.text.lines().enumerate() {
            let code = strip_comments(raw, &mut in_block);
            if code.trim().is_empty() {
                continue;
            }
            if code.contains("#[cfg(feature = \"") && code.contains("-reintroduce\")") {
                in_reintroduce_control = true;
                continue;
            }
            if in_reintroduce_control {
                if code == "}" {
                    in_reintroduce_control = false;
                }
                continue;
            }
            code_lines_scanned += 1;
            for pattern in GATE_PATH_BANS {
                if code.contains(pattern) {
                    violations.push(BoundaryViolation {
                        path: file.path.clone(),
                        line: i + 1,
                        pattern,
                        text: code.trim().to_owned(),
                    });
                }
            }
        }
    }
    if violations.is_empty() {
        Ok(GatePathReport {
            files_scanned,
            code_lines_scanned,
        })
    } else {
        Err(violations)
    }
}

// ------------------------------------------------------------ the review gate's five source rules

/// What a review-gate source rule walked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRuleReport {
    pub files_scanned: usize,
    pub code_lines_scanned: usize,
}

/// Does `path` name the module `module` — as `module.rs`, `module/mod.rs` or `module/<child>.rs`,
/// under any parent — the way Checkpoint 7b-ii's greps scope by MODULE PATH and never by directory
/// (`s7b-ii` §7b-ii.13: *"admits `policy.rs`, `policy/mod.rs` and `review/policy/*.rs` alike"*)?
fn names_module(path: &str, module: &str) -> bool {
    let p = path.replace('\\', "/");
    p.ends_with(&format!("/{module}.rs")) || p.contains(&format!("/{module}/"))
}

/// Blank the CONTENTS of every double-quoted string literal on one code line, keeping the quotes.
/// A call, a read or an extraction is code; the same spelling inside a string is a message or a
/// key, and a rule that matched it would fire on its own violation text — the self-referential
/// shape `s7a`:1032-1039 records, met again 2026-09-17 when the AC-302 rule reported its own
/// `pattern: "resolve_review_tier(<model value>)"` line. Char literals are left alone (AC-318's
/// `.find('{')` is a char). Escapes inside the literal are skipped with their escaped character.
fn blank_string_literals(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let mut chars = code.chars();
    let mut in_str = false;
    while let Some(c) = chars.next() {
        if in_str {
            match c {
                '\\' => {
                    chars.next();
                }
                '"' => {
                    in_str = false;
                    out.push('"');
                }
                _ => {}
            }
        } else {
            out.push(c);
            if c == '"' {
                in_str = true;
            }
        }
    }
    out
}

/// Scan `files` (those `select`ed) on code lines — string-literal contents blanked — for `bans`,
/// one violation per hit.
fn ban_tokens(
    files: &[SourceFile],
    select: impl Fn(&str) -> bool,
    bans: &[&'static str],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut files_scanned = 0_usize;
    let mut code_lines_scanned = 0_usize;
    for file in files {
        if !select(&file.path) {
            continue;
        }
        files_scanned += 1;
        let mut in_block = false;
        for (i, raw) in file.text.lines().enumerate() {
            let code = blank_string_literals(&strip_comments(raw, &mut in_block));
            if code.trim().is_empty() {
                continue;
            }
            code_lines_scanned += 1;
            for pattern in bans {
                if code.contains(pattern) {
                    violations.push(BoundaryViolation {
                        path: file.path.clone(),
                        line: i + 1,
                        pattern,
                        text: code.trim().to_owned(),
                    });
                }
            }
        }
    }
    if violations.is_empty() {
        Ok(ReviewRuleReport {
            files_scanned,
            code_lines_scanned,
        })
    } else {
        Err(violations)
    }
}

/// **AC-285's architecture lint**: `finding.evidence_label` has no reader inside the policy
/// module. The model's label is retained on the persisted record for drift monitoring and routed
/// nowhere; a policy module that read it would be ADR-0074's rejected *model-chosen evidence
/// class*. Scoped to files that name the `policy` module; the positive control is
/// `tests/fixtures/lint-controls/ac285_policy_reads_the_model_label.rs`.
///
/// # Errors
///
/// One violation per code line in a policy-module file naming `evidence_label`.
pub fn policy_reads_no_model_label_rule(
    files: &[SourceFile],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    ban_tokens(files, |p| names_module(p, "policy"), &["evidence_label"])
}

/// The tokens AC-286 bans inside the classifier module — on the module's own SYNTAX, because
/// *"a crate-level dependency-edge check alone would not catch a transitive route"*.
pub const CLASSIFIER_BANS: &[&str] = &["std::fs", "std::time", "std::env", "SystemTime", "Instant"];

/// **AC-286's source-level lint**: the classifier module reads no clock, no environment variable
/// and no file. Scoped to files that name the `classify` module; the positive control is
/// `tests/fixtures/lint-controls/ac286_classifier_reads_the_clock.rs`.
///
/// # Errors
///
/// One violation per banned token per code line.
pub fn classifier_purity_rule(
    files: &[SourceFile],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    ban_tokens(files, |p| names_module(p, "classify"), CLASSIFIER_BANS)
}

/// The shapes of text-to-JSON extraction AC-318 bans on the review path: parsing a string as
/// JSON, hunting for braces in prose, stripping a code fence, or reaching for a regex.
pub const PROSE_JSON_BANS: &[&str] = &[
    "from_str(",
    ".find('{')",
    ".find(\"{\")",
    "rfind('}')",
    "rfind(\"}\")",
    "trim_start_matches(\"",
    "strip_prefix(\"",
    "Regex",
    "regex::",
];

/// **AC-318's architecture lint**: zero calls into any text-to-JSON extraction helper on the review
/// path (every file under the `review` module). The payload is `structured_output` or it is an
/// error (AC-282); prose is never parsed. The lint-fail fixture is
/// `tests/fixtures/lint-controls/ac318_review_parses_prose_as_json.rs`.
///
/// # Errors
///
/// One violation per banned shape per code line.
pub fn review_path_no_prose_json_rule(
    files: &[SourceFile],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    ban_tokens(files, |p| names_module(p, "review"), PROSE_JSON_BANS)
}

/// The spellings of a model-produced value that may not appear in a `resolve_review_tier(…)`
/// argument list (AC-302; golden constraint 1's standing negative).
pub const MODEL_VALUE_TOKENS: &[&str] = &[
    "structured_output",
    "model",
    "Typed",
    "suggested",
    "payload",
    "result",
    "facet",
    "llm",
];

/// **AC-302's call-graph lint**: every CALL of `resolve_review_tier` in `src/` passes a pack- or
/// catalog-declared rigor and nothing a model produced. The definition line is not a call site;
/// a call site whose argument text names any of [`MODEL_VALUE_TOKENS`] is a violation.
///
/// # Errors
///
/// One violation per offending call line.
pub fn review_tier_call_graph_rule(
    files: &[SourceFile],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut files_scanned = 0_usize;
    let mut code_lines_scanned = 0_usize;
    for file in files {
        files_scanned += 1;
        let mut in_block = false;
        for (i, raw) in file.text.lines().enumerate() {
            let code = blank_string_literals(&strip_comments(raw, &mut in_block));
            if code.trim().is_empty() {
                continue;
            }
            code_lines_scanned += 1;
            let Some(at) = code.find("resolve_review_tier(") else {
                continue;
            };
            // the definition is `fn resolve_review_tier(`; an import or a bare path is not a call
            if code[..at].trim_end().ends_with("fn") || !code[at..].contains(')') {
                continue;
            }
            let args = &code[at + "resolve_review_tier(".len()..];
            if MODEL_VALUE_TOKENS.iter().any(|t| args.contains(t)) {
                violations.push(BoundaryViolation {
                    path: file.path.clone(),
                    line: i + 1,
                    pattern: "resolve_review_tier(<model value>)",
                    text: code.trim().to_owned(),
                });
            }
        }
    }
    if violations.is_empty() {
        Ok(ReviewRuleReport {
            files_scanned,
            code_lines_scanned,
        })
    } else {
        Err(violations)
    }
}

/// What [`block_construction_sites_rule`] walked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockSitesReport {
    pub files_walked: usize,
    /// Construction sites found INSIDE the owning module. A rule that found none walked the wrong
    /// tree: `review/policy.rs` constructs the one block value there is.
    pub sites_inside_owner: usize,
}

/// Is there a block-value CONSTRUCTION on this code line? Anchored on construction, not on the
/// token — `pub struct Block {`, `impl Block {` and a `Block { .. } =>` pattern are not (Checkpoint
/// 7b-ii's own note). The shapes: `Block::new(` / `BlockSet::new(`, and `Block {` / `BlockSet {`
/// preceded by `=` or `(` (an assignment or an argument), which is the checkpoint grep's pattern.
fn constructs_block(code: &str) -> bool {
    for name in ["BlockSet", "Block"] {
        let mut from = 0;
        while let Some(rel) = code[from..].find(name) {
            let at = from + rel;
            let end = at + name.len();
            let before = code[..at].trim_end();
            let word_start = at == 0
                || !code.as_bytes()[at - 1].is_ascii_alphanumeric()
                    && code.as_bytes()[at - 1] != b'_';
            let after = code[end..].trim_start();
            if word_start
                && (after.starts_with("::new")
                    || (after.starts_with('{') && (before.ends_with('=') || before.ends_with('('))))
            {
                return true;
            }
            from = end;
        }
    }
    false
}

/// **AC-294's module-boundary lint**: zero block-value construction sites outside the policy
/// module. The compile-fail case is the evidence; this is the floor beneath it, and
/// `tests/ui/ac294_block_constructor.rs` is its positive control (it constructs a `Block` and must
/// be reported when scanned as a non-policy source).
///
/// # Errors
///
/// One violation per construction site outside a file naming the `policy` module.
pub fn block_construction_sites_rule(
    files: &[SourceFile],
) -> Result<BlockSitesReport, Vec<BoundaryViolation>> {
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    let mut sites_inside_owner = 0_usize;
    for file in files {
        let inside = names_module(&file.path, "policy");
        let mut in_block = false;
        for (i, raw) in file.text.lines().enumerate() {
            let code = blank_string_literals(&strip_comments(raw, &mut in_block));
            if code.trim().is_empty() || !constructs_block(&code) {
                continue;
            }
            if inside {
                sites_inside_owner += 1;
            } else {
                violations.push(BoundaryViolation {
                    path: file.path.clone(),
                    line: i + 1,
                    pattern: "Block construction outside policy",
                    text: code.trim().to_owned(),
                });
            }
        }
    }
    if violations.is_empty() {
        Ok(BlockSitesReport {
            files_walked: files.len(),
            sites_inside_owner,
        })
    } else {
        Err(violations)
    }
}

/// What [`block_path_appends_its_own_event_rule`] found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockPathReport {
    /// Call sites of `review_block_event(` outside its own definition. Exactly one is the invariant.
    pub append_sites: usize,
}

/// **AC-307's architecture lint**: *"the block path and the event append are the same code path"*.
/// The `review_block` row is appended from exactly ONE site in `src/`, and that site sits inside a
/// `Decision::Block` match arm — a second site would let a block be recorded twice or a row be
/// written for no block, and `count(blocks) == count(block_events)` would fail in one direction or
/// the other.
///
/// # Errors
///
/// A violation per append site beyond the first, or for the one site when no `Decision::Block`
/// arm encloses it within the preceding lines.
pub fn block_path_appends_its_own_event_rule(
    files: &[SourceFile],
) -> Result<BlockPathReport, Vec<BoundaryViolation>> {
    let mut sites: Vec<(String, usize, String, bool)> = Vec::new();
    for file in files {
        let mut in_block = false;
        let lines: Vec<String> = file
            .text
            .lines()
            .map(|l| blank_string_literals(&strip_comments(l, &mut in_block)))
            .collect();
        for (i, code) in lines.iter().enumerate() {
            if !code.contains("review_block_event(") || code.contains("fn review_block_event(") {
                continue;
            }
            // the enclosing arm: a `Decision::Block(` within the six preceding code lines
            let enclosed = lines[i.saturating_sub(6)..i]
                .iter()
                .any(|l| l.contains("Decision::Block("));
            sites.push((file.path.clone(), i + 1, code.trim().to_owned(), enclosed));
        }
    }
    let mut violations: Vec<BoundaryViolation> = Vec::new();
    for (idx, (path, line, text, enclosed)) in sites.iter().enumerate() {
        if idx > 0 {
            violations.push(BoundaryViolation {
                path: path.clone(),
                line: *line,
                pattern: "a second review_block append site",
                text: text.clone(),
            });
        } else if !enclosed {
            violations.push(BoundaryViolation {
                path: path.clone(),
                line: *line,
                pattern: "review_block appended outside the Decision::Block arm",
                text: text.clone(),
            });
        }
    }
    if sites.is_empty() {
        violations.push(BoundaryViolation {
            path: "crates/rk-kernel/src/review/confirm.rs".to_owned(),
            line: 0,
            pattern: "no review_block append site at all",
            text: "a block path that writes no row is count(blocks) > count(block_events)"
                .to_owned(),
        });
    }
    if violations.is_empty() {
        Ok(BlockPathReport {
            append_sites: sites.len(),
        })
    } else {
        Err(violations)
    }
}

/// The ledger vocabulary the review module may not name (AC-315: *"a review can only add
/// findings, never mark an AC verified"*).
pub const LEDGER_TOKENS: &[&str] = &[
    "AcLedger",
    "LedgerStatus",
    "AcLedgerRow",
    "AssertionLocator",
    "::Pinned",
    "ledger::",
];

/// **AC-315's architecture lint**: no file of the review module names a ledger type or status.
///
/// # Errors
///
/// One violation per ledger token per code line in a `review` file.
pub fn review_touches_no_ledger_rule(
    files: &[SourceFile],
) -> Result<ReviewRuleReport, Vec<BoundaryViolation>> {
    ban_tokens(files, |p| names_module(p, "review"), LEDGER_TOKENS)
}
