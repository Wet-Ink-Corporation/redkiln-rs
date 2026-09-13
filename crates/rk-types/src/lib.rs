//! `rk-types` — the typed core.
//!
//! Owns ids; the typed artifacts (`Spec`, `ImplReport`, `Findings`, `DocSet`, digests); schema
//! derivation and the closed-output transform; `StepError`; `Drift`; `GateBasis`.
//!
//! It may depend on **no async runtime** — no `tokio`, no `async-std`, no `futures`, no
//! `async-trait` (ADR-0076).
//!
//! The S-BIN-blocked artifact type is **not** built here: that spike is registered and unrun, no
//! review evidence of any kind exists for items whose change is not a text diff, so its field set
//! cannot be fixed. It lives only as `rk-ports`' opaque newtype at Step 7a (ADR-0099 clause 5,
//! ADR-0093 decision 4), and Checkpoint 5 asserts the name appears nowhere under `crates/`.

pub mod artifact;
pub mod companion;
pub mod digest;
pub mod drift;
pub mod error;
pub mod event;
pub mod registry;
pub mod repro;
pub mod schema;
pub mod text;
pub mod time;
