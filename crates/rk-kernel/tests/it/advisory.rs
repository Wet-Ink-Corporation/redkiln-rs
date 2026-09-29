//! **AC-525's drift advisory** — `src/runtime/advisory.rs`. Slice 4h.
//!
//! One test, and its name is the runbook's: `ac525_drift_advisory_once` is asserted by Checkpoint
//! 7b's **second** (selection) name loop (:1511-1519) and the block halts without it.
//!
//! 🛑 **It is listed under Slice 3 and it is built here, and that is runbook conflict C-2.**
//! §7b.13 files this name in Slice 3's bullet list; Slice 3 is recorded closed at `8444221` without
//! it. AC-525's 2026-09-13 amendment clause 2 puts the derivation and the append **outside** the
//! pure function — *"at the kernel's step-3 boundary or in `reconcile()`"* — which is Slice 4's
//! subject and not Slice 3's. Residual 9 is FORCED on the mechanics (*"every name the block greps
//! must exist by the end of Slice 4 or the block halts"*) and equally clear on the record: **the
//! carry-forward line belongs in `PROGRESS.md`, which is outside this session's editable set, so it
//! is the operator's.**
//!
//! 🛑 **The `rk-select` half does LESS than the unamended wording suggests, and the amendment says
//! so in its own clause 5:** *"read literally, 'WHEN step 3 next classifies the root THEN exactly
//! one `drift_advisory` … is appended' puts the append inside step 3, and **it is not there**. Step 3
//! returns `Present`; every asserted behaviour — the derivation, the *exactly one*, the append, and
//! the carriage on the next `gate_opened` record — is `rk-kernel`'s at the boundary."* So this file
//! drives `step3` for the **signal** and the boundary for everything else, and the selection-side
//! half is a classification and nothing more.
//!
//! 🛑 **SM-022 has no baseline.** The register, §7b.13 and §7b.15 all repeat it: the noise rate is
//! **reported, never thresholded**. The count below is printed and no assertion turns on it being
//! small.

use std::sync::atomic::Ordering;

use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::advisory::{
    Advisory, AdvisorySink, carried_by_gate, derive_advisories, recorded_advisories,
};
use rk_kernel::select::{
    ArtifactRegistry, ArtifactState, Catalog, Classification, Digest, Instance, Item, Pack,
    digest_of, select, step3,
};
use rk_kernel::workflow::fsm::GateRecord;
use rk_ports::harness::Harness;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

use crate::runtime::{CountingHarness, ITEM, STAGE, counters, journal, ports, store};

const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");
const INCIDENT: &str = include_str!("../fixtures/packs/incident-forbidden.toml");

const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../fixtures/catalog-select/spec-authoring.toml"),
    ),
];

/// A **real** selection over the fixture pack and catalog — the one `it/fsm.rs`'s `planned()`
/// builds, re-exported here because `it/runtime.rs` needs a `NextAction::Dispatch` the FSM itself
/// produced rather than one a test hand-wrote.
///
/// # Panics
///
/// If the fixture pack, catalog, registry or facet registry stops loading.
#[must_use]
pub fn planned_selection() -> rk_kernel::select::Selection {
    let pack =
        Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("the fixture pack loads");
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");
    let item = Item::new(ITEM, "incident").with_toolchain("node", ["text"]);
    select(&pack, &catalog, &registry(), &facets(), STAGE, &item).expect("the selection resolves")
}

fn registry() -> ArtifactRegistry {
    ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the fixture registry loads")
}

fn facets() -> rk_kernel::registry::Registry {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry must load")
}

/// The vouched root and its in-set upstream, at a given upstream digest.
///
/// `VouchedBrief` is registered `source = "human"` (AC-067's GIVEN) and `MarketScan` is its edge
/// set — the same pair `it/short_circuit.rs`'s `vouched_root_stays_present` drives.
fn signal_at(upstream_digest: &str) -> Option<rk_kernel::select::DriftSignal> {
    let reg = registry();
    let kind = Instance::of("VouchedBrief");
    let state = ArtifactState::new(&kind).exists(digest_of(b"hand-written"));
    let upstream = [("MarketScan", digest_of(upstream_digest.as_bytes()))];
    let out = step3(&reg, &state, &["MarketScan"], &upstream);
    assert_eq!(
        out.classification,
        Classification::Present,
        "PREMISE: the vouched root stays Present — the advisory can never promote itself"
    );
    out.drift
}

/// **AC-525**, all five instruments, with the amendment's fourth as the discriminator.
///
/// | instrument | what it asserts here |
/// | --- | --- |
/// | *"exactly one `drift_advisory { root, upstream, old_digest, new_digest }`"* | one event, four members, no fifth |
/// | *"a re-entry test asserting no second advisory for the same digest"* | a second derivation over the same signal emits nothing |
/// | *"a gate test asserting the next `gate_opened` record carries it"* | [`carried_by_gate`], with an empty-list control |
/// | *"the AC-067 out-of-set fixture as the negative companion"* | an upstream outside the edge set produces no signal at all |
/// | **amendment (4)**: *"a replay over a run that emitted an advisory must re-derive the same DAG with **no store read**"* | the query counter is unchanged across the derivation **and** across the re-derivation, and the DAG's canonical bytes are byte-identical |
///
/// The fourth is the one that discriminates: *"the first four are all passed by an implementation
/// that de-duplicates by scanning the log; this one is not."*
#[test]
fn ac525_drift_advisory_once() {
    let signal = signal_at("scan-v2").expect("an in-set upstream moved, so step 3 reports drift");
    assert_eq!(signal.root, "VouchedBrief");
    assert_eq!(signal.upstream, "MarketScan");

    let (q, a) = counters();
    let db = store(&q, &a);
    let h = std::sync::Arc::new(CountingHarness::scripted(vec![]));
    let rt = Runtime::new(
        ports(
            std::sync::Arc::clone(&h) as std::sync::Arc<dyn Harness>,
            &db,
        ),
        "run-advisory",
        STAGE,
    );
    let mut sink = AdvisorySink::for_run();

    // PREMISE: the read counter CAN move. Without it, "unchanged" below is satisfied by a counter
    // that is broken.
    let baseline = q.load(Ordering::SeqCst);
    let _ = rt.journal(ITEM).expect("the store answers");
    assert!(
        q.load(Ordering::SeqCst) > baseline,
        "PREMISE: a store READ moves the query counter"
    );

    // ---- the derivation and the append, with the read counter watched ------------------------
    let reads_before = q.load(Ordering::SeqCst);
    let emitted = derive_advisories(&mut sink, std::slice::from_ref(&signal));
    assert_eq!(emitted.len(), 1, "AC-525: exactly one — never zero");
    assert_eq!(sink.admitted(), 1);
    assert_eq!(sink.suppressed(), 0);
    for advisory in &emitted {
        rt.append(advisory.to_event()).expect("the append resolves");
    }
    assert_eq!(
        q.load(Ordering::SeqCst),
        reads_before,
        "AMENDMENT (4), first half: the derivation and the append made ZERO store reads. An \
         implementation that de-duplicated by scanning the log would have moved this counter and \
         would still pass every other assertion in this test"
    );

    let recorded = journal(&db);
    let rows = recorded_advisories(&recorded);
    assert_eq!(rows.len(), 1, "exactly one `drift_advisory` in the journal");
    assert_eq!(rows[0].kind, EventKind::DriftAdvisory, "design §5 row 22");
    let payload = rows[0].payload.as_object().expect("an object payload");
    let mut keys: Vec<&str> = payload.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        Advisory::FIELDS.to_vec(),
        "all four fields the THEN names, and NO fifth"
    );
    assert_eq!(payload["root"], serde_json::json!("VouchedBrief"));
    assert_eq!(payload["upstream"], serde_json::json!("MarketScan"));
    assert_eq!(
        payload["new_digest"],
        serde_json::json!(digest_of(b"scan-v2").as_str())
    );
    assert_eq!(
        payload["old_digest"],
        serde_json::Value::Null,
        "a vouched root records `derived_from = []`, so there is no baseline — and the ABSENCE is \
         written rather than dropped, so a reader can tell it from a missing member"
    );

    // ---- "never one per re-entry", WITHIN the run ---------------------------------------------
    let again = derive_advisories(&mut sink, std::slice::from_ref(&signal));
    assert!(
        again.is_empty(),
        "AC-525: no second advisory for the same (root, upstream, new_digest) within the run"
    );
    assert_eq!(sink.admitted(), 1);
    assert_eq!(
        sink.suppressed(),
        1,
        "and the suppression is COUNTED, so \
        'no second advisory' is distinguishable from 'the sink was never asked'"
    );
    assert_eq!(recorded_advisories(&journal(&db)).len(), 1);

    // ---- THE CONTROL: the dedup is on the TRIPLE, not a refuse-everything rule ----------------
    let moved = signal_at("scan-v3").expect("a further in-set move is a further signal");
    let third = derive_advisories(&mut sink, std::slice::from_ref(&moved));
    assert_eq!(
        third.len(),
        1,
        "CONTROL: a DIFFERENT `new_digest` is a different triple and emits — without this, \
         'exactly one' is satisfied by a sink that admits nothing after the first"
    );
    assert_eq!(sink.admitted(), 2);
    assert_ne!(
        emitted[0].triple(),
        third[0].triple(),
        "and the two triples differ in exactly the member the THEN names"
    );

    // ---- AC-067's out-of-set NEGATIVE COMPANION -----------------------------------------------
    let reg = registry();
    let kind = Instance::of("VouchedBrief");
    let state = ArtifactState::new(&kind).exists(digest_of(b"hand-written"));
    let out_of_set = [("SmokeResult", digest_of(b"smoke-v9"))];
    let outcome = step3(&reg, &state, &["MarketScan"], &out_of_set);
    assert!(
        outcome.drift.is_none(),
        "AC-067: an upstream OUTSIDE the root's edge set emits nothing at all"
    );
    let before = sink.admitted();
    let none = derive_advisories(&mut sink, &[]);
    assert!(none.is_empty());
    assert_eq!(
        sink.admitted(),
        before,
        "and the boundary handed no signal derives no advisory"
    );

    // ---- "the next `gate_opened` record the item reaches carries that advisory" ---------------
    let gate_record = GateRecord::derive(
        "unknown_facet:criticality",
        "aaaa111122223333444455556666777788889999",
        Timestamp::from_unix_seconds(1_700_009_999),
    )
    .expect("the gate record derives");
    let bare = rk_kernel::runtime::gate_opened_event(&gate_record);
    let carrying = carried_by_gate(&bare, &emitted);
    let carried = carrying
        .payload
        .get("advisories")
        .and_then(serde_json::Value::as_array)
        .expect("the gate carries an `advisories` member");
    assert_eq!(carried.len(), 1, "the next gate carries the advisory");
    assert_eq!(carried[0], emitted[0].to_json());
    assert_eq!(
        carrying.payload.get("token"),
        bare.payload.get("token"),
        "the gate's DERIVED token is unchanged: the advisories ride BESIDE the three members \
         `GateRecord::derive` binds, so AC-039's binding is not disturbed by a drift note"
    );
    let empty = carried_by_gate(&bare, &[]);
    assert_eq!(
        empty
            .payload
            .get("advisories")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len),
        Some(0),
        "CONTROL: a gate reached before any advisory carries an EMPTY list, written rather than \
         omitted — otherwise a dropped carriage could not be told from an honest zero"
    );

    // ---- AMENDMENT (4), second half: the replay re-derives the same DAG with no store read ----
    // `select()` takes no store at all, which is a type fact; what this measures is that the whole
    // re-derivation leaves the read counter where it was.
    let pack =
        Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("the fixture pack loads");
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");
    let registry_ = registry();
    let facet_registry = facets();
    let item = Item::new(ITEM, "incident").with_toolchain("node", ["text"]);
    let first = select(&pack, &catalog, &registry_, &facet_registry, STAGE, &item)
        .expect("the selection resolves");
    let reads_at_replay = q.load(Ordering::SeqCst);
    let replayed = select(&pack, &catalog, &registry_, &facet_registry, STAGE, &item)
        .expect("the replay's selection resolves");
    assert_eq!(
        q.load(Ordering::SeqCst),
        reads_at_replay,
        "AMENDMENT (4): the replay re-derives the DAG with NO store read"
    );
    let a_bytes = first.canonical_bytes().expect("the selection serialises");
    let b_bytes = replayed
        .canonical_bytes()
        .expect("the selection serialises");
    assert_eq!(
        a_bytes, b_bytes,
        "and the DAG it re-derives is byte-identical under the canonical serialiser"
    );
    assert_eq!(
        first.dag(),
        replayed.dag(),
        "stated twice deliberately: the canonical bytes cover the whole selection, and this names \
         the DAG itself, which is the thing amendment (4)'s clause is about"
    );
    assert!(
        !first.dag().nodes().is_empty(),
        "PREMISE: the DAG is non-empty, or the byte-identity above holds over two nothings"
    );

    // ---- SM-022: REPORTED, NEVER THRESHOLDED --------------------------------------------------
    println!(
        "ac525 advisories: admitted={} suppressed={} journal_rows={} (SM-022 has no baseline; \
         this is reported and never thresholded)",
        sink.admitted(),
        sink.suppressed(),
        recorded_advisories(&journal(&db)).len(),
    );
    let _ = a.load(Ordering::SeqCst);
    let _: &Digest = emitted[0].new_digest();
}

/// **The own-change advisory** (AC-127's first half; `s8f` §8f.5; the render record ADR-0189
/// decision 10, proposed; D21-47, review m11). *(Added 2026-09-28, s8f build, package
/// S8F-P5-DRIFT-GATE-VOUCHED.)*
///
/// `Advisory::own_change(root, vouched, observed)` writes `"upstream": null` — the key PRESENT,
/// its value `null`, so *"all four fields and no fifth"* still holds against [`Advisory::FIELDS`] —
/// with `old_digest` the vouched digest and `new_digest` the observed one. Its key
/// `(root, None, new_digest)` is distinct from any upstream key over the same root and digest, so
/// one run's sink admits both. And the UPSTREAM advisory's bytes are unchanged by the type change:
/// `from_signal`'s `upstream` serialises as the same string it did before. Red under M50 (`""` in
/// place of `null`).
#[test]
fn an_own_change_advisory_writes_upstream_null_with_all_four_fields() {
    let vouched = digest_of(b"vouched bytes");
    let observed = digest_of(b"the root edited after vouching");
    let own = Advisory::own_change("VouchedBrief", vouched.clone(), observed.clone());

    let json = own.to_json();
    let object = json.as_object().expect("an object payload");
    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        Advisory::FIELDS.to_vec(),
        "all four fields, and no fifth: the key `upstream` is PRESENT"
    );
    assert_eq!(
        object.get("upstream"),
        Some(&serde_json::Value::Null),
        "the own-change mark is `\"upstream\": null` — never an empty string, never absent"
    );
    assert_eq!(own.upstream(), None, "the typed value is `None`");
    assert_eq!(object["root"], serde_json::json!("VouchedBrief"));
    assert_eq!(
        object["old_digest"],
        serde_json::json!(vouched.as_str()),
        "`old_digest` is the VOUCHED digest"
    );
    assert_eq!(
        object["new_digest"],
        serde_json::json!(observed.as_str()),
        "`new_digest` is the file's observed digest"
    );
    let event = own.to_event();
    assert_eq!(event.kind, EventKind::DriftAdvisory);
    assert_eq!(
        event.payload, json,
        "the event carries exactly the advisory"
    );

    // ---- the triple is distinct from any UPSTREAM triple over the same root and digest ----------
    let signal = signal_at("scan-v2").expect("an in-set upstream moved");
    let upstream = Advisory::from_signal(&signal);
    let upstream_over_same_digest = {
        let mut s = signal.clone();
        s.new_digest = observed.clone();
        Advisory::from_signal(&s)
    };
    assert_eq!(
        own.triple(),
        (
            "VouchedBrief".to_owned(),
            None,
            observed.as_str().to_owned()
        )
    );
    assert_ne!(own.triple(), upstream.triple());
    assert_ne!(
        own.triple(),
        upstream_over_same_digest.triple(),
        "the own-change key is never equal to an upstream key, even over the same root and digest"
    );
    let mut sink = AdvisorySink::for_run();
    assert!(sink.admit(&own), "the own change is admitted once per run");
    assert!(
        sink.admit(&upstream_over_same_digest),
        "and an upstream advisory over the same root and new digest is a DIFFERENT key"
    );
    assert!(
        !sink.admit(&own),
        "a second own change with the same observed digest is suppressed within the run"
    );

    // ---- an UPSTREAM advisory's bytes are unchanged by the type change --------------------------
    let bytes = rk_store::canon::canonical(&upstream.to_json()).expect("canonical");
    let expected = format!(
        "{{\"new_digest\":\"{}\",\"old_digest\":null,\"root\":\"VouchedBrief\",\"upstream\":\"MarketScan\"}}",
        signal.new_digest.as_str()
    );
    assert_eq!(
        String::from_utf8(bytes).expect("utf-8"),
        expected,
        "`Some(s)` serialises as the same string as before: no `drift_advisory` row, golden log or \
         `carried_by_gate` payload moves a byte"
    );
    assert_eq!(upstream.upstream(), Some("MarketScan"));
}
