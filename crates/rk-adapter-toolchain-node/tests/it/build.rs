//! **The Node `build` outcome** (s8c §8c.2; D21-62 D, provisional; record T = ADR-0169 decision 8,
//! proposed). `<program> tsc --noEmit` on the pinned `tsc 5.9.3`: exit `0` is `Ok(passing)` with
//! `errors = 0`; exit `2` is `Ok(failing)`, `errors` the count of `error TS<n>` lines with a floor
//! of 1, and it covers config errors, so **a broken `tsconfig.json` reads as a failing build, never a
//! passing one**; exit `1` (nothing was checked: no `tsconfig.json`, an unknown option, `-p` to a
//! missing project) and any other non-zero status are `Err(NonZero)`, cannot-run.
//!
//! **The four fixtures D21-62 D names**, each a throwaway project under this repository's
//! git-ignored `target/` (so no `tsconfig.json` is found above it, which is what makes (d)
//! discriminating), each carrying a `node_modules/.bin/tsc` shim that runs the Node fixture's own
//! installed `tsc` (so `npx` resolves it locally, never from the registry): (a) clean, (b) a type
//! error, (c) a `tsconfig.json` that is not JSON, (d) no `tsconfig.json` at all. They sit inside the
//! one named test, so the count of new tests does not move.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::{Path, PathBuf};

use rk_adapter_toolchain_node::NodeToolchain;
use rk_ports::toolchain::{StepBound, Toolchain};
use rk_types::error::StepError;
use rk_types::verb_output::BuildStatus;

use crate::support::{GENEROUS, config_with, fixture_project, minimal_env, preflight};

const TSCONFIG: &str = "{\"compilerOptions\": {\"strict\": true, \"noEmit\": true, \"target\": \"ES2022\", \"module\": \"ESNext\", \"moduleResolution\": \"Bundler\", \"skipLibCheck\": true}, \"include\": [\"src\"]}\n";

/// A throwaway project at `dir`: the given files, and a local `tsc` shim onto the fixture's own.
fn project(dir: &Path, files: &[(&str, &str)]) {
    for (rel, text) in files {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("mkdir");
        std::fs::write(path, text).expect("write a project file");
    }
    let bin = dir.join("node_modules").join(".bin");
    std::fs::create_dir_all(&bin).expect("mkdir node_modules/.bin");
    let real = fixture_project().join("node_modules").join(".bin");
    std::fs::write(
        bin.join("tsc"),
        format!(
            "#!/bin/sh\nexec \"{}\" \"$@\"\n",
            real.join("tsc").display()
        ),
    )
    .expect("write the sh shim");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(bin.join("tsc"), std::fs::Permissions::from_mode(0o755))
            .expect("chmod the shim");
    }
    std::fs::write(
        bin.join("tsc.cmd"),
        format!("@\"{}\" %*\r\n", real.join("tsc.CMD").display()),
    )
    .expect("write the cmd shim");
}

fn adapter_at(root: &Path, scratch: &Path) -> NodeToolchain {
    NodeToolchain::new(config_with(
        PathBuf::from(rk_adapter_toolchain_node::DEFAULT_PROGRAM),
        GENEROUS,
        root,
        minimal_env(),
        scratch,
    ))
    .expect("the shipped registry loads")
}

/// **A type error is `Ok` with `status: failing`**, and so is a broken `tsconfig.json`; no
/// `tsconfig.json` at all (tsc exit 1, nothing checked) is `Err(NonZero)` (D21-62 D; its mutant,
/// exit 1 read as `Ok(failing)`, is killed by case (d)).
#[test]
fn a_type_error_is_ok_with_status_failing() {
    preflight();
    let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
    std::fs::create_dir_all(&target).expect("the repository target dir");
    let tmp = tempfile::Builder::new()
        .prefix("s8c-node-build-")
        .tempdir_in(&target)
        .expect("a temp dir under the git-ignored target/");
    let scratch = tmp.path().join("scratch");

    // (b) a type error: two, counted.
    let typed = tmp.path().join("b-type-error");
    project(
        &typed,
        &[
            ("tsconfig.json", TSCONFIG),
            (
                "src/a.ts",
                "export const x: number = 'one';\nexport const y: string = 2;\n",
            ),
        ],
    );
    let report = adapter_at(&typed, &scratch)
        .build(StepBound::Undeclared)
        .expect("a type error is Ok(failing), never Err (record T decision 8; D21-62 D)");
    assert_eq!(report.status, BuildStatus::Failing, "{report:?}");
    assert_eq!(
        report.errors, 2,
        "two `error TS<n>` diagnostics: {report:?}"
    );
    assert_eq!(&report.argv[1..], ["tsc", "--noEmit"], "{report:?}");

    // (a) clean: passing, zero errors.
    let clean = tmp.path().join("a-clean");
    project(
        &clean,
        &[
            ("tsconfig.json", TSCONFIG),
            ("src/a.ts", "export const x: number = 1;\n"),
        ],
    );
    let report = adapter_at(&clean, &scratch)
        .build(StepBound::Undeclared)
        .expect("a clean project builds");
    assert_eq!(
        (report.status, report.errors),
        (BuildStatus::Passing, 0),
        "{report:?}"
    );

    // (c) a tsconfig.json that is not JSON: tsc exits 2, a failing build, never a passing one.
    let broken = tmp.path().join("c-bad-tsconfig");
    project(
        &broken,
        &[
            ("tsconfig.json", "{ this is not json\n"),
            ("src/a.ts", "export const x = 1;\n"),
        ],
    );
    let report = adapter_at(&broken, &scratch)
        .build(StepBound::Undeclared)
        .expect("a broken tsconfig is a failing build (the stated flaw), never cannot-run");
    assert_eq!(report.status, BuildStatus::Failing, "{report:?}");
    assert!(
        report.errors >= 1,
        "the floor of 1 under exit 2: {report:?}"
    );

    // (d) no tsconfig.json anywhere up the tree: tsc prints its help, checks nothing, exits 1.
    let bare = tmp.path().join("d-no-tsconfig");
    project(&bare, &[("src/a.ts", "export const x = 1;\n")]);
    match adapter_at(&bare, &scratch).build(StepBound::Undeclared) {
        Err(StepError::NonZero { code, .. }) => assert_eq!(code, Some(1), "tsc's own exit 1"),
        other => panic!(
            "tsc exit 1 checked nothing, so it is Err(NonZero), cannot-run, never a failing \
             build (D21-62 D); got {other:?}"
        ),
    }
}
