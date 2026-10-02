//! **The router surface** — AC-400, AC-186, AC-264, AC-402 (snapshot half), AC-441 (router half),
//! AC-403 (lint half), RB-M-06/D18-16, and the two grammars ADR-0128 and ADR-0129 fix.
//!
//! Three instruments, and AC-400's Verified-by asks for all three: a committed `trycmd` snapshot of
//! `--help` for the root and every verb; a lint over the **built `clap::Command` tree** rather than
//! over source text (D18-13); and a process-level refusal.
//!
//! **Why the lint walks the tree and not the source.** A grep would fire on a comment or a doc
//! string that merely NAMES `--resume`, and would miss a flag whose long name came from a `const`.
//! The tree is what the parser will actually accept, which is what the register's THEN is about.
//!
//! **Why the walk carries a coverage control.** A walker that stopped at the top level, or that
//! skipped arguments, would report "no denylisted name" having visited almost nothing — an absence
//! assertion that cannot fail. Every tree test therefore asserts [`assert_walk_covers`] FIRST: a
//! nested subcommand (`search`), a nested positional (`<QUERY>`) and a nested long flag
//! (`--stability-record`) must all be in the visited set before any absence is claimed.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_cli::cli::{self, DENYLIST};

use crate::{code_lines, lint_controls_dir, rs_files, src_dir};

/// The three names AC-402's Verified-by adds to the surface snapshot's refusals.
const RECONCILE_ESCAPES: [&str; 3] = ["--no-reconcile", "--skip-reconcile", "--fast"];

/// `<name>` forms the walk must have reached before any absence is asserted (§7d.3, D18-13).
const WALK_COVERAGE: [&str; 9] = [
    "run",
    "--lane",
    "catalog",
    "search",
    "<QUERY>",
    "lint",
    "--stability-record",
    "doctor",
    "--rebuild",
];

/// **Every name the built parser tree carries**, at every depth: subcommand names and their
/// aliases, long flags (with `--`), short flags (with `-`), and positional value names (in
/// `<ANGLE>` form).
///
/// This is the instrument of AC-400's lint half, and D18-13 says where it belongs: *"the lint is a
/// test walking the built `clap::Command` tree"*. It is a test, and it is here.
///
/// **Corrected 2026-09-19, the same day it was written.** The first draft put this walk in
/// `src/cli.rs` and gave a reason that is simply false: that an integration test cannot name
/// `clap::Command` because `rk-cli` publishes no `clap` dev-dependency. Cargo puts a package's
/// NORMAL dependencies in scope for its own test targets, and `clap` is one — measured by adding
/// `use clap::Command;` to this file and compiling, which it does. A test-only helper was therefore
/// sitting in the shipped library on a constraint that does not exist. Nothing about the manifest
/// changed to fix it; the fence in §7d.15 still carries no `clap` dev-row, and none is needed.
///
/// **Why it walks the tree and not the source**, which is the warrant that IS real: a grep would
/// fire on a comment or a doc string that merely names `--resume`, and would miss a flag whose long
/// name came from a `const`. The tree is what the parser will actually accept, which is what the
/// register's THEN is about.
fn parser_tree_names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    visit(&cli::command(), &mut names);
    names
}

fn visit(cmd: &clap::Command, names: &mut BTreeSet<String>) {
    names.insert(cmd.get_name().to_owned());
    for alias in cmd.get_all_aliases() {
        names.insert(alias.to_owned());
    }
    for arg in cmd.get_arguments() {
        if let Some(long) = arg.get_long() {
            names.insert(format!("--{long}"));
        }
        for alias in arg.get_all_aliases().unwrap_or_default() {
            names.insert(format!("--{alias}"));
        }
        if let Some(short) = arg.get_short() {
            names.insert(format!("-{short}"));
        }
        if arg.is_positional()
            && let Some(value_names) = arg.get_value_names()
        {
            for value_name in value_names {
                names.insert(format!("<{value_name}>"));
            }
        }
    }
    for sub in cmd.get_subcommands() {
        visit(sub, names);
    }
}

/// The premise every tree test asserts before it asserts an absence.
fn assert_walk_covers(names: &BTreeSet<String>) {
    for expected in WALK_COVERAGE {
        assert!(
            names.contains(expected),
            "the parser-tree walk did not reach `{expected}` — it visited {} names and none of them \
             is this one, so every absence asserted against this walk would be vacuous. Visited: \
             {names:?}",
            names.len()
        );
    }
}

/// Is `name` a resume-shaped name, compared exactly with and without its `--` (§7d.3)?
///
/// **Exact, never prefix and never substring.** A prefix rule would refuse `--from-scratch` on a
/// future verb for a name the list does not carry — the register's own *Limit* is that the denylist
/// is name-based — and a substring rule would fire on `continue` inside `--continue-on-error`.
fn is_denylisted(name: &str) -> bool {
    let bare = name.trim_start_matches('-');
    DENYLIST
        .iter()
        .any(|d| *d == name || d.trim_start_matches('-') == bare)
}

/// The committed `.trycmd` files — the snapshot bytes, as a set of paths.
fn trycmd_files() -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cmd");
    let mut out: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()))
        .map(|e| e.expect("a directory entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "trycmd"))
        .collect();
    out.sort();
    assert_eq!(
        out.len(),
        10,
        "expected ten .trycmd files under {} — one per verb (§7d.3's eight plus ADR-0163's `cost`) \
         and one for the root — found {}: {out:?}",
        dir.display(),
        out.len()
    );
    out
}

/// Split `text` into whole tokens, keeping `-` and `_` inside a token so `--retry-step` is one.
fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
        .collect()
}

/// AC-400 / AC-186 / AC-264 / AC-441's *"a committed snapshot test of the full `clap` argument
/// surface … which fails on any diff"*.
///
/// Red by any diff between `--help` of the root and every verb and the committed pair; a weaker red
/// snapshots the root only, or regenerates the snapshot in the same run — `TRYCMD=overwrite` is
/// never set in CI or in Checkpoint 7d's block, and the block asserts that it is unset.
///
/// **The premise below is not decoration, and it is here because this test passed vacuously once.**
/// `trycmd` takes its cases from ` ``` ` fenced blocks and, in its own words, *"everything out of
/// them is ignored"* — so nine `.trycmd` files carrying bare `$ redkiln … --help` lines with no
/// fence were each reported `... ignored` and the test went green having executed nothing. A
/// `TestCases` run returns no result a caller can assert on, so the guard has to be over the
/// committed bytes: ten files, eleven captures, and every file holding real captured help. A case
/// that is skipped leaves its block empty, and an empty block has no `Usage:` line.
#[test]
fn ac400_help_snapshot_every_verb() {
    let files = trycmd_files();
    let mut captures = 0;
    for file in &files {
        let text = std::fs::read_to_string(file).expect("a committed .trycmd");
        let here = text.matches("$ redkiln ").count();
        assert!(
            here >= 1,
            "{} carries no `$ redkiln …` command line",
            file.display()
        );
        assert!(
            text.contains("```console"),
            "{} has no ```console fence — trycmd ignores everything outside a fenced block, so an \
             unfenced case is silently skipped and this test would pass having run nothing",
            file.display()
        );
        assert!(
            text.contains("Usage:"),
            "{} carries no captured `Usage:` line — the block is empty, which is what a skipped \
             case leaves behind",
            file.display()
        );
        captures += here;
    }
    assert_eq!(
        captures, 11,
        "expected eleven `--help` captures across the ten files — one per verb (ADR-0163 made `cost` \
         the ninth), one for the root, and one more for `catalog search`, the nested sub-surface \
         §7d.3 names"
    );

    trycmd::TestCases::new()
        .default_bin_name("redkiln")
        .case("tests/cmd/*.trycmd");
}

/// AC-400's lint half, ADR-0107 decision 3 (D18-13).
///
/// Two conjuncts, because the register asks for two things the other does not give. The **tree
/// walk** pins what the parser accepts; the **snapshot scan** pins the register's third conjunct,
/// *"no denylisted name appears anywhere in the snapshot"*, which the walk cannot see — it reads
/// the parser, not the committed bytes.
///
/// The snapshot scan compares whole tokens against the eleven names **as spelled**, with no
/// `--`-stripping. The strip rule belongs to the tree arm alone: applied to help text it would fire
/// on the English word "from", which is not a denylist entry, and turn a correct tree red.
///
/// Declared mutant: `--resume` on `run`'s derive. Also red by an alias `resume` on `run`, and by a
/// subcommand three levels down carrying `--from`.
#[test]
fn ac400_denylist_absent_from_parser_tree() {
    assert_eq!(
        DENYLIST.len(),
        11,
        "ADR-0107 d3's eight plus the register's three"
    );

    let names = parser_tree_names();
    assert_walk_covers(&names);

    let offenders: Vec<&String> = names.iter().filter(|n| is_denylisted(n)).collect();
    assert!(
        offenders.is_empty(),
        "the parser tree carries resume-shaped names {offenders:?} — AC-400 refuses them as unknown \
         arguments, so they may not be accepted anywhere in the tree"
    );

    for file in trycmd_files() {
        let text = std::fs::read_to_string(&file).expect("a committed .trycmd");
        let hits: Vec<String> = tokens(&text)
            .into_iter()
            .filter(|t| DENYLIST.contains(&t.as_str()))
            .collect();
        assert!(
            hits.is_empty(),
            "the committed snapshot {} carries denylisted names {hits:?} — the register's third \
             conjunct is that no denylisted name appears anywhere in the snapshot",
            file.display()
        );
    }
}

/// AC-400 / AC-264 at the process boundary.
///
/// Red by `redkiln run --resume` exiting anything but `2`, or by stderr not naming the flag; a
/// weaker red asserts a non-zero exit — a `run` that PARSED `--resume` and then failed on a missing
/// item also exits non-zero, and that is the implementation this test exists to refuse.
#[test]
fn ac400_resume_refused_as_unknown_argument() {
    let mut cmd = crate::redkiln_command(crate::Launch::Fixture);
    cmd.args(["run", "an-item", "--resume"]);
    let out = crate::support::output_bounded(&mut cmd);

    assert_eq!(
        out.status.code(),
        Some(2),
        "`redkiln run an-item --resume` exited {:?}, not 2 — clap's usage code, kept (H-1)",
        out.status.code()
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unexpected argument '--resume'"),
        "stderr did not name the refused flag; it read:\n{stderr}"
    );
}

/// AC-402's snapshot half — its Verified-by's second instrument.
///
/// Red by `--no-reconcile`, `--skip-reconcile` or `--fast` anywhere in the tree; a weaker red checks
/// `run` only. Declared mutant: `--fast` on `run`.
#[test]
fn ac402_no_skip_reconcile_flag_in_snapshot() {
    let names = parser_tree_names();
    assert_walk_covers(&names);

    for escape in RECONCILE_ESCAPES {
        assert!(
            !names.contains(escape),
            "the parser tree carries `{escape}` — reconciliation is not optional and no flag may \
             skip it (AC-402)"
        );
    }
    for file in trycmd_files() {
        let text = std::fs::read_to_string(&file).expect("a committed .trycmd");
        for escape in RECONCILE_ESCAPES {
            assert!(
                !text.contains(escape),
                "the committed snapshot {} carries `{escape}`",
                file.display()
            );
        }
    }
}

/// AC-441's router half.
///
/// Red by a `migrate` subcommand or alias at any depth (the declared mutant). `redkiln-migrate` is
/// reachable only as its own binary; the build-graph half is the migrator's own test and the
/// block's `cargo metadata` walk.
#[test]
fn ac441_no_migrate_verb_in_tree() {
    let names = parser_tree_names();
    assert_walk_covers(&names);
    assert!(
        !names.contains("migrate"),
        "the router carries a `migrate` name — no subcommand may perform or trigger a migration \
         (AC-441)"
    );
}

/// RB-M-06 / D18-16; design §8.1's CONDITIONAL rows ship no flags.
///
/// Red by `--fix` or `--probe` on `doctor` (the declared mutant is `--fix`).
#[test]
fn doctor_fix_flag_absent_from_snapshot() {
    let names = parser_tree_names();
    assert_walk_covers(&names);

    for flag in ["--fix", "--probe"] {
        assert!(
            !names.contains(flag),
            "`doctor` carries `{flag}` — a CONDITIONAL row reports, it does not repair, and it does \
             not ship a flag to do either"
        );
    }
    for file in trycmd_files() {
        let text = std::fs::read_to_string(&file).expect("a committed .trycmd");
        for flag in ["--fix", "--probe"] {
            assert!(
                !text.contains(flag),
                "the committed snapshot {} carries `{flag}`",
                file.display()
            );
        }
    }
}

/// ADR-0128 decisions 1 and 2.
///
/// Red by `--lane hotfix` accepted, by `--item` accepted, by `--lane` being required; a weaker red
/// tests `fix` and `story` only. The refusal is a **usage** error and not a configuration one —
/// ADR-0128 decision 1: *"a third value is a usage error, not a configuration error"* — so each arm
/// asserts clap's `2`, and the closed-set arm asserts that the message names both admissible values.
#[test]
fn adr0128_run_lane_value_enum_closed() {
    let err = cli::try_parse_from(["redkiln", "run", "an-item", "--lane", "hotfix"])
        .expect_err("`--lane hotfix` is outside the closed set and must be refused");
    assert_eq!(
        err.exit_code(),
        2,
        "a closed-enum value outside its set is a usage error"
    );
    let rendered = err.to_string();
    assert!(
        rendered.contains("fix") && rendered.contains("story"),
        "the refusal did not name the two admissible values; it read:\n{rendered}"
    );

    let err = cli::try_parse_from(["redkiln", "run", "--item", "an-item"])
        .expect_err("ADR-0128 d1: there is no `--item` flag; the item is the first positional");
    assert_eq!(err.exit_code(), 2);

    cli::try_parse_from(["redkiln", "run", "an-item", "--lane", "fix"])
        .expect("`--lane fix` parses");
    cli::try_parse_from(["redkiln", "run", "an-item", "--lane", "story"])
        .expect("`--lane story` parses");
    cli::try_parse_from(["redkiln", "run", "an-item"])
        .expect("`--lane` is OPTIONAL — a run without it derives the lane (ADR-0128 d2)");
}

/// ADR-0129 decisions 1 and 2, AC-435's usage arm.
///
/// Red by `--token` accepted, by `answer x approve` accepted without `--explain-sufficed`, by
/// `--explain-sufficed yes` accepted without `--toml-opened`, by `--explain-sufficed maybe`
/// accepted; a weaker red asserts one of the four.
///
/// **Each arm asserts its own message as well as its code, and that is a DERIVED requirement.** The
/// records fix the codes only. Without the message conjunct all four arms are satisfied by a parser
/// with no `answer` verb at all — every one of them would exit `2` reading `unrecognized subcommand
/// 'answer'`, which is the wrong reason and is exactly the state this slice starts from.
#[test]
fn adr0129_answer_grammar_positional_option_no_token_flag() {
    let err = cli::try_parse_from(["redkiln", "answer", "an-item", "approve", "--token", "t"])
        .expect_err(
            "ADR-0129 d2: the token is resolved from the repository, never passed as a flag",
        );
    assert_eq!(err.exit_code(), 2);
    assert!(
        err.to_string().contains("--token"),
        "the refusal did not name `--token`; it read:\n{err}"
    );

    let err = cli::try_parse_from(["redkiln", "answer", "an-item", "approve"])
        .expect_err("`--explain-sufficed` is required (ADR-0129 d1)");
    assert_eq!(err.exit_code(), 2);
    assert!(
        err.to_string().contains("--explain-sufficed"),
        "the refusal did not name the missing flag; it read:\n{err}"
    );

    let err = cli::try_parse_from([
        "redkiln",
        "answer",
        "an-item",
        "approve",
        "--explain-sufficed",
        "yes",
    ])
    .expect_err("`--toml-opened` is required too (ADR-0129 d1 fixes TWO sufficiency flags)");
    assert_eq!(err.exit_code(), 2);
    assert!(
        err.to_string().contains("--toml-opened"),
        "the refusal did not name the missing flag; it read:\n{err}"
    );

    let err = cli::try_parse_from([
        "redkiln",
        "answer",
        "an-item",
        "approve",
        "--explain-sufficed",
        "maybe",
        "--toml-opened",
        "no",
    ])
    .expect_err("the sufficiency flags are closed at yes|no");
    assert_eq!(err.exit_code(), 2);
    assert!(
        err.to_string().contains("maybe"),
        "the refusal did not name the rejected value; it read:\n{err}"
    );

    cli::try_parse_from([
        "redkiln",
        "answer",
        "an-item",
        "approve",
        "--explain-sufficed",
        "yes",
        "--toml-opened",
        "no",
    ])
    .expect("the two required flags, both given, parse");

    let names = parser_tree_names();
    assert_walk_covers(&names);
    let sufficiency: Vec<&String> = names
        .iter()
        .filter(|n| n.as_str() == "--explain-sufficed" || n.as_str() == "--toml-opened")
        .collect();
    assert_eq!(
        sufficiency.len(),
        2,
        "ADR-0129 d1 fixes exactly two sufficiency flags — a parser that grew a third or dropped \
         one is red here. Found: {sufficiency:?}"
    );
}

/// AC-403's lint half, D-15: no verb prompts and no verb reads stdin, `init` included.
///
/// The pattern is `io::stdin` / `stdin()` and deliberately **not** `read_line(` — any `BufRead` has
/// one, and a ban scoped wider than its warrant blocks valid work (golden constraint 9).
///
/// **Self-controlled**: the scan runs over its own `lint-controls/` file and must find the token
/// there BEFORE it asserts zero hits under `src/`. A scanner that visited nothing would otherwise
/// report a pass. Declared mutant (derived — §7d.14's enumeration omits this name while its general
/// rule still binds): `let _ = std::io::stdin();` on a non-comment line of `src/verbs/mod.rs`.
#[test]
fn ac403_no_blocking_stdin_read_in_rk_cli() {
    let control = lint_controls_dir().join("ac403_stdin_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| l.contains("io::stdin") || l.contains("stdin()"))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line matching the ban — the ban is vacuous",
        control.display()
    );

    let mut hits = Vec::new();
    for file in rs_files(&src_dir()) {
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (line_no, line) in code_lines(&text) {
            if line.contains("io::stdin") || line.contains("stdin()") {
                hits.push(format!("{}:{line_no}: {}", file.display(), line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "rk-cli reads stdin — no verb may, `init` included (AC-403 as amended, D-15):\n{}",
        hits.join("\n")
    );
}
