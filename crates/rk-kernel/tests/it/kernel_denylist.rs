//! **AC-487** — the kernel denylist, its positive control, and the compile-fail fixture beside it.
//!
//! `s7b`:1176-1178 hands `ac487_kernel_denylist` here by name — *"the adapter-name /
//! `downcast_ref` / `downcast_mut` denylist plus its compile-fail fixture"* — and Checkpoint 7b's
//! second loop (`s7b`:1511-1519) filters on exactly that test name.
//!
//! # WHAT IS MEASURED, AND WHAT IS NOT
//!
//! | AC-487 clause | instrument here | status |
//! | --- | --- | --- |
//! | *"`rk-kernel` contains no comparison against an adapter identity"* | [`rk_kernel::kernel_denylist::kernel_denylist_rule`] over every `.rs` file under `crates/rk-kernel/src/` | **measured** |
//! | *"an introduced identity check fails the lint naming the file and line"* | the same rule over `tests/fixtures/lint-controls/ac487_identity_check_control.rs`, asserting the exact `(line, pattern)` set and that `Display` carries `file:line` | **measured** |
//! | *"no `Any::downcast`"*, on a **port object** | the compile-fail case `tests/ui/ac487_port_identity_is_not_askable.rs` — no port trait carries an `Any` supertrait, so the question cannot be asked at all | **measured**, by `trybuild` |
//! | *"no adapter-discriminant enum"*, under an arbitrary name | nothing. A substring rule reaches `enum AdapterKind`, not `enum Backend` | **ARMED, NOT DISCHARGED** |
//! | *"and the difference is expressed as a capability lookup"* | nothing here, and deliberately | **STRUCK** by the criterion's own note; carried by AC-494 and AC-495, which are built in `it/resolver.rs` and `it/pack_validation.rs` as of 2026-09-14. This cell read *"neither of which is built"* and is corrected in place: **this file still asserts nothing about a lookup**, and that is the split rather than a gap |
//!
//! 🛑 **The struck clause is not claimed anywhere in this file.** AC-487's note is explicit that the
//! lint *"proves the absence of identity checks; it cannot prove the presence of a capability
//! lookup"*. A green here says the kernel branches on no adapter identity. It does **not** say the
//! kernel looks a capability up, and it does not say the lookup is correct.
//!
//! # Why the positive control is what makes the green mean anything
//!
//! `kernel_denylist_rule` returning `Ok` over `crates/rk-kernel/src/` is satisfied by a rule that
//! returns `Ok` over every input — including one whose denylist is empty, or whose comment-stripper
//! ate the whole file. Three assertions below exist only to close that:
//!
//! 1. the control fixture produces an **exact** violation set, one per denylist entry;
//! 2. the naive whole-file `contains` the rule replaces is shown, from inside this binary, to report
//!    **two** hits over `src/` that the rule correctly does not — so the comment-stripping is doing
//!    work rather than being described as doing work;
//! 3. the report's walked counts are asserted against floors, so a walk that found nothing is red.
//!
//! **Measured from inside the test binary and not from a probe.** An earlier pass in this session
//! lost time to a Python probe that normalised CRLF and reported a 1,861-byte window where Rust saw
//! 30,288; every number below is read by the same `std::fs` the rule uses. What one clean run over
//! `crates/rk-kernel/src/` walks [measured 2026-09-14, over 22,546 source lines]: **45 files,
//! 13,219 code lines searched, 9,290 comment or blank lines skipped, 37 exempt lines — the one
//! bracketed table region and nothing else — 7 patterns, and 0 violations.**

use rk_kernel::kernel_denylist::{
    DENYLIST, DenyClause, DenylistViolation, KERNEL_SRC_PREFIX, SourceFile, TABLE_BEGIN, TABLE_END,
    kernel_denylist_rule, kernel_sources, kernel_src_root, read_rust_sources, scan_source,
    strip_comments,
};

/// `crates/rk-kernel/tests/fixtures/lint-controls/`, the directory `s7b-ii` §7b-ii.10 opens for
/// exactly this: a grep's positive control is an ordinary file carrying the banned token, and
/// `tests/ui/` cannot hold one because that is the `trybuild` glob root.
fn lint_controls_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lint-controls")
}

/// The control's own text, compiled into this binary so the assertion about line numbers is
/// checked against the same bytes the rule read.
const CONTROL: &str = include_str!("../fixtures/lint-controls/ac487_identity_check_control.rs");

/// The exact set the control must produce: **one violation per [`DENYLIST`] entry**, so an entry
/// that stopped matching anything would be caught here and not merely be carried in the table.
const EXPECTED_CONTROL_HITS: &[(usize, &str, DenyClause)] = &[
    (25, "any::Any", DenyClause::AnyTrait),
    (32, "rk-adapter-", DenyClause::AdapterCrateName),
    (37, "rk_adapter_", DenyClause::AdapterCrateName),
    (42, "downcast_ref", DenyClause::PortDowncast),
    (48, "downcast_mut", DenyClause::PortDowncast),
    (53, "downcast::<", DenyClause::AnyTrait),
    (57, "enum Adapter", DenyClause::DiscriminantEnum),
];

#[test]
fn ac487_kernel_denylist() {
    // ---------------------------------------------------------------------------------------
    // 0. THE DENYLIST CARRIES THE REGISTER'S OWN THREE ENTRIES.
    //
    //    "a kernel lint (denylist: adapter crate names, `downcast_ref`, `downcast_mut` on port
    //    objects)". A table that had drifted off those three would leave every assertion below
    //    green while asserting something the register did not ask for.
    // ---------------------------------------------------------------------------------------
    for required in ["rk-adapter-", "downcast_ref", "downcast_mut"] {
        assert!(
            DENYLIST.iter().any(|d| d.pattern == required),
            "AC-487's Verified-by names `{required}` as a denylist entry and the table does not \
             carry it"
        );
    }
    assert_eq!(
        DENYLIST.len(),
        EXPECTED_CONTROL_HITS.len(),
        "the control exercises one line per denylist entry; an entry added without a control line \
         is an entry nothing shows to match"
    );

    // ---------------------------------------------------------------------------------------
    // 1. THE THEN, FIRST CLAUSE: `rk-kernel` contains no comparison against an adapter identity.
    // ---------------------------------------------------------------------------------------
    let src_root = kernel_src_root();
    assert!(
        src_root.is_dir(),
        "PREMISE: {} is the crate's own src/ and must exist",
        src_root.display()
    );
    let files = kernel_sources().expect("crates/rk-kernel/src is readable");

    // PREMISE, and it is the one that stops a green over nothing: the walk found the real tree.
    assert!(
        files.len() >= 40,
        "PREMISE: the walk found {} .rs files under src/; it found 45 when this was written, and \
         a rule that walked a handful of them would return Ok for the wrong reason",
        files.len()
    );
    assert!(
        files
            .iter()
            .any(|f| f.path == "crates/rk-kernel/src/lib.rs"),
        "PREMISE: the walk must reach the crate root; paths are {:?}",
        files
            .iter()
            .map(|f| f.path.as_str())
            .take(5)
            .collect::<Vec<_>>()
    );
    assert!(
        files
            .iter()
            .any(|f| f.path.starts_with("crates/rk-kernel/src/workflow/")),
        "PREMISE: the walk must RECURSE -- src/workflow/ holds eight modules and a non-recursive \
         walk would miss every one of them while still reporting a dozen files"
    );
    // "NAMING THE FILE" MEANS A FILE SOMEONE CAN OPEN. `read_rust_sources` reports paths relative
    // to the root it walked, so `src/runtime.rs` would come back as `runtime.rs` -- which names
    // two files in this repository, the other being `tests/it/runtime.rs`.
    assert!(
        files.iter().all(|f| f.path.starts_with(KERNEL_SRC_PREFIX)),
        "every path in the kernel's own scan must be repository-relative; got {:?}",
        files
            .iter()
            .map(|f| f.path.as_str())
            .take(3)
            .collect::<Vec<_>>()
    );

    let report = match kernel_denylist_rule(&files) {
        Ok(report) => report,
        Err(violations) => panic!(
            "AC-487: rk-kernel compares against an adapter identity in {} place(s):\n{}",
            violations.len(),
            violations
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        ),
    };

    assert_eq!(report.files_walked, files.len());
    assert_eq!(report.patterns_checked, DENYLIST.len());
    assert!(
        report.code_lines_scanned >= 10_000,
        "PREMISE: {} code lines were searched; 13,219 were searched when this floor was set \
         [measured 2026-09-14 from inside this binary], and a stripper that ate the crate would \
         report Ok having searched almost nothing",
        report.code_lines_scanned
    );
    assert!(
        report.comment_lines_skipped >= 5_000,
        "PREMISE: {} comment/blank lines were skipped; 9,290 were skipped when this floor was set \
         [measured 2026-09-14]. This crate's headers are its record, and a zero here would mean \
         the stripper is inert -- leaving the control below as the only thing telling the two \
         apart",
        report.comment_lines_skipped
    );

    // ---------------------------------------------------------------------------------------
    // 2. THE NAIVE LINT THIS ONE REPLACES IS SHOWN TO BE WRONG, FROM INSIDE THIS BINARY.
    //
    //    Without this, "excludes comments" is a claim in a header. With it, the difference is a
    //    measured number over the same bytes: the whole-file `contains` reports hits on prose
    //    describing the ban, and the rule reports none.
    // ---------------------------------------------------------------------------------------
    let naive: Vec<&str> = files
        .iter()
        .filter(|f| f.text.contains("rk-adapter-"))
        .map(|f| f.path.as_str())
        .collect();
    assert!(
        naive.len() >= 2,
        "PREMISE for the comment-stripping half: a whole-file `contains(\"rk-adapter-\")` found {} \
         file(s) under src/; it found THREE when this was written -- lib.rs, workflow/retry.rs and \
         kernel_denylist.rs itself, whose header explains the ban at length. If it found none, the \
         rule's Ok above is unearned: there would be nothing for the stripper to get right",
        naive.len()
    );

    // ---------------------------------------------------------------------------------------
    // 3. THE THEN, SECOND CLAUSE: an introduced identity check FAILS the lint, NAMING THE FILE
    //    AND LINE. The rule is run over the control the same way it is run over src/.
    // ---------------------------------------------------------------------------------------
    let controls_root = lint_controls_root();
    assert!(
        controls_root.is_dir(),
        "PREMISE: {} is `s7b-ii` §7b-ii.10's directory and AC-487's positive control lives there, \
         NOT under tests/ui/ -- a trybuild glob case must FAIL to compile and this file must not \
         be compiled at all",
        controls_root.display()
    );
    let control_files =
        read_rust_sources(&controls_root).expect("tests/fixtures/lint-controls is readable");

    let control_name = "ac487_identity_check_control.rs";
    assert!(
        control_files.iter().any(|f| f.path == control_name),
        "PREMISE: {control_name} must be in the walk; found {:?}",
        control_files
            .iter()
            .map(|f| f.path.as_str())
            .collect::<Vec<_>>()
    );
    // The file compiled into this binary and the file the rule read are the same bytes.
    assert!(
        control_files
            .iter()
            .any(|f| f.path == control_name && f.text == CONTROL),
        "PREMISE: the control read from disk differs from the one include_str! compiled in -- \
         every line number below is then about a file nobody has seen"
    );

    let violations = kernel_denylist_rule(&control_files)
        .expect_err("AC-487: the control carries seven identity checks and the lint MUST bite");

    let mine: Vec<&DenylistViolation> = violations
        .iter()
        .filter(|v| v.file() == control_name)
        .collect();
    let got: Vec<(usize, &str, DenyClause)> = mine
        .iter()
        .map(|v| (v.line(), v.pattern(), v.clause()))
        .collect();
    assert_eq!(
        got,
        EXPECTED_CONTROL_HITS,
        "the lint must flag exactly the seven code lines of the control, one per denylist entry, \
         in line order. Full messages:\n{}",
        mine.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );

    // AND EACH LINE NUMBER IS THE LINE IT NAMES. A rule off by one, or reporting 0-based lines,
    // satisfies the equality above only because the constant was written from its own output; this
    // reads the line back out of the fixture and requires the token to be on it.
    let control_lines: Vec<&str> = CONTROL.lines().collect();
    for v in &mine {
        let line = control_lines.get(v.line() - 1).unwrap_or_else(|| {
            panic!(
                "{} names line {} and the file has {} lines -- the number is \
                 not a line number at all",
                v.file(),
                v.line(),
                control_lines.len()
            )
        });
        assert!(
            line.contains(v.pattern()),
            "{}:{} is reported for `{}`, and line {} of the fixture reads `{}` -- the location is \
             wrong, which is the whole of AC-487's second THEN clause",
            v.file(),
            v.line(),
            v.pattern(),
            v.line(),
            line.trim()
        );
    }

    // "NAMING THE FILE AND LINE" IS ABOUT THE MESSAGE, NOT ONLY THE STRUCT. An operator reads the
    // Display; a violation whose accessors carry a location its message does not print names
    // nothing.
    for v in &mine {
        let rendered = v.to_string();
        assert!(
            rendered.contains(&format!("{}:{}", v.file(), v.line())),
            "the lint's own message must name the file and the line (AC-487's THEN, second \
             clause); it reads: {rendered}"
        );
    }

    // ---------------------------------------------------------------------------------------
    // 4. THE CONTROL'S PROSE IS NOT FLAGGED, WHICH IS THE DISCRIMINATING HALF.
    //
    //    Its header names `rk-adapter-sqlite`, `downcast_ref` and `downcast_mut` in `//!` lines,
    //    exactly as src/lib.rs:172 and src/workflow/retry.rs:493 do. A rule with the stripping
    //    removed flags every code line below AND these -- and would still look like it worked.
    // ---------------------------------------------------------------------------------------
    let header_lines: Vec<usize> = (1..=23).collect();
    for line in header_lines {
        assert!(
            !mine.iter().any(|v| v.line() == line),
            "line {line} of the control is `//!` prose and must NOT be flagged; it reads `{}`",
            control_lines[line - 1].trim()
        );
    }
    let naive_control_hits = CONTROL
        .lines()
        .enumerate()
        .filter(|(_, l)| DENYLIST.iter().any(|d| l.contains(d.pattern)))
        .count();
    assert!(
        naive_control_hits > mine.len(),
        "PREMISE: the naive whole-line `contains` finds {naive_control_hits} lines and the rule \
         finds {} -- if the two agreed, the control's prose would not be exercising the stripper \
         and assertion 4 would be vacuous",
        mine.len()
    );

    // ---------------------------------------------------------------------------------------
    // 5. THE ONE EXEMPTION IS BOUNDED, AND THE BOUND IS ASSERTED RATHER THAN DESCRIBED.
    //
    //    A denylist must SPELL what it bans, so the table's own `pattern:` lines carry the tokens
    //    and are skipped. That is a hole, and these are its two walls: the region exists exactly
    //    once and only in the rule's own file, and inside it only TABLE-SHAPED lines are skipped.
    //
    //    THE FIRST DRAFT OF THIS WAS A PER-LINE TRAILING COMMENT AND `cargo fmt` BROKE IT
    //    [measured 2026-09-14]: rustfmt split each entry across five lines and left the marker on
    //    the closing brace, exposing seven `pattern:` lines, and the rule reported nine violations
    //    against its own table. The marker COUNT was unchanged at 8, so a budget assertion did not
    //    see it -- which is why the second wall below is a smuggled line and not a number.
    // ---------------------------------------------------------------------------------------
    let markers: Vec<(&str, usize, &str)> = files
        .iter()
        .flat_map(|f| {
            f.text
                .lines()
                .enumerate()
                .filter(|(_, l)| l.trim() == TABLE_BEGIN || l.trim() == TABLE_END)
                .map(move |(i, l)| (f.path.as_str(), i + 1, l.trim()))
        })
        .collect();
    assert_eq!(
        markers.len(),
        2,
        "the scan has exactly ONE exempt region -- one open marker and one close. Found {markers:?}"
    );
    assert_eq!(
        (markers[0].0, markers[0].2, markers[1].0, markers[1].2),
        (
            "crates/rk-kernel/src/kernel_denylist.rs",
            TABLE_BEGIN,
            "crates/rk-kernel/src/kernel_denylist.rs",
            TABLE_END
        ),
        "the region must open and close, in that order, in the rule's own file: {markers:?}"
    );
    assert!(
        markers[0].1 < markers[1].1,
        "the region opens before it closes: {markers:?}"
    );
    assert_eq!(
        report.table_lines_skipped,
        markers[1].1 - markers[0].1 + 1,
        "every line of the region, markers included, is accounted for -- the rule and this test \
         must agree on what the exemption covers"
    );

    // THE SECOND WALL, AND IT IS THE ONE THAT MATTERS: the region does not exempt whatever is
    // inside it. A line that is not table-shaped is flagged even between the markers -- so a
    // forgotten close marker, or an identity check written into the table, is not a bypass.
    let smuggled = format!(
        "{TABLE_BEGIN}\n    pattern: \"downcast_ref\",\n    let hidden = id == \
         \"rk-adapter-sqlite\";\n{TABLE_END}\n"
    );
    let smuggled_hits = scan_source("smuggled.rs", &smuggled);
    assert_eq!(
        smuggled_hits.len(),
        1,
        "inside the region the table row is exempt and the planted check is NOT; got {smuggled_hits:?}"
    );
    assert_eq!(
        (smuggled_hits[0].line(), smuggled_hits[0].pattern()),
        (3, "rk-adapter-"),
        "the planted check is on line 3 and must be the one thing reported"
    );

    // ---------------------------------------------------------------------------------------
    // 6. THE COMPILE-FAIL FIXTURE THE REGISTER NAMES BESIDE THE LINT.
    //
    //    "a kernel lint ...; a compile-fail fixture" -- two instruments, and the second is NOT the
    //    first. Checkpoint 7b's by-path loop (s7b:1584-1589) enumerates six ui/ cases and neither
    //    of these is among them, so as with the other unasserted cases the path assertion is made
    //    from the test that carries the criterion (tests/it/ui.rs's header records the pattern).
    // ---------------------------------------------------------------------------------------
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    for f in [
        "ac487_port_identity_is_not_askable.rs",
        "ac487_port_identity_is_not_askable.stderr",
    ] {
        let p = ui.join(f);
        assert!(
            p.is_file(),
            "AC-487's Verified-by names a compile-fail fixture beside the lint; {} is missing",
            p.display()
        );
        assert!(
            std::fs::metadata(&p).expect("readable").len() > 0,
            "{} is empty -- an empty .stderr makes the case green whenever compilation fails for \
             ANY reason (s7a §7a.8)",
            p.display()
        );
    }
}

/// **The comment scanner's own unit cases, because `scan_source` cannot show them.**
///
/// Assertion 2 of [`ac487_kernel_denylist`] proves the stripper removes `//!` prose. It does not
/// reach the three shapes that a `starts_with("//")` filter gets wrong and that this crate's source
/// actually contains: a trailing comment on a code line, a `/* … */` block spanning lines, and a
/// `//` inside a string literal. `src/workflow/breaker.rs`:772 (`l.split('"')`) and
/// `src/predicate/lexer.rs`:179 (`c == b'"'`) are why the char-literal case is here too: a scanner
/// that treated that quote as opening a string would run the rest of the line in string state and
/// scan its trailing comment as code.
#[test]
fn the_denylist_scanner_reads_code_and_not_prose() {
    let mut in_block = false;
    assert_eq!(strip_comments("//! downcast_ref", &mut in_block).trim(), "");
    assert_eq!(strip_comments("/// downcast_mut", &mut in_block).trim(), "");
    assert_eq!(
        strip_comments("let x = 1; // downcast_ref", &mut in_block).trim(),
        "let x = 1;"
    );

    // A `//` inside a string literal is CODE and must survive: a denied token in a string is
    // exactly the "adapter name string" clause's shape.
    assert!(
        strip_comments(r#"let s = "http://rk-adapter-x";"#, &mut in_block).contains("rk-adapter-x"),
        "a `//` inside a string literal must not start a comment"
    );

    // A char literal holding a quote must not desync the scanner.
    assert_eq!(
        strip_comments("let q = '\"'; // downcast_ref", &mut in_block).trim(),
        "let q = '\"';"
    );

    // A block comment carries across lines, which no per-line prefix filter can do.
    assert!(!in_block, "PREMISE: state is clean before the block case");
    assert_eq!(
        strip_comments("code_a(); /* downcast_ref", &mut in_block).trim(),
        "code_a();"
    );
    assert!(
        in_block,
        "the scanner must carry block state to the next line"
    );
    assert_eq!(
        strip_comments("downcast_mut is discussed here", &mut in_block).trim(),
        ""
    );
    assert_eq!(
        strip_comments("still inside */ code_b();", &mut in_block).trim(),
        "code_b();"
    );
    assert!(!in_block, "and must leave it at the terminator");

    // And the whole thing composed: a file whose ONLY mention of a denied token is in prose has no
    // violations, while the same token one line down in code has exactly one.
    let prose_only = "//! rk-adapter-sqlite is forbidden here\nfn f() {}\n";
    assert_eq!(scan_source("p.rs", prose_only), Vec::new());

    let with_code =
        "//! rk-adapter-sqlite is forbidden here\nfn f() -> bool { id == \"rk-adapter-sqlite\" }\n";
    let hits = scan_source("p.rs", with_code);
    assert_eq!(hits.len(), 1, "got {hits:?}");
    assert_eq!(
        hits[0].line(),
        2,
        "the code line is line 2, not the prose on line 1"
    );
    assert_eq!(hits[0].file(), "p.rs");
}

/// **The rule is not satisfied by an empty input, and says so.**
///
/// `kernel_denylist_rule(&[])` returns `Ok`. That is correct and it is also the shape of every
/// vacuous green this programme has paid for, so the report carries what it walked and
/// [`ac487_kernel_denylist`]'s floors read it. This test pins that the counters are real rather
/// than constants.
#[test]
fn the_denylist_report_counts_what_it_walked() {
    let empty = kernel_denylist_rule(&[]).expect("no files, no violations");
    assert_eq!(empty.files_walked, 0);
    assert_eq!(empty.code_lines_scanned, 0);
    assert_eq!(empty.comment_lines_skipped, 0);
    assert_eq!(
        empty.patterns_checked,
        DENYLIST.len(),
        "the table is checked even over no input; a zero here would mean an emptied denylist"
    );

    let one = kernel_denylist_rule(&[SourceFile {
        path: "x.rs".to_owned(),
        text: "// a comment\nfn f() {}\n\nfn g() {}\n".to_owned(),
    }])
    .expect("nothing denied");
    assert_eq!(one.files_walked, 1);
    assert_eq!(one.code_lines_scanned, 2, "two code lines, not four");
    assert_eq!(one.comment_lines_skipped, 2, "the comment and the blank");
}
