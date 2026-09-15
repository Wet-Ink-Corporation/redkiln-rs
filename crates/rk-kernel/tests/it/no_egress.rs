//! **AC-498** — the kernel and pack-validation suites complete with **zero model dispatches**, and
//! nothing in the crate's resolved graph could make an outbound call.
//!
//! AC-498: *"GIVEN the test suites / WHEN they run / THEN the `rk-kernel` and pack-validation test
//! suites complete with **zero model dispatches and zero outbound network calls**, using fakes for
//! all five ports / AND a test that reaches a real adapter fails the suite."* *Verified by:* *"a CI
//! job with network egress denied and no adapter crates linked; a dispatch counter asserted at 0."*
//!
//! The mechanism here is **D-2, taken 2026-09-14**: all three halves are built, and the egress half
//! is a CI job in a container with `--network none` rather than a marketplace action.
//!
//! # 🛑 WHAT THE THREE IN-TREE ASSERTIONS CANNOT SEE, SAID FIRST SO NOTHING BELOW READS AS COVERING IT
//!
//! A resolved-graph assertion answers *"is a networking **crate** linked?"* It cannot answer *"can
//! this process open a socket?"*, because **`std::net::TcpStream` is in `std`** and reaches no
//! `Cargo.toml`, no `Cargo.lock` and no `resolve.nodes[]` entry. One `std::net::TcpStream::connect`
//! written into `rk-kernel` tomorrow leaves every assertion in this file green.
//!
//! **And no lint closes it either.** Measured 2026-09-14 against the workspace root:
//! `grep -niE 'net|tcp|socket|reqwest|hyper|ureq|curl' clippy.toml deny.toml` returns **0 lines**.
//! `clippy.toml`'s `disallowed-methods` carries AC-001's three clocks, AC-002's three
//! non-deterministic ids, AC-003's **two sleeps** (`std::thread::sleep`, `tokio::time::sleep`) and
//! ADR-0083's two env setters — and **no network entry at all**, on either list. So AC-003's
//! network half is enforced by **nothing** in this tree and stays **ARMED**.
//!
//! **The CI job is the only instrument that reaches that class**, because a denied-egress container
//! fails the `connect` itself rather than the manifest that did not declare it. `.github/workflows/
//! ci.yml`'s `no-egress` job is that instrument, and block 4 below asserts it is **present** — never
//! that it has run.
//!
//! # WHAT IS MEASURED, AND WHAT IS NOT
//!
//! | AC-498 clause | instrument here | status |
//! | --- | --- | --- |
//! | *"zero model dispatches"* | block 1: **one caller-owned `AtomicU32`**, bound as the only `Harness` a sweep of both named suites can reach, read on this thread after the sweep | **measured** |
//! | …and that the zero is a reading and not a rule that cannot fire | block 1's **control**: the same fake, same crate, a well-formed key — the counter reads **1** | **measured** |
//! | *"using fakes for all five ports"* | block 1's exhaustive destructure of `rk_ports::root::Ports` — a sixth port stops this file compiling, and a `Runtime` cannot be built over four | **a type fact** |
//! | *"no adapter crates linked"* | block 2: AC-484's own walk ([`rk_kernel::adapter_edge::resolved_reading`], re-used through [`rk_kernel::no_egress::resolved_egress_surface`] — **not a second walker**) over real `cargo metadata` output | **measured** |
//! | *"zero outbound network calls"*, as a **linkage** property | block 3: no package in the resolved graph matches D-2's seven tokens, cross-checked against `cargo tree -p rk-kernel -e normal` | **measured, and bounded — see below** |
//! | …and that **that** rule can fire | block 3's control: the same reader over a synthetic graph reaching `tokio-util` transitively, which it must name — and `hyperlocal`, which it must **not** | **measured** |
//! | *"zero outbound network calls"*, as a **runtime** property | **nothing in this file.** The `no-egress` CI job, asserted present only | **ARMED, NOT DISCHARGED** |
//! | *"a test that reaches a real adapter fails the suite"* | **AC-484's** committed fixture edge and its feature flip (`tests/it/adapter_edge.rs`), not re-asserted here | **measured there** |
//!
//! # 🛑 THREE BOUNDS ON BLOCK 3, EACH ONE A HOLE THIS READING LEAVES OPEN
//!
//! 1. **It is a denylist of seven tokens**, fixed by D-2 from a measurement (`cargo tree -p
//!    rk-kernel -e normal` finds **0** of `reqwest|hyper|tokio|ureq|curl|rustls|openssl`). A
//!    networking crate named none of those — `native-tls`, `ring`, `socket2`, `mio` — is invisible
//!    to it. Widening the list is a decision, not a local edit, and is not taken here.
//! 2. **It walks `kind: null` edges only**, because it re-uses AC-484's walk rather than writing a
//!    second one. A networking crate arriving as a **transitive dev-dependency** of `rk-kernel` is
//!    therefore not reached, even though it *is* linked into the test binary this criterion is
//!    about. Block 3b closes the **direct** dev edges by name and **only** those; the dev closure is
//!    not walked and is not claimed.
//! 3. **Linked is not called.** A crate in the graph that never opens a socket and a crate that does
//!    are the same fact to this reading. That asymmetry is why the CI job is not optional.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use rk_kernel::adapter_edge::{ADAPTER_CRATE_PREFIX, SUBJECT_CRATE};
use rk_kernel::no_egress::{
    NETWORKING_CRATES, networking_token, resolved_egress_surface, subject_dev_dependencies,
};
use rk_kernel::pack_validation::pack_capability_routing_rule;
use rk_kernel::resolver::{
    CapabilityClaim, CapabilityNeed, ClaimedCapabilities, dispatch_if_resolved,
};
use rk_kernel::runtime::Runtime;
use rk_ports::harness::Harness;
use rk_ports::root::Ports;

use crate::runtime::{
    AID, CountingHarness, CountingStore, ITEM, STEP, counters, ok_typed, ports, spec, store,
};

/// The pack-validation suite's own two fixtures — the same files `tests/it/pack_validation.rs`
/// drives, included here rather than re-authored so the sweep below runs the **suite's** work and
/// not a smaller imitation of it.
const UNROUTABLE_PACK: &str = include_str!("../fixtures/packs/ac495-unroutable.toml");
/// Its routed sibling.
const ROUTED_PACK: &str = include_str!("../fixtures/packs/ac495-routed.toml");

/// The workflow this file asserts the presence of a job in. `include_str!` and not a path read: a
/// missing workflow must be a **compile** error, not a `Result` a future editor can absorb.
const CI_WORKFLOW: &str = include_str!("../../../../.github/workflows/ci.yml");

/// A registry claiming the alternative token and **not** the one the unroutable pack's only route
/// needs — `tests/it/pack_validation.rs`'s `configured_adapter`, which is deliberately non-empty:
/// a rule that refused every pack would pass against a registry that claimed nothing.
fn configured_adapter() -> ClaimedCapabilities {
    ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new("whole-suite", "2026-09-14", "toolchain:whole-suite@1")
            .expect("a row"),
        CapabilityClaim::new("json-schema", "2026-09-14", "harness:schema-probe@3").expect("a row"),
    ])
    .expect("two distinct tokens")
}

#[test]
fn ac498_no_egress_dispatch_zero() {
    // =========================================================================================
    // 1. THE DISPATCH COUNTER, ASSERTED AT 0.
    //
    //    ONE `AtomicU32`, owned by this test, moved only by the `Harness` fake bound into the
    //    `Ports` every driver below is given. It is read HERE, on this thread, after the sweep —
    //    so the 0 is a reading off an instrument the code under test cannot write to, rather than
    //    a number a run printed about itself (`ReplayPorts.dispatches`' own model).
    // =========================================================================================
    let (queries, appends) = counters();
    let db = store(&queries, &appends);
    let sweep = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"never": "reached"}),
    )]));

    // *"using fakes for all five ports"* — as a TYPE FACT and not a sentence. A sixth port stops
    // this file compiling; a `Runtime` cannot be constructed over four.
    let bound = ports(Arc::clone(&sweep) as Arc<dyn Harness>, &db);
    let Ports {
        store: _,
        harness: _,
        tracker: _,
        docs: _,
        toolchain: _,
    } = &bound;

    // ---- 1a. the PACK-VALIDATION suite's work, under the bound counter ----------------------
    let unroutable = pack_capability_routing_rule(
        "ac495-unroutable.toml",
        UNROUTABLE_PACK,
        &configured_adapter(),
    );
    assert!(
        unroutable.is_err(),
        "PREMISE: the unroutable fixture must still be unroutable, or this sweep drives nothing"
    );
    let routed =
        pack_capability_routing_rule("ac495-routed.toml", ROUTED_PACK, &configured_adapter());
    assert!(
        routed.is_ok(),
        "PREMISE: the routed fixture must still validate cleanly: {routed:?}"
    );

    // ---- 1b. the resolver's refusal, with the CLOSURE WIRED TO THE PORT ----------------------
    //
    //     `dispatch_if_resolved`'s closure is what a `pass` would run. Wiring it to the same fake
    //     makes *"never a `pass`"* a movement of THIS counter rather than a `match` arm a test
    //     wrote: if the refusal ever stopped coming first, the number below would be 1.
    let mut side = CountingStore::new(&queries, &appends);
    let needs = vec![CapabilityNeed::new(AID, STEP, "affected-selection")];
    let refused = dispatch_if_resolved(&needs, &configured_adapter(), || {
        sweep.dispatch(&spec(), &mut side)
    });
    assert!(
        refused.is_err(),
        "PREMISE: the resolver must refuse a need nothing claims, or the closure above ran"
    );

    // ---- 1c. the KERNEL suite's boundary, on the three malformed keys AC-010 names -----------
    let runtime = Runtime::new(bound, "run-no-egress", crate::runtime::STAGE);
    for (item, aid) in [("", AID), (ITEM, ""), (ITEM, "impl-report")] {
        let got = runtime.dispatch_once(item, aid, STEP, 1, &spec());
        assert!(
            got.is_err(),
            "PREMISE: ({item:?}, {aid:?}) is a malformed key and the boundary must refuse it"
        );
    }

    // ---- 1d. THE READING ---------------------------------------------------------------------
    assert_eq!(
        sweep.dispatches(),
        0,
        "AC-498: the kernel and pack-validation suites complete with ZERO model dispatches. This \
         counter is this test's own and the bound fake is the only `Harness` anything above could \
         reach"
    );
    assert_eq!(
        appends.load(Ordering::SeqCst),
        0,
        "and nothing was journaled either: a `dispatch` row with no dispatch behind it would put \
         an activation into the log that never happened"
    );

    // ---- 1e. THE CONTROL, and it is the whole difference between a reading and a vacuum ------
    //
    //     A counter that cannot move reports 0 forever. A SECOND fake, a well-formed key, the same
    //     crate and the same boundary: the number must be 1.
    let (q2, a2) = counters();
    let db2 = store(&q2, &a2);
    let control = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"reached": true}),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&control) as Arc<dyn Harness>, &db2),
        "run-control",
        crate::runtime::STAGE,
    );
    rt2.dispatch_once(ITEM, AID, STEP, 1, &spec())
        .expect("a well-formed key reaches the port");
    assert_eq!(
        control.dispatches(),
        1,
        "CONTROL: the instrument MOVES. Without this, the 0 above is satisfied by a kernel that \
         cannot dispatch at all and by a counter nothing is wired to"
    );
    assert_eq!(
        sweep.dispatches(),
        0,
        "and the sweep's counter is still 0 — the two fakes are separate instruments"
    );

    // =========================================================================================
    // 2. *"no adapter crates linked"* — AC-484's OWN WALK, re-used.
    //
    //    D-2: reuse AC-484's machinery rather than writing a second graph walker. The report below
    //    is produced by `adapter_edge::resolved_reading` and the adapter half of it is that
    //    function's `findings` unchanged.
    // =========================================================================================
    let metadata_json = crate::adapter_edge::metadata(&[]);
    let surface =
        resolved_egress_surface(&metadata_json, SUBJECT_CRATE).expect("cargo metadata parses");

    assert!(
        surface.packages_walked >= 20,
        "PREMISE: the transitive walk from `{SUBJECT_CRATE}` reached {} packages; a walk that \
         visited a handful would report `nothing linked` for the wrong reason",
        surface.packages_walked
    );
    assert!(
        surface.reached.iter().any(|n| n == "rusqlite")
            && surface.reached.iter().any(|n| n == "libsqlite3-sys"),
        "PREMISE: the walk must be TRANSITIVE — rusqlite and libsqlite3-sys are reached only \
         through rk-store — or every absence below is an absence over a walk that went nowhere"
    );
    assert_eq!(
        surface.adapters,
        Vec::new(),
        "AC-498's *no adapter crates linked*: nothing matching `{ADAPTER_CRATE_PREFIX}*` is in \
         `{SUBJECT_CRATE}`'s resolved graph"
    );

    // =========================================================================================
    // 3. *"zero outbound network calls"*, as the LINKAGE property a graph can carry.
    // =========================================================================================
    assert_eq!(
        NETWORKING_CRATES,
        [
            "reqwest", "hyper", "tokio", "ureq", "curl", "rustls", "openssl"
        ],
        "D-2 fixes the set from a measurement taken 2026-09-14; widening it is a decision and not \
         a local edit"
    );
    assert_eq!(
        surface.networking,
        Vec::new(),
        "AC-498: no networking crate in `{SUBJECT_CRATE}`'s resolved graph. Reached: {:?}",
        surface.reached
    );
    assert!(surface.verdict().is_ok(), "{:?}", surface.verdict());

    // ---- 3a. the MATCHING RULE, and the two traps it is written around -----------------------
    assert_eq!(networking_token("tokio"), Some("tokio"), "the token itself");
    assert_eq!(
        networking_token("tokio-util"),
        Some("tokio"),
        "and its hyphenated siblings, which is how every one of these ships"
    );
    assert_eq!(networking_token("openssl-sys"), Some("openssl"));
    assert_eq!(networking_token("rustls-pki-types"), Some("rustls"));
    assert_eq!(
        networking_token("hyperlocal"),
        None,
        "🛑 THE TRAP: a `contains` rule flags `hyperlocal` for `hyper`, and a rule that fires on \
         innocent packages is the rule that gets deleted rather than fixed"
    );
    assert_eq!(
        networking_token("libsqlite3-sys"),
        None,
        "and the crate rk-store actually pulls is not a networking crate"
    );
    assert_eq!(networking_token("serde_json"), None);

    // ---- 3b. THE CONTROL: the same reader over a graph that DOES carry one --------------------
    //
    //     There is no feature flip for this half the way AC-484 has one: a real networking crate
    //     cannot be added as a fixture edge, because it would enter `Cargo.lock` — which `cargo
    //     audit` and `cargo deny` read — and fetching it needs the very network this criterion is
    //     about. So the rule is shown to FIRE over a synthetic `cargo metadata` document instead,
    //     and that limit is stated rather than glossed.
    //
    //     It carries a synthetic ADAPTER crate too, and that half is not decoration either: block 2
    //     asserts an EMPTY adapter list, which a reader that dropped its findings on the floor
    //     would also satisfy. AC-484's feature flip proves `resolved_reading` fires; this proves
    //     `resolved_egress_surface` still carries what it was handed.
    let adapter_pkg = format!("{ADAPTER_CRATE_PREFIX}synthetic");
    let synthetic = format!(
        r#"{{
      "packages": [
        {{"id": "p-subject", "name": "rk-kernel", "dependencies": []}},
        {{"id": "p-mid", "name": "mid-crate", "dependencies": []}},
        {{"id": "p-tokio", "name": "tokio-util", "dependencies": []}},
        {{"id": "p-local", "name": "hyperlocal", "dependencies": []}},
        {{"id": "p-adapter", "name": "{adapter_pkg}", "dependencies": []}}
      ],
      "resolve": {{"nodes": [
        {{"id": "p-subject", "deps": [
          {{"pkg": "p-mid", "name": "mid_crate", "dep_kinds": [{{"kind": null}}]}},
          {{"pkg": "p-adapter", "name": "synthetic", "dep_kinds": [{{"kind": null}}]}}]}},
        {{"id": "p-mid", "deps": [
          {{"pkg": "p-tokio", "name": "tokio_util", "dep_kinds": [{{"kind": null}}]}},
          {{"pkg": "p-local", "name": "hyperlocal", "dep_kinds": [{{"kind": null}}]}}]}},
        {{"id": "p-tokio", "deps": []}},
        {{"id": "p-local", "deps": []}},
        {{"id": "p-adapter", "deps": []}}
      ]}}
    }}"#
    );
    let caught = resolved_egress_surface(&synthetic, SUBJECT_CRATE).expect("the synthetic parses");
    assert_eq!(
        caught
            .networking
            .iter()
            .map(|n| (n.package.as_str(), n.token))
            .collect::<Vec<_>>(),
        vec![("tokio-util", "tokio")],
        "the rule must catch a networking crate reached TRANSITIVELY and must leave `hyperlocal` \
         alone; it found {:?}",
        caught.networking
    );
    assert_eq!(
        caught
            .adapters
            .iter()
            .map(|a| a.adapter_crate.as_str())
            .collect::<Vec<_>>(),
        vec![adapter_pkg.as_str()],
        "and the adapter findings must SURVIVE the classification — block 2's empty list is a \
         reading only if a non-empty one would come through; it found {:?}",
        caught.adapters
    );
    assert_eq!(
        caught.networking[0].route,
        vec!["rk-kernel", "mid-crate", "tokio-util"],
        "and it must report the ROUTE, or a transitive hit is unactionable"
    );
    let violation = caught
        .verdict()
        .expect_err("a graph carrying a networking crate must not return a clean verdict");
    let rendered = violation.to_string();
    assert!(
        rendered.contains("tokio-util") && rendered.contains(adapter_pkg.as_str()),
        "the violation must NAME both classes — a CI reader who cannot see which crate was found \
         cannot cut the edge. Rendered:\n{rendered}"
    );

    // ---- 3c. the DIRECT dev edges, and only those --------------------------------------------
    //
    //     Bound 2 in the header: the walk above is over `kind: null` edges, so a networking crate
    //     arriving as a dev-dependency is outside it — while still being LINKED into the very test
    //     binary AC-498 is about. This closes the direct dev edges by name. The dev CLOSURE is not
    //     walked and is not claimed.
    let dev =
        subject_dev_dependencies(&metadata_json, SUBJECT_CRATE).expect("the same bytes parse");
    assert!(
        dev.iter().any(|d| d == "trybuild"),
        "PREMISE: `{SUBJECT_CRATE}`'s dev-dependencies must be readable and non-empty, or this \
         assertion is an absence over an empty list; got {dev:?}"
    );
    let dev_networking: Vec<&String> = dev
        .iter()
        .filter(|d| networking_token(d).is_some())
        .collect();
    assert_eq!(
        dev_networking,
        Vec::<&String>::new(),
        "no DIRECT dev-dependency of `{SUBJECT_CRATE}` is a networking crate either"
    );

    // ---- 3d. the CROSS-CHECK, against the reading crates/rk-kernel/Cargo.toml itself names ----
    let tree = crate::adapter_edge::cargo(&["tree", "-p", "rk-kernel", "-e", "normal"]);
    assert!(
        tree.contains("rk-store"),
        "PREMISE: `cargo tree -p rk-kernel -e normal` must show the edges that DO exist:\n{tree}"
    );
    for token in NETWORKING_CRATES {
        assert!(
            !tree.contains(token),
            "`cargo tree -p rk-kernel -e normal` carries `{token}` — the independent instrument \
             D-2's measurement was taken with disagrees with the `resolve` reading:\n{tree}"
        );
    }

    // =========================================================================================
    // 4. THE CI JOB — ASSERTED PRESENT, AND NOTHING MORE.
    //
    //    This is the ONLY instrument that reaches `std::net::TcpStream`, and it is the one thing
    //    here that this test cannot execute. What a committed assertion CAN do is stop the job
    //    being deleted silently, which is the failure mode a workflow file has.
    // =========================================================================================
    assert!(
        CI_WORKFLOW.contains("\n  no-egress:"),
        "AC-498's named instrument is *a CI job with network egress denied*; the `no-egress` job \
         must exist in .github/workflows/ci.yml"
    );
    // 🛑 CODE LINES ONLY, AND THAT IS THE WHOLE ASSERTION.
    //
    // This read `CI_WORKFLOW.contains("--network none")` until 2026-09-14, and it was satisfied
    // by a COMMENT. The literal occurs SEVEN times in ci.yml and FIVE are inside the job’s own
    // explanatory block (:95, :102, :107, :118, :158); only :165 and :177 are the executable
    // `docker run` invocations. Deleting the flag from BOTH `docker run` lines and leaving the
    // prose left this test GREEN over a job that denied nothing [measured 2026-09-14 by mutation,
    // ci.yml restored byte-exactly afterwards].
    //
    // It is the same defect this programme has now been bitten by repeatedly, and Checkpoint 7b
    // documents it against its own `#[deny]` grep in those words: *"A grep cannot tell a comment
    // from an attribute."* The repair is the same one: strip comment lines first, then assert the
    // flag appears on a line that actually RUNS the container.
    let code: Vec<&str> = CI_WORKFLOW
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect();
    let denied: Vec<&&str> = code
        .iter()
        .filter(|l| l.contains("--network none"))
        .collect();
    assert!(
        !denied.is_empty(),
        "…and the denial must be `--network none` on a line that RUNS — it is dependency-free, so \
         no marketplace action enters a repo gated on `cargo-deny` and `cargo-audit` (D-2). \
         Comment lines are stripped before this check precisely because the prose above the job \
         names the flag five times and would satisfy a bare `contains`."
    );
    for line in &denied {
        assert!(
            line.contains("docker run"),
            "every executable `--network none` must be on a `docker run` invocation, or the flag \
             is being passed to something that does not create the network namespace: {line:?}"
        );
    }

    // The third-job claim is asserted on the STRUCTURE, not on two substring hits. `gates:` and
    // `structural:` surviving says nothing about whether their bodies were gutted; what D-2
    // promises is that ci.yml gained one job and that the other two are still jobs with steps.
    let job_names: Vec<&str> = code
        .iter()
        .filter(|l| l.starts_with("  ") && !l.starts_with("   ") && l.trim_end().ends_with(':'))
        .map(|l| l.trim().trim_end_matches(':'))
        .collect();
    for expected in ["gates", "no-egress", "structural"] {
        assert!(
            job_names.contains(&expected),
            "ci.yml must carry the job {expected:?}; found {job_names:?}. D-2 adds ONE job and \
             touches neither existing one"
        );
    }
}
