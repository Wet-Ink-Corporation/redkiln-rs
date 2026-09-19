//! **The router** — one `clap` `Command` named `redkiln`, and the resume-shaped denylist AC-400
//! refuses (`s7d` §7d.3; ADR-0107 decision 3; ADR-0128; ADR-0129).
//!
//! Every verb is a read of the world, a decision, and an exit (design §8.1). No verb prompts and no
//! verb reads stdin (D-15), so nothing here takes an interactive flag and nothing here is a mode.
//!
//! **The subcommand set is closed.** §7d.3 lists eight and says *"and no others"*: a ninth is a
//! decision, taken in a record, not a derive attribute. At this slice each of the eight resolves to
//! a stub in [`crate::verbs`]; Slices 2–4 replace them one lane at a time.
//!
//! **What is NOT here, and is not an omission.** There is no `--resume`, no `--continue` and no
//! `resume` verb: `redkiln run <item>` is the sole entry for a first run and for one taken up after
//! a gate was answered (AC-186, AC-264; ADR-0128 decision 1). There is no `--item` flag; the item is
//! the first positional. There is no `migrate` verb — the converter is its own binary (AC-441).
//! There is no `--no-reconcile`, `--skip-reconcile` or `--fast`: reconciliation is not optional
//! (AC-402). And `doctor` ships neither `--fix` nor `--probe` — a CONDITIONAL row reports, it does
//! not repair (RB-M-06, D18-16).

use std::path::PathBuf;

/// The eleven resume-shaped names AC-400 refuses, verbatim: ADR-0107 decision 3's eight plus the
/// acceptance register's earlier three.
///
/// **Matching is EXACT, on the name with and without its `--`** — never a prefix rule, which would
/// refuse `--from-scratch` on some future verb for a name this list does not carry, and never a
/// substring rule, which would fire on `continue` inside `--continue-on-error`. The register's own
/// *Limit* is that the denylist is name-based, and `ac400_denylist_absent_from_parser_tree` is what
/// holds the rule to that.
pub const DENYLIST: [&str; 11] = [
    "resume",
    "continue",
    "stay",
    "--stay",
    "--resume",
    "--continue",
    "--from",
    "--pick-up",
    "--retry-step",
    "--from-step",
    "--again",
];

/// `redkiln` — backlog-as-code, with the loop in the binary.
#[derive(Debug, clap::Parser)]
#[command(
    name = "redkiln",
    bin_name = "redkiln",
    version,
    about = "Backlog-as-code: the kernel selects, a harness executes, git records.",
    disable_help_subcommand = true
)]
pub struct Cli {
    // The ONE global option (H-3). Short only: §7d.3 names `-C <PATH>` and nothing else, and a
    // long form invented here would be a permanent row in the committed surface snapshot for as
    // long as the binary exists. The doc comment below is USER-FACING help and carries no warrant:
    // a committed --help snapshot is read by operators, not by this runbook.
    /// Act on the repository at this path instead of the working directory
    #[arg(short = 'C', value_name = "PATH", global = true)]
    pub cwd: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

/// The eight verbs, and no others (§7d.3).
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Take the next activity for an item and exit on what it decided.
    Run(RunArgs),
    /// Answer an open human gate, by the option its question declared.
    Answer(AnswerArgs),
    /// Print the roll-up across every item the store knows.
    Status,
    /// Print why an item reached the state it is in, from its recorded run.
    Explain(ExplainArgs),
    /// Report what is wrong with this repository's configuration and state.
    Doctor(DoctorArgs),
    /// Create this repository's `.redkiln/` scaffold.
    Init,
    /// Read the activity catalog.
    Catalog(CatalogArgs),
    /// Check a facet registry against a rule and exit by its table.
    Lint(LintArgs),
}

/// `redkiln run <ITEM> [--lane <LANE>]` — ADR-0128 decision 1.
#[derive(Debug, clap::Args)]
pub struct RunArgs {
    // The first positional, never a flag: ADR-0128 decision 1 — "There is no `--item` flag."
    /// The item to act on
    #[arg(value_name = "ITEM")]
    pub item: String,

    // Optional by ADR-0128 decision 2: absent, the lane is derived from the item, and an item it
    // cannot be derived for is a CONFIGURATION error — while a third VALUE here is a usage error.
    /// Which lane the item belongs to
    #[arg(long, value_enum, value_name = "LANE")]
    pub lane: Option<Lane>,
}

/// The closed lane set. A third value is a usage error, not a configuration error
/// (ADR-0128 decision 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Lane {
    /// The reactive lane
    Fix,
    /// The planned lane
    Story,
}

/// `redkiln answer <ITEM> <OPTION> [--attach <KIND>=<PATH>]... --explain-sufficed <yes|no>
/// --toml-opened <yes|no>` — ADR-0129 decision 1.
#[derive(Debug, clap::Args)]
pub struct AnswerArgs {
    /// The item whose gate is open
    #[arg(value_name = "ITEM")]
    pub item: String,

    // A free string at the parser, NOT a value enum of two literals. The admissible options belong
    // to the question in the registry; validating them here would freeze one question's answer set
    // into the binary, which ADR-0129 decision 3 names and rejects. An option the question does not
    // declare is still a usage error with zero events — decided against the registry, not here.
    /// The option id, as the gate's question declared it
    #[arg(value_name = "OPTION")]
    pub option: String,

    /// An artifact the chosen option admits, as KIND=PATH; repeatable
    #[arg(long, value_name = "KIND=PATH", value_parser = parse_attachment)]
    pub attach: Vec<Attachment>,

    // Required, and so is the next one: ADR-0129 decision 1 fixes exactly TWO sufficiency flags.
    // A non-Option field is how clap is told to require it, so the refusal is a usage error raised
    // before any store read — the register's "cannot be submitted with the field absent".
    /// Did reading the explanation suffice to decide?
    #[arg(long, value_enum, value_name = "yes|no")]
    pub explain_sufficed: YesNo,

    /// Was the item's own file opened before deciding?
    #[arg(long, value_enum, value_name = "yes|no")]
    pub toml_opened: YesNo,
}

/// The closed set both sufficiency flags take. Exactly two such flags exist
/// (ADR-0129 decision 1); a third would be a record, not an attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum YesNo {
    Yes,
    No,
}

/// One `--attach` occurrence, split on its first `=`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    /// The artifact kind name.
    pub kind: String,
    /// A file in the working tree.
    pub path: PathBuf,
}

/// Split one `--attach` value. **The split only** — whether the chosen option admits this kind, and
/// whether the file is there, are the gate's questions and are decided where the registry is
/// (ADR-0063, ADR-0129 decision 4), not in a value parser.
fn parse_attachment(raw: &str) -> Result<Attachment, String> {
    raw.split_once('=').map_or_else(
        || {
            Err(format!(
                "`{raw}` is not <KIND>=<PATH> — the kind and the file are separated by `=`"
            ))
        },
        |(kind, path)| {
            if kind.is_empty() || path.is_empty() {
                Err(format!(
                    "`{raw}` has an empty kind or an empty path — both halves of <KIND>=<PATH> are \
                     required"
                ))
            } else {
                Ok(Attachment {
                    kind: kind.to_owned(),
                    path: PathBuf::from(path),
                })
            }
        },
    )
}

/// `redkiln explain <ITEM>`.
#[derive(Debug, clap::Args)]
pub struct ExplainArgs {
    // AC-432: an item with no recorded resolution is an ERROR rather than a recomputation — the
    // NoRecord row of the exit table, not a silent re-derivation.
    /// The item whose recorded run to read
    #[arg(value_name = "ITEM")]
    pub item: String,
}

/// `redkiln doctor [--rebuild]`.
#[derive(Debug, clap::Args)]
pub struct DoctorArgs {
    /// Re-derive the tier-2 projection from git alone and compare it with what is stored
    #[arg(long)]
    pub rebuild: bool,
}

/// `redkiln catalog search <QUERY>` — the one verb with a sub-surface.
#[derive(Debug, clap::Args)]
pub struct CatalogArgs {
    #[command(subcommand)]
    pub command: CatalogCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum CatalogCommand {
    /// Find activities whose declared keywords match
    Search(CatalogSearchArgs),
}

#[derive(Debug, clap::Args)]
pub struct CatalogSearchArgs {
    /// What to look for
    #[arg(value_name = "QUERY")]
    pub query: String,
}

/// `redkiln lint (--facet-stability | --facet-provenance) <FACETS_TOML> [--stability-record <PATH>]
/// [--pinned-corpus <PATH>]` — the two kernel facet rules, wrapped.
#[derive(Debug, clap::Args)]
#[command(group(
    clap::ArgGroup::new("rule").required(true).multiple(false).args(["facet_stability", "facet_provenance"])
))]
pub struct LintArgs {
    /// Check every declared facet against its stability record at the pinned corpus
    #[arg(long)]
    pub facet_stability: bool,

    /// Check that every declared facet value has a registered fact behind it
    #[arg(long)]
    pub facet_provenance: bool,

    /// The facet registry to read
    #[arg(value_name = "FACETS_TOML")]
    pub facets_toml: PathBuf,

    /// Stability records; absent, the rule runs over an empty map
    #[arg(long, value_name = "PATH")]
    pub stability_record: Option<PathBuf>,

    /// The pinned corpus results; absent, the location CORPUS_PIN records is used
    #[arg(long, value_name = "PATH")]
    pub pinned_corpus: Option<PathBuf>,
}

/// The built parser tree — what the parser will actually accept.
///
/// `pub` because AC-400's lint half is a test that WALKS this tree (D18-13), and the walk lives in
/// `tests/it/surface.rs` where the register's wording puts it. Handing the test the built `Command`
/// rather than a set of names computed here keeps the instrument in the test and leaves this module
/// with the router and its denylist, which is all it is for.
#[must_use]
pub fn command() -> clap::Command {
    <Cli as clap::CommandFactory>::command()
}

/// Parse an argument vector, returning `clap`'s own error rather than exiting.
///
/// `main` calls this and exits through the error's own `exit()`; the tests call it so a refusal can
/// be READ — its code and its message — instead of killing the test process.
///
/// # Errors
/// Whatever `clap` refuses: an unknown argument, a missing required flag, a value outside a closed
/// set. Every one of them is exit code `2` (§7d.4's `Usage` row).
pub fn try_parse_from<I, T>(args: I) -> Result<Cli, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    <Cli as clap::Parser>::try_parse_from(args)
}
