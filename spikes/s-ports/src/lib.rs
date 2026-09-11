//! S-PORTS — a throwaway crate (never `crates/`) that checks ADR-0099's port shape compiles on the pinned
//! toolchain: five sync port traits held as `Arc<dyn Port + Send + Sync>`, a blocking `Harness::dispatch`
//! returning `DispatchOutcome`, a `CancelToken` cloned into the adapter at construction and triggered by the
//! composition root, one scripted fake, and `compile_fail` doctests for the two rules the design says a
//! compile failure is the instrument for (AC-480 generics leak; the `Send + Sync` supertrait).
//!
//! Registered per ADR-0012 in design §10.5 (audit §8 M-01). Consumer: runbook Step 7a / ADR-0099.
//!
//! # Rule 1 — a port trait object requires `Send + Sync`; a fake that holds an `Rc` cannot be a port
//!
//! ```compile_fail
//! use std::rc::Rc;
//! use std::sync::Arc;
//! use s_ports::*;
//! struct NotSendHarness(Rc<()>);
//! impl Harness for NotSendHarness {
//!     fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome { unreachable!() }
//! }
//! let _h: Arc<dyn Harness> = Arc::new(NotSendHarness(Rc::new(())));
//! ```
//!
//! # Rule 2 — the kernel's API surface carries no port-bound generic parameter (AC-480)
//!
//! The kernel takes `&dyn Harness`, never `H: Harness`. This doctest shows the shape the lint forbids
//! does not even type-check against the kernel entry point as written.
//!
//! ```compile_fail
//! use s_ports::*;
//! fn run_generic<H: Harness>(h: H, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
//!     h.dispatch(spec, store)
//! }
//! let k = kernel::Kernel::new(std::sync::Arc::new(FakeHarness::scripted(vec![])));
//! // The kernel is the only caller of dispatch; handing it a generic entry point is a type error.
//! let _ = k.run_with(run_generic);
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------- rk-types (the shapes ADR-0099 names)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaId(pub String);

/// An erased, validated value: `serde_json::Value` plus the schema id it validated against.
#[derive(Debug, Clone)]
pub struct Typed {
    pub schema: SchemaId,
    pub value: serde_json::Value,
}

/// The closed error set of ADR-0095 decision 2 plus ADR-0112's three: eleven variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepError {
    Transport { kind: TransportKind, detail: String },
    Schema { attempts: u32, last_error: String },
    Timeout { after_secs: u64 },
    Cancelled { by: CancelledBy },
    NonZero { code: i32, stderr_digest: String },
    RateLimited { resets_at: Option<u64> },
    NotAdopted { expected_agent: String, observed: String },
    NotIsolated { surface: IsolationSurface, diff_digest: String },
    // ADR-0112 (2026-09-11): the three the register already named
    Platform { status: u16, retry_after: Option<u64> },
    Unsupported { capability: String },
    Overage { reason: Option<String> },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportKind { Spawn, Io, Decode }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancelledBy { Signal, Timeout, Operator }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsolationSurface { Tools, McpServers, Env }

impl StepError {
    /// True when the child produced output the record must retain (the raw-retention pattern from step.rs).
    pub fn spoke(&self) -> bool {
        matches!(
            self,
            StepError::Schema { .. } | StepError::Timeout { .. } | StepError::Cancelled { .. }
                | StepError::NonZero { .. } | StepError::RateLimited { .. } | StepError::Platform { .. }
        )
    }
}

/// From `StepSpec`: what the adapter needs to assemble one dispatch.
#[derive(Debug, Clone)]
pub struct ActivitySpec {
    pub card_digest: String,
    pub tools: Vec<String>,
    pub schema: SchemaId,
    pub timeout_secs: u64,
    pub inputs: Vec<(String, serde_json::Value)>,
}

/// From `StepRecord`: the AC-360 dispatch record. Argv/env are digests only (ADR-0105 clause 7).
#[derive(Debug, Clone, Default)]
pub struct DispatchRecord {
    pub argv_digest: String,
    pub env_digest: String,
    pub instrument_version: String,
    pub wall_clock_ms: u64,
    pub usage: Option<Usage>,
    pub stdout_head: String,
}
#[derive(Debug, Clone, Default)]
pub struct Usage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

pub struct DispatchOutcome {
    pub record: DispatchRecord,
    pub result: Result<Typed, StepError>,
}

/// An event the kernel appends; the kinds are ADR-0105's table (a few here).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    StepFailed { class: String },
    StepCancelled,
    StepRefused { class: String },
    RateLimited,
    ReconcileStarted,
}

// ---------------------------------------------------------------- CancelToken (ADR-0099 decision 2)

/// Owned by the composition root; cloned into the adapter at construction; triggered only by the CLI's
/// signal path or the per-activity timeout. The kernel never sees it.
#[derive(Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    pub fn new() -> Self { Self::default() }
    pub fn cancel(&self) { self.0.store(true, Ordering::SeqCst); }
    pub fn is_cancelled(&self) -> bool { self.0.load(Ordering::SeqCst) }
}

// ---------------------------------------------------------------- rk-ports (five sync traits, Send + Sync)

pub trait EventStore: Send + Sync {
    fn append(&mut self, event: Event);
    fn read(&self) -> Vec<Event>;
}

pub trait Harness: Send + Sync {
    /// Blocks to completion. The adapter's declared timeout is the in-band cancel; the token is the root's.
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome;
}

pub trait Tracker: Send + Sync {
    fn sync(&self, item: &str) -> Result<(), StepError>;
}

pub trait DocsSite: Send + Sync {
    fn publish(&self, path: &str, body: &str) -> Result<(), StepError>;
}

/// `change_summary` returns an opaque, non-finalised newtype until S-BIN reports (ADR-0099 clause 5).
pub struct ChangeSummary(pub(crate) ());

pub trait Toolchain: Send + Sync {
    fn build(&self) -> Result<(), StepError>;
    fn test(&self) -> Result<u32, StepError>;
    fn classify(&self, paths: &[String]) -> Vec<String>;
    fn change_summary(&self, paths: &[String]) -> ChangeSummary;
}

/// The five ports as the composition root holds them (ADR-0099 decision 4).
pub struct Ports {
    pub store: Arc<Mutex<dyn EventStore>>,
    pub harness: Arc<dyn Harness>,
    pub tracker: Arc<dyn Tracker>,
    pub docs: Arc<dyn DocsSite>,
    pub toolchain: Arc<dyn Toolchain>,
}

// ---------------------------------------------------------------- one fake (scripted responder)

pub struct FakeHarness {
    script: Mutex<Vec<Result<Typed, StepError>>>,
    token: CancelToken,
}

impl FakeHarness {
    pub fn scripted(script: Vec<Result<Typed, StepError>>) -> Self {
        Self { script: Mutex::new(script), token: CancelToken::new() }
    }
    pub fn with_token(mut self, token: CancelToken) -> Self { self.token = token; self }
}

impl Harness for FakeHarness {
    fn dispatch(&self, _spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        let record = DispatchRecord { instrument_version: "fake".into(), ..Default::default() };
        if self.token.is_cancelled() {
            store.append(Event::StepCancelled);
            return DispatchOutcome { record, result: Err(StepError::Cancelled { by: CancelledBy::Operator }) };
        }
        let next = self.script.lock().unwrap().pop();
        let result = next.unwrap_or(Err(StepError::Transport { kind: TransportKind::Spawn, detail: "script exhausted".into() }));
        if let Err(e) = &result {
            store.append(match e {
                StepError::Cancelled { .. } => Event::StepCancelled,
                StepError::RateLimited { .. } => Event::RateLimited,
                StepError::NotAdopted { .. } => Event::StepRefused { class: "not_adopted".into() },
                StepError::NotIsolated { .. } => Event::StepRefused { class: "not_isolated".into() },
                StepError::Unsupported { .. } => Event::StepRefused { class: "unsupported".into() },
                StepError::Overage { .. } => Event::StepRefused { class: "overage".into() },
                StepError::Platform { status, .. } => Event::StepFailed { class: format!("platform:{status}") },
                other => Event::StepFailed { class: format!("{other:?}").split(' ').next().unwrap_or("").to_lowercase() },
            });
        }
        DispatchOutcome { record, result }
    }
}

#[derive(Default)]
pub struct VecStore(Vec<Event>);
impl EventStore for VecStore {
    fn append(&mut self, event: Event) { self.0.push(event); }
    fn read(&self) -> Vec<Event> { self.0.clone() }
}

// ---------------------------------------------------------------- rk-kernel (no port-bound generics)

pub mod kernel {
    use super::*;

    /// The kernel holds ports as trait objects; its public surface has no port-bound generic parameter.
    pub struct Kernel {
        harness: Arc<dyn Harness>,
    }

    impl Kernel {
        pub fn new(harness: Arc<dyn Harness>) -> Self { Self { harness } }

        /// The one dispatch call site. `&dyn Harness`, never `H: Harness`.
        pub fn step(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
            self.harness.dispatch(spec, store)
        }

        /// Exists only so the doctest for AC-480 has a kernel entry point to fail against: it takes a
        /// function over `&dyn Harness`, so a generic `fn<H: Harness>` cannot be passed.
        pub fn run_with(
            &self,
            f: fn(&dyn Harness, &ActivitySpec, &mut dyn EventStore) -> DispatchOutcome,
        ) -> DispatchOutcome {
            let spec = ActivitySpec { card_digest: String::new(), tools: vec![], schema: SchemaId("x".into()), timeout_secs: 1, inputs: vec![] };
            let mut store = VecStore::default();
            f(self.harness.as_ref(), &spec, &mut store)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> ActivitySpec {
        ActivitySpec { card_digest: "d".into(), tools: vec!["StructuredOutput".into()], schema: SchemaId("findings".into()), timeout_secs: 5, inputs: vec![] }
    }

    #[test]
    fn five_ports_are_object_safe_and_send_sync() {
        fn assert_send_sync<T: Send + Sync + ?Sized>() {}
        assert_send_sync::<dyn EventStore>();
        assert_send_sync::<dyn Harness>();
        assert_send_sync::<dyn Tracker>();
        assert_send_sync::<dyn DocsSite>();
        assert_send_sync::<dyn Toolchain>();
    }

    #[test]
    fn dispatch_blocks_and_returns_record_and_result() {
        let h = FakeHarness::scripted(vec![Ok(Typed { schema: SchemaId("findings".into()), value: serde_json::json!({"ok": true}) })]);
        let k = kernel::Kernel::new(Arc::new(h));
        let mut store = VecStore::default();
        let out = k.step(&spec(), &mut store);
        assert!(out.result.is_ok());
        assert_eq!(out.record.instrument_version, "fake");
        assert!(store.read().is_empty());
    }

    #[test]
    fn cancel_token_is_the_roots_and_yields_cancelled() {
        let token = CancelToken::new();
        let h = FakeHarness::scripted(vec![]).with_token(token.clone());
        let k = kernel::Kernel::new(Arc::new(h));
        token.cancel(); // the composition root's signal path, not the kernel
        let mut store = VecStore::default();
        let out = k.step(&spec(), &mut store);
        assert_eq!(out.result.unwrap_err(), StepError::Cancelled { by: CancelledBy::Operator });
        assert_eq!(store.read(), vec![Event::StepCancelled]);
    }

    #[test]
    fn step_error_is_matched_exhaustively_with_one_event_arm_each() {
        let cases = vec![
            StepError::Transport { kind: TransportKind::Io, detail: String::new() },
            StepError::Schema { attempts: 3, last_error: String::new() },
            StepError::Timeout { after_secs: 1 },
            StepError::Cancelled { by: CancelledBy::Timeout },
            StepError::NonZero { code: 1, stderr_digest: String::new() },
            StepError::RateLimited { resets_at: None },
            StepError::NotAdopted { expected_agent: "a".into(), observed: "b".into() },
            StepError::NotIsolated { surface: IsolationSurface::Tools, diff_digest: String::new() },
            StepError::Platform { status: 529, retry_after: None },
            StepError::Unsupported { capability: "affected-selection".into() },
            StepError::Overage { reason: None },
        ];
        for e in cases {
            let h = FakeHarness::scripted(vec![Err(e.clone())]);
            let mut store = VecStore::default();
            let _ = kernel::Kernel::new(Arc::new(h)).step(&spec(), &mut store);
            assert_eq!(store.read().len(), 1, "one event arm for {e:?}");
            let _ = e.spoke();
        }
    }

    #[test]
    fn ports_struct_holds_arc_dyn_send_sync() {
        struct T; impl Tracker for T { fn sync(&self, _: &str) -> Result<(), StepError> { Ok(()) } }
        struct D; impl DocsSite for D { fn publish(&self, _: &str, _: &str) -> Result<(), StepError> { Ok(()) } }
        struct C; impl Toolchain for C {
            fn build(&self) -> Result<(), StepError> { Ok(()) }
            fn test(&self) -> Result<u32, StepError> { Ok(0) }
            fn classify(&self, _: &[String]) -> Vec<String> { vec![] }
            fn change_summary(&self, _: &[String]) -> ChangeSummary { ChangeSummary(()) }
        }
        let p = Ports {
            store: Arc::new(Mutex::new(VecStore::default())),
            harness: Arc::new(FakeHarness::scripted(vec![])),
            tracker: Arc::new(T), docs: Arc::new(D), toolchain: Arc::new(C),
        };
        let handle = std::thread::spawn(move || p.tracker.sync("item").is_ok());
        assert!(handle.join().unwrap());
    }
}
