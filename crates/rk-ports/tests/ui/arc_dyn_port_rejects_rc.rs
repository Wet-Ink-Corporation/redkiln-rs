//! Design §3.3 P10's own named instrument: *"a compile-fail fixture for a port trait without the
//! supertraits"* (design:288). This file **must fail to compile**.
//!
//! Every port trait carries `Send + Sync` as supertraits (ADR-0099 decision 4), and the
//! composition root holds each as `Arc<dyn Port + Send + Sync>`. A fake that holds an `Rc` is
//! therefore not a port: the `dyn` boundary is load-bearing for the fakes, and a port that could
//! be neither sent nor shared cannot be held at the root at all.
//!
//! The `.stderr` beside this file is what pins the REASON. An expected error code on a
//! `compile_fail` doctest fence is inert at this pin, measured on 1.97.1 and on 1.98.1.

use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use std::rc::Rc;
use std::sync::Arc;

struct NotSendHarness(Rc<()>);

impl Harness for NotSendHarness {
    fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        unreachable!()
    }
}

fn main() {
    let _h: Arc<dyn Harness> = Arc::new(NotSendHarness(Rc::new(())));
}
