//! **`R-change-extension-and-numstat-marker`** — the `change` facet's deriver, and its call site in
//! `build_item` (`s8d` §8d.7; R6 = ADR-0178, proposed). *(Added 2026-09-26, `s8d` build, package
//! P7.)*
//!
//! `facets.toml` declares `change` `source = "derived"` with `deriver =
//! "R-change-extension-and-numstat-marker"` (*"git metadata only: extension table + --numstat's
//! binary marker"*), scored at `change` (0.72 [0.54, 0.85], 29) at pin `47faec9f` (ADR-0093
//! decision 2). Until this stage nothing ran it: `build_item` nulled every non-kernel facet the
//! record omitted and read a record-supplied `change` as if it were a human value.
//!
//! **The rule, ported verbatim from `spikes/p1/derive-facets.py`'s `derive`.** Over the changed
//! paths and the paths `--numstat` marks binary (`-\t-`):
//!
//! - `text` — some path is neither generated nor marked binary;
//! - `binary` — some path is marked binary, or matches the `unreal` table;
//! - `generated` — some path matches the generated table.
//!
//! The two tables are the Python's `RULES["generated"]` and `RULES["toolchain"]`'s `unreal` row,
//! each hand-translated below (this crate carries no regex engine) and pinned by
//! `change_deriver::build_item_derives_change_over_the_recorded_range`. As in the Python, an
//! unreal-table path `--numstat` did not mark is BOTH `binary` (by the table) and `text`.
//!
//! **The range, read through `rk-git`.** The item's changeset is `HEAD..refs/heads/rk-run/<item>`
//! — the commits on the item's run branch that the checkout does not hold — read by the wrapper's
//! `log` (`GitOp::Log`, a read on ADR-0105's list) with `--numstat --no-renames -z`. No ref, or a
//! ref with nothing past `HEAD`, is an **empty range, and the deriver abstains** (`None`): never
//! `[]` and never `text` (§8d.7: *"An empty range abstains, never `[]` and never `"text"`"*).

use std::collections::BTreeSet;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;

/// The deriver's id, as `facets.toml` names it.
pub const DERIVER: &str = "R-change-extension-and-numstat-marker";

/// Why the deriver could not read its range. A read failure is a refusal, never an abstention: a
/// changeset that exists and could not be read is not the absence of one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeriveError {
    pub detail: String,
}

impl std::fmt::Display for DeriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

impl std::error::Error for DeriveError {}

/// `(^|/)(dist|build|out|node_modules|target|bin|obj)/` — a DIRECTORY component so named.
fn in_generated_dir(path: &str) -> bool {
    let mut segments: Vec<&str> = path.split('/').collect();
    segments.pop(); // the file name is not a directory component
    segments.iter().any(|s| {
        matches!(
            *s,
            "dist" | "build" | "out" | "node_modules" | "target" | "bin" | "obj"
        )
    })
}

/// `\.generated\.\w+$` — a `.generated.<word>` suffix, `\w` being `[A-Za-z0-9_]`.
fn generated_suffix(path: &str) -> bool {
    path.rfind(".generated.").is_some_and(|at| {
        let rest = &path[at + ".generated.".len()..];
        !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    })
}

/// The Python's `RULES["generated"]`.
fn is_generated(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    in_generated_dir(path)
        || [".lock", ".min.js", ".snap"]
            .iter()
            .any(|s| path.ends_with(s))
        || generated_suffix(path)
        || matches!(name, "package-lock.json" | "pnpm-lock.yaml" | "Cargo.lock")
}

/// The Python's `RULES["toolchain"]` `unreal` row: `\.(uasset|umap|uproject|uplugin)$|(^|/)(Content|Config)/.*\.ini$`.
fn is_unreal(path: &str) -> bool {
    let by_extension = [".uasset", ".umap", ".uproject", ".uplugin"]
        .iter()
        .any(|s| path.ends_with(s));
    let mut segments: Vec<&str> = path.split('/').collect();
    segments.pop();
    let by_ini =
        path.ends_with(".ini") && segments.iter().any(|s| matches!(*s, "Content" | "Config"));
    by_extension || by_ini
}

/// The `change` values `paths` carry, given which of them `--numstat` marked binary — sorted, and
/// empty only for an empty path list (which [`derive`] never classifies: it abstains).
#[must_use]
pub fn classify(paths: &[String], binary: &[String]) -> Vec<String> {
    let mut change: BTreeSet<&str> = BTreeSet::new();
    if paths
        .iter()
        .any(|p| !is_generated(p) && !binary.contains(p))
    {
        change.insert("text");
    }
    if !binary.is_empty() || paths.iter().any(|p| is_unreal(p)) {
        change.insert("binary");
    }
    if paths.iter().any(|p| is_generated(p)) {
        change.insert("generated");
    }
    change.into_iter().map(str::to_owned).collect()
}

/// **Derive `change` over the item's changeset, `HEAD..refs/heads/rk-run/<item>`.** `Ok(None)` is
/// the abstention: no run branch, or nothing on it past `HEAD`.
///
/// # Errors
/// [`DeriveError`] when the run branch exists and its range cannot be read.
pub fn derive(
    exec: &GitExec,
    repo: &WorktreePath,
    item: &str,
) -> Result<Option<Vec<String>>, DeriveError> {
    let run_ref = format!("refs/heads/rk-run/{item}");
    // No run branch: the item has no changeset. `--verify --quiet` exits non-zero and writes
    // nothing when the ref is absent, which is the one outcome read as "none".
    if exec
        .run(
            GitOp::RevParse,
            repo,
            ["rev-parse", "--verify", "--quiet", &run_ref],
        )
        .is_err()
    {
        return Ok(None);
    }
    let out = exec
        .run(
            GitOp::Log,
            repo,
            [
                "log",
                "--numstat",
                "--no-renames",
                "-z",
                "--format=",
                &format!("HEAD..{run_ref}"),
            ],
        )
        .map_err(|e| DeriveError {
            detail: format!(
                "{DERIVER}: the item's changeset HEAD..{run_ref} could not be read: {e:?}"
            ),
        })?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut paths: Vec<String> = Vec::new();
    let mut binary: Vec<String> = Vec::new();
    for record in text.split('\0') {
        let record = record.trim_start_matches(['\n', '\r']);
        let mut parts = record.splitn(3, '\t');
        let (Some(added), Some(deleted), Some(path)) = (parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        if path.is_empty() {
            continue;
        }
        if !paths.iter().any(|p| p == path) {
            paths.push(path.to_owned());
        }
        if added == "-" && deleted == "-" && !binary.iter().any(|p| p == path) {
            binary.push(path.to_owned());
        }
    }
    if paths.is_empty() {
        return Ok(None);
    }
    Ok(Some(classify(&paths, &binary)))
}
