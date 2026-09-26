//! `p9_pin` — the ONE reader of the frozen `spikes/p9` tree, at the commit its measurements were taken
//! at (D21-47 **V2**: *"Every test pinning a historical `spikes/p9` measurement reads the p9 tree at its
//! pinned commit via git, never the working tree."*).
//!
//! Five tests pin such a measurement (s8b §8b.3): `rigor::rigor_monotonicity_p9_historical_2026_09_13`,
//! `memo::ac018_frozen_p9_catalog_is_red`, `quality_block::ac527_frozen_p9_catalog_is_red`, and the two
//! lattice parity tests. Each reads [`root`], never a `"spikes/p9…"` literal, so a story-lane edit to the
//! working tree's `spikes/p9/packs/spike.toml` (D21-41 f, `s8j`) cannot move a byte-pinned figure.
//!
//! **How it reads.** [`materialise_at`] lists `git ls-tree -r -z <sha> -- <rel>` and writes each blob
//! from `git cat-file blob <sha>:<path>` into a fresh directory under `CARGO_TARGET_TMPDIR`.
//! `cat-file blob` applies no smudge and no line-ending conversion, so the bytes are the blobs' own.
//! The pin reads the object store so that it is independent of the working tree's state: the working
//! tree's `spikes/p9` may MOVE (D21-47 V2; the story lane edits `packs/spike.toml`), and a pinned
//! measurement must not move with it. *(dated 2026-09-25, s8b refuting round 3: this said "no
//! checkout of this repository is a byte-faithful export". ADR-0164's `* text=auto eol=lf` made
//! checkouts faithful on 2026-09-24; that is not why the helper uses `cat-file`.)*
//!
//! **Its materialisations are not removed.** Each test process writes one copy of `spikes/p9`
//! (about 1.7 MB, 183 blobs at `P9_PIN`) to `CARGO_TARGET_TMPDIR/p9-pin-<sha12>-spikes_p9-<pid>`, and
//! nothing deletes it at exit, so `target/tmp` grows by one copy per run until `cargo clean`. A sweep
//! of other processes' directories could remove one that a concurrent run is reading, so none is done
//! here; the cleanup is recorded open *(dated 2026-09-25, s8b refuting round 3)*.
//!
//! Every call goes through
//! `rk_git::git_exec::run`, the one wrapper (ADR-0077), with `-c safe.directory=*` so that the no-egress
//! container job can read a checkout its user does not own.
//!
//! **It refuses rather than falls back.** A commit absent from the clone (a depth-1 CI checkout) panics
//! naming the shallow checkout. It never reads the working tree instead: a fallback would turn the pin
//! into a working-tree read exactly when the pin is missing.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use rk_git::git_exec;

const P9_PIN: &str = "ca8f3874ed9185b419b00b21b33713ba128de217";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

fn git(args: &[&str]) -> Vec<u8> {
    let mut all = vec!["-c", "safe.directory=*"];
    all.extend_from_slice(args);
    match git_exec::run(&workspace_root(), &all) {
        Ok(out) => out.stdout,
        Err(e) => panic!("p9_pin: `git {}` failed: {e:?}", args.join(" ")),
    }
}

/// The `spikes/p9` tree materialised at `P9_PIN`, once per test process.
pub fn root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| materialise_at(P9_PIN, "spikes/p9"))
        .as_path()
}

/// `<rel>` as it stood at commit `sha`, read from the object store into a fresh directory. Returns the
/// materialised `<rel>`.
pub fn materialise_at(sha: &str, rel: &str) -> PathBuf {
    let exists = git_exec::run(
        &workspace_root(),
        [
            "-c",
            "safe.directory=*",
            "cat-file",
            "-e",
            &format!("{sha}^{{commit}}"),
        ],
    );
    assert!(
        exists.is_ok(),
        "p9_pin: commit {sha} is not in this clone — a shallow checkout (actions/checkout's default \
         fetch-depth is 1) holds no pinned tree, and this helper never falls back to the working tree"
    );
    let base = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "p9-pin-{}-{}-{}",
        &sha[..12],
        rel.replace('/', "_"),
        std::process::id()
    ));
    if base.exists() {
        std::fs::remove_dir_all(&base).expect("p9_pin: a stale materialisation can be removed");
    }
    let listing = git(&["ls-tree", "-r", "-z", sha, "--", rel]);
    let mut written = 0usize;
    for entry in listing.split(|b| *b == 0).filter(|e| !e.is_empty()) {
        let entry = std::str::from_utf8(entry).expect("p9_pin: ls-tree output is UTF-8");
        let (meta, path) = entry
            .split_once('\t')
            .expect("p9_pin: an ls-tree entry is `<mode> <type> <oid>\\t<path>`");
        if meta.split(' ').nth(1) != Some("blob") {
            continue;
        }
        let bytes = git(&["cat-file", "blob", &format!("{sha}:{path}")]);
        let dest = base.join(path);
        std::fs::create_dir_all(dest.parent().expect("a blob path has a parent"))
            .expect("p9_pin: the destination directory can be created");
        std::fs::write(&dest, bytes).expect("p9_pin: a blob can be written");
        written += 1;
    }
    assert!(
        written > 0,
        "p9_pin: `{rel}` at {sha} lists no blobs — there is nothing to read, and an empty tree must \
         not stand for the pinned one"
    );
    base.join(rel)
}

/// At `b66ba53`, `spikes/p9/catalog/threat-model.toml` still carried the `auth` term that Step 7e later
/// removed from the working tree. A helper that read the working tree would return the later line, so
/// this is the one test that can see the difference: at `P9_PIN` the two agree (mutant M28).
/// *(dated 2026-09-25, s8b refuting round 3: they agree on the paths the five pinned tests read —
/// `catalog/`, `facets.toml`, `artifacts.toml` and `packs/` — not on `spikes/p9` as a whole, which
/// changed at `bf12a1c`.)* *(dated 2026-09-25, s8b refuting round 4: it also asserts that [`root`]
/// itself is bound to `P9_PIN` — over `catalog-b/api-contract-spec.toml`, which differs between the
/// pin and the working tree — and lies under `CARGO_TARGET_TMPDIR` (mutant M28b).)*
#[test]
fn reads_the_object_store_at_the_named_commit_not_the_working_tree() {
    const OLD: &str = "applicable_if = \"auth or data_migration or external_dep\"";
    let at = materialise_at("b66ba539791175fa5f559e12c31f964a4d1bdbeb", "spikes/p9");
    let pinned = std::fs::read_to_string(at.join("catalog/threat-model.toml"))
        .expect("threat-model.toml exists at b66ba53");
    let working =
        std::fs::read_to_string(workspace_root().join("spikes/p9/catalog/threat-model.toml"))
            .expect("threat-model.toml exists in the working tree");
    assert!(
        !working.contains(OLD),
        "control: the working tree no longer carries the auth term, so it cannot stand in for b66ba53"
    );
    assert!(
        pinned.contains(OLD),
        "the materialised file does not carry b66ba53's line — the helper read the working tree, not \
         the object store"
    );
    let n = std::fs::read_dir(root().join("catalog"))
        .expect("the pinned catalog directory exists")
        .filter(|e| {
            e.as_ref()
                .is_ok_and(|e| e.path().extension().is_some_and(|x| x == "toml"))
        })
        .count();
    assert_eq!(
        n, 102,
        "the pinned p9 catalog holds {n} activities, not 102"
    );

    // Refuting round 4 (2026-09-25): `root()`'s binding to `P9_PIN` is observable here, not only
    // `materialise_at`'s. `catalog-b/api-contract-spec.toml` changed at `bf12a1c` (ADR-0166), after
    // the pin [measured: `git diff --name-only ca8f387 -- spikes/p9` names it]; the other paths the
    // five pinned tests read agree between the two, so a `root()` that read the working tree, or
    // another commit, would survive them. It does not survive this (mutant M28b).
    // *(dated 2026-09-25, s8b refuting round 6: "or another commit" overstated it.)* This check rules
    // out a `root()` that reads the working tree, or any commit at or after `bf12a1c`. The file's
    // blob is the same at every commit from `021c75a` to `P9_PIN` (`git log` names only `021c75a` and
    // `bf12a1c` for it), so a `root()` bound to a pre-pin commit such as `b66ba53` would pass.
    let differs = "catalog-b/api-contract-spec.toml";
    let blob = git(&["cat-file", "blob", &format!("{P9_PIN}:spikes/p9/{differs}")]);
    let wt = std::fs::read(workspace_root().join("spikes/p9").join(differs))
        .expect("the working tree's catalog-b file reads");
    assert_ne!(
        blob, wt,
        "control: at P9_PIN and in the working tree `{differs}` must differ, or this cannot tell them apart"
    );
    let via_root = std::fs::read(root().join(differs)).expect("root() carries the catalog-b file");
    assert_eq!(
        via_root, blob,
        "root()'s `{differs}` is not P9_PIN's blob — root() is not bound to P9_PIN"
    );
    assert_ne!(
        via_root, wt,
        "root()'s `{differs}` is the working tree's bytes — root() read the working tree"
    );
    assert!(
        root().starts_with(env!("CARGO_TARGET_TMPDIR")),
        "root() is {} — not a materialisation under CARGO_TARGET_TMPDIR",
        root().display()
    );
}
