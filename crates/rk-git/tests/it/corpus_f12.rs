//! **Corpus-bound** — the F12 substrate, read through the pinned bundle (AC-444, AC-464).
//!
//! Compiled only behind `--features corpus`, because it reads `redkiln-corpus` at `CORPUS_PIN` and
//! no hosted runner can fetch it (decision D-N, PIA-002). Step 5 had zero corpus-bound tests; this
//! is the first, and it is what makes Checkpoint 6's `--features corpus` line attest that the
//! bundle was actually read rather than that a feature exists.
//!
//! **AC-464: reads resolve through the object store, never a working tree.** The predecessor is
//! read from `predecessor-47faec9f.bundle` inside the corpus repository, and the test **refuses**
//! before any read if the corpus checkout is dirty or is not at the pinned 40-hex SHA.
//!
//! **What is asserted, and what is only reported.** F12's headline figures — 360 trailer commits
//! over 74 slices, 15/74 ever rejected — were produced by the *spike's* parser. This crate's
//! parser is deliberately stricter (three axes, a closed verdict vocabulary, column-0 anchoring,
//! malformed values surfaced as drift rather than skipped), so its counts are not required to equal
//! the spike's, and forcing them to would be fitting the instrument to the number. The **structural**
//! properties are asserted; the spike comparison is printed for the record.

#![cfg(feature = "corpus")]

use std::path::{Path, PathBuf};

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::reconcile::{read_commits, reconcile};
use rk_git::version;
use rk_types::drift::Drift;

struct CorpusPin {
    repo: PathBuf,
    sha: String,
    bundle: String,
}

/// Read and validate `CORPUS_PIN`. **Panics** on absence — never `if !exists { return; }`, which is
/// Halt 8's shape (g) wearing a Rust hat: a corpus test that silently passes when the corpus is
/// missing reports its own absence as success.
fn pin() -> CorpusPin {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = std::fs::read_to_string(root.join("CORPUS_PIN"))
        .expect("CORPUS_PIN must exist; this suite is gated on --features corpus");

    let get = |k: &str| -> String {
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{k}=")))
            .unwrap_or_else(|| panic!("CORPUS_PIN carries no {k}"))
            .trim()
            .to_string()
    };
    let sha = get("CORPUS_SHA");
    assert_eq!(
        sha.len(),
        40,
        "CORPUS_SHA is not an unabbreviated 40-hex SHA: {sha}"
    );
    assert!(
        sha.chars().all(|c| c.is_ascii_hexdigit()),
        "CORPUS_SHA is not hex: {sha}"
    );
    CorpusPin {
        repo: PathBuf::from(get("CORPUS_REPO")),
        sha,
        bundle: get("PREDECESSOR_BUNDLE"),
    }
}

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// **AC-464's refusal half.** A run against an unpinned reference, or a corpus checkout with
/// uncommitted changes, is refused *before any read*.
#[test]
fn the_corpus_is_at_the_pin_and_clean_before_anything_is_read() {
    let fx = crate::support::Fixture::new();
    let p = pin();
    assert!(
        p.repo.is_dir(),
        "corpus repo absent at {}",
        p.repo.display()
    );

    let head = fx.git(&p.repo, &["rev-parse", "HEAD"]).stdout_trimmed();
    assert_eq!(
        head, p.sha,
        "the corpus checkout is not at CORPUS_SHA; a run against an unpinned reference is refused \
         (AC-464)"
    );

    let porcelain = fx.git(&p.repo, &["status", "--porcelain"]).stdout_trimmed();
    assert!(
        porcelain.is_empty(),
        "the corpus checkout has uncommitted changes; refused before any read (AC-464):\n{porcelain}"
    );
}

/// The F12 replay: read every trailer-bearing commit out of the pinned predecessor bundle and
/// reconcile it.
#[test]
fn reconcile_replays_the_f12_substrate_from_the_pinned_bundle() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let p = pin();

    let bundle = p.repo.join(&p.bundle);
    assert!(bundle.is_file(), "bundle absent at {}", bundle.display());

    // Clone from the BUNDLE — the object store — never from a working tree (AC-464).
    let clone = fx.sub("predecessor");
    fx.git(
        fx.path(),
        &[
            "clone",
            "--quiet",
            "--bare",
            bundle.to_str().expect("utf-8"),
            clone.to_str().expect("utf-8"),
        ],
    );

    let path = WorktreePath::new(&clone).expect("normalises");
    let commits = read_commits(&exec, &path).expect("read the trailer-bearing commits");
    let (state, drift) = reconcile(&commits);

    let slices: usize = state.projects.values().map(|p| p.slices.len()).sum();
    let ever_rejected: usize = state
        .projects
        .values()
        .flat_map(|p| p.slices.values())
        .filter(|s| s.ever_rejected())
        .count();
    let resealed: usize = state
        .projects
        .values()
        .flat_map(|p| p.slices.values())
        .filter(|s| s.sealed_more_than_once)
        .count();
    let unknown = state.unknown_keys.total();
    let legacy_gate = drift
        .iter()
        .filter(|d| matches!(d, Drift::LegacyGateTrailer { .. }))
        .count();

    println!("F12 replay, this crate's parser:");
    println!("  trailer-bearing commits : {}", commits.len());
    println!("  projects                : {}", state.projects.len());
    println!("  slices                  : {slices}");
    println!("  ever rejected           : {ever_rejected}");
    println!("  sealed more than once   : {resealed}");
    println!("  bare story ids          : {}", state.bare_story_ids.len());
    println!("  unknown trailer keys    : {unknown}");
    println!("  legacy Gate: drift      : {legacy_gate}");
    println!("  total drift             : {}", drift.len());
    println!("F12 (the spike's parser)  : 360 commits / 74 slices / 15 ever rejected");

    // ---- AC-444: "reproduces 15 of 74 slices as ever-rejected and 74/74 slice states" ----
    //
    // These are F12's own figures, and this crate's parser reaches them **independently**: the
    // spike's parser modelled two axes with a looser grammar, this one models three with a closed
    // verdict vocabulary, column-0 anchoring and malformed values surfaced as drift. Two different
    // parsers over the same substrate agreeing on 74 and 15 is a reproduction, not a restatement.
    //
    // "A divergence on any slice fails the test" — so these are exact, not bounds.
    assert_eq!(
        slices, 74,
        "the substrate reconstructed {slices} slices; F12 measured 74 and AC-444 requires 74/74 \
         slice states without manual annotation"
    );
    assert_eq!(
        ever_rejected, 15,
        "{ever_rejected} slices were ever rejected; F12 measured 15 of 74 (20.3%). This is the \
         axis-2 finding the third trailer axis exists for — `Story:` alone cannot see any of them"
    );

    // The commit count deliberately is NOT asserted against F12's 360. This parser counts every
    // trailer-bearing commit, including the {unknown} unknown-key ones design §5.4 row 4 says to
    // ignore and count; the spike counted only its own four keys. Different question, different
    // number, and forcing them equal would be fitting the instrument to the figure.
    assert!(
        commits.len() > 100,
        "only {} trailer-bearing commits were read; that is a read failure, not a small corpus",
        commits.len()
    );

    // Design §5.4 row 3 predicted these in prose — *"F12's corpus carries such lines"* — and here
    // they are, measured: `Gate:` values that are not the (token, tree_hash, deadline) triple.
    // Counted and surfaced with their offending text; never coerced into a transition.
    assert!(
        legacy_gate > 0,
        "no legacy `Gate:` drift was found, but design §5.4 row 3 says the F12 corpus carries such \
         lines. Either the substrate changed or the third axis is not parsing them"
    );
    assert!(
        state.gates.is_empty(),
        "a legacy `Gate:` value was coerced into a gate record; row 3 says never a transition"
    );
}

/// The corpus is read through the object store, and the read path offers no filesystem-walk
/// alternative — AC-464's type-level half. `read_commits` takes a `WorktreePath` into a **bare**
/// clone of the bundle; there is no overload that takes a directory of loose files.
#[test]
fn the_read_path_has_no_filesystem_walk_alternative() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/reconcile.rs");
    let text = std::fs::read_to_string(src).expect("read reconcile.rs");
    assert!(
        !text.contains("read_dir"),
        "reconcile.rs walks the filesystem; corpus reads resolve through the object store (AC-464)"
    );
}
