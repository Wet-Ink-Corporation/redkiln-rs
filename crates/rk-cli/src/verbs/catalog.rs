//! **`catalog search <QUERY>`** — RA-M-02, D18-15. §7d.3, verbatim:
//!
//! > `rk_kernel::catalog::search(&catalog, query)` over the `keywords` field the loader now reads on
//! > `Activity`; an absent `.redkiln/catalog/` is `Catalog::default()` for this read —
//! > `Catalog::from_sources` refuses an empty set by design (*"an empty catalog is an error, not an
//! > empty success"*) and that refusal stays `run`'s (the no-pack refusal is `run`'s — D18-8), so the
//! > block's `catalog search` on the init-only repository exits `0`; prints the matching activity ids.
//!
//! # 🛑 AN ABSENT CATALOG IS `Catalog::default()`, NOT `from_dir`'s REFUSAL
//!
//! `Catalog::from_dir` on a missing path surfaces *"an empty catalog is an error, not an empty
//! success"* — a **`run`-only** refusal (D18-8). Mapping it here would put `run`'s error on a read
//! verb and turn §7d.17's `catalog search` line on the init-only repository from `0` into `3`.
//! `redkiln init` creates only `.redkiln/config.toml`, so the absent directory IS the block's case.
//!
//! It searches `keywords` and **nothing else** — not `id`, not `tags`, not `provides`. §7d.7 makes
//! that the discriminating half of AC-436's positive control: *"a search that greps ids or
//! descriptions instead of `keywords` is red too"*.

use std::io::Write;

use crate::cli::{CatalogCommand, Cli};
use crate::exit::{Exit, Halt};
use crate::repo;

/// The outer verb: discover the repository, read the catalog, delegate.
///
/// # Errors
/// Never halts — every refusal is a value. The signature matches the other verbs so the router's
/// arms are one shape.
pub fn run(cli: &Cli, command: &CatalogCommand) -> Result<Exit, Halt> {
    run_to(cli, command, &mut std::io::stdout())
}

/// *(dated 2026-09-26, s8c build; D21-64 ruling 3.)* [`run`] with the writer handed in; an
/// in-process caller passes its own, so nothing it renders reaches libtest's stdout.
///
/// # Errors
/// As [`run`].
pub fn run_to(cli: &Cli, command: &CatalogCommand, out: &mut dyn Write) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let dir = std::path::Path::new(repo.worktree.as_str())
        .join(".redkiln")
        .join("catalog");
    let catalog = if dir.is_dir() {
        match rk_kernel::catalog::Catalog::from_dir(&dir) {
            Ok(catalog) => catalog,
            Err(e) => {
                eprintln!("redkiln: {e}");
                return Ok(Exit::ConfigError);
            }
        }
    } else {
        // See the header: the refusal `from_dir` would raise here is `run`'s, not this verb's.
        rk_kernel::catalog::Catalog::default()
    };
    match command {
        CatalogCommand::Search(args) => Ok(search(&catalog, &args.query, out)),
    }
}

/// Print the ids whose declared keywords match, one per line. **Never an error**: §7d.17's block
/// asserts *"a search that finds nothing is not an error"*, and an empty result prints nothing.
pub fn search(catalog: &rk_kernel::catalog::Catalog, query: &str, out: &mut dyn Write) -> Exit {
    for activity in catalog.search(query) {
        writeln!(out, "{}", activity.id).unwrap_or(());
    }
    Exit::Success
}
