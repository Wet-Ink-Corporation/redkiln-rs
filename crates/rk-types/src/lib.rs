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
//! *(amended 2026-09-25, s8a fix round 1, ADR-0165: no longer so. `ChangeSummary` is an enum defined
//! only in `rk-ports`, with a `Text` arm finalised now and a `Binary` slot sealed until S-BIN; an
//! implementor may name it, and Checkpoint 5 bans a definition, alias, renaming import or impl of it
//! outside `rk-ports`, not the name. Nothing of it is built here still.)*

pub mod activity;
pub mod artifact;
pub mod companion;
pub mod digest;
pub mod docset;
pub mod drift;
pub mod error;
pub mod event;
pub mod event_kind;
pub mod registry;
pub mod repro;
pub mod retention;
/// The review gate's record types (Step 7b-ii): the reviewed-tree outcome and the quarantine.
pub mod review;
pub mod schema;
pub mod text;
pub mod time;
pub mod typed;
/// The dispatch record's usage-adjacent types (Step 7c Slice 1; ADR-0126 decision 2): `Usage`,
/// `CacheCreation`, the window reading, the timings, the two-armed CLI version, the cost string.
pub mod usage;
/// The verify gate's record type (Step 7b-ii): the adapter's report with its closed basis and reason.
pub mod verify;
