//! **AC-294's third instrument** — *"a module-boundary architecture lint asserting zero
//! construction sites outside"* the module that owns the type — built 2026-09-15 for the `--stay`
//! class half of the criterion.
//!
//! # WHAT IS MEASURED HERE, AND WHAT IS NOT
//!
//! | AC-294 clause | instrument here | status |
//! | --- | --- | --- |
//! | *"a module-boundary architecture lint asserting zero construction sites outside"* its owning module, for the **`--stay` class** state | [`rk_kernel::module_boundary::run_state_boundary_rule`] over every `.rs` file under `crates/rk-kernel/src/`, **code lines only** | **measured** |
//! | *"a private constructor"* | `RunState::initial` is `pub(in crate::workflow)`, the type derives no `Default` and every field is private | **measured**, by the type |
//! | *"a compile-fail test (`trybuild`) that adds one"* | `tests/ui/stay_class_state_is_unconstructible.rs` + `.stderr`, all three routes | **measured**, by `trybuild` |
//! | *"a block value is constructed anywhere outside `rk-kernel::policy`"*, and *"every block value … originated in a `(class, stage_weight)` lookup"* | **nothing, and nothing here may be read as covering it** | **ARMED, NOT DISCHARGED** — measured 2026-09-15, code lines only: there is **no `policy` module, no block-value type, no `ReviewWeight` and no `stage_weight`** anywhere under `crates/*/src/`. `lib.rs`:298 quotes `s7b-ii`:534-539 on a `policy` module that does not exist yet. A lint over a type this tree does not have would be a green over nothing |
//!
//! 🛑 **The criterion's own subject is a BLOCK VALUE and this tree has none.** What is linted here
//! is the `--stay` class state AC-294 is extended to carry — `RunState`, the type
//! `tests/ui/stay_class_state_is_unconstructible.rs` names — and the boundary is
//! `crate::workflow`, which is exactly the scope `RunState::initial`'s `pub(in crate::workflow)`
//! declares. When `rk-kernel::policy` exists, its block value takes a second table entry beside
//! this one; it does not take this one's green.
//!
//! # Why the boundary rule is not the same claim as the `trybuild` case
//!
//! The `trybuild` case proves the type cannot be constructed **from outside the crate**. It says
//! nothing about `crates/rk-kernel/src/runtime.rs` — which is *inside* the crate, outside
//! `crate::workflow`, and where a `pub(crate)` widening would land silently. `RequiredSet`'s
//! doctrine one module over (`src/select.rs`:97-105) is exact: *"A `pub(crate)` constructor, or a
//! `#[cfg(test)]` public one, silently converts it from a type fact into a convention."* This lint
//! is what reads the in-crate half, and it is the half a compile-fail case structurally cannot.
//!
//! # 🛑 THE LINT SCANS CODE LINES ONLY, AND THE FIXTURE PROVES IT RATHER THAN SAYING IT
//!
//! Three guards in this codebase were satisfied by the comments explaining them, and this file
//! family documents its own bans by name — so a lint that counted a construction site mentioned in
//! a doc comment would be that defect a fourth time. The control fixture carries every banned
//! spelling in a doc comment, a line comment **and** a block comment; part 4 asserts the rule
//! reports **none** of them, and part 5 shows, from inside this binary, that the naive
//! comment-blind scan the rule replaces reports **more** hits over the same bytes. The
//! comment-stripping is therefore measured rather than described.
//!
//! # What would make a green here vacuous, and what stops each
//!
//! * *A rule that matches nothing.* Its report would be `Ok` over the real tree with zero sites
//!   found. Part 1 asserts a **floor** on sites found inside the owner, and part 3 asserts the
//!   control's **exact** violation set — one per banned spelling.
//! * *A walk that found no files.* Part 1 asserts a floor on `files_walked` and on
//!   `code_lines_scanned`, on `kernel_denylist`'s precedent.
//! * *A ban rather than a boundary.* A rule that refused `RunState` construction **everywhere**
//!   would be green on part 3 and would be wrong: `fsm.rs` constructs one. Part 6 scans the
//!   control's own bytes at a path **inside** the owner and asserts **zero** violations.
//! * *A substring match.* `pub fn one() -> RunState {`, `pub struct RunState {` and
//!   `impl RunState {` all carry the `RunState {` spelling and none constructs anything. The
//!   fixture carries all three and part 3's exact set excludes every one.

use rk_kernel::module_boundary::{
    BoundaryViolation, CONSTRUCTION_PATTERNS, OWNER_FILE, OWNER_PREFIX, is_owner, kernel_sources,
    run_state_boundary_rule, scan_source,
};

/// The control's own text, compiled into this binary so every line number below is checked against
/// the same bytes the rule reads. **Never compiled as code** — it lives under
/// `tests/fixtures/lint-controls/`, not under `tests/ui/`, which is the `trybuild` glob root.
const CONTROL: &str =
    include_str!("../fixtures/lint-controls/ac294_runstate_construction_control.rs");

/// A path outside the owning module, for scanning [`CONTROL`] as if it had been committed there.
const OUTSIDE_PATH: &str = "crates/rk-kernel/src/runtime/ledger.rs";
/// The same bytes, at a path **inside** the owning module. Part 6's control.
const INSIDE_PATH: &str = "crates/rk-kernel/src/workflow/fsm.rs";

/// The exact set the control must produce: **one violation per banned spelling**, so a spelling
/// that stopped matching anything is caught here rather than merely carried in the table.
const EXPECTED_CONTROL_HITS: &[(usize, &str)] = &[
    (31, "RunState::initial("),
    (35, "RunState {"),
    (42, "RunState::default()"),
];

/// A file whose ONLY occurrences of every banned spelling are in comments. Zero violations, or the
/// lint is the fourth instance of this codebase's oldest defect.
const COMMENTS_ONLY: &str = r#"//! RunState::initial("INC-1", "implement") in a module doc.
/// RunState { item, stage } in an item doc.
// RunState::default() in a line comment.
/*
 * RunState::initial("x", "y"), RunState { .. }, RunState::default()
 */
pub fn nothing() {}
"#;

/// The naive rule this one replaces: whole-line `contains`, comments and all. Shown to over-report.
fn naive_hits(text: &str) -> usize {
    text.lines()
        .filter(|line| CONSTRUCTION_PATTERNS.iter().any(|p| line.contains(p)))
        .count()
}

#[test]
fn ac294_run_state_construction_sites_are_module_bounded() {
    // -------------------------------------------------------------------------------------------
    // 0. THE TABLE CARRIES EVERY ROUTE TO A VALUE, OR THE RULE POLICES A SUBSET OF THEM.
    //
    //    `tests/ui/stay_class_state_is_unconstructible.rs` names three and closes all three: the
    //    constructor, `Default`, and the struct literal. A source rule that watched fewer than
    //    three would leave the unwatched one as the route a later widening takes.
    // -------------------------------------------------------------------------------------------
    for required in ["RunState::initial(", "RunState {", "RunState::default()"] {
        assert!(
            CONSTRUCTION_PATTERNS.contains(&required),
            "`{required}` is one of the three routes the trybuild case names and the rule's table \
             does not carry it"
        );
    }
    assert_eq!(
        CONSTRUCTION_PATTERNS.len(),
        EXPECTED_CONTROL_HITS.len(),
        "the control exercises one line per banned spelling; a spelling added without a control \
         line is a spelling nothing shows to match"
    );

    // -------------------------------------------------------------------------------------------
    // 1. THE THEN, OVER THE REAL TREE: zero construction sites outside the owning module.
    // -------------------------------------------------------------------------------------------
    let files = kernel_sources().expect("crates/rk-kernel/src is readable");
    let report = match run_state_boundary_rule(&files) {
        Ok(report) => report,
        Err(violations) => panic!(
            "AC-294: `RunState` is constructed outside `crate::workflow`, which turns \
             `pub(in crate::workflow) fn initial` from a type fact into a convention: {}",
            violations
                .iter()
                .map(|v| format!("{}:{} `{}`", v.file(), v.line(), v.pattern()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };

    // THE PREMISES, and they are what stop the Ok above being an Ok over nothing.
    assert!(
        report.files_walked >= 40,
        "PREMISE: the walk found {} .rs files under src/; it found 46 when this was written, and a \
         rule that walked a handful of them would return Ok for the wrong reason",
        report.files_walked
    );
    assert!(
        report.code_lines_scanned >= 10_000,
        "PREMISE: only {} code lines were searched; a comment-stripper that ate the file would \
         return Ok over nothing",
        report.code_lines_scanned
    );
    assert!(
        report.comment_lines_skipped > 0,
        "PREMISE: the stripper skipped no comment lines at all, so it is not running"
    );
    assert!(
        report.sites_inside_owner >= 3,
        "PREMISE: the rule found {} construction sites INSIDE `crate::workflow`; `fsm.rs` alone \
         carries five on code lines, so a rule finding none is a rule matching nothing and its \
         `Ok` above means nothing",
        report.sites_inside_owner
    );
    assert_eq!(
        report.patterns_checked,
        CONSTRUCTION_PATTERNS.len(),
        "the report must say how many spellings it checked, or a table emptied by accident reads \
         as a clean run"
    );

    // THE ONE EXEMPTION IS COUNTED AND ITS SIZE IS ASSERTED, on `tests/it/kernel_denylist.rs`'s
    // discipline. `module_boundary.rs`'s own table is the only marked region under `src/`: two
    // markers and the three rows between them. An exemption that silently grew would be the one way
    // this rule could be turned off without a single assertion moving.
    assert_eq!(
        report.table_lines_skipped,
        2 + CONSTRUCTION_PATTERNS.len(),
        "the marked exemption must be exactly the two markers and one row per spelling; it \
         covered {} lines",
        report.table_lines_skipped
    );

    // -------------------------------------------------------------------------------------------
    // 2. THE BOUNDARY IS THE ONE `pub(in crate::workflow)` DECLARES, AND NOT A PATH THAT RESEMBLES IT.
    // -------------------------------------------------------------------------------------------
    assert!(
        is_owner(OWNER_FILE),
        "`{OWNER_FILE}` is the module's own file"
    );
    assert!(
        is_owner("crates/rk-kernel/src/workflow/fsm.rs"),
        "a file under the module directory is inside the boundary"
    );
    assert!(
        !is_owner("crates/rk-kernel/src/runtime.rs"),
        "`runtime.rs` is INSIDE the crate and OUTSIDE `crate::workflow` — it is the file the \
         trybuild case structurally cannot see, and the whole reason this lint exists"
    );
    assert!(
        !is_owner("crates/rk-kernel/src/workflow_helpers.rs"),
        "a prefix match on `{OWNER_PREFIX}` without its trailing slash would swallow a sibling \
         file whose name merely starts with `workflow`"
    );

    // -------------------------------------------------------------------------------------------
    // 3. THE POSITIVE CONTROL — the exact violation set, one line per banned spelling.
    //
    //    Without this, part 1 is satisfied by a rule that returns `Ok` for every input.
    // -------------------------------------------------------------------------------------------
    let found = scan_source(OUTSIDE_PATH, CONTROL);
    let rendered: Vec<(usize, &str)> = found.iter().map(|v| (v.line(), v.pattern())).collect();
    assert_eq!(
        rendered.as_slice(),
        EXPECTED_CONTROL_HITS,
        "the control must produce exactly one violation per banned spelling, at the code lines \
         that construct — and no others"
    );
    for v in &found {
        assert_eq!(
            v.file(),
            OUTSIDE_PATH,
            "AC-294's THEN is a build failure, so a violation must name the file an operator opens"
        );
    }

    // -------------------------------------------------------------------------------------------
    // 4. THE COMMENT CASE — the fourth guard's control, and the reason this test exists in this
    //    shape. A lint satisfied by the prose explaining it is this codebase's oldest defect.
    // -------------------------------------------------------------------------------------------
    let comment_lines: &[usize] = &[16, 17, 21, 25, 26, 29];
    for line in comment_lines {
        assert!(
            !found.iter().any(|v| v.line() == *line),
            "the control's line {line} carries a banned spelling in a COMMENT and the rule \
             reported it as a construction site"
        );
    }
    assert!(
        scan_source(OUTSIDE_PATH, COMMENTS_ONLY).is_empty(),
        "a file whose only occurrences are a module doc, an item doc, a line comment and a block \
         comment must produce ZERO violations"
    );
    assert!(
        naive_hits(COMMENTS_ONLY) >= 4,
        "PREMISE: the comment-only fixture really does carry the spellings — otherwise the \
         assertion above passes over a file with nothing in it"
    );

    // -------------------------------------------------------------------------------------------
    // 5. THE COMMENT-STRIPPING IS MEASURED, NOT DESCRIBED. The naive whole-line `contains` this
    //    rule replaces is run over the same bytes, from inside this binary, and over-reports.
    // -------------------------------------------------------------------------------------------
    assert!(
        naive_hits(CONTROL) > found.len(),
        "the naive comment-blind scan reported {} hits over the control and the rule reported {}; \
         if they agree, the stripping is doing no work and part 4 is passing by luck",
        naive_hits(CONTROL),
        found.len()
    );

    // -------------------------------------------------------------------------------------------
    // 6. A BOUNDARY, NOT A BAN. The same bytes at a path INSIDE the owner are lawful.
    //
    //    Without this, a rule that refused `RunState` construction everywhere would pass parts 3
    //    and 4 and would be red on `fsm.rs`'s own five sites — which is the opposite criterion.
    // -------------------------------------------------------------------------------------------
    assert!(
        scan_source(INSIDE_PATH, CONTROL).is_empty(),
        "the identical text at `{INSIDE_PATH}` must produce no violation: AC-294 asserts zero \
         construction sites OUTSIDE the owning module, not zero construction sites"
    );

    // -------------------------------------------------------------------------------------------
    // 7. THE RULE REPORTS EVERY VIOLATION, NOT THE FIRST. An operator repairing an introduced
    //    construction site needs the whole set, on `kernel_denylist_rule`'s precedent.
    // -------------------------------------------------------------------------------------------
    let synthetic = vec![rk_kernel::module_boundary::SourceFile {
        path: OUTSIDE_PATH.to_owned(),
        text: CONTROL.to_owned(),
    }];
    let violations: Vec<BoundaryViolation> = run_state_boundary_rule(&synthetic)
        .expect_err("a file with three construction sites outside the owner is a failure");
    assert_eq!(
        violations.len(),
        EXPECTED_CONTROL_HITS.len(),
        "the rule returns the whole set and not the first violation"
    );
}
