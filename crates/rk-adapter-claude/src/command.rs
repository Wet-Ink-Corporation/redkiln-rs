//! The command builder — ONE builder assembles every argv, and the lints that read it share it.
//!
//! (`s7c` §7c.5; ADR-0108 decision 4: *"the two lints share the command builder"*; AC-523's fourth
//! instrument, the argv cap, sits on the same path.)
//!
//! **The golden order, and where it comes from.** The vector has ONE element order and its source
//! is the lifted `build_args` in `runner/src/runner.rs` at `61f590e5…` (§7c.2; TD3-2: the lifted
//! order IS the golden order), which pushed `--strict-mcp-config` **after** `--permission-mode` and
//! `extra_flags` last [measured 2026-09-17]:
//!
//! ```text
//! -p --output-format stream-json --verbose --model <tier> [--effort <e>]
//! --agents <json> --agent <name> --json-schema <schema> --permission-mode <m>
//! --strict-mcp-config --permission-prompts none [--system-prompt <card body>]
//! ```
//!
//! The first six elements and the element set are CORPUS-MEASURED: `results/h1-gap.jsonl`'s six
//! dispatch rows (F28's `default` arm) all carry `["-p", "--output-format", "stream-json",
//! "--verbose", "--model", "haiku", "--strict-mcp-config"]`, and `results/h1-sysprompt.jsonl`'s
//! two carry the same followed by `"--system-prompt", <45 bytes>` — LAST, where its `extra_flags`
//! put it (§7c.5; the extracted rows are under `tests/fixtures/recorded/` with PROVENANCE). The
//! authored elements — `--agents`/`--agent` (AC-203), `--json-schema` (AC-205), `--permission-mode`
//! (AC-214), `--permission-prompts none` (AC-215) — are each warranted by a criterion and were NOT
//! measured in that combination at the pin; the one recorded card-dispatch argv is F20's
//! (`--agents … --agent … --json-schema … --permission-mode bypassPermissions --strict-mcp-config`,
//! no `--system-prompt`). **The live probe (§7c.15) is the first measurement of the full vector.**
//! `--allowed-tools` (the runner's optional element) is not emitted: the pre-spawn tool set is the
//! card's declared `tools` list inside the `--agents` object and nothing else (ADR-0108 d2).
//!
//! **Per-profile delivery** (AC-383; AC-211 PROVISIONAL on S-H1c; decision #19 of 2026-09-11):
//! under `static` the card body is `--system-prompt <body>`, appended last, AND the card travels
//! as `--agents`/`--agent` for adoption — **both are passed** (O-5, OPEN: the combination is
//! unmeasured at the pin; the probe measures it and a refusal by the CLI is a Halt, not a
//! re-tune). Under `default` `--system-prompt` is omitted and the body is prepended to the user
//! message on stdin (F28's measured arm: 18,603 cache-creation flat at 65.2 min and 49.7 h), the
//! body and the message separated by one blank line — the separator is authored, not measured.
//! Neither append flag is ever emitted (AC-211: the append route is S-H1c, unrun; the block bans
//! both tokens over `src/`). **The `--system-prompt` bytes are the card body and nothing else**
//! (AC-209): no volatile input reaches any argv element — the user message is stdin — so two
//! dispatches of one card version hash the same system prompt whatever their inputs.
//!
//! **The lints, on this path, each a [`ConfigError`] before spawn** (§7c.4):
//!
//! - **AC-203** — `--agents` without `--agent` is refused ([`ConfigError::AgentsWithoutAgent`]):
//!   F7 measured that `--agents` alone leaves the card inert for the main turn. The binder always
//!   adopts; [`CommandSpec::adopt_agent`] exists so the register's *"fixture spec that must fail"*
//!   is a spec, not a comment.
//! - **AC-205 / AC-417** — a schema-bearing dispatch whose card's `tools` omit `StructuredOutput`
//!   is refused ([`ConfigError::StructuredOutputAbsent`]); every dispatch here carries a schema
//!   (`ActivitySpec.schema` is not optional). F20 is why this is configuration-time: the allowlist
//!   silently voids `--json-schema` and the process exits 0 reporting success (Halt 3).
//! - **AC-132** — the per-request budget precondition: ONE output schema (the builder takes exactly
//!   one, by type) and NO strict user-defined tool on the command line; a spec that adds one is
//!   refused ([`ConfigError::StrictToolOnArgv`]) — re-opening the budget is a record's decision,
//!   not a builder flag.
//! - **The card-level lints, again, over the bound card** (`card::lint_card`; §7c.12: the binder
//!   runs *"every card-level and argv-level lint of §7c.4 at bind time"*): an empty name, an empty
//!   tool list or a blank tool (AC-493), a tool on the answer path (AC-269), a blank
//!   `permission_mode` (AC-214) — refused here whatever built the card, since an `AgentCard` is a
//!   `pub` struct a caller may assemble by literal.
//! - **AC-214** — `--permission-mode` is unconditional, from the card's explicit value; a silent
//!   card is refused at load (`AuthoredCard::from_file`) and a blank one here. The lint half — that
//!   the push sits outside every guard — is ARMED (§7c.5): read this function.
//! - **AC-215** (argv half) — `--permission-prompts none`, unconditional. The stdin write-then-
//!   shutdown is Slice 5's (`runner.rs`).
//! - **AC-216 / AC-511** — the bare flag on no input: there is no code path that pushes it; the
//!   matrix property test is the instrument, the block's `rk_absent` the cheap second line (the
//!   token itself is spelt only in the lint control, so the `src/` tree carries it nowhere).
//! - **AC-523's cap (RT-1)** — the command line measures at or over [`ARGV_CAP_BYTES`] →
//!   [`ConfigError::ArgvAtOrOverCap`]. **The measure** ([`command_line_units`]) is the command
//!   line `std::process::Command` builds for `CreateProcessW` on Windows, in UTF-16 units: the
//!   program in quotes, then for each element a space and the element quoted and escaped by
//!   MSVCRT's rules (quoted when empty or carrying a space or tab; each `"` escaped with a
//!   backslash, backslashes before a `"` doubled, trailing backslashes doubled when quoted), then
//!   the terminating NUL — `std`'s `make_command_line` / `append_arg`, re-stated. The constant is
//!   named as RT-1 spells it, in "bytes", and the documented limit is 32,767 *characters* including
//!   the null: for ASCII the two coincide, and the measure counts what the limit counts — a
//!   non-ASCII prompt measures FEWER units than bytes, which is the unit the limit is in (RAISED
//!   beside RT-1: the owed record should name the unit). A vector that measures exactly the cap is
//!   refused — one unit stricter than the documented maximum, the safe direction; one under is not.
//!   **Precondition: the program is not a batch file.** For a `.bat`/`.cmd` `std` builds
//!   `cmd.exe /d /c "<resolved path> …"` (`make_bat_command_line`) — longer, quoted differently —
//!   and this measure would under-count; `Config::new` refuses such a `harness_binary`
//!   (`ConfigError::HarnessBinaryIsBatch`), so every program this measure sees is the executable.
//!   The same measure applies on every platform (a POSIX `ARG_MAX` is megabytes; the cap is the
//!   Windows floor, routed — a record naming the value is OWED). `--system-prompt-file` is S-SPF,
//!   registered and unused (decision #18).
//!
//! **What this module does not do**: spawn (Slice 5's `runner.rs`, through `rk-containment`),
//! read the environment (the child map is `env.rs`'s, over the `Config`), or read `system/init`
//! (AC-202: no isolation assertion reads the adapter's own argv — this file is not in the
//! `init|isolation` module filter and nothing in those modules names `argv`).

use std::path::{Path, PathBuf};

use crate::card::{AgentCard, CacheProfile, lint_card};
use crate::config::{Config, ConfigError};

/// The argv cap (RT-1; decision #18 of 2026-09-11): Windows `CreateProcessW`'s documented
/// command-line limit, 32,767 characters including the terminating null.
///
/// A command line that measures AT or over it is refused before spawn; the fixture at the cap is
/// `tests/fixtures/argv/at-the-cap.card.json`. Routed, not decided — a record naming the value is
/// OWED (§7c.1).
pub const ARGV_CAP_BYTES: usize = 32_767;

/// The tool a schema-bearing dispatch's card must declare (AC-205; ADR-0108).
pub const STRUCTURED_OUTPUT: &str = "StructuredOutput";

/// What one assembly takes.
///
/// The bound card, the ONE output schema, adoption, the strict tools a spec would add (always
/// empty from the binder; the AC-132 fixture adds one), and the user message (stdin — never argv).
#[derive(Debug, Clone, Copy)]
pub struct CommandSpec<'a> {
    /// The bound card (the role's tier injected — `binder.rs`).
    pub card: &'a AgentCard,
    /// The one output schema, as JSON; the `--json-schema` element is its compact serialisation.
    pub schema: &'a serde_json::Value,
    /// `--agent <name>` beside `--agents` — `false` is AC-203's refusal.
    pub adopt_agent: bool,
    /// Strict user-defined tools a spec would put on the command line — any is AC-132's refusal.
    pub strict_tools: &'a [String],
    /// The user message, written to the child's stdin then shut (Slice 5). Under `default` the
    /// card body is prepended to it here.
    pub user_message: &'a str,
}

/// What the builder produced: the program, the argv (the golden vector), the stdin payload, and
/// the command line's measure against the cap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assembled {
    /// `Config::harness_binary`.
    pub program: PathBuf,
    /// The argv, in the ONE stated order.
    pub argv: Vec<String>,
    /// The bytes written to stdin before it is shut: the user message, with the card body at its
    /// head under `default`.
    pub stdin: String,
    /// [`command_line_units`] of `program` + `argv` — under the cap, or this value would not exist.
    pub command_line_units: usize,
}

/// The UTF-16 length of one element as `std` quotes it for `CreateProcessW` (MSVCRT rules).
fn quoted_units(arg: &str) -> usize {
    let quote = arg.is_empty() || arg.contains(' ') || arg.contains('\t');
    let mut units = if quote { 2 } else { 0 };
    let mut backslashes = 0usize;
    for c in arg.chars() {
        if c == '\\' {
            backslashes += 1;
        } else {
            if c == '"' {
                units += backslashes + 1;
            }
            backslashes = 0;
        }
        units += c.len_utf16();
    }
    if quote {
        units += backslashes;
    }
    units
}

/// The command line `std::process::Command` would build for `program` + `argv` on Windows, in
/// UTF-16 units.
///
/// The quoted program, a space and the quoted-and-escaped element for each element, and the
/// terminating null — what `CreateProcessW`'s 32,767 limit counts (the module doc).
///
/// **For a non-batch program only**: a `.bat`/`.cmd` is spawned by `std` through `cmd.exe /d /c`
/// on a different, longer line; `Config::new` refuses such a program so this measure is never
/// asked about one.
#[must_use]
pub fn command_line_units(program: &Path, argv: &[String]) -> usize {
    let program: usize = program.to_string_lossy().chars().map(char::len_utf16).sum();
    2 + program + argv.iter().map(|a| 1 + quoted_units(a)).sum::<usize>() + 1
}

/// Assemble the command: the golden argv in the ONE stated order, the stdin payload per profile,
/// and every argv-level lint of §7c.4 — refused before any spawn.
///
/// # Errors
/// The card-level lints over the bound card (`card::lint_card`: [`ConfigError::CardNameEmpty`],
/// [`ConfigError::ToolsEmpty`], [`ConfigError::AnswerApiExposed`],
/// [`ConfigError::PermissionModeAbsent`]); then [`ConfigError::AgentsWithoutAgent`] (AC-203);
/// [`ConfigError::StructuredOutputAbsent`] (AC-205); [`ConfigError::StrictToolOnArgv`] (AC-132);
/// [`ConfigError::ArgvAtOrOverCap`] (AC-523's cap).
pub fn assemble(config: &Config, spec: &CommandSpec<'_>) -> Result<Assembled, ConfigError> {
    let card = spec.card;
    // The card-level lints, over the BOUND card, whatever built it (§7c.12).
    lint_card(&card.name, &card.tools, &card.permission_mode)?;
    // AC-203: `--agents` alone defines; only `--agent` adopts (F7).
    if !spec.adopt_agent {
        return Err(ConfigError::AgentsWithoutAgent {
            card: card.name.clone(),
        });
    }
    // AC-205: every dispatch here carries a schema, so the declared list must carry the tool.
    if !card.tools.iter().any(|t| t == STRUCTURED_OUTPUT) {
        return Err(ConfigError::StructuredOutputAbsent {
            card: card.name.clone(),
        });
    }
    // AC-132: no strict user-defined tool on the command line.
    if let Some(tool) = spec.strict_tools.first() {
        return Err(ConfigError::StrictToolOnArgv { tool: tool.clone() });
    }

    let mut argv: Vec<String> = vec![
        "-p".to_owned(),
        "--output-format".to_owned(),
        "stream-json".to_owned(),
        "--verbose".to_owned(),
        "--model".to_owned(),
        card.model.clone(),
    ];
    // Present exactly when the bound card carries one (the binder puts it there only on a tier
    // that accepts it — the lifted rule).
    if let Some(effort) = &card.effort {
        argv.push("--effort".to_owned());
        argv.push(effort.clone());
    }
    argv.push("--agents".to_owned());
    argv.push(card.to_agents_json());
    argv.push("--agent".to_owned());
    argv.push(card.name.clone());
    argv.push("--json-schema".to_owned());
    argv.push(spec.schema.to_string());
    // AC-214: unconditional, the card's explicit value.
    argv.push("--permission-mode".to_owned());
    argv.push(card.permission_mode.clone());
    // AC-202: unconditional; the flag in argv is never evidence of isolation — `system/init` is.
    argv.push("--strict-mcp-config".to_owned());
    // AC-215 (argv half): unconditional.
    argv.push("--permission-prompts".to_owned());
    argv.push("none".to_owned());
    // AC-383 / AC-211: the delivery route, LAST under static (h1-sysprompt's position).
    let stdin = match card.cache_profile {
        CacheProfile::Static => {
            argv.push("--system-prompt".to_owned());
            argv.push(card.prompt.clone());
            spec.user_message.to_owned()
        }
        CacheProfile::Default => format!(
            "{}

{}",
            card.prompt, spec.user_message
        ),
    };

    let program = config.harness_binary().to_path_buf();
    let units = command_line_units(&program, &argv);
    // AC-523's cap (RT-1): at or over the cap is refused, before spawn.
    if units >= ARGV_CAP_BYTES {
        return Err(ConfigError::ArgvAtOrOverCap {
            units,
            cap: ARGV_CAP_BYTES,
        });
    }
    Ok(Assembled {
        program,
        argv,
        stdin,
        command_line_units: units,
    })
}
