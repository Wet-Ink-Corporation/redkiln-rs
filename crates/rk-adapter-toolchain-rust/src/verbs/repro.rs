//! **`repro_place`**: the item's committed repro pair, appended to its target in the run tree
//! (ADR-0204 decisions 1-3, proposed; added 2026-11-08, s8 repro-pair route).
//!
//! The registry row is `toolchain`-routed and `side_effecting`, so the kernel makes `place` a
//! step-grain act that commits to `rk-run/<item>` (ADR-0188 decision 2). The verb reads the `pair`
//! row input, the `ReproPairInput` envelope `rk-cli` binds from the run tree's
//! `.redkiln/intake/<item>.repro.{toml,rs}`, and maps the validated pair to a target under the
//! project root, which is the run-owned worktree (ADR-0137 decision 2):
//!
//! | kind | target | command |
//! | --- | --- | --- |
//! | `it` | `crates/<crate>/tests/it/<module>.rs` | `cargo test -p <crate> --test it -- <module>::<test> --exact` |
//! | `unit` | `crates/<crate>/src/<file>` | `cargo test -p <crate> --lib -- <module>::<test> --exact` |
//!
//! `crates/<crate>` is the package's directory, which is its name in this layout. The command is
//! the one `confirm` runs (through the kernel's one selection parser) and the one the emitted
//! `ReproCase.command` carries, so verify's `confirmation-test` re-runs the same test.
//!
//! **Order of work.** Decode the envelope: `Absent` and `Invalid` refuse naming the paths and the
//! reason. The target must exist as a regular file whose canonical path stays under the canonical
//! project root (no symlink escape). It is read capped at the landing bound
//! ([`rk_git::landing::MAX_LANDED_BLOB_BYTES`], ADR-0188 decision 5), and the file after the append
//! must fit it too. If it already holds the appended block anywhere (a re-entry, possibly after
//! later edits below it), the placement is answered with `appended: false` and nothing is written.
//! If it holds `fn <test>(` otherwise, it is refused. Otherwise `"\n" + source` (with a trailing newline added when the source lacks one)
//! is appended with one `write_all`.
//!
//! **Refusal classes.** A precondition refusal is `Unsupported` (`RetryClass::Never`), not
//! [`super`]'s `refuse` (`Transport { Decode }`, which retries with backoff), because a
//! missing or invalid pair does not improve when repeated. A failed read or write is
//! `Transport { Io }`.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rk_git::landing::MAX_LANDED_BLOB_BYTES;
use rk_ports::toolchain::VerbOutput;
use rk_types::error::{StepError, TransportKind};
use rk_types::repro_pair::{ReproPair, ReproPairInput, ReproTarget};
use rk_types::verb_output::ReproPlacement;
use serde::Deserialize;

use crate::Config;

/// The verb's name, which leads every refusal.
const VERB: &str = "repro_place";

/// Where a pair goes and how it runs: the pure mapping, with no filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Target {
    /// The repo-relative file, `/`-separated.
    path: String,
    /// `<module>::<test>`.
    test: String,
    /// The plain command whose ` -- ` tail selects exactly that test.
    command: String,
}

/// The target and command for `pair` (the module doc's table).
fn target_of(pair: &ReproPair) -> Target {
    let krate = pair.krate();
    let test = format!("{}::{}", pair.module(), pair.test());
    let (path, scope) = match pair.target() {
        ReproTarget::It => (
            format!("crates/{krate}/tests/it/{}.rs", pair.module()),
            "--test it",
        ),
        ReproTarget::Unit { file } => (format!("crates/{krate}/src/{}", file.as_str()), "--lib"),
    };
    let command = format!("cargo test -p {krate} {scope} -- {test} --exact");
    Target {
        path,
        test,
        command,
    }
}

/// `repro_place` (registry row `toolchain`, side-effecting). Reads `inputs["pair"]`.
///
/// # Errors
/// `Unsupported` naming the verb for a missing, absent, invalid or undecodable pair, a missing or
/// escaping or over-cap target, or a prior different `fn <test>(` in it; `Transport { Io }` for a
/// read or write that failed.
pub(super) fn repro_place(
    cfg: &Config,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<VerbOutput, StepError> {
    let Some(raw) = inputs.get("pair") else {
        return Err(unsupported(
            "no `pair` input is bound; the composition root binds the item's repro pair \
             envelope as `repro_place.pair` (ADR-0204 decision 2)"
                .to_owned(),
        ));
    };
    let envelope = ReproPairInput::deserialize(raw).map_err(|e| {
        unsupported(format!(
            "the `pair` input is not a repro pair envelope: {e}"
        ))
    })?;
    let pair = match envelope {
        ReproPairInput::Present(pair) => pair,
        ReproPairInput::Absent { looked_for } => {
            return Err(unsupported(format!(
                "the item has no repro pair (looked for {}); the fix lane reproduces only from \
                 the item's committed pair, never from agent prose (ADR-0204)",
                looked_for.join(", ")
            )));
        }
        ReproPairInput::Invalid { path, why } => {
            return Err(unsupported(format!(
                "the repro pair at {path} is invalid: {why}"
            )));
        }
    };
    let placement = place(Path::new(cfg.project_root().as_str()), &pair)?;
    let value = serde_json::to_value(&placement).map_err(|e| StepError::Transport {
        kind: TransportKind::Decode,
        detail: format!("the ReproPlacement output did not serialise: {e}"),
    })?;
    Ok(VerbOutput {
        kind: "ReproPlacement".to_owned(),
        value,
    })
}

/// Append `pair` to its target under `root`, or answer the placement a prior call made.
fn place(root: &Path, pair: &ReproPair) -> Result<ReproPlacement, StepError> {
    let target = target_of(pair);
    let file = contained_file(root, &target.path)?;
    let existing = read_capped(&file, &target.path)?;
    let mut block = String::with_capacity(pair.source().len().saturating_add(2));
    block.push('\n');
    block.push_str(pair.source());
    if !block.ends_with('\n') {
        block.push('\n');
    }
    // A prior placement may have been followed by later edits, so the block anywhere is placed.
    let appended = if contains(&existing, block.as_bytes()) {
        false
    } else {
        let needle = format!("fn {}(", pair.test());
        if contains(&existing, needle.as_bytes()) {
            return Err(unsupported(format!(
                "{} already holds `{needle}`, and not as the pair's bytes; a second \
                 test of that name would not compile, so it is refused rather than appended",
                target.path
            )));
        }
        let after = existing.len().saturating_add(block.len());
        if u64::try_from(after).map_or(true, |n| n > MAX_LANDED_BLOB_BYTES) {
            return Err(unsupported(format!(
                "{} would be {after} bytes after the append, over the {MAX_LANDED_BLOB_BYTES}-byte \
                 landing bound (ADR-0188 decision 5)",
                target.path
            )));
        }
        let mut out = OpenOptions::new().append(true).open(&file).map_err(|e| {
            io(format!(
                "{VERB}: {} did not open to append: {e}",
                target.path
            ))
        })?;
        out.write_all(block.as_bytes())
            .map_err(|e| io(format!("{VERB}: the append to {} failed: {e}", target.path)))?;
        true
    };
    Ok(ReproPlacement {
        path: target.path,
        test: target.test,
        command: target.command,
        expect: pair.expected().to_owned(),
        source_sha256: pair.source_digest().as_str().to_owned(),
        spec_sha256: pair.spec_digest().as_str().to_owned(),
        appended,
    })
}

/// The canonical path of `rel` under `root`: an existing regular file inside the canonical root.
fn contained_file(root: &Path, rel: &str) -> Result<PathBuf, StepError> {
    let joined = root.join(rel);
    let file = match joined.canonicalize() {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(unsupported(format!(
                "the target {rel} does not exist in the run tree; the pair names a module file \
                 that the crate must already hold"
            )));
        }
        Err(e) => return Err(io(format!("{VERB}: {rel} did not resolve: {e}"))),
    };
    let base = root
        .canonicalize()
        .map_err(|e| io(format!("{VERB}: the project root did not resolve: {e}")))?;
    if !file.starts_with(&base) {
        return Err(unsupported(format!(
            "the target {rel} resolves outside the project root, to {}",
            file.display()
        )));
    }
    let meta = std::fs::metadata(&file).map_err(|e| io(format!("{VERB}: {rel}: {e}")))?;
    if !meta.is_file() {
        return Err(unsupported(format!(
            "the target {rel} is not a regular file"
        )));
    }
    Ok(file)
}

/// `file`'s bytes, refused when over the landing bound.
fn read_capped(file: &Path, rel: &str) -> Result<Vec<u8>, StepError> {
    let handle = std::fs::File::open(file).map_err(|e| io(format!("{VERB}: {rel}: {e}")))?;
    let mut bytes = Vec::new();
    handle
        .take(MAX_LANDED_BLOB_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|e| io(format!("{VERB}: {rel} did not read: {e}")))?;
    if u64::try_from(bytes.len()).map_or(true, |n| n > MAX_LANDED_BLOB_BYTES) {
        return Err(unsupported(format!(
            "the target {rel} is over the {MAX_LANDED_BLOB_BYTES}-byte landing bound (ADR-0188 \
             decision 5)"
        )));
    }
    Ok(bytes)
}

/// Whether `haystack` holds `needle` (an empty needle is held everywhere, and never reaches
/// `windows(0)`).
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}

/// A precondition refusal: `Unsupported`, leading with the verb.
fn unsupported(why: String) -> StepError {
    StepError::Unsupported {
        capability: format!("{VERB}: {why}"),
    }
}

/// A failed read or write.
const fn io(detail: String) -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rk_types::repro_pair::ReproPairSpec;

    fn pair(kind: &str, module: &str, file: Option<&str>) -> ReproPair {
        let spec = ReproPairSpec {
            kind: kind.to_owned(),
            krate: "rk-cli".to_owned(),
            module: module.to_owned(),
            test: "t".to_owned(),
            expect: "e".to_owned(),
            file: file.map(str::to_owned),
        };
        ReproPair::new(spec, "fn t() {}\n".to_owned(), b"").expect("a valid pair")
    }

    #[test]
    fn an_it_target_is_the_module_file_under_tests_it() {
        assert_eq!(
            target_of(&pair("it", "doctor", None)),
            Target {
                path: "crates/rk-cli/tests/it/doctor.rs".to_owned(),
                test: "doctor::t".to_owned(),
                command: "cargo test -p rk-cli --test it -- doctor::t --exact".to_owned(),
            }
        );
    }

    #[test]
    fn a_unit_target_is_the_src_file_under_lib() {
        assert_eq!(
            target_of(&pair("unit", "doctor::repro", Some("verbs/doctor.rs"))),
            Target {
                path: "crates/rk-cli/src/verbs/doctor.rs".to_owned(),
                test: "doctor::repro::t".to_owned(),
                command: "cargo test -p rk-cli --lib -- doctor::repro::t --exact".to_owned(),
            }
        );
    }

    #[test]
    fn contains_finds_a_needle_and_not_a_near_miss() {
        assert!(contains(b"a fn t( b", b"fn t("));
        assert!(!contains(b"a fn tt( b", b"fn t("));
        assert!(!contains(b"", b"fn t("));
    }
}
