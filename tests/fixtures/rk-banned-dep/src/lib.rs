//! The AC-005 must-fail fixture: it reintroduces one crate from ADR-0086's ban list.
//!
//! The dependency is OPTIONAL and behind the non-default `reintroduce` feature, so the everyday
//! `cargo deny check bans` gate resolves without it and stays green, while
//! `cargo deny --all-features check bans` resolves it and must exit non-zero.
//!
//! `deny.toml`'s `[graph]` must therefore never set `all-features = true` — that would pull this
//! dependency into the default graph and make the everyday gate permanently red, which is how a
//! gate gets deleted rather than fixed (DECISIONS-2026-09-12 E4).
