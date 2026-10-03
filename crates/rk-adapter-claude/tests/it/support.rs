//! Shared helpers (s7c §7c.18): a valid `Config`, the fixture cards and roles under
//! `tests/fixtures/`, the one tiny schema every bind in this binary uses — and, since Slice 5, the
//! spawn seam: the fixture child as `harness_binary`, the marker directory and mode in the captured
//! map (§7c.12: *"the seam pins the argv and the env carries the script"*), transcripts composed
//! from the labelled parts under `fixtures/streams/` and the byte-exact rows under
//! `fixtures/recorded/`, and one real `dispatch` through `ClaudeHarness` with the `VecStore` fake
//! as the surface.
//!
//! Every `Config` here is built from a captured map the TEST writes as a value (ADR-0083; AC-217):
//! nothing reads or sets the process environment — `set_var` is clippy-banned workspace-wide and
//! the block's `rk_absent` over `tests/it` would name it. The fixture child is reached through
//! the compile-time `env!("CARGO_BIN_EXE_rk-fixture-child-claude")` (§7c.17), never `std::env::var`
//! and never the process cwd.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_ports::cancel::CancelToken;
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_ports::fakes::VecStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::SchemaId;

use rk_adapter_claude::ClaudeHarness;
use rk_adapter_claude::binder::{BindRequest, Binder, Role, Schema};
use rk_adapter_claude::card::{AgentCard, AuthoredCard, Effort, ModelRow, Models};
use rk_adapter_claude::config::{Config, ConfigValues, Margins, MismatchPolicy};

/// `tests/fixtures/`, absolute — through the compile-time manifest dir, never the process cwd.
#[must_use]
pub fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// The text of a fixture file.
#[must_use]
pub fn fixture(rel: &str) -> String {
    let path = fixtures().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The register's 0.07 / 0.18 vectors as ticks (AC-377) — above two on both windows.
#[must_use]
pub const fn margins_ok() -> Margins {
    Margins {
        five_hour: 7,
        seven_day: 18,
    }
}

/// The values a valid `Config` is built from: an EMPTY captured map, `claude` as the program (the
/// probe's value — every spawn-observing test replaces it with the fixture child at Slice 5), the
/// TTL pair left to the record's value, the margins above and refusal on a version mismatch (O-4's
/// default). Tests mutate what they need.
#[must_use]
pub fn values() -> ConfigValues {
    ConfigValues {
        captured_env: BTreeMap::new(),
        harness_binary: PathBuf::from("claude"),
        prompt_cache_ttl: None,
        subagent_prompt_cache_ttl: None,
        margins: margins_ok(),
        on_version_mismatch: MismatchPolicy::Refuse,
    }
}

/// A `Config` over `values()` — the construction every lint-free path starts from.
///
/// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137.)* With the shared settings
/// directory ([`shared_settings_dir`]): every bind checks one, and a bind writes nothing there.
#[must_use]
pub fn config() -> Config {
    Config::new(values())
        .expect("the baseline values construct")
        .with_settings_dir(shared_settings_dir())
}

/// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; `s8h` §8h.6; D21-137.)* The settings
/// directory a test `Config` that owns no marker directory carries:
/// `<CARGO_TARGET_TMPDIR>/rk-adapter-claude-settings`, emptied once per test process and outside
/// every test's worktree (each worktree is its own temp directory). A bind writes nothing here; a dispatch over such a
/// `Config` writes its file here, and the next test process's first call removes it.
#[must_use]
pub fn shared_settings_dir() -> PathBuf {
    static DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    DIR.get_or_init(|| {
        let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("rk-adapter-claude-settings");
        if dir.exists() {
            // A settings file is written read-only: clear the flag before removing it.
            for entry in std::fs::read_dir(&dir)
                .expect("the shared settings dir lists")
                .flatten()
            {
                let path = entry.path();
                if let Ok(meta) = std::fs::metadata(&path) {
                    let mut perms = meta.permissions();
                    #[expect(
                        clippy::permissions_set_readonly_false,
                        reason = "test cleanup: a settings file is written read-only (D21-137) and \
                                  the shared directory is emptied once per test process"
                    )]
                    perms.set_readonly(false);
                    let _ = std::fs::set_permissions(&path, perms);
                }
                let _ = std::fs::remove_file(&path);
            }
        }
        std::fs::create_dir_all(&dir).expect("the shared settings dir");
        dir
    })
    .clone()
}

/// A `Config` whose captured map is `pairs`, everything else `values()`.
pub fn config_with_env(
    pairs: &[(&str, &str)],
) -> Result<Config, rk_adapter_claude::config::ConfigError> {
    let mut v = values();
    v.captured_env = pairs
        .iter()
        .map(|(k, val)| ((*k).to_owned(), (*val).to_owned()))
        .collect();
    Config::new(v).map(|c| c.with_settings_dir(shared_settings_dir()))
}

/// A catalog card by stem (`tests/fixtures/cards/catalog/<stem>.json`), validated.
#[must_use]
pub fn catalog_card(stem: &str) -> AuthoredCard {
    let text = fixture(&format!("cards/catalog/{stem}.json"));
    AuthoredCard::from_file(stem, &text)
        .unwrap_or_else(|e| panic!("catalog card {stem} must load: {e}"))
}

/// A red card's load result by stem (`tests/fixtures/cards/red/<stem>.json`).
pub fn red_card(stem: &str) -> Result<AuthoredCard, rk_adapter_claude::config::ConfigError> {
    let text = fixture(&format!("cards/red/{stem}.json"));
    AuthoredCard::from_file(stem, &text)
}

/// A role fixture by stem (`tests/fixtures/cards/roles/<stem>.json`).
#[must_use]
pub fn role(stem: &str) -> Role {
    let text = fixture(&format!("cards/roles/{stem}.json"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("role {stem} must parse: {e}"))
}

/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)* The `[models]` table every bind in this binary
/// resolves a role's row key through: ADR-0197 decision 4's three rows (proposed), the spellings
/// `s8h` §8h.3 fixes. `haiku` rejects `--effort` (the lifted `EFFORTLESS_TIERS = ['haiku']`).
#[must_use]
pub fn models() -> Models {
    [
        ("haiku", "claude-haiku-4-5-20251001", false),
        ("sonnet", "claude-sonnet-5", true),
        ("opus", "claude-opus-5-5", true),
    ]
    .into_iter()
    .map(|(k, id, accepts_effort)| {
        (
            k.to_owned(),
            ModelRow {
                id: id.to_owned(),
                accepts_effort,
            },
        )
    })
    .collect()
}

/// The one-field schema every bind in this binary carries: `{ ok: boolean }`, required. Its JSON
/// is fixed here and in the golden argv fixtures, so the `--json-schema` element is deterministic.
#[must_use]
pub fn schema() -> Schema {
    Schema {
        id: SchemaId::new("probe-ok"),
        json: serde_json::json!({
            "type": "object",
            "properties": { "ok": { "type": "boolean" } },
            "required": ["ok"]
        }),
    }
}

/// A worktree path for a bind — a fixed, absolute-looking value; nothing is spawned into it at
/// Slice 4.
#[must_use]
pub fn worktree() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"D:\rk\worktrees\run-1")
    } else {
        PathBuf::from("/rk/worktrees/run-1")
    }
}

// ---------------------------------------------------------------- Slice 5: the spawn seam

/// The fixture child, compiled under this crate's own target name (§7c.17) — the `harness_binary`
/// of every spawn-observing test. Compile-time: cargo sets `CARGO_BIN_EXE_<name>` for the owning
/// package's own tests.
#[must_use]
pub fn fixture_child() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_rk-fixture-child-claude"))
}

/// The two names the fixture child reads at its own edge (§7c.12): the mode and the marker dir.
pub const MODE_KEY: &str = "RK_FIXTURE_MODE";
pub const MARKER_DIR_KEY: &str = "RK_FIXTURE_MARKER_DIR";

/// A marker directory: one file per fixture-child invocation (§7c.17), so *"dispatch count 0"*
/// and *"exactly one spawn"* are FILE COUNTS. Halt 6: every test makes its own.
pub struct Markers {
    dir: tempfile::TempDir,
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137.)* The settings
    /// directory a spawn `Config` over these markers carries: its own temp directory, beside and
    /// never under the marker directory or the worktree, removed with the test.
    settings: tempfile::TempDir,
}

/// One marker file, parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    pub pid: u32,
    pub mode: String,
    pub argv: Vec<String>,
    pub extra: Vec<String>,
}

impl Markers {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().expect("a temp marker dir"),
            settings: tempfile::tempdir().expect("a temp settings dir"),
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    /// The settings directory a spawn `Config` over these markers carries (D21-137).
    #[must_use]
    pub fn settings_dir(&self) -> &Path {
        self.settings.path()
    }

    /// Every `*.marker` file, parsed, in directory order.
    #[must_use]
    pub fn read(&self) -> Vec<Marker> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(self.dir.path()).expect("marker dir readable") {
            let path = entry.expect("entry").path();
            if path.extension().is_none_or(|e| e != "marker") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("marker readable");
            let mut m = Marker {
                pid: 0,
                mode: String::new(),
                argv: Vec::new(),
                extra: Vec::new(),
            };
            for line in text.lines() {
                if let Some(p) = line.strip_prefix("pid=") {
                    m.pid = p.parse().expect("pid");
                } else if let Some(mode) = line.strip_prefix("mode=") {
                    m.mode = mode.to_owned();
                } else if let Some(a) = line.strip_prefix("arg=") {
                    m.argv.push(a.to_owned());
                } else {
                    m.extra.push(line.to_owned());
                }
            }
            out.push(m);
        }
        out
    }

    /// The number of marker files — the spawn count (§7c.18: a file count, not a mock).
    #[must_use]
    pub fn count(&self) -> usize {
        self.read().len()
    }

    /// The markers with `mode=<mode>`.
    #[must_use]
    pub fn with_mode(&self, mode: &str) -> Vec<Marker> {
        self.read().into_iter().filter(|m| m.mode == mode).collect()
    }
}

/// `ConfigValues` for a spawn: the fixture child as the program, the marker directory and the
/// mode in the captured map (names no denylist carries and no adapter `src/` file reads), the
/// rest `values()`.
#[must_use]
pub fn spawn_values(markers: &Markers, mode: &str) -> ConfigValues {
    let mut v = values();
    v.harness_binary = fixture_child();
    v.captured_env.insert(MODE_KEY.to_owned(), mode.to_owned());
    v.captured_env.insert(
        MARKER_DIR_KEY.to_owned(),
        markers.path().to_string_lossy().into_owned(),
    );
    v
}

/// A `Config` over [`spawn_values`].
#[must_use]
pub fn spawn_config(markers: &Markers, mode: &str) -> Config {
    Config::new(spawn_values(markers, mode))
        .expect("the spawn values construct")
        .with_settings_dir(markers.settings_dir().to_path_buf())
}

/// A transcript file composed from parts, written to a temp file the fixture child replays
/// (`replay:<file>` / `wait-stdin-eof:<file>`).
pub struct Transcript {
    _dir: tempfile::TempDir,
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

/// One part of a transcript.
pub enum Part<'a> {
    /// A fixture file under `tests/fixtures/`, appended whole (its bytes, its line terminators).
    File(&'a str),
    /// Raw bytes, appended as given (the caller supplies the terminator).
    Bytes(Vec<u8>),
}

#[must_use]
pub fn transcript(parts: &[Part<'_>]) -> Transcript {
    let dir = tempfile::tempdir().expect("a temp transcript dir");
    let mut bytes = Vec::new();
    for part in parts {
        match part {
            Part::File(rel) => {
                let p = fixtures().join(rel);
                bytes.extend_from_slice(
                    &std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display())),
                );
            }
            Part::Bytes(b) => bytes.extend_from_slice(b),
        }
    }
    let path = dir.path().join("transcript.jsonl");
    std::fs::write(&path, &bytes).expect("write transcript");
    Transcript {
        _dir: dir,
        path,
        bytes,
    }
}

/// The good transcript: the scripted init, then the scripted `Ok` result.
#[must_use]
pub fn ok_transcript() -> Transcript {
    transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ])
}

/// The byte span of the first `unknown_events` element of the e1-smoke probe row `probe_row`
/// (0-based among the two probe rows): the runner's re-serialised `rate_limit_event` line, cut
/// from the recorded row's bytes (RE-SERIALISED at the pin), never re-serialised again here.
#[must_use]
pub fn e1_smoke_rate_limit_event(probe_row: usize) -> Vec<u8> {
    let bytes = std::fs::read(fixtures().join("recorded/e1-smoke.L1-3.jsonl")).expect("e1-smoke");
    let row = bytes
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .nth(probe_row + 1)
        .expect("the probe row");
    let key = b"\"unknown_events\":[";
    let start = row
        .windows(key.len())
        .position(|w| w == key)
        .expect("unknown_events on the row")
        + key.len();
    // walk the balanced object, string-aware
    let mut depth = 0usize;
    let mut i = start;
    let mut in_string = false;
    let mut escaped = false;
    loop {
        let c = row[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'"' {
                in_string = false;
            }
        } else {
            match c {
                b'"' => in_string = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return row[start..=i].to_vec();
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
}

/// A prompt larger than the pipe buffer (§7c.11: *"a prompt fixture larger than the pipe buffer —
/// ≥ 1 MiB — so the parent's write blocks until the child's close and fails deterministically"*):
/// the stage's ONE MiB.
///
/// **History, so the figure is not raised again for the wrong reason**: this was 8 MiB from
/// 2026-09-18 until the runtime-safety review of Slice 5 (R5-1), on a misdiagnosis — a 1 MiB
/// write to a `close-stdin-early` child came back `Transport { kind: Decode }` and the note blamed
/// *"the pipe's kernel buffer"*. The cause was the adapter's: tokio 1.53.1's Windows `ChildStdin`
/// is `Blocking<ArcFile>`, whose `poll_write` copies up to 2 MiB into a buffer and returns
/// `Ready(Ok(n))` before the `WriteFile` runs and whose `poll_shutdown` is `Ready(Ok(()))` with no
/// wait, so `write_all` + `shutdown` completed `Ok` while the only write was still in flight and
/// its `os error 109` was dropped; only a payload spanning two chunks (> 2 MiB) surfaced the
/// first chunk's error on the second `poll_write`. The runner now `flush`es before dropping the
/// handle (`runner.rs`), which awaits the in-flight write and surfaces its error at any size, and
/// the stage's 1 MiB is what this returns.
#[must_use]
pub fn megabyte_prompt() -> String {
    "the prompt that does not fit a pipe buffer. ".repeat(1024 * 1024 / 44 + 1)
}

/// A bound probe-card spec over `config` (the binder fills the card store): the haiku role, the
/// catalog's `probe` card, the one schema, `timeout_secs`, the run-owned `worktree`, and
/// `inputs` carrying the user message under `"prompt"`. Returns the config with the store filled
/// and the spec.
#[must_use]
pub fn bind_probe(
    config: Config,
    timeout_secs: u64,
    worktree: &Path,
    prompt: &str,
) -> (Config, ActivitySpec) {
    bind_probe_with_inputs(config, timeout_secs, worktree, prompt, &[])
}

/// As [`bind_probe`], with `extra` inputs bound after the `"prompt"` entry (ADR-0163: the
/// `"item"` input `verbs/run.rs` binds, carried here as a value the TEST writes).
#[must_use]
pub fn bind_probe_with_inputs(
    config: Config,
    timeout_secs: u64,
    worktree: &Path,
    prompt: &str,
    extra: &[(&str, serde_json::Value)],
) -> (Config, ActivitySpec) {
    bind_probe_as(config, "haiku", timeout_secs, worktree, prompt, extra)
}

/// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.5.)* As
/// [`bind_probe_with_inputs`], with the probe card bound under the role fixture `role_stem`
/// (`tests/fixtures/cards/roles/<role_stem>.json`) — so a test can request another `[models]`
/// row's full id (`sonnet-high` → `claude-sonnet-5`).
#[must_use]
pub fn bind_probe_as(
    config: Config,
    role_stem: &str,
    timeout_secs: u64,
    worktree: &Path,
    prompt: &str,
    extra: &[(&str, serde_json::Value)],
) -> (Config, ActivitySpec) {
    let mut inputs = vec![("prompt".to_owned(), serde_json::json!(prompt))];
    inputs.extend(extra.iter().map(|(k, v)| ((*k).to_owned(), v.clone())));
    // *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137.)* A test `Config` built
    // by `Config::new` alone carries no settings directory; the shared one is supplied here, and
    // one the test supplied is kept.
    let config = if config.settings_dir().is_some() {
        config
    } else {
        config.with_settings_dir(shared_settings_dir())
    };
    let card = catalog_card("probe");
    let r = role(role_stem);
    let sch = schema();
    let models = models();
    let mut binder = Binder::new(config);
    let spec = binder
        .bind(BindRequest {
            role_name: role_stem,
            role: &r,
            card: &card,
            schema: &sch,
            timeout_secs,
            worktree,
            inputs,
            activity_id_at_version: None,
            models: &models,
        })
        .expect("the probe card binds");
    (binder.into_config(), spec)
}

/// A harness over `config` with a fresh token (the root's clone in production).
#[must_use]
pub fn harness(config: Config) -> ClaudeHarness {
    ClaudeHarness::new(config, CancelToken::new()).expect("the harness constructs")
}

/// A harness over `config` with `token` (cloned in at construction — ADR-0099 d2).
#[must_use]
pub fn harness_with_token(config: Config, token: CancelToken) -> ClaudeHarness {
    ClaudeHarness::new(config, token).expect("the harness constructs")
}

/// The `dispatch_completed` rows in a `VecStore`, deserialised back to records.
#[must_use]
pub fn completed_rows(store: &VecStore) -> Vec<DispatchRecord> {
    store
        .events()
        .iter()
        .filter(|e| e.kind == EventKind::DispatchCompleted)
        .map(|e| serde_json::from_value(e.payload.clone()).expect("a DispatchRecord payload"))
        .collect()
}

/// The `rate_limit_reading` rows in a `VecStore`, as their payloads, in journal order
/// (ADR-0163 clause 3's fixed key set).
#[must_use]
pub fn reading_rows(store: &VecStore) -> Vec<serde_json::Value> {
    store
        .events()
        .iter()
        .filter(|e| e.kind == EventKind::RateLimitReading)
        .map(|e| e.payload.clone())
        .collect()
}

/// The kinds a `VecStore` holds, in journal order.
#[must_use]
pub fn kinds(store: &VecStore) -> Vec<EventKind> {
    store.events().iter().map(|e| e.kind).collect()
}

/// A store whose every `append` fails, counting the attempts (AC-368's fault-injecting fake).
#[derive(Debug, Default)]
pub struct FailingStore {
    pub attempts: usize,
}

impl EventStore for FailingStore {
    fn append(&mut self, _event: Event) -> Result<(), StepError> {
        self.attempts += 1;
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail: "disk full (injected)".to_owned(),
        })
    }
    /// *(dated 2026-09-28, s8e build; ADR-0181, proposed.)* The port's fifth method on this test
    /// double. The anchor is not modelled here, so the row is appended exactly as `append`
    /// appends it; the anchored behaviour is `VecStore`'s and `SqliteStore`'s, under test there.
    fn append_tier1(
        &mut self,
        event: Event,
        _anchor: &rk_ports::event_store::TierOneAnchor,
    ) -> Result<(), StepError> {
        self.append(event)
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        Ok(Vec::new())
    }
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Err(StepError::Unsupported {
            capability: "snapshot".to_owned(),
        })
    }
    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "rebuild".to_owned(),
        })
    }
}

/// Everything one dispatch through the fixture child produced.
pub struct Replayed {
    pub outcome: DispatchOutcome,
    pub store: VecStore,
    pub markers: Markers,
    pub worktree: tempfile::TempDir,
}

/// One real `dispatch` with the child in `replay:<transcript>` mode: a fresh marker dir, the
/// probe card bound with `timeout_secs`, a temp worktree, a fresh `VecStore`.
#[must_use]
pub fn dispatch_replay(t: &Transcript, timeout_secs: u64) -> Replayed {
    dispatch_mode(
        &format!("replay:{}", t.path.display()),
        timeout_secs,
        "hello",
    )
}

/// As [`dispatch_replay`], with `extra` inputs bound beside the prompt
/// ([`bind_probe_with_inputs`]).
#[must_use]
pub fn dispatch_replay_with_inputs(
    t: &Transcript,
    timeout_secs: u64,
    extra: &[(&str, serde_json::Value)],
) -> Replayed {
    let markers = Markers::new();
    let worktree = tempfile::tempdir().expect("a temp worktree");
    let config = spawn_config(&markers, &format!("replay:{}", t.path.display()));
    let (config, spec) =
        bind_probe_with_inputs(config, timeout_secs, worktree.path(), "hello", extra);
    let h = harness(config);
    let mut store = VecStore::default();
    let outcome = h.dispatch(&spec, &mut store);
    Replayed {
        outcome,
        store,
        markers,
        worktree,
    }
}

/// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* As [`dispatch_replay`],
/// with the probe card bound under the role fixture `role_stem` ([`bind_probe_as`]).
#[must_use]
pub fn dispatch_replay_as(t: &Transcript, role_stem: &str, timeout_secs: u64) -> Replayed {
    let markers = Markers::new();
    let worktree = tempfile::tempdir().expect("a temp worktree");
    let config = spawn_config(&markers, &format!("replay:{}", t.path.display()));
    let (config, spec) = bind_probe_as(
        config,
        role_stem,
        timeout_secs,
        worktree.path(),
        "hello",
        &[],
    );
    let h = harness(config);
    let mut store = VecStore::default();
    let outcome = h.dispatch(&spec, &mut store);
    Replayed {
        outcome,
        store,
        markers,
        worktree,
    }
}

/// As [`dispatch_replay`], with the fixture child in an arbitrary mode and a given prompt.
#[must_use]
pub fn dispatch_mode(mode: &str, timeout_secs: u64, prompt: &str) -> Replayed {
    let markers = Markers::new();
    let worktree = tempfile::tempdir().expect("a temp worktree");
    let config = spawn_config(&markers, mode);
    let (config, spec) = bind_probe(config, timeout_secs, worktree.path(), prompt);
    let h = harness(config);
    let mut store = VecStore::default();
    let outcome = h.dispatch(&spec, &mut store);
    Replayed {
        outcome,
        store,
        markers,
        worktree,
    }
}

/// §7c.18's marker-count BELT for a refusal test whose `Config::new` returned `Ok`: re-validate
/// the same `values` with the fixture child as the program and a fresh marker dir in the map,
/// bind the probe card, dispatch ONCE through the real `dispatch` (the child in `exit:0` mode)
/// and return the marker count — which a refusing implementation never lets reach 1. Vacuous
/// unless called on the `Ok` branch, which is where every refusal test calls it.
#[must_use]
pub fn belt_dispatch_count(mut values: ConfigValues) -> usize {
    let markers = Markers::new();
    values.harness_binary = fixture_child();
    values
        .captured_env
        .insert(MODE_KEY.to_owned(), "exit:0".to_owned());
    values.captured_env.insert(
        MARKER_DIR_KEY.to_owned(),
        markers.path().to_string_lossy().into_owned(),
    );
    let config = Config::new(values)
        .expect("the belt re-validates what the test's Config::new accepted, plus two seam names")
        .with_settings_dir(markers.settings_dir().to_path_buf());
    let worktree = tempfile::tempdir().expect("a temp worktree");
    let (config, spec) = bind_probe(config, 30, worktree.path(), "belt");
    let h = harness(config);
    let mut store = VecStore::default();
    let _ = h.dispatch(&spec, &mut store);
    markers.count()
}

/// The belt for a BIND-level refusal whose `bind` returned `Ok(spec)`: dispatch that spec once
/// over `config` (built from [`spawn_config`], so the program is the fixture child and the marker
/// dir is in the map) and return the marker count.
#[must_use]
pub fn belt_dispatch_spec(config: Config, spec: &ActivitySpec, markers: &Markers) -> usize {
    let h = harness(config);
    let mut store = VecStore::default();
    let _ = h.dispatch(spec, &mut store);
    markers.count()
}

/// Wait (bounded parks, never `sleep`) until `pred` is true or `max_polls` × 20 ms elapse.
pub fn wait_until<T: Truthy>(max_polls: u32, mut pred: impl FnMut() -> T) -> T {
    for _ in 0..max_polls {
        let v = pred();
        if v.is_truthy() {
            return v;
        }
        std::thread::park_timeout(std::time::Duration::from_millis(20));
    }
    pred()
}

/// What [`wait_until`] polls for: a `bool`, or an `Option` that is `Some`.
pub trait Truthy {
    fn is_truthy(&self) -> bool;
}

impl Truthy for bool {
    fn is_truthy(&self) -> bool {
        *self
    }
}

impl<T> Truthy for Option<T> {
    fn is_truthy(&self) -> bool {
        self.is_some()
    }
}

/// The `holder_pid=` the `orphan` marker in `dir` carries, once it exists — the orphan mode
/// writes its marker only after the holder's own marker appeared and the intermediate exited.
#[must_use]
pub fn orphan_holder_pid_in(dir: &Path) -> Option<u32> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let text = std::fs::read_to_string(entry.path()).ok()?;
        if text.lines().any(|l| l == "mode=orphan") {
            return text
                .lines()
                .find_map(|l| l.strip_prefix("holder_pid=")?.parse().ok());
        }
    }
    None
}

/// One `dispatch` on its own thread, joined under a bound of `secs` (F2: a containment that
/// leaves a pipe holder alive would otherwise hang the suite on the stderr reader); `None` when
/// the bound expired — the caller reports the holders' liveness. The thread is detached on
/// expiry; the harness is held as `Arc` so the caller keeps its probes.
#[must_use]
pub fn dispatch_bounded(
    h: std::sync::Arc<ClaudeHarness>,
    spec: ActivitySpec,
    secs: u64,
) -> Option<(DispatchOutcome, VecStore)> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut store = VecStore::default();
        let outcome = h.dispatch(&spec, &mut store);
        let _ = tx.send((outcome, store));
    });
    rx.recv_timeout(std::time::Duration::from_secs(secs)).ok()
}

/// The number of marker files in `dir` — a free function so a spawned thread can call it on a
/// path it owns.
#[must_use]
pub fn marker_count_in(dir: &Path) -> usize {
    std::fs::read_dir(dir)
        .map(|d| {
            d.flatten()
                .filter(|e| e.path().extension().is_some_and(|x| x == "marker"))
                .count()
        })
        .unwrap_or(0)
}

/// The pid of the first marker in `dir`.
///
/// # Panics
/// If there is none.
#[must_use]
pub fn first_marker_pid_in(dir: &Path) -> u32 {
    for entry in std::fs::read_dir(dir).expect("dir").flatten() {
        let p = entry.path();
        if p.extension().is_some_and(|x| x == "marker") {
            let text = std::fs::read_to_string(&p).expect("marker");
            return text
                .lines()
                .find_map(|l| l.strip_prefix("pid=")?.parse().ok())
                .expect("pid=");
        }
    }
    panic!("no marker in {}", dir.display());
}

/// The belt for a CARD-level refusal whose load / lint returned `Ok`: bind the authored card
/// under a role naming the `[models]` row `tier` over a spawn config (the fixture child in `exit:0` mode, a fresh marker
/// dir), dispatch once if the bind passes, and return the marker count — 0 unless something
/// spawned. A bind that refuses spawns nothing and counts 0 (the refusal is the test's own
/// assertion; this is the belt behind it).
#[must_use]
pub fn belt_dispatch_authored(card: &AuthoredCard, tier: &str, effort: Option<Effort>) -> usize {
    let markers = Markers::new();
    let worktree = tempfile::tempdir().expect("a temp worktree");
    let sch = schema();
    let r = Role {
        model: Some(tier.to_owned()),
        effort,
    };
    let models = models();
    let mut binder = Binder::new(spawn_config(&markers, "exit:0"));
    let bound = binder.bind(BindRequest {
        role_name: "belt",
        role: &r,
        card,
        schema: &sch,
        timeout_secs: 30,
        worktree: worktree.path(),
        inputs: vec![("prompt".to_owned(), serde_json::json!("belt"))],
        activity_id_at_version: None,
        models: &models,
    });
    if let Ok(spec) = bound {
        let h = harness(binder.into_config());
        let mut store = VecStore::default();
        let _ = h.dispatch(&spec, &mut store);
    }
    markers.count()
}

/// The belt for a BOUND card a test built by literal (`command.rs`'s `AgentCard`s): the authored
/// shape is recovered from the bound one and [`belt_dispatch_authored`] runs over it.
#[must_use]
pub fn belt_dispatch_agent(card: &AgentCard) -> usize {
    let authored = AuthoredCard {
        name: card.name.clone(),
        description: card.description.clone(),
        prompt: card.prompt.clone(),
        tools: card.tools.clone(),
        cache_profile: card.cache_profile,
        permission_mode: card.permission_mode.clone(),
    };
    // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)* The bound card's model is a row key (a card a
    // test built by literal) or a row's full id (a card the binder bound); either maps back to
    // its row key, and anything else to `opus`.
    let rows = models();
    let tier = if rows.contains_key(&card.model) {
        card.model.clone()
    } else {
        rows.iter()
            .find(|(_, r)| r.id == card.model)
            .map_or_else(|| "opus".to_owned(), |(k, _)| k.clone())
    };
    let effort = card
        .effort
        .as_deref()
        .and_then(|e| serde_json::from_value(serde_json::json!(e)).ok());
    belt_dispatch_authored(&authored, &tier, effort)
}

/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.2; D21-48 X12.)* The
/// `--permission-mode` choices of a captured `claude --help` (`tests/fixtures/cli-help/`), in their
/// listed order: the quoted names between `(choices:` and the next `)` after the first
/// `--permission-mode`, read across line breaks (the help wraps the list). `None` when the capture
/// has no `--permission-mode` line or no choices list after it — Checkpoint 8h (9)'s `hchoices`,
/// restated in Rust.
#[must_use]
pub fn help_permission_mode_choices(text: &str) -> Option<Vec<String>> {
    let at = text.find("--permission-mode")?;
    let rest = &text[at..];
    let open = rest.find("(choices:")?;
    // the flag's own description runs up to the list: no `(` between them (the checkpoint's `[^(]*`)
    if rest[..open].contains('(') {
        return None;
    }
    let list = &rest[open + "(choices:".len()..];
    let close = list.find(')')?;
    let names: Vec<String> = list[..close]
        .split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect();
    if names.is_empty() { None } else { Some(names) }
}

/// The help captures under `tests/fixtures/cli-help/`, as `(version, text)`, sorted by version.
#[must_use]
pub fn help_captures() -> Vec<(String, String)> {
    let dir = fixtures().join("cli-help");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path = entry.expect("entry").path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if let Some(version) = name.strip_suffix(".permission-mode.txt") {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            out.push((version.to_owned(), text));
        }
    }
    out.sort();
    out
}

/// Whether `pid` is dead, polled for up to 5 s (250 polls of 20 ms, [`wait_until`]'s tick) — the
/// dispatch tests' counterpart of rk-containment's own `wait_dead`. On POSIX `Group::kill` is a
/// `killpg` that does not wait for its members, and an orphan the kill reached is reparented to
/// PID 1 (or the nearest subreaper) and reads alive through `kill(pid, 0)` until that process reaps
/// it, so a ZOMBIE counts as dead here: [`rk_containment::process_start_time`] reads nothing for
/// one (state `Z`). A container whose PID 1 never reaps leaves the zombie in place indefinitely;
/// it is still a killed process, which is what these assertions are about. On Windows the kill
/// through the job has already waited for every member, and `is_alive` is the reading.
#[must_use]
pub fn wait_dead(pid: u32) -> bool {
    wait_until(250, || {
        !rk_containment::is_alive(pid)
            || (cfg!(unix) && rk_containment::process_start_time(pid).is_none())
    })
}
