//! AC-480's AND clause (register :6912) — the holding shape, and what the supertraits buy.
//!
//! Both tests carry over from the reference shape F41 checked on the pin
//! (`spikes/s-ports/src/lib.rs`:265 and :324), re-imported against `rk-types` rather than against
//! the spike's local copies of `StepError` and friends.

use std::sync::{Arc, Mutex};

use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::harness::Harness;
use rk_ports::root::Ports;
use rk_ports::toolchain::Toolchain;
use rk_ports::tracker::Tracker;

use crate::support::{findings, spec};

#[test]
fn five_ports_are_object_safe_and_send_sync() {
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn EventStore>();
    assert_send_sync::<dyn Harness>();
    assert_send_sync::<dyn Tracker>();
    assert_send_sync::<dyn DocsSite>();
    assert_send_sync::<dyn Toolchain>();
}

/// The only assertion that shows the `Send + Sync` supertraits buy what ADR-0099 clause 4 says
/// they buy: the whole `Ports` struct moves across a thread boundary and every port is still
/// callable on the other side.
///
/// **The `lock()` here is the composition root's, modelled.** ADR-0115 clause 4 puts the
/// acquisition at the call boundary and releases it when the dispatch returns; `rk-ports`' own
/// `src/` writes no `lock()` call site at all, because the root is `rk-cli` at Step 7d. What this
/// test pins is the consequence of the holding shape: a bare `Arc<dyn EventStore>` yields `&T` and
/// never `&mut T`, so reverting the `store` field to it stops `Harness::dispatch`'s
/// `&mut dyn EventStore` argument from compiling [measured, ADR-0115:130-132].
#[test]
fn ports_struct_holds_arc_dyn_send_sync() {
    let ports = Ports {
        store: Arc::new(Mutex::new(VecStore::default())),
        harness: Arc::new(FakeHarness::scripted(vec![Ok(findings(
            serde_json::json!({"ok": true}),
        ))])),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::default()),
    };

    let handle = std::thread::spawn(move || {
        let mut guard = ports
            .store
            .lock()
            .expect("a poisoned store lock halts the run; it is never recovered (ADR-0118)");
        let outcome = ports.harness.dispatch(&spec(), &mut *guard);
        drop(guard);
        outcome.result.is_ok() && ports.tracker.project("item").is_ok()
    });

    assert!(
        handle.join().expect("the ports moved across the boundary"),
        "every port is callable after Ports moved to another thread"
    );
}
