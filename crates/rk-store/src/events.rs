//! `t2_event`'s insert and its selector's `SELECT` — the one copy of each statement.
//!
//! *(dated 2026-09-28, s8e build; ADR-0181 decision 4 and ADR-0182 decision 4, proposed.)* The
//! SQLite adapter's `append` and `query` ran these statements inline, and `query` ignored its
//! selector (`fn query(&self, _by: Query)` over `SELECT kind, payload FROM t2_event ORDER BY id`,
//! RT-4). They live here now, beside `crate::tier_one`, so `query(ByItem)` is honoured in
//! `rk-store`'s SQL (`s8e` §8e.10) and an anchored row is written by the same statement as an
//! ordinary one. The interim `item` key is the event's (`Event::item`), persisted in the column
//! `0002_gate_transitions` adds, and read back onto the event — never into its payload.

use rusqlite::Connection;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::conn::StoreError;

/// Which rows [`select`] returns, in append order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selector<'a> {
    /// Every row.
    All,
    /// The rows keyed to this item, and — in the interim, ADR-0182 decision 4 — the rows that
    /// carry no key. Never a row keyed to another item.
    Item(&'a str),
    /// The rows anchored to this commit (ADR-0181 decision 12). An unanchored row never matches.
    Sha(&'a str),
}

/// Insert one `t2_event` row: the kind's wire name, the payload as JSON text, the anchor (NULL for
/// an ordinary append) and the interim item key.
///
/// # Errors
/// [`StoreError::Sqlite`] when the payload does not serialise or the statement refuses.
pub fn insert(
    conn: &Connection,
    event: &Event,
    anchor_sha: Option<&str>,
) -> Result<(), StoreError> {
    let payload =
        serde_json::to_string(&event.payload).map_err(|e| StoreError::Sqlite(e.to_string()))?;
    conn.execute(
        "INSERT INTO t2_event (kind, payload, anchor_sha, item) VALUES (?1, ?2, ?3, ?4)",
        (event.kind.wire_name(), &payload, anchor_sha, event.item()),
    )?;
    Ok(())
}

/// The rows the selector names, in append order, each decoded back into an [`Event`] carrying its
/// interim key.
///
/// # Errors
/// [`StoreError::Sqlite`] when the statement refuses, a row names a kind this binary's
/// [`EventKind`] does not (a row a later binary wrote, or a corrupt one — a typed failure, never a
/// row silently dropped from the journal), or a payload is not JSON.
pub fn select(conn: &Connection, selector: Selector<'_>) -> Result<Vec<Event>, StoreError> {
    let (sql, key) = match selector {
        Selector::All => ("SELECT kind, payload, item FROM t2_event ORDER BY id", None),
        Selector::Item(item) => (
            "SELECT kind, payload, item FROM t2_event WHERE item = ?1 OR item IS NULL ORDER BY id",
            Some(item),
        ),
        Selector::Sha(sha) => (
            "SELECT kind, payload, item FROM t2_event WHERE anchor_sha = ?1 ORDER BY id",
            Some(sha),
        ),
    };
    let mut stmt = conn.prepare(sql)?;
    let decode = |r: &rusqlite::Row<'_>| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, Option<String>>(2)?,
        ))
    };
    let rows = match key {
        Some(k) => stmt
            .query_map([k], decode)?
            .collect::<Result<Vec<_>, _>>()?,
        None => stmt.query_map([], decode)?.collect::<Result<Vec<_>, _>>()?,
    };
    let mut out = Vec::with_capacity(rows.len());
    for (kind, payload, item) in rows {
        let kind = EventKind::from_wire(&kind).ok_or_else(|| {
            StoreError::Sqlite(format!(
                "t2_event carries kind `{kind}`, which this binary's EventKind does not name"
            ))
        })?;
        let payload =
            serde_json::from_str(&payload).map_err(|e| StoreError::Sqlite(e.to_string()))?;
        let event = Event::new(kind, payload);
        out.push(match item {
            Some(item) => event.with_item(item),
            None => event,
        });
    }
    Ok(out)
}
