//! **`Config` — configuration as a value, declared where it is accepted** (ADR-0083; Step 7d, the
//! first of §7d.1's eight kernel edits, **D18-14**).
//!
//! ADR-0083's Decision: *"Configuration reaches the kernel as a value: a `Config` struct constructed
//! at the composition root and passed in."* C-01 **RB-M-05** found that no crate declared the type
//! at all, so the sentence had no referent. The operator's decision of 2026-09-18 puts it **here**,
//! and the warrant is the sentence itself: the kernel is what must ACCEPT the value, so the kernel
//! is what declares its shape. The alternatives were named and rejected — `rk-types` (which would
//! make it a wire type, and it is not one) and an `rk-cli`-private struct (which the kernel could
//! not then accept by name).
//!
//! **This crate still reads no environment and no clock, and this type does not change that.** The
//! values below are read once, at the edge, by `rk-cli`'s `env` module while it builds this struct;
//! the kernel receives the finished value. Nothing here calls anything.
//!
//! **What is deliberately NOT a field.** `captured_env` is not one (**RT-34**): the captured
//! environment map is an `rk-cli` value handed to `AdapterSet::resolve` *beside* the config file,
//! never carried into the kernel — a kernel that held the process environment would be reading it
//! one indirection later, which is the whole of what ADR-0083 forbids. And nothing here is an
//! event payload (**D18-14**): what a run-start row records is the resolved adapter identity, not
//! this struct.
//!
//! **Its one kernel consumer at this slice is [`crate::replay::RunInputs::with_config`]**, where it
//! is an `Option<Config>` **omitted from the recorded JSON when `None`** — see that method for why
//! omission rather than a `null` is what keeps every recorded `run_id` byte-stable.

/// The configuration the composition root resolved, as a value the kernel accepts.
///
/// Constructed only at `rk-cli`'s root. The fields are the **adapter construction values** D18-14
/// names plus the one `doctor` window the config file carries; each is a resolved name, not a path
/// and not a handle, because a handle would be a port implementor and the kernel holds those only
/// through `rk-ports`.
/// **No `Serialize`/`Deserialize`, deliberately.** Nothing needs them — the recorded form is
/// [`Self::to_json`] — and a `Deserialize` derive would be a SECOND public construction path that
/// bypasses [`Self::new`], so the toolchain normalisation documented there would stop being a
/// property of the type and become a property of one of its two constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    harness: String,
    store: String,
    toolchains: Vec<String>,
    unread_window_runs: u32,
}

impl Config {
    /// Build the value. `toolchains` is sorted here rather than trusted from the caller, so the
    /// recorded form is a function of the set and not of the order a file happened to list it in —
    /// the same rule `RunInputs::with_catalog` applies for the same reason.
    #[must_use]
    pub fn new(harness: &str, store: &str, toolchains: &[String], unread_window_runs: u32) -> Self {
        let mut toolchains = toolchains.to_vec();
        toolchains.sort();
        Self {
            harness: harness.to_owned(),
            store: store.to_owned(),
            toolchains,
            unread_window_runs,
        }
    }

    /// The resolved harness adapter's name.
    #[must_use]
    pub fn harness(&self) -> &str {
        &self.harness
    }

    /// The resolved store adapter's name.
    #[must_use]
    pub fn store(&self) -> &str {
        &self.store
    }

    /// The resolved toolchain adapters' names, sorted.
    #[must_use]
    pub fn toolchains(&self) -> &[String] {
        &self.toolchains
    }

    /// `doctor`'s unread window, in runs.
    #[must_use]
    pub const fn unread_window_runs(&self) -> u32 {
        self.unread_window_runs
    }

    /// The recorded form — object keys in declaration order, which the canonical serialiser sorts
    /// anyway before any digest is taken.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "harness": self.harness,
            "store": self.store,
            "toolchains": self.toolchains,
            "unread_window_runs": self.unread_window_runs,
        })
    }
}
