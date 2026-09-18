//! `rk-adapter-claude` — the real `Harness` row (design §9.1; ADR-0094 decision 1's first adapter;
//! `s7c` §7c.4–§7c.12). **Slice 4: part A, configuration time** (§7c.20 item 4) — what is refused
//! before spawn is refused as configuration. Part B — the runtime, the stream decoder, the
//! classifier, `dispatch` itself — is Slice 5's and is not here; **no stub stands in for it**: the
//! crate compiles without a `Harness` impl, and a fake dispatch would give the Slice 4 refusal
//! tests an `Ok` branch that reaches no spawn (§7c.18's reason).
//!
//! **What this crate is** (ADR-0099: `dispatch(&self, spec: &ActivitySpec, store: &mut dyn
//! EventStore) -> DispatchOutcome`, blocking to completion; the `CancelToken` cloned in at
//! construction; *"the kernel never calls cancel"*): one `claude -p` child per dispatch, spawned
//! through `rk-containment` (ADR-0127), fed the golden argv and the constructed environment,
//! its stream consumed inside, every termination appending one `dispatch_completed` row
//! (ADR-0126). Constructed once at the composition root, never per call (design §9.1), as
//! `ClaudeHarness::new(config: Config, token: CancelToken) -> Result<Self, ConfigError>` (Slice 5).
//!
//! **Part A, module by module** — every configuration-time lint of §7c.4, each a typed
//! [`config::ConfigError`] that **never crosses the port** (ADR-0112 closes `StepError` at eleven;
//! a `ConfigError` is the return of `Config::new` / `Binder::bind` and never of `dispatch`):
//!
//! | module | what it holds | criteria |
//! | --- | --- | --- |
//! | [`config`] | `Config` (the pins, the TTL pair, `Margins`, the captured map, `harness_binary`, the mismatch policy, the digest-keyed card store only the binder fills) as a VALUE (ADR-0083); `Config::new`'s lints; `ConfigError`; `TESTED_CLI_VERSION` (D17-11) | AC-217, AC-488, AC-526, AC-377/AC-220, AC-226 (config arms) |
//! | [`env_denylist`] | the ONE committed list (decision #16 of 2026-09-11; RT-25's tracker names) | AC-217, AC-422, AC-488, AC-509 |
//! | [`env`] | the child environment: the captured map plus both TTLs, one value — a refusal upstream, never a subtraction here | AC-210, AC-385, AC-526's AND |
//! | [`card`] | the lift from `runner/src/step.rs` at the pin: `Tier`, `Effort`, `AgentCard`; the authored card record and its validator | AC-493, AC-383, AC-214, AC-118/AC-418, AC-269 |
//! | [`command`] | ONE builder, the golden argv in ONE order, per-profile delivery, the argv lints and the cap (`ARGV_CAP_BYTES`, RT-1) | AC-203, AC-205/AC-417, AC-132, AC-209, AC-211 (PROVISIONAL), AC-214, AC-215 (argv half), AC-216/AC-511, AC-523's cap, AC-383 |
//! | [`binder`] | D-11: role tier → bound card → `ActivitySpec`; owns the `Config` while it fills its card store and hands it back for `ClaudeHarness::new`; RT-21's `"worktree"` key | AC-212, AC-204's declared list |
//! | [`registry`] | the dated capability registry — the `dispatch` row, and nothing else | AC-238, AC-489, AC-492 (RT-24) |
//!
//! **The fixtures** (§7c.17): `tests/fixtures/cards/` (the catalog AC-269 iterates — O-6 — with the
//! probe's haiku card and its role; the red cards), `argv/` (the golden vectors and the at-the-cap
//! card), `recorded/` (the h1-gap / h1-sysprompt dispatch rows extracted through the corpus object
//! store, with `PROVENANCE.md`), `registry/` and `config/` (the must-fail registries and margins),
//! and `lint-controls/` (the positive control for every absence ban in Checkpoint 7c — never
//! compiled). The second `[[bin]]`, `rk-fixture-child-claude`, is `rk-containment`'s fixture child
//! compiled under this crate's own target name (§7c.17); Slice 5 spawns it.
//!
//! **Lint posture.** Every override in this crate is `#[expect(clippy::…, reason = "…")]` per site
//! (ADR-0102 decision 3) — and at Slice 4 there are none: no clock, no sleep, no `HashMap`, no
//! `Command`, no environment read (the block's AC-217 ban: no `src/` file here reads the process
//! environment — the captured map is a `Config` value), no `f64` (the margin parser walks digits).
//! `thiserror` for the error enum (decision 6); `anyhow` nowhere. `tokio` and `rk-containment` are
//! on the manifest per §7c.19 and are Slice 5's to consume; `jsonschema` is NOT — as a normal edge
//! its default resolvers put six crates under licences `deny.toml` does not allow into this
//! crate's normal graph [measured 2026-09-18, `cargo deny check licenses` exit 4], the decision is
//! the operator's (`Cargo.toml` says which two), and the edge is deferred to Slice 5, which
//! consumes it.

pub mod binder;
pub mod card;
pub mod command;
pub mod config;
pub mod env;
pub mod env_denylist;
pub mod registry;
