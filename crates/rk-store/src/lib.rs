//! `rk-store` — the typed store: canonical serialiser, the memo key, identity, connection
//! discipline, the section locator, retention.
//!
//! **One store per repository, at the common dir** (AC-178). Resolving from the common dir rather
//! than from wherever a process is standing is what makes that true from a linked worktree, and
//! S-GIX/a's **go** means gix answers that question directly (ADR-0091).
//!
//! **Two tiers, split by the deleted-database test** (ADR-0109 decision 2): *if the database were
//! deleted, would `redkiln run` take a different next action?* If yes, it is tier 1 and it is
//! written to git first. Tier 2 is the event store's alone, rebuildable or disposable — and the
//! step memo is tier 2, *"rebuildable by re-execution, never by rebuild"* (ADR-0097 decision 1).
//! [`tiered_state!`] makes a field with no answer a compile error.
//!
//! **Markdown is a rendering, never a source** (ADR-0006, AC-120). Nothing here parses structure
//! out of prose: [`locator`] is the single module that looks at markdown at all, it returns a byte
//! range, and no typed field is ever populated from what is inside it.
//!
//! What this crate does **not** do, and where it went instead: the EventStore *port* and its SQLite
//! adapter are `rk-ports` (7a) and `rk-adapter-sqlite` (7c). `rk-store` owns the contract ADR-0105
//! clause 7 describes and the connection discipline that contract is about.

/// `rk-store-artifacts` (design §9.1) — the review and verify gates' stored records: the finding
/// identity (AC-305) and the advisory sink's accounting (AC-310, AC-530). Step 7b-ii.
pub mod artifacts;
pub mod canon;
pub mod conn;
/// `t2_event`'s insert and its selector's `SELECT` — ADR-0181 decision 4, ADR-0182 decision 4.
pub mod events;
pub mod identity;
pub mod locator;
pub mod memo;
/// The migration runner and the one opening transaction — ADR-0181 decision 6.
pub mod migrate;
/// The render, write and read path -- the render record ADR-0189, proposed (`s8f` §8f.5, §8f.6).
/// *(dated 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ.)*
pub mod render;
pub mod retention;
pub mod schema_lint;
pub mod tier;
/// The tier-1 statement functions — ADR-0181 decisions 3 and 4. Called only by the SQLite
/// adapter's `append_tier1` and `rebuild`.
pub mod tier_one;
