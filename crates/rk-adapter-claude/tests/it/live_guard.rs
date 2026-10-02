//! **Every `live::` test opens with the launch-record guard** (`s8h` §8h.7; s8h review #6). A code-line
//! lint over `tests/it/live.rs`, compiled OUTSIDE the `live` feature so `cargo test --workspace` runs it
//! while the paid tests stay uncompiled.
//!
//! *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* §8h.7: *"Every `live::` test
//! asserts the same variable as its first statement … Each `#[ignore]` reason names `tools/live-run.sh
//! probe live::<name>` instead of a raw `cargo` line."* The lint asserts, over `live.rs`'s CODE lines
//! (a `//`, `/*` or `*` line is dropped first, so prose naming the guard cannot satisfy it):
//! - the `#[ignore` count equals the guard-statement count;
//! - each guard is its function's first statement;
//! - no ignore reason names `cargo test`, and each names `tools/live-run.sh probe live::<its name>`;
//! - the guard function's body reads `REDKILN_LAUNCH_RECORD`, so the statement is not a no-op.
//!
//! It is a guard against accident, not a security boundary: the variable can be forged (§8h.7).

/// The one guard statement every `live::` test opens with.
const GUARD: &str = "require_launch_record();";

/// The guard function's signature line in `live.rs`.
const GUARD_FN: &str = "fn require_launch_record() {";

/// Whether a line is prose: the house code-line rule (a guard satisfied by its own prose is the
/// failure this exists for).
fn is_comment(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("//") || t.starts_with("/*") || t.starts_with('*')
}

/// Every violation of the four rules in `src`, one line each; empty when the file conforms.
fn violations(src: &str) -> Vec<String> {
    let code: Vec<&str> = src
        .lines()
        .filter(|l| !is_comment(l))
        .map(str::trim)
        .collect();
    let mut bad = Vec::new();
    let ignores: Vec<usize> = (0..code.len())
        .filter(|&i| code[i].starts_with("#[ignore"))
        .collect();
    let guards = code.iter().filter(|l| **l == GUARD).count();
    if ignores.is_empty() {
        bad.push("PREMISE: no #[ignore] test was read".to_owned());
    }
    if ignores.len() != guards {
        bad.push(format!(
            "{} #[ignore] tests but {guards} guard statements",
            ignores.len()
        ));
    }
    for &i in &ignores {
        let attr = code[i];
        let Some(fn_at) = (i + 1..code.len()).find(|&j| code[j].starts_with("fn ")) else {
            bad.push(format!("{attr}: no fn follows"));
            continue;
        };
        let name = code[fn_at]["fn ".len()..]
            .split(['(', '<', ' '])
            .next()
            .unwrap_or("")
            .to_owned();
        let reason = attr
            .split_once('"')
            .and_then(|(_, r)| r.rsplit_once('"'))
            .map_or("", |(r, _)| r);
        if reason.contains("cargo test") {
            bad.push(format!("{name}: its #[ignore] reason names `cargo test`"));
        }
        if !reason.contains(&format!("tools/live-run.sh probe live::{name}")) {
            bad.push(format!(
                "{name}: its #[ignore] reason does not name `tools/live-run.sh probe live::{name}`"
            ));
        }
        let Some(open) = (fn_at..code.len()).find(|&j| code[j].ends_with('{')) else {
            bad.push(format!("{name}: no body opens"));
            continue;
        };
        let first = (open + 1..code.len()).find(|&j| !code[j].is_empty());
        if first.map(|j| code[j]) != Some(GUARD) {
            bad.push(format!(
                "{name}: its first statement is not `{GUARD}` (got {:?})",
                first.map(|j| code[j])
            ));
        }
    }
    match code.iter().position(|l| *l == GUARD_FN) {
        None => bad.push(format!("the guard function `{GUARD_FN}` is not defined")),
        Some(at) => {
            let body_end = (at + 1..code.len())
                .find(|&j| code[j] == "}")
                .unwrap_or(code.len());
            if !code[at..body_end]
                .iter()
                .any(|l| l.contains("\"REDKILN_LAUNCH_RECORD\""))
            {
                bad.push(
                    "the guard function's body does not read \"REDKILN_LAUNCH_RECORD\"".to_owned(),
                );
            }
        }
    }
    bad
}

const GUARD_DEF: &str = "fn require_launch_record() {\n    let p = std::env::var_os(\"REDKILN_LAUNCH_RECORD\");\n    assert!(p.is_some());\n}\n";

/// §8h.7 / s8h review #6: at `fc80687` five `#[ignore]` tests carried no guard. Every `#[ignore]`
/// test in `live.rs` opens with the launch-record guard, and its reason names the launcher, never a
/// raw `cargo test` line. Each rule is first proved on a synthetic file it must refuse.
#[test]
fn every_live_test_opens_with_the_launch_record_guard() {
    // synthetic controls: each must be refused, or the reader is blind
    let good = format!(
        "{GUARD_DEF}#[test]\n#[ignore = \"PAID; run only as: tools/live-run.sh probe live::a\"]\nfn a() {{\n    require_launch_record();\n    // prose\n}}\n"
    );
    assert_eq!(
        violations(&good),
        Vec::<String>::new(),
        "CONTROL: a conforming file passes"
    );
    let unguarded = good.replace("    require_launch_record();\n", "");
    assert!(
        !violations(&unguarded).is_empty(),
        "CONTROL: an ignored test without the guard must be refused"
    );
    let second = good.replace(
        "    require_launch_record();\n",
        "    let x = 1;\n    require_launch_record();\n",
    );
    assert!(
        !violations(&second).is_empty(),
        "CONTROL: a guard that is not the first statement must be refused"
    );
    let prose = good.replace(
        "    require_launch_record();\n",
        "    // require_launch_record();\n",
    );
    assert!(
        !violations(&prose).is_empty(),
        "CONTROL: a guard in a comment must not satisfy the lint"
    );
    let cargo = good.replace(
        "PAID; run only as: tools/live-run.sh probe live::a",
        "run as: cargo test -p x --features live -- --ignored a; or tools/live-run.sh probe live::a",
    );
    assert!(
        !violations(&cargo).is_empty(),
        "CONTROL: a reason naming `cargo test` must be refused"
    );
    let noop = good.replace("REDKILN_LAUNCH_RECORD", "SOMETHING_ELSE");
    assert!(
        !violations(&noop).is_empty(),
        "CONTROL: a guard function that does not read the variable must be refused"
    );
    // the file
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it/live.rs");
    let src = std::fs::read_to_string(&path).expect("tests/it/live.rs");
    let bad = violations(&src);
    assert!(
        bad.is_empty(),
        "live.rs: every live:: test must open with the launch-record guard (s8h §8h.7):\n{}",
        bad.join("\n")
    );
    // PREMISE: the eight live tests §8h.7 names are the ones the lint read
    for name in [
        "live_dispatch_probe",
        "live_disallowed_tool_terminates",
        "s_retry_throttle_margin_driver",
        "s_retry_exhaustion_on_result_driver",
        "s_ttl1_subagent_card_driver",
        "live_denial_channel_preprobe",
        "live_confinement_and_denial_probe",
        "live_init_equality_probe",
    ] {
        assert!(
            src.lines()
                .filter(|l| !is_comment(l))
                .any(|l| l.trim_start().starts_with(&format!("fn {name}("))),
            "PREMISE: live.rs defines `{name}` (s8h §8h.7's eight)"
        );
    }
}
