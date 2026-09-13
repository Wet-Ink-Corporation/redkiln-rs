//! ADR-0115:130-132's own measured finding: *"The bare `Arc<dyn EventStore>` form does not compile
//! against `&mut dyn EventStore` — that is the finding."* This file **must fail to compile**.
//!
//! An operator who follows a citation to design §3.3 P10 — which still ends *"a bare trait object
//! and not a newtype"* with no ADR-0115 carve-out — writes the shape below and cannot produce the
//! `&mut` that ADR-0099 clause 1's `dispatch` takes: an `Arc<dyn T>` yields `&T` and never
//! `&mut T`.
//!
//! **This case carries TWO errors, and only the first tracks this crate.** An earlier revision
//! declared its own local `WithdrawnPorts` holder and said so in a note — which made it a pin on a
//! property of `Arc` rather than on [`rk_ports::root::Ports`]' field, and it therefore stayed green
//! when that field was reverted to the withdrawn shape [measured 2026-09-13 on the pin]. Error 1
//! below now names the real field, so under the withdrawn shape it disappears and the committed
//! `.stderr` DRIFTS — which is what a `trybuild` snapshot catches and a bare `compile_fail` fence
//! cannot. Error 2 is the reason ADR-0115:130-132 records, kept because the `.stderr` pinning
//! *why* the withdrawn shape was rejected is this fixture's job.
//!
//! Note the ordering of instruments this leaves: `ports_struct_holds_arc_dyn_send_sync` stops
//! COMPILING under that mutation, so the `it` binary fails before this harness runs. This case is
//! the reason-pinning layer behind it, not the first line of defence.

use rk_ports::event_store::EventStore;
use rk_ports::fakes::{FakeHarness, VecStore};
use rk_ports::harness::Harness;
use rk_ports::root::Ports;
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::typed::SchemaId;
use std::sync::Arc;

/// ERROR 1 — the field-identity half, and the only half sensitive to `Ports`' own declaration.
/// `store` is `Arc<Mutex<dyn EventStore>>` (ADR-0115 clause 1), so this binding is a type error.
fn store_field_is_not_the_withdrawn_shape(ports: &Ports) -> &Arc<dyn EventStore> {
    &ports.store
}

struct WithdrawnPorts {
    store: Arc<dyn EventStore>,
    harness: Arc<dyn Harness>,
}

fn main() {
    let ports = WithdrawnPorts {
        store: Arc::new(VecStore::default()),
        harness: Arc::new(FakeHarness::scripted(Vec::new())),
    };
    let spec = ActivitySpec {
        card_digest: Digest::from_hex(&"a".repeat(64)).expect("64 lowercase hex"),
        tools: Vec::new(),
        schema: SchemaId::new("findings"),
        timeout_secs: 1,
        inputs: Vec::new(),
    };

    // ERROR 2 — ADR-0115:130-132's own measured finding, the REASON the shape was withdrawn.
    let _ = ports.harness.dispatch(&spec, &mut *ports.store);
}
