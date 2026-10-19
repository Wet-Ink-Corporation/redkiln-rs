//! `Config` — the adapter's configuration as a VALUE, its constructor's lints, and the typed refusal.
//!
//! (ADR-0083; ADR-0099's *"the captured environment is a value in `Config`"*; the refusal every
//! configuration-time lint in this crate answers — `s7c` §7c.4, §7c.6, §7c.8, §7c.12.)
//!
//! **What `Config` is** (§7c.12, *"the adapter's constructor, stated once"*): the pins —
//! [`TESTED_CLI_VERSION`], declared in the one form Checkpoint 7c's anchored grep reads (D17-11:
//! `claude --version` measured ~~`2.1.278 (Claude Code)` on 2026-09-19~~ `2.1.303 (Claude Code)` on
//! 2026-10-19, D21-164; ADR-0078: *"the pin is recorded with the measurement"*), never read from the process environment — the two TTLs
//! (ADR-0078 (ii); AC-210; [`CACHE_TTL`]), **`margins: Margins`** (a value handed in, never a pack
//! the adapter parses — §7c.8), the captured environment map and `harness_binary` (default `claude`;
//! the tests set the fixture child through `env!("CARGO_BIN_EXE_rk-fixture-child-claude")` at Slice
//! 5), plus the configured outcome of a CLI version mismatch ([`MismatchPolicy`]; AC-226; O-4).
//! `ClaudeHarness::new(config: Config, token: CancelToken)` is Slice 5's; `Config` is what it takes.
//!
//! **Configuration-time refusals are [`ConfigError`], never a `StepError` variant, and they never
//! cross the port** (§7c.4): ADR-0112 closes `StepError` at eleven and `Harness::dispatch` returns
//! `DispatchOutcome { record, result }` and nothing else (ADR-0099 decision 1), so a refusal that
//! happened inside `dispatch` would have no representable outcome. The path, stated: the
//! environment and margin lints run in [`Config::new`] (here); the card and argv lints run in
//! `Binder::bind` (`binder.rs`, through `command::assemble`) — both BEFORE `dispatch`, which
//! receives only a bound, validated spec. `ConfigError` is one enum for both ends because every
//! refusal is the same kind of thing: a configuration the adapter will not dispatch from.
//!
//! **The lints [`Config::new`] runs, in order, each a refusal naming what it found:**
//!
//! 1. **AC-217 / AC-488 — a denylisted name in the captured map** → [`ConfigError::DenylistedEnv`]
//!    `{ name }`. The list is [`crate::env_denylist`]'s; the check is a REFUSAL, never a strip
//!    (§7c.6: the 211-line file said "minus a denylist" as a subtraction; the register's THEN says
//!    refusal). No map carrying a listed name becomes a `Config`, so `env::child_env` — which takes
//!    a `Config` — cannot construct a child environment carrying one.
//! 2. **AC-526 — the TTL pair** → [`ConfigError::TtlPairIncomplete`] on exactly one of the two
//!    configured values, [`ConfigError::TtlPairMismatch`] on two different ones; both name both
//!    variables. The four register cases resolve as: neither → the record's [`CACHE_TTL`] on both
//!    (ADR-0078 (ii): *"the runner environment sets `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and
//!    `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h`"* — the recorded value, not a code guess; AC-210's
//!    THEN names it); first only / second only → refused; both equal → that value on both. The
//!    resolved pair is ONE string, [`Config::cache_ttl`], so a constructed child environment cannot
//!    carry two values (AC-526's AND).
//! 3. **AC-377 / AC-220 — a margin under two ticks** → [`ConfigError::MarginBelowTwoTicks`]
//!    `{ window, ticks }`, per window. [`MIN_MARGIN_TICKS`] is 2 — *"at least two instrument ticks
//!    (≥0.02 at the measured 0.01 resolution)"* (AC-220). The margin comes from the pack, never from
//!    a code default: **[`Margins`] implements no `Default`** (AC-377's *"a compile-fail or grep
//!    rule that `Margins` implements no `Default`"* is the block's derive-Default walk, re-run in
//!    `ac377_…`'s body) and both windows are required members, so a pack that names no margin, or
//!    a margin for one window, does not deserialise.
//! 4. **A batch-file program** → [`ConfigError::HarnessBinaryIsBatch`] `{ path }`: the argv cap
//!    (`command.rs`) measures the command line `std` builds for a NON-batch program; for a
//!    `.bat`/`.cmd` (the npm shim form of `claude`) `std` builds `cmd.exe /d /c "…"` instead, longer
//!    and differently quoted, and the cap would under-measure. The executable is named, not the shim.
//!
//! **Decimal margins** ([`margin_ticks_from_decimal`]; O-3, OPEN — built as DECISIONS-2026-09-17's
//! recommended default pending the operator's answer, RAISED in §7c.22): a pack margin written in
//! decimal is parsed to hundredths by **CEILING** — `0.005 → 1`, `0.01 → 1`, `0.02 → 2`, `0.011 → 2`
//! — never by rounding to nearest, because a margin is a floor and a larger margin refuses earlier,
//! the safe direction (Halt 12). **Utilisation readings are never rounded** — AC-374's ban is on the
//! reading, and that scanner lives in `headroom.rs` (Slice 5); this parser touches margins only. It
//! walks the decimal digits as bytes and holds no float: an `f64` anywhere on the margin path would
//! be the rescaling the register forbids by another name.
//!
//! **The version-mismatch policy** ([`MismatchPolicy`]; AC-226; O-4): *"the configured outcome for
//! a mismatch fires — either refusal, or a recorded `Degraded`-style allowance that the
//! configuration explicitly opted into"*. Refusal is the default in the sense O-4 fixes — *"a default
//! of `Degraded` would let a version drift pass silently"* — and it is realised here as an explicit
//! member with **no `Default`**: the composition root (7d) names the policy from the pack, and the
//! only value it may name without an opt-in is `Refuse`. The runtime arms — the child's own
//! `system/init` version read against the pin under each policy — are Slice 5's (`init.rs`).
//!
//! **The card store** ([`Config::cards`]; §7c.12; the task's `cards` member): the digest-keyed store
//! of BOUND cards `dispatch` reads the body from — *"a `Config` cannot hold a card that fails
//! [the lints]"* (§7c.4) is literal here: the only writer is `Binder::bind` (`binder.rs`), which
//! owns the `Config` while it binds and hands it back through `Binder::into_config` with every
//! card that passed the card and argv lints and none that did not. A [`Bound`] entry is the
//! [`AgentCard`] plus every schema it was bound with, by [`SchemaId`] — the card's digest is over
//! the `--agents` bytes alone (ADR-0099:61: *"a digest, never the card body"*), so one card bound
//! for two activities with two schemas is one entry carrying both, and `dispatch` reads
//! `cards[spec.card_digest].schemas[spec.schema]`; a spec whose digest or schema is not there is
//! RT-20's residual, `Transport { kind: Spawn }` before spawn (Slice 5). **How the card body
//! reaches the adapter** (this store, not `inputs`) is routed and RAISED (§7c.22).
//!
//! **The store refuses a conflict** (`Config::insert_card`, `pub(crate)`): the digest covers the
//! `--agents` bytes and NOT `cache_profile` (the adapter's routing key, `#[serde(skip)]`), so two
//! cards equal in every CLI-facing member and different in profile are one digest — and since
//! `dispatch` reads the delivery route from the stored card, a keep-first store would deliver the
//! second activity under the wrong route. A second bind of the same bytes under the other profile
//! is [`ConfigError::CardStoreConflict`], and the first entry stands.
//!
//! **What `Config` never holds**: an unbound card, a worktree (the spec's, RT-21), a
//! process-environment read. **What is never printed**: a captured VALUE — `Debug` on `Config`,
//! `ConfigValues` (and `env::ChildEnv`) renders the map's keys and a count (ADR-0105 clause 7).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_types::digest::Digest;
use rk_types::typed::SchemaId;
use serde::{Deserialize, Serialize};

use crate::card::AgentCard;
use crate::env::{PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL};
use crate::env_denylist::first_denylisted;

/// The pinned CLI version (D17-11; D21-164).
///
/// `claude --version` = ~~`2.1.278 (Claude Code)`, measured 2026-09-19 at Step 7d Slice 1 and
/// re-teed into `RUNBOOK-instrument.txt` there~~ `2.1.303 (Claude Code)`, measured 2026-10-19 on
/// the operator's box at s8j's pin move and re-written into `RUNBOOK-instrument.txt` by printf
/// there.
///
/// A `Config` value, recorded on every `dispatch_completed` row as `instrument_version` (ADR-0126
/// decision 1) and the value a child's `system/init` version is compared against (AC-226, AC-385).
/// Declared in exactly this form: Checkpoint 7c greps `pub const TESTED_CLI_VERSION: &str =
/// "<pin>"` against the re-teed file.
// Moved 2.1.275 -> 2.1.277 on 2026-09-18 (Halt 2, the second move of Step 7c): the CLI auto-updated between
// the stage-open re-tee and the first live run, whose init reported `claude_code_version: 2.1.277`, and the
// adapter refused the dispatch at the init line under MismatchPolicy::Refuse -- as designed (AC-226).
//
// Moved 2.1.277 -> 2.1.278 on 2026-09-21 (Step 7d Slice 4), completing a move HALF-TAKEN on 2026-09-19: Halt 2
// fired at Slice 1, the instrument file was re-teed to 2.1.278 and this constant was deliberately NOT moved
// with it, because `s7d` 7d.15 closed this crate and the operator's call that day was "record + re-tee, RAISE
// the adapter edit". 7d.15 was amended on 2026-09-21 to admit THIS edit and no other, and Checkpoint 7d's
// Halt 2 -- which compares the live binary, the instrument file and this constant -- is what the move unblocks.
//
// THE EVIDENCE HALF IS OWED AND IS NOT CLAIMED BY THIS LINE. The records measured at 2.1.277 (init.rs's RT-27
// shape, registry.rs's probe row, streams/README.md's shape section) keep the version they were taken at; a
// dated confirmation at 2.1.278 is appended to them once `live_dispatch_probe` has run -- one paid haiku
// dispatch the operator runs in the foreground, as its own #[ignore] reason prescribes.
//
// *(dated 2026-10-19, s8j build, D21-164)* Moved 2.1.278 -> 2.1.303 at s8j's pin move (s8h §8h.8 steps 2-3,
// s8j §8j.6 step 1). The CLI moved to 2.1.303 by a manual update after S-BIN pinned 2.1.289 (Halt 2), and
// the operator ruled that the pin is the installed 2.1.303. `claude --version` printed `2.1.303 (Claude
// Code)` on the operator's box, re-written into `RUNBOOK-instrument.txt` by printf in the same commit, which
// also carries D21-48 X12's argv spelling change (command.rs). As in 2026-09-21's move, the records measured
// at earlier versions keep them, and `live_dispatch_probe` at the new pin is OWED: one paid haiku dispatch,
// on the operator's go-ahead (s8h §8h.8 step 4).
pub const TESTED_CLI_VERSION: &str = "2.1.303";

/// The CLI version that provides `--permission-prompts` — the register's floor for AC-215.
///
/// **2.1.259 [documented]** (§7c.5: *"the pin is 2.1.277, above it, recorded beside
/// `TESTED_CLI_VERSION` in `Config`"*, quoted as §7c.5 wrote it; the pin has since moved to **2.1.278**
/// — 2026-09-21, `s7d` §7d.15 as amended — and 2.1.259 is below that too, which is what the compare below
/// asserts rather than either literal). *(dated 2026-10-19, D21-164)* The pin is now **2.1.303**, still
/// above the floor; the compare asserts it.
/// Declared beside the pin in the pin's form; the probe's tag lift at Slice 5 asserts the recorded
/// version at or above it. The `[surface unverified]` tag on the floor stands until the probe has
/// run (AC-215).
pub const PERMISSION_PROMPTS_FLOOR: &str = "2.1.259";

/// The TTL both cache variables are set to when the pack names neither — ADR-0078 (ii)'s value,
/// AC-210's THEN (`CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and**
/// `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h`).
///
/// A recorded value, not a code guess; the setting is free and reversible whichever way S-TTL1
/// falls (ADR-0078's own ground), and no run may be REPORTED as 1 h on its strength (clause (ii)).
pub const CACHE_TTL: &str = "1h";

/// The smallest margin the throttle may be configured with, in ticks (hundredths): AC-220's
/// *"at least two instrument ticks"*, AC-377's *"`margin_ticks < 2` … refused"*.
pub const MIN_MARGIN_TICKS: u16 = 2;

/// The throttle's margins, one per visible window, in integer ticks (hundredths of utilisation —
/// AC-374's unit; ADR-0110).
///
/// **No `Default`** (AC-377; RT-8; Halt 11): a margin comes from the pack, and a value that can be
/// summoned from nothing is a code margin with a different spelling (the runner's `max_five_hour:
/// 0.90` was one). Both members are required on deserialise: a pack that names no margin, or names
/// one window, is refused at the reader, before `Config::new`.
///
/// Exactly two windows (AC-375): `five_hour` and `seven_day`, the two the CLI's
/// `rate_limit_event` reports under `unifiedWindows`; the family-keyed `seven_day_opus` /
/// `seven_day_sonnet` are tolerated by the decoder and named by nothing (§7c.8).
///
/// Consumed by `throttle::Guard::check` at Slice 5 (`ticks >= 100 - margin_ticks` refuses — AC-376);
/// declared here because it is a `Config` value (§7c.8) and the derive-Default walk searches this
/// crate for it by name (§7c.19).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Margins {
    /// The margin on the `five_hour` window, in ticks.
    pub five_hour: u16,
    /// The margin on the `seven_day` window, in ticks.
    pub seven_day: u16,
}

impl Margins {
    /// The margin for `window`.
    #[must_use]
    pub const fn ticks(self, window: Window) -> u16 {
        match window {
            Window::FiveHour => self.five_hour,
            Window::SevenDay => self.seven_day,
        }
    }
}

/// The two visible windows (AC-375), named as the CLI names them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Window {
    FiveHour,
    SevenDay,
}

impl Window {
    /// The key under `unifiedWindows`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FiveHour => "five_hour",
            Self::SevenDay => "seven_day",
        }
    }
}

/// Parse a decimal margin (`"0.02"`, `"0.005"`, `"1"`) to ticks by CEILING to hundredths — O-3's
/// recommended default, for margins ONLY (readings are never rounded — AC-374).
///
/// The text is digits, optionally a point and more digits, and nothing else: no sign, no
/// exponent, no whitespace, no trailing point. The integer part contributes `int * 100`; the first
/// two fractional digits contribute tens and units of ticks; **any non-zero digit beyond the second
/// adds one tick** — `0.005 → 1`, `0.011 → 2`, `0.020 → 2`, `0.0200 → 2`. No float is read at any
/// point: the digits are walked as bytes.
///
/// # Errors
/// [`ConfigError::MarginNotDecimal`] naming the text when it is not a decimal of that shape, or
/// when the ticks would not fit a `u16` (a margin over 655 whole units is not a margin).
pub fn margin_ticks_from_decimal(text: &str) -> Result<u16, ConfigError> {
    let refuse = || ConfigError::MarginNotDecimal {
        text: text.to_owned(),
    };
    let (int, frac) = match text.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (text, None),
    };
    let all_digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if !all_digits(int) || frac.is_some_and(|f| !all_digits(f)) {
        return Err(refuse());
    }
    // Every step is checked: a text whose ticks do not fit is REFUSED, never a debug-build panic
    // on the addition after a multiplication that happened to fit (`42949672.99` was one).
    let whole: u32 = int.parse().map_err(|_| refuse())?;
    let frac = frac.unwrap_or("");
    let digit = |i: usize| frac.as_bytes().get(i).map_or(0, |b| u32::from(b - b'0'));
    // CEILING: any non-zero digit past the hundredths adds one tick (O-3: a margin is a floor).
    let carry = u32::from(frac.bytes().skip(2).any(|b| b != b'0'));
    let ticks = whole
        .checked_mul(100)
        .and_then(|t| t.checked_add(digit(0) * 10 + digit(1)))
        .and_then(|t| t.checked_add(carry))
        .ok_or_else(refuse)?;
    u16::try_from(ticks).map_err(|_| refuse())
}

/// The configured outcome of a CLI version mismatch (AC-226; O-4): refusal, or an explicitly
/// opted-in `Degraded` allowance.
///
/// **No `Default`** — the composition root names it; refusal is what it names unless the pack opted
/// in (O-4: a default of `Degraded` would let a version drift pass silently, which AC-385's warning
/// exists to prevent). Wire form: `refuse` / `degraded`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchPolicy {
    /// The dispatch is refused: the child is killed through the group and the row carries
    /// `Observed(v)` with `Transport { kind: Decode }` (RT-20 as amended; Slice 5's `init.rs`).
    Refuse,
    /// The dispatch proceeds and every cache metric taken in the run carries the mismatch warning
    /// (AC-385's `mismatch_warning(&record)`, Slice 5).
    Degraded,
}

/// What the composition root hands [`Config::new`]: every configuration value, unvalidated, as the
/// pack reader and the one environment capture at the edge produced them (ADR-0083).
///
/// `Config::new` is the only way to a `Config`, so no unvalidated value is ever read by a builder.
/// `Debug` is hand-written: the captured map renders as its KEYS, never a value (the module doc's
/// *"what is never printed"*).
#[derive(Clone, PartialEq, Eq)]
pub struct ConfigValues {
    /// The parent environment, captured ONCE at the composition root and handed in as a value —
    /// the adapter never reads the process environment (AC-217's AND).
    pub captured_env: BTreeMap<String, String>,
    /// The program to spawn: `claude` for the probe and for 7d's root; the fixture child in tests.
    pub harness_binary: PathBuf,
    /// The pack's `CLAUDE_CODE_PROMPT_CACHE_TTL`, if it names one (AC-526's first).
    pub prompt_cache_ttl: Option<String>,
    /// The pack's `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, if it names one (AC-526's second).
    pub subagent_prompt_cache_ttl: Option<String>,
    /// The pack's margins, in ticks (AC-377: never a code default).
    pub margins: Margins,
    /// The pack's configured outcome for a CLI version mismatch (AC-226; O-4).
    pub on_version_mismatch: MismatchPolicy,
}

/// One entry of the card store: a bound card and every output schema it was bound with, by id
/// (the module doc's *"the card store"*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    /// The bound card — the authored card with the role's tier injected (`binder.rs`).
    pub card: AgentCard,
    /// The schemas this card was bound for, by the id the spec carries: the `--json-schema`
    /// element and Slice 5's result validation read the JSON here.
    pub schemas: BTreeMap<SchemaId, serde_json::Value>,
}

/// The validated configuration — constructible only through [`Config::new`], whose lints every
/// value below has passed, and whose card store only `Binder::bind` fills.
///
/// Fields are private so a `Config` cannot be assembled around them. `Debug` is hand-written:
/// the captured map renders as its KEYS and a count, never a value — the map is the parent
/// environment minus a denylist, and a proxy URL with an embedded credential or a token the
/// denylist does not name would otherwise print from any `{config:?}` (ADR-0105 clause 7: the
/// child environment is where credentials live).
#[derive(Clone, PartialEq, Eq)]
pub struct Config {
    captured_env: BTreeMap<String, String>,
    harness_binary: PathBuf,
    cache_ttl: String,
    margins: Margins,
    on_version_mismatch: MismatchPolicy,
    tested_cli_version: String,
    cards: BTreeMap<Digest, Bound>,
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY.)*
    /// The caller-supplied directory each dispatch writes its child-settings file into; `None`
    /// until [`Config::with_settings_dir`] sets it, and then every bind refuses.
    settings_dir: Option<PathBuf>,
}

/// The captured map as `Debug` renders it in this crate: `{ keys: [..], count: n }` — the names,
/// never the values.
pub(crate) struct EnvKeys<'a>(pub(crate) &'a BTreeMap<String, String>);

impl std::fmt::Debug for EnvKeys<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapturedEnv")
            .field("keys", &self.0.keys().collect::<Vec<_>>())
            .field("count", &self.0.len())
            .finish()
    }
}

impl std::fmt::Debug for ConfigValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigValues")
            .field("captured_env", &EnvKeys(&self.captured_env))
            .field("harness_binary", &self.harness_binary)
            .field("prompt_cache_ttl", &self.prompt_cache_ttl)
            .field("subagent_prompt_cache_ttl", &self.subagent_prompt_cache_ttl)
            .field("margins", &self.margins)
            .field("on_version_mismatch", &self.on_version_mismatch)
            .finish()
    }
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("captured_env", &EnvKeys(&self.captured_env))
            .field("harness_binary", &self.harness_binary)
            .field("cache_ttl", &self.cache_ttl)
            .field("margins", &self.margins)
            .field("on_version_mismatch", &self.on_version_mismatch)
            .field("tested_cli_version", &self.tested_cli_version)
            .field("cards", &self.cards.keys().collect::<Vec<_>>())
            .field("settings_dir", &self.settings_dir)
            .finish()
    }
}

/// Why a configuration was refused — by [`Config::new`] (the environment and margin lints) or by
/// `Binder::bind` / `command::assemble` (the card and argv lints).
///
/// **Never a `StepError` variant and never a return of `dispatch`** (§7c.4; ADR-0112). Every
/// variant names what was found: the variable, the card, the tool, the window, the measure.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigError {
    /// AC-217 / AC-488: a name on the committed denylist is present in the captured environment.
    /// Refused, never stripped.
    #[error(
        "{name} is present in the captured environment and is on the committed denylist \
         (rk-adapter-claude/src/env_denylist.rs; AC-217, AC-488): the adapter refuses to construct \
         — it is never stripped and passed through; unset it outside the process"
    )]
    DenylistedEnv { name: String },
    /// AC-526: exactly one of the two TTL variables was configured. Names both.
    #[error(
        "{present} is configured without {absent}: the TTL pair is set together or not at all \
         (AC-526; ADR-0078 (ii)) — {} and {} must carry one value",
        PROMPT_CACHE_TTL,
        SUBAGENT_PROMPT_CACHE_TTL
    )]
    TtlPairIncomplete {
        present: &'static str,
        absent: &'static str,
    },
    /// AC-526: the two TTL variables were configured with different values. Names both.
    #[error(
        "{}={prompt} and {}={subagent} disagree: the TTL pair carries ONE value (AC-526; ADR-0078 (ii))",
        PROMPT_CACHE_TTL,
        SUBAGENT_PROMPT_CACHE_TTL
    )]
    TtlPairMismatch { prompt: String, subagent: String },
    /// AC-377 / AC-220: a window's margin is under two ticks.
    #[error(
        "the {} margin is {ticks} tick(s); a margin is at least {MIN_MARGIN_TICKS} instrument ticks \
         (AC-220, AC-377) and comes from the pack, never a code default",
        window.as_str()
    )]
    MarginBelowTwoTicks { window: Window, ticks: u16 },
    /// A decimal margin that is not a decimal of the accepted shape.
    #[error(
        "{text:?} is not a decimal margin (digits, optionally a point and digits; no sign, exponent \
         or whitespace) — a margin that cannot be read is refused, never read as a number"
    )]
    MarginNotDecimal { text: String },
    /// The card file does not parse as the closed card record (a member it does not declare, a
    /// malformed value).
    #[error("card {card}: the card file is not a card record: {detail}")]
    CardFile { card: String, detail: String },
    /// AC-493 (PIA-060): the card carries no `tools` key — an absent list inherits everything.
    #[error(
        "card {card}: no `tools` key — the pre-spawn tool set is the card's declared list and \
         nothing else, never \"inherit everything\" (AC-493; ADR-0108 d2)"
    )]
    ToolsAbsent { card: String },
    /// AC-493: the card's `tools` is empty, or names an empty tool.
    #[error(
        "card {card}: `tools` is empty (or names an empty tool) — an empty allowlist fails open \
         (AC-493; L4.4)"
    )]
    ToolsEmpty { card: String },
    /// AC-383: the card declares no `cache_profile`.
    #[error(
        "card {card}: no `cache_profile` — a card declares `static` or `default` (AC-383); the \
         profile selects the delivery route and there is no default"
    )]
    CacheProfileAbsent { card: String },
    /// AC-214: the card is silent on `permission_mode`.
    #[error(
        "card {card}: no `permission_mode` — `--permission-mode` takes an explicit value from the \
         card and there is no implicit default (AC-214)"
    )]
    PermissionModeAbsent { card: String },
    /// AC-118 / AC-418: an AUTHORED card carries `model` or `effort` — the tier is the role's.
    #[error(
        "card {card}: the authored card carries `{member}` — the tier is owned by the role and \
         injected by the binder; an authored card never carries it (AC-118, AC-418)"
    )]
    TierLeak { card: String, member: String },
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.2; ADR-0197
    /// decision 2, proposed.)* The card's `permission_mode` is outside the CLI's own closed set
    /// (`card::PERMISSION_MODES`: the `--help` choices plus `default`). A misspelling is refused.
    #[error(
        "card {card}: permission_mode `{mode}` is outside the CLI's closed set (the `claude --help` \
         choices plus `default`; s8h §8h.2) — a misspelt mode is refused, never passed through"
    )]
    PermissionModeOutsideSet { card: String, mode: String },
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.3; ADR-0197
    /// decision 4, proposed.)* The role names a model key the pack's `[models]` table lacks.
    #[error(
        "card {card}: role {role} names model `{key}`, which the pack's [models] table does not \
         declare — the full id has no source (s8h §8h.3)"
    )]
    ModelKeyAbsent {
        card: String,
        role: String,
        key: String,
    },
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.2; D21-48 X12;
    /// ADR-0197 decision 3, proposed.)* The `Config`'s pin has no row in
    /// `command::PERMISSION_MODE_ARGV`, so the argv spelling of a card's `default` is unknown.
    /// There is no fallback to the card's spelling.
    #[error(
        "the CLI pin {version} has no row in PERMISSION_MODE_ARGV — the argv spelling of a card's \
         `default` is unknown at this pin, and the card's own spelling is never used in its place \
         (s8h §8h.2; D21-48 X12)"
    )]
    NoPermissionModeSpelling { version: String },
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.6.)* The child's
    /// confinement settings cannot be assembled from the worktree (a relative or non-drive path, or
    /// an ancestor level that cannot be listed).
    #[error("the child's confinement settings cannot be assembled (s8h §8h.6): {detail}")]
    Confinement { detail: String },
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY.)*
    /// No settings directory was supplied: the child's settings have nowhere to be written, and
    /// nothing falls back to an argv without them.
    #[error(
        "no settings directory was supplied for the child's settings file — the caller supplies \
         one outside the worktree, and no dispatch runs without its settings (s8h §8h.6; D21-137)"
    )]
    SettingsDirUnset,
    /// The settings directory does not exist (D21-137).
    #[error(
        "the settings directory {} does not exist — the caller creates it before the first \
         dispatch (s8h §8h.6; D21-137)",
        dir.display()
    )]
    SettingsDirMissing { dir: PathBuf },
    /// The settings directory lies under the worktree, where the child could reach its own fence
    /// (D21-137: compared with `\\?\` stripped, case-folded on Windows).
    #[error(
        "the settings directory {} lies under the worktree {} — the child's settings are written \
         outside the worktree (s8h §8h.6; D21-137)",
        dir.display(),
        worktree.display()
    )]
    SettingsDirUnderWorktree { dir: PathBuf, worktree: PathBuf },
    /// The settings path is not an absolute, non-verbatim UTF-8 path (D21-137).
    #[error(
        "the settings path {} is refused: {why} (s8h §8h.6; D21-137)",
        path.display()
    )]
    SettingsPathRefused { path: PathBuf, why: &'static str },
    /// The settings JSON is over the CLI's settings-file limit (D21-137).
    #[error(
        "the child's settings measure {bytes} bytes, over the CLI's {cap}-byte settings-file limit \
         (`Kp`, found by string search, not executed) — refused before spawn (s8h §8h.6; D21-137)"
    )]
    SettingsTooLarge { bytes: usize, cap: usize },
    /// The settings file could not be written (D21-137): a name that already exists (`create_new`),
    /// or any I/O failure. Nothing is spawned without the file.
    #[error(
        "the child's settings file {} could not be written: {detail} — nothing was spawned \
         (s8h §8h.6; D21-137)",
        path.display()
    )]
    SettingsWrite { path: PathBuf, detail: String },
    /// AC-269: a tool on the card exposes the answer, mint or resolve path.
    #[error(
        "card {card}: tool `{tool}` exposes the answer API — no card exposes the answer, mint or \
         resolve path (AC-269; ADR-0009)"
    )]
    AnswerApiExposed { card: String, tool: String },
    /// AC-205 / AC-417: a schema-bearing dispatch whose card's `tools` omit `StructuredOutput`.
    #[error(
        "card {card}: `tools` omits StructuredOutput on a dispatch that carries a JSON schema — the \
         allowlist would silently void --json-schema at exit 0 (AC-205; F20; ADR-0108)"
    )]
    StructuredOutputAbsent { card: String },
    /// AC-212 (b): the role declares no model.
    #[error(
        "card {card}: role {role} declares no model — the tier is traceable only through the role \
         (AC-212; AC-118)"
    )]
    ModelUntraceable { card: String, role: String },
    /// AC-212 (a): the role declares no effort on a tier that takes one.
    #[error(
        "card {card}: role {role} declares no effort for tier {tier}, which takes one — the effort \
         is traceable only through the role (AC-212)"
    )]
    EffortUntraceable {
        card: String,
        role: String,
        tier: String,
    },
    /// The role declares an effort on a tier that rejects the parameter (haiku).
    #[error(
        "card {card}: role {role} declares an effort for tier {tier}, which rejects the parameter \
         — a spec error, not a runtime one"
    )]
    EffortRefusedByTier {
        card: String,
        role: String,
        tier: String,
    },
    /// AC-203: `--agents` without `--agent` — the card would be inert for the main turn.
    #[error(
        "card {card}: --agents without --agent — `--agents` alone only defines the card and the \
         main turn does not adopt it (AC-203; F7)"
    )]
    AgentsWithoutAgent { card: String },
    /// AC-132: a strict user-defined tool on the command line re-opens the per-request budget.
    #[error(
        "strict tool `{tool}` on the command line — the schema budget is per request across all \
         strict tool schemas plus the output schema, and the lint precondition is one output schema \
         and no strict tool (AC-132; ADR-0108 d4)"
    )]
    StrictToolOnArgv { tool: String },
    /// AC-523's cap (RT-1): the command line measures at or over the cap.
    #[error(
        "the command line measures {units} UTF-16 units, at or over the {cap}-unit cap \
         (CreateProcessW's documented limit; RT-1; decision #18 of 2026-09-11) — refused before spawn"
    )]
    ArgvAtOrOverCap { units: usize, cap: usize },
    /// RT-21: the worktree cannot be put on the spec (not UTF-8, or the activity's inputs already
    /// carry the key).
    #[error("the worktree cannot be placed on the spec's inputs (RT-21): {detail}")]
    Worktree { detail: String },
    /// The card has no name — the `--agent` element and the `--agents` key would be empty.
    #[error(
        "the card has no name: the `--agent` element and the `--agents` key are the name (AC-203)"
    )]
    CardNameEmpty,
    /// The card store already holds a DIFFERENT bound card under this digest: the same `--agents`
    /// bytes bound under the other `cache_profile` (the profile is not in the digested bytes). A
    /// silent keep-first would deliver the second activity under the wrong route (AC-383).
    #[error(
        "card {card}: the card store already holds a different bound card under digest {} — the \
         same --agents bytes under the other cache_profile; the delivery route is the card's \
         (AC-383) and a store cannot hold two routes under one digest",
        digest.as_str()
    )]
    CardStoreConflict { card: String, digest: Digest },
    /// The configured program is a batch file (`.bat` / `.cmd`): `std` would spawn it through
    /// `cmd.exe /d /c "…"` with a different command line from the one the argv cap measures
    /// (`command::command_line_units` re-states the NON-batch `make_command_line`), so the cap
    /// would under-measure. Name the executable (`claude.exe`, not the npm shim).
    #[error(
        "harness binary {} is a batch file (.bat/.cmd): std spawns those through `cmd.exe /d /c`          with a longer, differently quoted command line than the argv cap measures — name the          executable itself",
        path.display()
    )]
    HarnessBinaryIsBatch { path: PathBuf },
    /// The adapter's own `multi_thread` runtime could not be built (`ClaudeHarness::new`; ADR-0076,
    /// RT-15) — a construction failure, before any dispatch.
    #[error("the adapter's tokio runtime could not be built: {detail}")]
    Runtime { detail: String },
    /// The shipped capability registry does not load (`registry.rs`; AC-489) — a build defect
    /// surfaced at construction, before any dispatch.
    #[error("the shipped capability registry does not load: {detail}")]
    Registry { detail: String },
}

impl Config {
    /// Validate `values` into a `Config` — the environment and margin lints (§7c.4), in the order
    /// the module doc states.
    ///
    /// # Errors
    /// [`ConfigError::DenylistedEnv`] for the first denylisted name in the captured map (AC-217,
    /// AC-488); [`ConfigError::TtlPairIncomplete`] / [`ConfigError::TtlPairMismatch`] for a TTL
    /// pair that is not one value on both (AC-526); [`ConfigError::MarginBelowTwoTicks`] for a
    /// window under two ticks (AC-377, AC-220); [`ConfigError::HarnessBinaryIsBatch`] for a
    /// `.bat`/`.cmd` program (the argv cap measures the non-batch command line — `command.rs`).
    pub fn new(values: ConfigValues) -> Result<Self, ConfigError> {
        if let Some(name) = first_denylisted(&values.captured_env) {
            return Err(ConfigError::DenylistedEnv {
                name: name.to_owned(),
            });
        }
        let cache_ttl = match (values.prompt_cache_ttl, values.subagent_prompt_cache_ttl) {
            (None, None) => CACHE_TTL.to_owned(),
            (Some(_), None) => {
                return Err(ConfigError::TtlPairIncomplete {
                    present: PROMPT_CACHE_TTL,
                    absent: SUBAGENT_PROMPT_CACHE_TTL,
                });
            }
            (None, Some(_)) => {
                return Err(ConfigError::TtlPairIncomplete {
                    present: SUBAGENT_PROMPT_CACHE_TTL,
                    absent: PROMPT_CACHE_TTL,
                });
            }
            (Some(prompt), Some(subagent)) => {
                if prompt != subagent {
                    return Err(ConfigError::TtlPairMismatch { prompt, subagent });
                }
                prompt
            }
        };
        for window in [Window::FiveHour, Window::SevenDay] {
            let ticks = values.margins.ticks(window);
            if ticks < MIN_MARGIN_TICKS {
                return Err(ConfigError::MarginBelowTwoTicks { window, ticks });
            }
        }
        // The argv cap's precondition (`command::command_line_units`): `std` spawns a `.bat`/`.cmd`
        // through `cmd.exe /d /c "…"` on a command line the measure does not state.
        if values
            .harness_binary
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("bat") || e.eq_ignore_ascii_case("cmd"))
        {
            return Err(ConfigError::HarnessBinaryIsBatch {
                path: values.harness_binary,
            });
        }
        Ok(Self {
            captured_env: values.captured_env,
            harness_binary: values.harness_binary,
            cache_ttl,
            margins: values.margins,
            on_version_mismatch: values.on_version_mismatch,
            tested_cli_version: TESTED_CLI_VERSION.to_owned(),
            cards: BTreeMap::new(),
            settings_dir: None,
        })
    }

    /// The captured environment, as handed in — with no denylisted name on it.
    #[must_use]
    pub const fn captured_env(&self) -> &BTreeMap<String, String> {
        &self.captured_env
    }

    /// The program every dispatch spawns.
    #[must_use]
    pub fn harness_binary(&self) -> &Path {
        &self.harness_binary
    }

    /// The ONE TTL value both cache variables are set to (AC-210; AC-526's AND).
    #[must_use]
    pub fn cache_ttl(&self) -> &str {
        &self.cache_ttl
    }

    /// The throttle's margins, both windows at or above [`MIN_MARGIN_TICKS`].
    #[must_use]
    pub const fn margins(&self) -> Margins {
        self.margins
    }

    /// The configured outcome of a CLI version mismatch (AC-226).
    #[must_use]
    pub const fn on_version_mismatch(&self) -> MismatchPolicy {
        self.on_version_mismatch
    }

    /// The pin, as a `Config` value ([`TESTED_CLI_VERSION`]) — the row's `instrument_version`.
    #[must_use]
    pub fn tested_cli_version(&self) -> &str {
        &self.tested_cli_version
    }

    /// This `Config` at another pin.
    ///
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.2; D21-48 X12.)*
    /// The argv spelling lookup is keyed by [`Config::tested_cli_version`], never by
    /// [`TESTED_CLI_VERSION`] directly, *"so a test can build a `Config` at another pin"*. The
    /// composition root never calls this: `Config::new` fills the pin from the constant.
    #[must_use]
    pub fn at_pin(mut self, version: &str) -> Self {
        version.clone_into(&mut self.tested_cli_version);
        self
    }

    /// This `Config` with the directory each dispatch writes its child-settings file into.
    ///
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; `s8h` §8h.6; D21-137, TAKEN
    /// PROVISIONALLY.)* The location is an input: the adapter never runs git to find it and has
    /// no default. `rk-cli`'s `run` passes a run directory outside the worktree, created before
    /// the first bind; a test passes a temp directory. It is checked at every bind and every
    /// dispatch (`command::check_settings_dir`).
    #[must_use]
    pub fn with_settings_dir(mut self, dir: PathBuf) -> Self {
        self.settings_dir = Some(dir);
        self
    }

    /// The settings directory, or `None` when none was supplied (every bind then refuses).
    #[must_use]
    pub fn settings_dir(&self) -> Option<&Path> {
        self.settings_dir.as_deref()
    }

    /// The card store: every bound card, by the digest the spec carries (the module doc).
    #[must_use]
    pub const fn cards(&self) -> &BTreeMap<Digest, Bound> {
        &self.cards
    }

    /// The bound card and its schemas under `digest` — what `dispatch` reads by
    /// `spec.card_digest`; `None` is RT-20's residual.
    #[must_use]
    pub fn bound(&self, digest: &Digest) -> Option<&Bound> {
        self.cards.get(digest)
    }

    /// The one writer of the card store — `Binder::bind`'s, after every lint has passed. Inserting
    /// the same card again adds the schema and changes nothing else; a DIFFERENT card under the
    /// same digest — the same `--agents` bytes under the other `cache_profile`, which the digest
    /// does not cover — is refused and the entry is left as it was.
    ///
    /// # Errors
    /// [`ConfigError::CardStoreConflict`] naming the card and the digest.
    pub(crate) fn insert_card(
        &mut self,
        card: AgentCard,
        id: SchemaId,
        json: serde_json::Value,
    ) -> Result<(), ConfigError> {
        let digest = card.digest();
        match self.cards.get_mut(&digest) {
            Some(entry) if entry.card != card => Err(ConfigError::CardStoreConflict {
                card: card.name,
                digest,
            }),
            Some(entry) => {
                entry.schemas.insert(id, json);
                Ok(())
            }
            None => {
                self.cards.insert(
                    digest,
                    Bound {
                        card,
                        schemas: BTreeMap::from([(id, json)]),
                    },
                );
                Ok(())
            }
        }
    }
}
