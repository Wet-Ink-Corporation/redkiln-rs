//! **`Config`, the value ADR-0083 names** — declared here because here is where it is accepted, and
//! threaded into [`rk_kernel::replay::RunInputs`] without moving a single `run_id` already recorded.
//!
//! Step 7d Slice 2, the first of §7d.1's eight kernel edits (**D18-14**). C-01 **RB-M-05** found
//! that ADR-0083's *"a `Config` struct constructed at the composition root and passed in"* had no
//! referent anywhere in the workspace; this module is the test that it now has one, and that adding
//! it cost nothing.

use std::path::Path;

use rk_kernel::config::Config;
use rk_kernel::replay;

/// The one committed run record on this tree, whose **directory name IS its `run_id`**
/// (`{stage}-{digest[..16]}`, `RunInputs::run_id`). That coincidence is what makes it an anchor: it
/// was computed and written down before this slice existed, so recomputing it now compares the
/// present serialiser against a byte string nobody can quietly update.
const RECORDED_RUN: &str = "implement-bf1d8cec3b22d288";

/// The environment-read spellings conjunct 4 refuses inside the kernel — the same four Checkpoint
/// 7d's `rk_absent` line uses for `rk-cli`, so the two agree on what an environment read is.
///
/// Module level, not inside the test: `clippy::items_after_statements` is denied workspace-wide.
const ENV_SPELLINGS: [&str; 4] = ["std::env::var", "env::var(", "env::vars(", "env::var_os("];

fn recorded_run_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("logs")
        .join(RECORDED_RUN)
}

/// ADR-0083, D18-14; §7d.1 edit (1); §7d.15's byte-stability clause.
///
/// **Four** conjuncts. The fourth was missing when this slice first shipped, and its absence was
/// the exact shape this tree has been burned by before: the module under test carried a sentence
/// saying the kernel reads no environment, and nothing read the module. Measured 2026-09-19, with
/// the scan absent: `std::env::var("RK_HARNESS_OVERRIDE")` inserted into `Config::new` left all 181
/// kernel tests green AND both Checkpoint 7b and Checkpoint 7b-ii at rc=0 — so the slice's own
/// stated proof passed the mutant. §7d.14 had declared the red all along: *"or by `Config` being
/// constructible from the environment inside the kernel (a source scan: no `std::env` in
/// `config.rs`)"*.
///
/// The four, with the second being the one that matters:
///
/// 1. the type is **the kernel's** — `rk_kernel::config::Config` resolves, which is the whole of
///    what RB-M-05 found missing;
/// 2. the committed run's `run_id` is **unchanged** by this slice, recomputed from its own recorded
///    inputs and compared both to what `run.json` records and to the directory the record was
///    written under. This subsumes "a `RunInputs` carrying no config records no `config` key":
///    `run_id` is a digest over the canonical bytes of that object, so a key written as `null` —
///    which is what `json!` does for the `deviation` and `runtime` `Option`s beside it — moves the
///    digest and fails here. It is the stronger reading of the two, and it needs no widening of
///    this crate's API to reach a private `to_json`;
/// 3. threading a config in **does** move the `run_id` — the positive control, without which a
///    `with_config` that silently dropped its argument would satisfy (2) forever;
/// 4. **`config.rs` names no environment read.** ADR-0083's whole content is that the kernel
///    receives configuration as a value rather than fetching it; a kernel that read `std::env`
///    while BUILDING that value would satisfy the letter of "passed in" and none of its point.
///    Scanned over code lines only and self-controlled first, because the module's own header
///    states the rule in prose and an unanchored grep would be satisfied by the explanation.
///
/// Red by: the absent module (conjunct 1); a `config` key written unconditionally, which changes
/// the canonical bytes of every recorded input object (conjunct 2); a `with_config` that does not
/// store what it is given (conjunct 3); any environment read in `config.rs` (conjunct 4).
#[test]
fn adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs() {
    let dir = recorded_run_dir();
    assert!(
        dir.is_dir(),
        "the committed run record {} is absent — this test would otherwise assert byte-stability \
         against nothing",
        dir.display()
    );

    let run = replay::read_run(&dir).expect("the committed run record reads");

    // (2) and (3) are ONE assertion, and deliberately so. `run_id` is
    // `sha256_lp([SCHEMA, stage, canonical(inputs)])` and `inputs_digest` is a digest over the same
    // canonical bytes — so "the recorded object gained no `config` key" is not a separate thing to
    // check by inspecting the object. A `config: null` written the way `json!` writes `deviation`
    // and `runtime` would change those bytes, and both values below would move. Comparing the
    // digests against a COMMITTED artefact is the stronger instrument, and it needs no widening of
    // this crate's API to reach a private `to_json`.
    let recomputed = run.inputs.run_id().expect("the run_id recomputes");
    assert_eq!(
        recomputed,
        run.run_id(),
        "the committed run's run_id moved: `run.json` records {}, and recomputing from its own \
         recorded inputs now gives {recomputed}. That is exactly what an unconditional `config` \
         key does to every run ever recorded, which is why §7d.15 makes the whole-checkpoint \
         re-run of 7b and 7b-ii the proof of this edit",
        run.run_id()
    );
    assert_eq!(
        recomputed, RECORDED_RUN,
        "the run_id no longer matches the directory the record was written under"
    );

    // (3) The positive control: threading a config in DOES move it.
    let toolchains = vec![
        "rk-adapter-toolchain-node".to_owned(),
        "rk-adapter-toolchain-rust".to_owned(),
    ];
    let config = Config::new("rk-adapter-claude", "rk-adapter-sqlite", &toolchains, 50);
    let threaded = run.inputs.clone().with_config(config);
    let moved = threaded.run_id().expect("the threaded run_id computes");
    assert_ne!(
        moved, recomputed,
        "threading a Config left the run_id unchanged — so `with_config` dropped its argument, or \
         the value never reaches the recorded object, and conjuncts (2) and (3) above would pass \
         over a member that does nothing"
    );

    // (4) The kernel does not read its environment while building the value.
    let lint_controls = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("lint-controls");
    let control = lint_controls.join("adr0083_kernel_config_env_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    assert!(
        code_lines(&control_text)
            .iter()
            .any(|l| ENV_SPELLINGS.iter().any(|s| l.contains(s))),
        "the control {} carries no NON-COMMENT environment read — the scan below is vacuous",
        control.display()
    );

    let config_rs = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("config.rs");
    let config_text = std::fs::read_to_string(&config_rs).expect("config.rs is readable");
    let hits: Vec<String> = code_lines(&config_text)
        .into_iter()
        .filter(|l| ENV_SPELLINGS.iter().any(|s| l.contains(s)))
        .collect();
    assert!(
        hits.is_empty(),
        "`{}` reads the process environment. ADR-0083 puts that read at the composition root and \
         hands the kernel the finished value; a kernel that fetched it while building `Config` \
         would satisfy the letter of \"passed in\" and none of its point:\n{}",
        config_rs.display(),
        hits.join("\n")
    );
}

/// Source lines that carry CODE — comment lines dropped, so a scan is never satisfied by the prose
/// that explains it. The same filter Checkpoint 7d's shell helpers apply.
fn code_lines(src: &str) -> Vec<String> {
    src.lines()
        .filter(|line| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') || t.is_empty())
        })
        .map(str::to_owned)
        .collect()
}
