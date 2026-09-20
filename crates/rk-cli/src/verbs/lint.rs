//! **`lint (--facet-stability | --facet-provenance) <FACETS_TOML>`** — the CLI surface over two
//! kernel rules.
//!
//! §7d.3, verbatim: *"`--facet-stability <fixture>` over a facets file with an `llm`-sourced facet
//! exits `3` and names the violation (`rk_kernel::facet_stability::facet_stability_rule`'s
//! fail-closed arm, AC-071's); `--facet-provenance` likewise over a declared-surface violation
//! (`facet_provenance_rule`, AC-520's); a clean pair exits `0`. The lints' criteria are 7b's and
//! 7e's; **7d claims the surface only** and neither id."*
//!
//! **Neither AC-071 nor AC-520 is claimed here.** Do not add either id to a test name, a
//! `Verified-by`, or a traceability row.
//!
//! # 🛑 THE CORPUS PIN IS A FILE READ, NEVER AN ENVIRONMENT READ
//!
//! §7d.3 says so in those words: *"the pin string is `CORPUS_PIN`'s `CORPUS_SHA=` line read from the
//! repository root — **a file read, not an env read**"*. ADR-0083 confines `std::env::var` to
//! `rk-cli`'s `env` module and Checkpoint 7d enforces it with an absence ban over
//! `crates/rk-cli/src`; reading a `CORPUS_SHA` environment variable would be both a ban violation
//! and the wrong source.
//!
//! # 🛑 `--facet-provenance` PRINTS ITS REPORT ON THE `Ok` ARM
//!
//! `facet_provenance_rule` returns `Result<ProvenanceReport, _>` rather than `Result<(), _>`, and
//! the type's own doc says why: *"`Ok(())` is the wrong answer to AC-520's second AND-clause"* — a
//! facet that is NOT MEASURABLE is *"reported so — neither passed nor failed"*. Swallowing the report
//! would reproduce at the CLI the exact defect the type was created to fix. The two counts are
//! printed beside the list because they are what stop an enumeration of **nothing** reading as clean.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use rk_kernel::registry::Registry;

use crate::cli::{Cli, LintArgs};
use crate::exit::{Exit, Halt};
use crate::repo;

/// The outer verb.
///
/// # Errors
/// Never halts — every refusal is a value on the table.
pub fn run(cli: &Cli, args: &LintArgs) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let root = Path::new(repo.worktree.as_str()).to_path_buf();
    let mut out = std::io::stdout();
    Ok(run_with(args, &root, &mut out))
}

/// The inner verb — the in-process twin `lint_wraps_kernel_facet_rules_and_exits_by_table` drives.
///
/// `root` is the repository root the `CORPUS_PIN` file is read from.
pub fn run_with(args: &LintArgs, root: &Path, out: &mut dyn Write) -> Exit {
    let Some(registry_src) = read(&args.facets_toml) else {
        return Exit::ConfigError;
    };
    let registry = match Registry::from_toml(&registry_src) {
        Ok(registry) => registry,
        Err(e) => {
            eprintln!(
                "redkiln: {} is not a facet registry: {e}",
                args.facets_toml.display()
            );
            return Exit::ConfigError;
        }
    };

    if args.facet_stability {
        return stability(args, &registry, root);
    }
    // The clap group is `required(true).multiple(false)`, so exactly one of the two is set and this
    // is the other. No wildcard and no third branch: a mode outside the group cannot reach here.
    provenance(args, &registry, out)
}

fn stability(args: &LintArgs, registry: &Registry, root: &Path) -> Exit {
    // §7d.3: "absent -> an empty map, the zero-`llm` branch". The name is explanatory — with an
    // empty map a registry carrying an `llm` facet falls straight into the fail-closed
    // `StabilityViolation::NoRecord`, and a registry with no `llm` facet passes, because the rule
    // `continue`s on every non-`Llm` source.
    let records = match &args.stability_record {
        None => BTreeMap::new(),
        Some(path) => {
            let Some(src) = read(path) else {
                return Exit::ConfigError;
            };
            match Registry::stability_records_from_str(&src) {
                Ok(records) => records,
                Err(e) => {
                    eprintln!("redkiln: {} is not a stability record: {e}", path.display());
                    return Exit::ConfigError;
                }
            }
        }
    };
    let Some(pin) = corpus_sha(root) else {
        return Exit::ConfigError;
    };
    match rk_kernel::facet_stability::facet_stability_rule(registry, &records, &pin) {
        Ok(()) => Exit::Success,
        Err(violations) => {
            // EVERY violation, never the first: both rules collect all clauses by design.
            for violation in &violations {
                eprintln!("redkiln: {}: {violation}", violation.facet());
            }
            Exit::ConfigError
        }
    }
}

fn provenance(args: &LintArgs, registry: &Registry, out: &mut dyn Write) -> Exit {
    let Some(path) = args.pinned_corpus.as_ref() else {
        // §6.2 of the contract: the `<CORPUS_REPO>/results/` default is genuinely underspecified and
        // is RAISED rather than invented here. The tests pass the flag; `s8`'s invocation does too.
        eprintln!(
            "redkiln: `--pinned-corpus <PATH>` is required at this stage. Its default — \
             `<CORPUS_REPO>/results/` per CORPUS_PIN — is named by `s7d` §7d.3 and specified by no \
             record, and a path invented here would be read as one that was decided."
        );
        return Exit::ConfigError;
    };
    let Some(src) = read(path) else {
        return Exit::ConfigError;
    };
    let corpus = match Registry::pinned_corpus_from_str(&src) {
        Ok(corpus) => corpus,
        Err(e) => {
            eprintln!("redkiln: {} is not a pinned corpus: {e}", path.display());
            return Exit::ConfigError;
        }
    };
    match rk_kernel::facet_provenance::facet_provenance_rule(registry, &corpus) {
        Ok(report) => {
            // See the header: the report is the answer, not decoration.
            writeln!(
                out,
                "derived facets: {}; records read: {}",
                report.derived_facets, report.records_read
            )
            .unwrap_or(());
            for row in &report.not_measurable {
                writeln!(out, "{row}").unwrap_or(());
            }
            Exit::Success
        }
        Err(violations) => {
            for violation in &violations {
                eprintln!(
                    "redkiln: {} [{}]: {violation}",
                    violation.facet(),
                    violation.clause()
                );
            }
            Exit::ConfigError
        }
    }
}

/// Read a file, or print the refusal and return `None`. Every arm is exit `3` — §7d.4's row: *"a
/// refusal of the repository's recorded state, read as configuration of the item"*, and the facet
/// registry IS recorded repository state.
fn read(path: &Path) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(src) => Some(src),
        Err(e) => {
            eprintln!("redkiln: {} could not be read: {e}", path.display());
            None
        }
    }
}

/// The value of `CORPUS_PIN`'s `CORPUS_SHA=` line, read from the repository root.
///
/// A `KEY=value` text file; the precedent parser is `crates/rk-git/tests/it/corpus_f12.rs`. **A file
/// read, not an env read** — see the module header.
fn corpus_sha(root: &Path) -> Option<String> {
    let path = root.join("CORPUS_PIN");
    let src = read(&path)?;
    let found = src
        .lines()
        .filter_map(|l| l.strip_prefix("CORPUS_SHA="))
        .map(str::trim)
        .find(|v| !v.is_empty())
        .map(str::to_owned);
    if found.is_none() {
        eprintln!(
            "redkiln: {} carries no CORPUS_SHA= line — the stability rule compares the pin it is \
             given, and an empty one would compare every record against nothing",
            path.display()
        );
    }
    found
}
