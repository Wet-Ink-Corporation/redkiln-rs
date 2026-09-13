-- 0001_initial -- the tier-1/tier-2 split, as tables.
--
-- Every column that references the repository carries a `-- repo-key` marker and MUST appear in
-- the sibling `.keys` file with a declared domain of `sha` or `tree` (AC-166). The marker is what
-- makes the lint decidable: the criterion was repaired precisely because "a lint cannot recognise
-- a branch name by inspection", so the column says what it is rather than the lint guessing.

CREATE TABLE IF NOT EXISTS rk_identity (
    id             INTEGER PRIMARY KEY CHECK (id = 1),
    root_commit    TEXT    NOT NULL,   -- repo-key
    common_dir     TEXT    NOT NULL,
    schema_version INTEGER NOT NULL
);

-- ---------------------------------------------------------------- tier 1: git first
-- These rows never exist without their trailer commit. A failed git write leaves ZERO of them
-- (AC-160..AC-163), which is why every one is anchored to a commit sha.

CREATE TABLE IF NOT EXISTS t1_stage_transition (
    id         INTEGER PRIMARY KEY,
    item       TEXT NOT NULL,
    from_stage TEXT NOT NULL,
    to_stage   TEXT NOT NULL,
    anchor_sha TEXT NOT NULL,   -- repo-key
    tree_hash  TEXT NOT NULL    -- repo-key
);

CREATE TABLE IF NOT EXISTS t1_gate (
    id         INTEGER PRIMARY KEY,
    token      TEXT NOT NULL UNIQUE,
    kind       TEXT NOT NULL,
    tree_hash  TEXT NOT NULL,   -- repo-key
    deadline   TEXT NOT NULL,
    answered   INTEGER NOT NULL DEFAULT 0,
    anchor_sha TEXT NOT NULL    -- repo-key
);

CREATE TABLE IF NOT EXISTS t1_slice_seal (
    id         INTEGER PRIMARY KEY,
    project    TEXT NOT NULL,
    slice      TEXT NOT NULL,
    verdict    TEXT NOT NULL,
    anchor_sha TEXT NOT NULL    -- repo-key
);

-- ---------------------------------------------------------------- tier 2: rebuildable or disposable
-- Truncating all of this leaves the stage-level next action unchanged (AC-164), and `doctor`
-- classifies the loss as telemetry-only, never corruption.

CREATE TABLE IF NOT EXISTS t2_event (
    id         INTEGER PRIMARY KEY,
    kind       TEXT NOT NULL,
    payload    TEXT NOT NULL,
    anchor_sha TEXT             -- repo-key
);

-- The memo is TIER 2 (ADR-0097 decision 1): rebuildable by re-execution, never by rebuild. Its key
-- is a content digest, not a repository reference, so it carries no `repo-key` marker.
CREATE TABLE IF NOT EXISTS t2_memo (
    key         TEXT PRIMARY KEY,
    result_json TEXT NOT NULL,
    live        INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS t2_usage (
    id                 INTEGER PRIMARY KEY,
    argv_digest        TEXT NOT NULL,
    env_digest         TEXT NOT NULL,
    instrument_version TEXT NOT NULL
);
