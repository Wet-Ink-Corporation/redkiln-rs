//! The tier-1 statement functions — ADR-0181 decisions 3 and 4 (`s8e` §8e.2; D21-41 e).
//!
//! *(dated 2026-09-28, s8e build; ADR-0181, proposed.)* **The SQL against a `t1_*` table lives
//! here and nowhere else** (ADR-0158's bullet *"Writing `t1_*` rows from `rk-cli`, or any SQL
//! against them outside `rk-store`"*, honoured). Every function takes a connection the caller
//! holds inside ONE write transaction (`Writer::transaction`), so an act's anchored `t2_event` row
//! and its `t1_*` row both exist or neither does. The callers are the SQLite adapter's
//! `append_tier1` and — when `s8e` §8e.9 lands it — its `rebuild`; nothing outside `rk-store` and
//! `rk-adapter-sqlite` names this module. *(dated 2026-09-28, `s8e` build, package S8E-P7: §8e.9
//! landed it — [`stored_rows`] is `SqliteStore::snapshot`'s read and [`rebuild`] its write, both
//! append-only, ADR-0181 decision 11.)*
//!
//! **Append-only.** No function here updates or deletes a row. A gate's state is the fold over its
//! `t1_gate_transition` rows, one per transition, `UNIQUE (token, state)` (D21-43 N6; D21-46 R2).
//! **Nothing is invented**: a `NOT NULL` member the event does not carry refuses the write as
//! [`StoreError::TierOne`], naming the member.

use rusqlite::Connection;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::conn::StoreError;
use crate::events;

/// A gate transition's state — the four N6 names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Opened,
    Answered,
    Expired,
    Voided,
}

impl GateState {
    /// The column's spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Opened => "opened",
            Self::Answered => "answered",
            Self::Expired => "expired",
            Self::Voided => "voided",
        }
    }
}

/// One `t1_gate_transition` row, less its anchor.
///
/// `token` and `state` are `NOT NULL`; every other member is nullable, because the answered,
/// expired and void payloads do not carry every member the `Gate:` value does (ADR-0181's stated
/// negative). `stage` and `run_id` are set on the `opened` row only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateTransition {
    pub token: String,
    pub state: GateState,
    pub tree_hash: Option<String>,
    pub deadline: Option<String>,
    pub kind: Option<String>,
    pub reach: Option<String>,
    pub option_id: Option<String>,
    pub stage: Option<String>,
    pub run_id: Option<String>,
}

/// The anchor an act's rows are written against: the trailer commit, its tree, and — for a gate
/// opening — the opening stage run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anchor<'a> {
    pub sha: &'a str,
    pub tree: &'a str,
    pub opening_run: Option<(&'a str, &'a str)>,
}

/// Insert one `t1_stage_transition` row — a boundary between two stages.
///
/// # Errors
/// [`StoreError::Sqlite`] when the statement refuses.
pub fn insert_stage_transition(
    conn: &Connection,
    item: &str,
    from_stage: &str,
    to_stage: &str,
    anchor_sha: &str,
    tree_hash: &str,
) -> Result<(), StoreError> {
    conn.execute(
        "INSERT INTO t1_stage_transition (item, from_stage, to_stage, anchor_sha, tree_hash) \
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (item, from_stage, to_stage, anchor_sha, tree_hash),
    )?;
    Ok(())
}

/// Append one `t1_gate_transition` row. A second row for one `(token, state)` is refused by the
/// table's `UNIQUE`; a re-reach mints a fresh token (ADR-0105 clause 10).
///
/// # Errors
/// [`StoreError::Sqlite`] when the statement refuses.
pub fn insert_gate_transition(
    conn: &Connection,
    row: &GateTransition,
    anchor_sha: &str,
) -> Result<(), StoreError> {
    conn.execute(
        "INSERT INTO t1_gate_transition (token, state, tree_hash, deadline, kind, reach, option_id, \
         stage, run_id, anchor_sha) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &row.token,
            row.state.as_str(),
            &row.tree_hash,
            &row.deadline,
            &row.kind,
            &row.reach,
            &row.option_id,
            &row.stage,
            &row.run_id,
            anchor_sha,
        ),
    )?;
    Ok(())
}

/// Insert one `t1_slice_seal` row.
///
/// # Errors
/// [`StoreError::Sqlite`] when the statement refuses.
pub fn insert_slice_seal(
    conn: &Connection,
    project: &str,
    slice: &str,
    verdict: &str,
    anchor_sha: &str,
) -> Result<(), StoreError> {
    conn.execute(
        "INSERT INTO t1_slice_seal (project, slice, verdict, anchor_sha) VALUES (?1, ?2, ?3, ?4)",
        (project, slice, verdict, anchor_sha),
    )?;
    Ok(())
}

/// A payload member as text: a string as itself, a number or a boolean as its JSON spelling
/// (the deadline is integer unix seconds, D21-43 N8, and `t1_gate_transition.deadline` holds the
/// decimal); absent or `null` as `None`.
fn text(event: &Event, member: &str) -> Option<String> {
    match event.payload.get(member)? {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Null => None,
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        other => Some(other.to_string()),
    }
}

fn required(event: &Event, member: &str) -> Result<String, StoreError> {
    text(event, member).ok_or_else(|| {
        StoreError::TierOne(format!(
            "a `{}` row carries no `{member}` in its payload, and a tier-1 row never invents a \
             member (ADR-0181 decision 3)",
            event.kind.wire_name()
        ))
    })
}

/// The one transaction body `append_tier1` runs: the anchored `t2_event` row, then — when the
/// event's kind has a tier-1 table — that table's row (ADR-0181 decision 3's closed mapping).
///
/// - `stage_transition` carrying `from`: `t1_stage_transition`, its tree the anchor's; one with no
///   `from` is a lane entry, not a boundary, and writes the anchored `t2_event` row only.
/// - `gate_opened` / `gate_answered` / `gate_expired` / `gate_answer_void`: `t1_gate_transition`,
///   state `opened` / `answered` / `expired` / `voided`.
/// - `slice_sealed`: `t1_slice_seal`.
/// - every other kind: the anchored `t2_event` row only (a tier-2 row of the same boundary act).
///
/// # Errors
/// [`StoreError::TierOne`] naming a missing `NOT NULL` member; [`StoreError::Sqlite`] when a
/// statement refuses. The caller's transaction then rolls back both rows.
pub fn append(conn: &Connection, event: &Event, anchor: &Anchor<'_>) -> Result<(), StoreError> {
    events::insert(conn, event, Some(anchor.sha))?;
    let gate_state = match event.kind {
        EventKind::StageTransition => {
            if let Some(from) = text(event, "from") {
                // *(dated 2026-09-28, `s8e` build, package S8E-P7.)* A boundary's tree is the anchor
                // commit's tree (ADR-0105 clause 4) and the column is NOT NULL: an empty one is
                // refused rather than stored, which is what a snapshot row without a tree would be.
                if anchor.tree.is_empty() {
                    return Err(StoreError::TierOne(
                        "a `stage_transition` boundary carries no anchor tree, and \
                         `t1_stage_transition.tree_hash` is the anchor commit's tree (ADR-0105 \
                         clause 4); nothing is invented"
                            .to_owned(),
                    ));
                }
                let item = event.item().ok_or_else(|| {
                    StoreError::TierOne(
                        "a `stage_transition` boundary carries no interim `item` key, and \
                         `t1_stage_transition.item` is NOT NULL (ADR-0181 decision 3; ADR-0182)"
                            .to_owned(),
                    )
                })?;
                let to = required(event, "to")?;
                insert_stage_transition(conn, item, &from, &to, anchor.sha, anchor.tree)?;
            }
            return Ok(());
        }
        EventKind::SliceSealed => {
            let project = required(event, "project")?;
            let slice = required(event, "slice")?;
            let verdict = required(event, "verdict")?;
            return insert_slice_seal(conn, &project, &slice, &verdict, anchor.sha);
        }
        EventKind::GateOpened => GateState::Opened,
        EventKind::GateAnswered => GateState::Answered,
        EventKind::GateExpired => GateState::Expired,
        EventKind::GateAnswerVoid => GateState::Voided,
        _ => return Ok(()),
    };
    let (stage, run_id) = match (gate_state, anchor.opening_run) {
        (GateState::Opened, Some((stage, run))) => (Some(stage.to_owned()), Some(run.to_owned())),
        _ => (None, None),
    };
    let row = GateTransition {
        token: required(event, "token")?,
        state: gate_state,
        tree_hash: text(event, "tree_hash").or_else(|| text(event, "bound_tree_hash")),
        deadline: text(event, "deadline"),
        kind: text(event, "kind"),
        reach: text(event, "reach"),
        option_id: text(event, "option").or_else(|| text(event, "verdict")),
        stage,
        run_id,
    };
    insert_gate_transition(conn, &row, anchor.sha)
}

/// One row a store holds, as `SqliteStore::snapshot` reads it and `SqliteStore::rebuild` writes it
/// (ADR-0181 decision 11; `rk_ports::event_store::SnapshotRow`'s store-side twin). *(dated
/// 2026-09-28, `s8e` build, package S8E-P7.)*
#[derive(Debug, Clone, PartialEq)]
pub struct StoredRow {
    /// The event, carrying its interim `item` key.
    pub event: Event,
    /// `t2_event.anchor_sha`.
    pub anchor_sha: Option<String>,
    /// The anchor commit's tree, where a table stores it: a boundary's `t1_stage_transition`.
    pub tree: Option<String>,
    /// A gate opening's `(stage, run_id)`, where its `opened` transition row stores both.
    pub opening_run: Option<(String, String)>,
}

/// Every `t2_event` row in append order, each with its anchor and the two `t1_*` facts a payload
/// does not carry (the boundary's tree, the opening run). Read-only.
///
/// # Errors
/// [`StoreError::Sqlite`] when a statement refuses or a row does not decode (a kind this binary
/// does not name is a typed failure, never a dropped row — `events::select`'s rule).
pub fn stored_rows(conn: &Connection) -> Result<Vec<StoredRow>, StoreError> {
    let mut stmt = conn.prepare("SELECT anchor_sha FROM t2_event ORDER BY id")?;
    let anchors: Vec<Option<String>> = stmt
        .query_map([], |r| r.get::<_, Option<String>>(0))?
        .collect::<Result<_, _>>()?;
    let events = events::select(conn, events::Selector::All)?;
    if events.len() != anchors.len() {
        return Err(StoreError::Sqlite(format!(
            "t2_event read {} rows and {} anchors",
            events.len(),
            anchors.len()
        )));
    }
    let mut out = Vec::with_capacity(events.len());
    for (event, anchor_sha) in events.into_iter().zip(anchors) {
        let mut tree = None;
        let mut opening_run = None;
        if let Some(sha) = anchor_sha.as_deref() {
            match event.kind {
                EventKind::StageTransition => {
                    if let (Some(item), Some(from), Some(to)) =
                        (event.item(), text(&event, "from"), text(&event, "to"))
                    {
                        tree = conn
                            .query_row(
                                "SELECT tree_hash FROM t1_stage_transition WHERE anchor_sha = ?1 \
                                 AND item = ?2 AND from_stage = ?3 AND to_stage = ?4 ORDER BY id \
                                 LIMIT 1",
                                (sha, item, &from, &to),
                                |r| r.get::<_, String>(0),
                            )
                            .map(Some)
                            .or_else(none_when_absent)?;
                    }
                }
                EventKind::GateOpened => {
                    if let Some(token) = text(&event, "token") {
                        opening_run = conn
                            .query_row(
                                "SELECT stage, run_id FROM t1_gate_transition WHERE anchor_sha = \
                                 ?1 AND token = ?2 AND state = 'opened' ORDER BY id LIMIT 1",
                                (sha, &token),
                                |r| {
                                    Ok((
                                        r.get::<_, Option<String>>(0)?,
                                        r.get::<_, Option<String>>(1)?,
                                    ))
                                },
                            )
                            .map(|(stage, run)| stage.zip(run))
                            .or_else(none_when_absent)?;
                    }
                }
                _ => {}
            }
        }
        out.push(StoredRow {
            event,
            anchor_sha,
            tree,
            opening_run,
        });
    }
    Ok(out)
}

/// `QueryReturnedNoRows` is an absent fact, not a failure.
fn none_when_absent<T>(e: rusqlite::Error) -> Result<Option<T>, StoreError> {
    match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.into()),
    }
}

/// **Rebuild, append-only** (ADR-0181 decision 11): the store's current rows must be a prefix of
/// `rows`, and the rest are written in order — an unanchored row through `events::insert`, an
/// anchored one through [`append`], whose closed mapping writes its `t1_*` row. So a rebuild from a
/// snapshot of the current state writes nothing, a fresh store (the staging file) gains every row,
/// and nothing is ever updated or deleted. The caller's one transaction makes it all-or-nothing.
///
/// # Errors
/// [`StoreError::TierOne`] when the store's rows are not a prefix of `rows`, or an anchored row
/// cannot fill its `t1_*` row; [`StoreError::Sqlite`] when a statement refuses.
pub fn rebuild(conn: &Connection, rows: &[StoredRow]) -> Result<usize, StoreError> {
    let held = stored_rows(conn)?;
    if held.len() > rows.len() || rows[..held.len()] != held[..] {
        return Err(StoreError::TierOne(format!(
            "rebuild: the store's {} row(s) are not a prefix of the snapshot's {}; a rebuild \
             neither updates nor deletes (AC-364; ADR-0181 decision 11)",
            held.len(),
            rows.len()
        )));
    }
    for row in &rows[held.len()..] {
        match row.anchor_sha.as_deref() {
            None => events::insert(conn, &row.event, None)?,
            Some(sha) => append(
                conn,
                &row.event,
                &Anchor {
                    sha,
                    tree: row.tree.as_deref().unwrap_or_default(),
                    opening_run: row
                        .opening_run
                        .as_ref()
                        .map(|(stage, run)| (stage.as_str(), run.as_str())),
                },
            )?,
        }
    }
    Ok(rows.len() - held.len())
}
