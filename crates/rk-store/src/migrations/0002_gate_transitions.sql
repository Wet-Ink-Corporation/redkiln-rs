-- 0002_gate_transitions -- D21-46 R2's forward migration; ADR-0181 decisions 5 and 6 (proposed).
--
-- A gate's lifecycle is append-only and its state is a fold (D21-43 N6: "`t1_gate` is append-only
-- and state is a fold."). `t1_gate` could not hold it: `token ... UNIQUE` admits one row per token
-- and its `answered` column could only move by an in-place write, which AC-364 forbids. So:
--
--   * `t1_gate_transition` holds ONE ROW PER TRANSITION (opened, answered, expired, voided), with
--     `UNIQUE (token, state)` and no column a later write could move. A re-reach mints a fresh
--     token (ADR-0105 clause 10), so the pair stays unique. The opening run's `stage` and `run_id`
--     sit on the `opened` row only; the descriptive members are nullable because the answered,
--     expired and void payloads do not carry every member the `Gate:` value does, and a writer
--     never invents one (the rebuild fills them from the trailer value).
--   * `t1_gate` is DROPPED. Nothing ever wrote it: 0 rows in every store measured (s8e 8e.0 items
--     2 and 4, 2026-09-28). Its `answered` column is never written again, because it no longer
--     exists.
--   * `t2_event` gains the interim `item` key (ADR-0182), beside `anchor_sha` and outside the
--     payload, with an index for the `ByItem` selector.
--
-- A store at schema version 1 is REFUSED by the identity check and rebuilt from git by
-- `doctor --rebuild`, never migrated in place (AC-180). This file runs once per store: the runner
-- counts applied migrations in `PRAGMA user_version` (rk_store::migrate).

CREATE TABLE IF NOT EXISTS t1_gate_transition (
    id         INTEGER PRIMARY KEY,
    token      TEXT NOT NULL,
    state      TEXT NOT NULL CHECK (state IN ('opened', 'answered', 'expired', 'voided')),
    tree_hash  TEXT,            -- repo-key
    deadline   TEXT,
    kind       TEXT,
    reach      TEXT,
    option_id  TEXT,
    stage      TEXT,
    run_id     TEXT,
    anchor_sha TEXT NOT NULL,   -- repo-key
    UNIQUE (token, state)
);

DROP TABLE IF EXISTS t1_gate;

ALTER TABLE t2_event ADD COLUMN item TEXT;

CREATE INDEX IF NOT EXISTS t2_event_item ON t2_event (item);
