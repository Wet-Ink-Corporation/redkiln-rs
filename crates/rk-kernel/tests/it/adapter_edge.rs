//! **AC-484** — `rk-kernel` has no `rk-adapter-*` edge, and the fixture that proves the assertion
//! can see one.
//!
//! AC-484: *"GIVEN the kernel and the adapters / WHEN the dependency graph is read / THEN
//! `rk-kernel` has no dependency edge — direct or transitive — on any `rk-adapter-*` crate / AND an
//! added edge fails CI **naming the adapter crate**."* *Verified by:* *"a `cargo metadata`
//! dependency-graph assertion in CI; a fixture commit adding `rk-adapter-sqlite` to `rk-kernel`
//! that must fail; the same `deny.toml` `[bans].deny` list AC-005 names."*
//!
//! The mechanism here is **D-1, taken 2026-09-14**: `tests/fixtures/rk-adapter-sqlite/` is a
//! minimal feature-gated workspace MEMBER on the `tests/fixtures/rk-banned-dep/` and
//! `tests/fixtures/rk-clock-read/` precedent (DECISIONS-2026-09-12 E4), and `rk-kernel` carries an
//! **optional** edge to it behind the non-default `adapter-edge-reintroduce` feature. The
//! "fixture commit" the register asks for is therefore **committed and permanent** rather than made
//! and reverted — a reverted observation is not an assertion — and the must-fail half is reached by
//! a feature flip inside this one test:
//!
//! ```text
//! green:  cargo metadata                                             -> no rk-adapter-* in rk-kernel's RESOLVED graph
//! RED:    cargo metadata --features rk-kernel/adapter-edge-reintroduce -> the assertion fails NAMING rk-adapter-sqlite
//! ```
//!
//! # 🛑 THE SUBTLETY THAT IS THE WHOLE TEST: `resolve`, NEVER `packages`
//!
//! `cargo metadata` publishes **two** readings of the same dependency relation and they disagree
//! about exactly this fixture:
//!
//! | reading | JSON path | what it is | says of `rk-adapter-sqlite` with the feature OFF |
//! | --- | --- | --- | --- |
//! | manifest | `packages[name=="rk-kernel"].dependencies[]` | what the **manifest declares**, optional or not, feature selection ignored | **present** |
//! | resolved | `resolve.nodes[id==<rk-kernel>].deps[]` | what the **resolver selected** under the features actually in force | **absent** |
//!
//! Asserting over `packages` would make the green half **impossible**: the optional edge is listed
//! there whether or not it is enabled, so the default `cargo metadata` would report a violation on
//! a workspace that builds no adapter code at all. Asserting over `resolve` is the only reading
//! that answers AC-484's question. [`ac484_adapter_edge_fixture`] takes **both** readings off the
//! same bytes and asserts they disagree, so the trap is a committed assertion rather than a comment
//! warning a future reader about it.
//!
//! A second trap sits one level down and is asserted with it: `resolve.nodes[].deps[].name` is the
//! **lib** name (`rk_adapter_sqlite`), not the package name. AC-484's THEN is *"naming the adapter
//! crate"*, so [`rk_kernel::adapter_edge`] resolves every `deps[].pkg` id back through `packages[]`
//! and reports the package name; the test asserts the hyphenated spelling appears in the violation's
//! `Display`.
//!
//! # WHAT IS MEASURED, AND WHAT IS NOT
//!
//! | AC-484 clause | instrument here | status |
//! | --- | --- | --- |
//! | *"no dependency edge — direct or transitive — on any `rk-adapter-*` crate"* | [`rk_kernel::adapter_edge::resolved_reading`] over real `cargo metadata` output, walking normal edges transitively from `rk-kernel` | **measured** |
//! | *"an added edge fails … naming the adapter crate"* | the same walk over `cargo metadata --features rk-kernel/adapter-edge-reintroduce`, asserting the route **and** the hyphenated package name in `Display` | **measured** |
//! | the fixture is a workspace **member** (D-1) | `workspace_members` carries the fixture's id, so Step 8 (4)'s `cargo metadata --no-deps` membership check resolves | **measured** |
//! | the fixture *"pulls nothing"* (AC-528, and `cargo audit`/`cargo deny` read the lock) | the fixture package's resolved node has **zero** deps in both readings | **measured** |
//! | cross-check against the reading `crates/rk-kernel/Cargo.toml` itself names | `cargo tree -p rk-kernel -e normal`, green and red | **measured** |
//! | *"the same `deny.toml` `[bans].deny` list AC-005 names"* | **nothing**, and it cannot be built — see the OWED erratum below | **ARMED, NOT DISCHARGED** |
//!
//! # 🛑 ERRATUM OWED — AC-484's THIRD INSTRUMENT DOES NOT SURVIVE CONTACT WITH `deny.toml`
//!
//! **Recorded, not fixed.** The register is not edited here; this is the measurement and the
//! reasoning a future editor needs.
//!
//! AC-484's *Verified by* names three instruments. The third is *"the same `deny.toml`
//! `[bans].deny` list AC-005 names (ADR-0100 decision 3), so the two instruments read one committed
//! list."*
//!
//! **Measured 2026-09-14 against `deny.toml` on this tree:** `[bans].deny` names
//! `temporalio-sdk`, `temporalio-sdk-core`, `graph-flow`, `dbos` and `durable` — **five workflow
//! engines and no adapter**. The list AC-484 points at does not mention an `rk-adapter-*` crate,
//! so the instrument as written measures nothing about this criterion today.
//!
//! **And it cannot be repaired by adding the name.** `cargo deny`'s bans are **workspace-wide** —
//! `deny.toml` has no per-crate scoping, exactly as `clippy.toml` has none (`clippy.toml`:3-4 says
//! so of itself) — while AC-484 is a **per-crate edge** property: *"`rk-kernel` has no dependency
//! edge"*. Banning `rk-adapter-sqlite` workspace-wide would forbid it in **`rk-cli`**, which
//! legitimately depends on adapters at Step 7d. A workspace-wide ban cannot express "absent from
//! this crate's graph and present in that one", which is the only thing AC-484 asks.
//!
//! **What is owed:** an erratum against AC-484 withdrawing the third *Verified by* clause, leaving
//! the `cargo metadata` assertion and the fixture crate — the two instruments that do carry it —
//! standing. Nothing in this file claims the clause is covered, and the table above marks it
//! **ARMED, NOT DISCHARGED** rather than silently dropping it.
//!
//! # Why the cargo invocations are in the test and not in `src/`
//!
//! [`rk_kernel::adapter_edge`] is a **pure reader**: it takes `cargo metadata`'s JSON as a `&str`
//! and returns a report. It spawns nothing. The two `std::process::Command` calls live here, in the
//! test binary, on `crates/rk-store/tests/it/contention.rs`:77-85's precedent — `clippy.toml`'s
//! type ban is on `Command` **for git** (ADR-0077:70, ADR-0105 clause 5 enumerating the git write
//! set) and neither covers a non-git child. The `#[expect]` below carries that warrant, and
//! `#[expect]` rather than `#[allow]` means the exception fails the build the day it stops being
//! needed (ADR-0102 decision 3).
//!
//! Keeping the spawn out of `src/` also keeps the kernel honest: a library function that shells out
//! to `cargo` inside `rk-kernel` would be a process launch in the crate AC-003's ban is written
//! about, bought for nothing — the parsing is the part that has to be right, and it is pure.

use std::path::{Path, PathBuf};
use std::process::Output;

use rk_kernel::adapter_edge::{
    ADAPTER_CRATE_PREFIX, AdapterEdgeReport, Reading, SUBJECT_CRATE, manifest_reading,
    resolved_reading,
};

/// The fixture crate D-1 names. Written out as a constant because three different assertions below
/// have to agree about the spelling, and the hyphen is the half AC-484's *"naming the adapter
/// crate"* turns on.
const FIXTURE_CRATE: &str = "rk-adapter-sqlite";

/// The non-default feature of `rk-kernel` that re-introduces the edge.
const REINTRODUCE: &str = "rk-kernel/adapter-edge-reintroduce";

/// The workspace root — `crates/rk-kernel/../..`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Run one `cargo` subcommand at the workspace root and return its stdout, refusing loudly on a
/// non-zero exit rather than parsing an empty string.
///
/// `CARGO` is set by cargo for every test binary it runs; the fallback keeps the test runnable
/// under a bare `cargo` on `PATH`.
#[expect(
    clippy::disallowed_types,
    reason = "AC-484's named instrument is `a cargo metadata dependency-graph assertion`, so the \
              test must run `cargo` as a child process. `clippy.toml`'s std::process::Command ban \
              is Checkpoint 6's git ban -- ADR-0077:70's one-wrapper rule, whose write set \
              ADR-0105 clause 5 enumerates -- and neither covers a non-git child. The precedent \
              for this exact exception in a test binary is \
              crates/rk-store/tests/it/contention.rs:77-85, AC-181's two-process test."
)]
pub(crate) fn cargo(args: &[&str]) -> String {
    let bin = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let root = workspace_root();
    let out: Output = std::process::Command::new(&bin)
        .args(args)
        .current_dir(&root)
        .output()
        .unwrap_or_else(|e| panic!("could not spawn `{bin} {}`: {e}", args.join(" ")));
    assert!(
        out.status.success(),
        "`cargo {}` exited {:?}\n--- stderr ---\n{}",
        args.join(" "),
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("cargo's stdout is UTF-8")
}

/// `cargo metadata` at the workspace root, with the feature selection the caller wants in force.
pub(crate) fn metadata(extra: &[&str]) -> String {
    let mut args = vec![
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        "Cargo.toml",
    ];
    args.extend_from_slice(extra);
    cargo(&args)
}

#[test]
fn ac484_adapter_edge_fixture() {
    // ---------------------------------------------------------------------------------------
    // 0. PREMISES. The fixture exists on disk, pulls nothing, and is a workspace MEMBER.
    //
    //    D-1's whole reason for a MEMBER rather than an excluded package: Step 8 (4)'s
    //    `cargo metadata --no-deps` membership check resolves for a member and would not for an
    //    excluded one. A fixture cargo cannot see is a fixture nothing checks.
    // ---------------------------------------------------------------------------------------
    let fixture_dir = workspace_root().join("tests/fixtures").join(FIXTURE_CRATE);
    assert!(
        fixture_dir.join("Cargo.toml").is_file(),
        "PREMISE: D-1's fixture crate must be committed at {}",
        fixture_dir.display()
    );

    let green_json = metadata(&[]);
    assert!(
        green_json.len() > 10_000,
        "PREMISE: `cargo metadata` returned {} bytes; the real workspace graph is far larger and a \
         truncated read would make every assertion below vacuous",
        green_json.len()
    );

    // ---------------------------------------------------------------------------------------
    // 1. THE THEN: no `rk-adapter-*` edge, direct or transitive, in the RESOLVED graph.
    // ---------------------------------------------------------------------------------------
    let green = resolved_reading(&green_json, SUBJECT_CRATE).expect("cargo metadata parses");
    assert_eq!(green.reading, Reading::Resolved);
    assert_eq!(
        green.subject, SUBJECT_CRATE,
        "the walk must start at the crate the criterion names"
    );

    // PREMISE, and it is the one that stops a green over nothing: the walk reached the real graph.
    assert!(
        green.packages_walked >= 20,
        "PREMISE: the transitive walk from `{SUBJECT_CRATE}` reached {} packages; `rk-store` alone \
         pulls rusqlite and libsqlite3-sys, and a walk that visited a handful of nodes would \
         report `no adapter edge` for the wrong reason",
        green.packages_walked
    );
    assert!(
        green.route_to("rk-store").is_some() && green.route_to("rk-git").is_some(),
        "PREMISE: the walk must find the edges the manifest DOES declare; it found {:?}",
        green.reached_names()
    );
    assert!(
        green.route_to("rusqlite").is_some(),
        "PREMISE: the walk must be TRANSITIVE -- rusqlite is reached only through rk-store -- and \
         AC-484's THEN says `direct or transitive`"
    );

    assert_eq!(
        green.findings,
        Vec::new(),
        "AC-484's THEN: `rk-kernel` has no dependency edge, direct or transitive, on any \
         `{ADAPTER_CRATE_PREFIX}*` crate"
    );
    assert!(
        green.verdict().is_ok(),
        "the verdict must agree with the finding list: {:?}",
        green.verdict()
    );

    // ---------------------------------------------------------------------------------------
    // 2. 🛑 THE TRAP, ASSERTED OVER THE SAME BYTES.
    //
    //    The MANIFEST view lists the optional edge whether or not it is enabled. An assertion
    //    written over `packages[]` would make step 1 impossible. This is the committed proof that
    //    the rule reads `resolve` and that the difference is real rather than remembered.
    // ---------------------------------------------------------------------------------------
    let green_manifest =
        manifest_reading(&green_json, SUBJECT_CRATE).expect("the same bytes parse");
    assert_eq!(green_manifest.reading, Reading::Manifest);
    assert_eq!(
        green_manifest
            .findings
            .iter()
            .map(|f| f.adapter_crate.as_str())
            .collect::<Vec<_>>(),
        vec![FIXTURE_CRATE],
        "the MANIFEST reading of the SAME `cargo metadata` output must list `{FIXTURE_CRATE}` \
         while the feature is OFF -- that is why the assertion is over `resolve` and never over \
         `packages`. If this ever comes back empty, either the optional edge was deleted from \
         crates/rk-kernel/Cargo.toml or cargo changed the meaning of `packages[].dependencies`, \
         and in EITHER case the resolved-graph green above has stopped proving anything."
    );
    assert!(
        green_manifest.verdict().is_err(),
        "the two readings must DISAGREE on this tree; that disagreement is the subtlety this test \
         exists to pin"
    );

    // ---------------------------------------------------------------------------------------
    // 3. PREMISE: the fixture pulls NOTHING.
    //
    //    AC-528 forbids an async runtime in `rk-kernel`'s resolved graph directly or transitively,
    //    and `cargo audit` / `cargo deny` read the LOCK -- which an optional dependency enters
    //    whether or not it is enabled (tests/fixtures/rk-banned-dep/Cargo.toml says so in terms).
    //    A fixture with dependencies would put a transitive tree under two gates for no benefit.
    // ---------------------------------------------------------------------------------------
    assert_eq!(
        rk_kernel::adapter_edge::resolved_dep_count(&green_json, FIXTURE_CRATE)
            .expect("the fixture is in the graph as a workspace member"),
        0,
        "D-1: the fixture crate must pull NOTHING -- it enters Cargo.lock either way, and \
         `cargo audit --deny unmaintained` reads the lock"
    );
    assert!(
        rk_kernel::adapter_edge::is_workspace_member(&green_json, FIXTURE_CRATE)
            .expect("the same bytes parse"),
        "D-1: the fixture is a workspace MEMBER (Cargo.toml's `members = [\"crates/*\", \
         \"tests/fixtures/*\"]`), so Step 8 (4)'s `cargo metadata --no-deps` membership check \
         resolves for it; it would not for an excluded package"
    );

    // ---------------------------------------------------------------------------------------
    // 4. THE MUST-FAIL HALF: the same assertion, with the fixture's edge selected, FAILS and
    //    NAMES the adapter crate.
    // ---------------------------------------------------------------------------------------
    let red_json = metadata(&["--features", REINTRODUCE]);
    let red = resolved_reading(&red_json, SUBJECT_CRATE).expect("cargo metadata parses");

    assert_eq!(
        red.findings
            .iter()
            .map(|f| f.adapter_crate.as_str())
            .collect::<Vec<_>>(),
        vec![FIXTURE_CRATE],
        "with `--features {REINTRODUCE}` the RESOLVED graph must carry the edge; if this is empty \
         the fixture is inert and the green above is a green over a rule that cannot fire"
    );

    let violation = red
        .verdict()
        .expect_err("the reintroduced edge must produce a violation");
    let rendered = violation.to_string();
    assert!(
        rendered.contains(FIXTURE_CRATE),
        "AC-484's THEN is `an added edge fails CI NAMING the adapter crate`, and the rendered \
         violation must carry the HYPHENATED package name rather than the lib name \
         `rk_adapter_sqlite` that `resolve.nodes[].deps[].name` supplies. Rendered:\n{rendered}"
    );
    assert!(
        rendered.contains(SUBJECT_CRATE),
        "the violation must name the crate whose graph was read, or a CI reader cannot tell which \
         edge to cut. Rendered:\n{rendered}"
    );

    // The ROUTE, not merely the name: AC-484's THEN covers transitive edges, so the report has to
    // say how the adapter was reached or a transitive hit is unactionable.
    let route = &red.findings[0].route;
    assert_eq!(
        route,
        &vec![SUBJECT_CRATE.to_string(), FIXTURE_CRATE.to_string()],
        "the fixture's edge is DIRECT, so its route is exactly two hops; a longer route here means \
         the walk found a different adapter and the test is measuring the wrong thing"
    );

    // ---------------------------------------------------------------------------------------
    // 5. CROSS-CHECK, against the reading crates/rk-kernel/Cargo.toml itself names.
    //
    //    That manifest's closing comment says Checkpoint 7b asserts this over
    //    `cargo tree -p rk-kernel -e normal`. `cargo tree` respects default features, so it is an
    //    INDEPENDENT instrument: it agrees with `resolve` and disagrees with `packages`, which is
    //    a third witness that the reading chosen above is the right one.
    // ---------------------------------------------------------------------------------------
    let tree_green = cargo(&["tree", "-p", "rk-kernel", "-e", "normal"]);
    assert!(
        tree_green.contains("rk-store") && tree_green.contains("rk-git"),
        "PREMISE: `cargo tree -p rk-kernel -e normal` must show the edges that DO exist:\n{tree_green}"
    );
    assert!(
        !tree_green.contains(ADAPTER_CRATE_PREFIX) && !tree_green.contains("rk_adapter_"),
        "`cargo tree -p rk-kernel -e normal` must show no adapter crate:\n{tree_green}"
    );

    let tree_red = cargo(&[
        "tree",
        "-p",
        "rk-kernel",
        "-e",
        "normal",
        "--features",
        REINTRODUCE,
    ]);
    assert!(
        tree_red.contains(FIXTURE_CRATE),
        "with the feature on, the independent `cargo tree` instrument must agree with the \
         `resolve` reading and show `{FIXTURE_CRATE}`:\n{tree_red}"
    );
}

/// A compile-time witness that the report type is nameable from outside the crate, so a future
/// consumer (Step 8's membership check, or `rk-cli`) binds the same reader rather than writing a
/// second one over `packages[]`.
const _: fn(&AdapterEdgeReport) -> usize = |r| r.packages_walked;
