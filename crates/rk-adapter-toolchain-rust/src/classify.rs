//! Per-path classification: `Text`, `Binary` or `Generated`, decided from the path and its bytes.
//!
//! `classify(path) -> Text | Binary | Generated` — per path, never per repo (design:269;
//! ADR-0029 (i): *"computed per item from affected paths via `Toolchain::classify`, never per
//! repo, because a story touching Node + UE content needs both change summaries"*; AC-497's
//! classify half, PROVISIONAL while S-BIN is unrun — `s7c` §7c.14).
//!
//! **What decides.** The path first, then the bytes:
//!
//! - **Generated** — a path under a build output or dependency tree (`target/`, `node_modules/`,
//!   `dist/`, `build/`, `.git/`), or a lockfile a tool writes and a reviewer does not read
//!   (`Cargo.lock`, `pnpm-lock.yaml`, `package-lock.json`, `yarn.lock`, `poetry.lock`,
//!   `Pipfile.lock`, `go.sum`), or a `*.d` / `*.rlib` / `*.rmeta` / `*.pdb` / `*.pyc` artefact;
//! - **Binary** — an extension the S-BIN design fixture and ordinary repositories carry as bytes:
//!   the Unreal pair `.uasset` / `.umap` (AC-497's own examples), images, fonts, archives,
//!   executables and objects, documents (`.pdf`, `.docx`, …), media;
//! - **Text** — an extension known to be source or prose (`.rs`, `.toml`, `.md`, `.py`, `.ts`,
//!   `.js`, `.json`, `.yaml`, `.txt`, `.sh`, `.ps1`, `.cmd`, …) — **honestly for non-Rust paths**:
//!   a `spikes/*.py` classifies `Text`, which says nothing about Rust and is the reading
//!   ADR-0029 (i) asks for;
//! - otherwise, when the file exists under the project root, **its bytes**: a NUL within the
//!   first 8 KiB is git's own binary heuristic and answers `Binary`; no NUL, or no file (an
//!   affected path may be a deletion), answers `Text`. *(amended 2026-09-25, s8a fix round 12:
//!   git's window is 8000 bytes, not 8 KiB. This test read 8192, so a blob whose first NUL sat in
//!   bytes 8000..8191 was `Binary` here and text to `rk-git`'s range read, and `change_summary`'s
//!   answer moved with the checkout. It now reads `rk-git`'s own window, 8000 bytes.)*
//!   *(Amended 2026-09-25, s8a fix round 14: that byte read is `classify`'s alone.
//!   `change_summary` reads a committed range, so it takes the path rules (`classify_by_path`)
//!   and decides the bytes on the range's blobs, never on the working tree.)*
//!
//! No spawn, no `cargo metadata` read: the classification is a property of the path and its
//! bytes, and `change_summary` — the method that would read a range — is S-BIN's and answers
//! `Unsupported` (D17-9). *(amended 2026-09-24, s8a, ADR-0165: `change_summary` now reads the
//! range through `rk-git` and classifies each changed path with THIS function; its `Text` arm is
//! answered, and only the `Binary` arm is S-BIN's.)* *(Noted 2026-09-25, s8a, D21-51: "no spawn, no
//! `cargo metadata` read" is now the decision, not an implementation choice. ADR-0165 decision 9
//! supersedes ADR-0094 decision 1's clause in part for `classify`, and the adapter's
//! `classify::classify_source_spawns_nothing` lints this file's code lines.)* The tables are the implementer's (`s7c`
//! §7c.1: no record fixes them) and are stated here so a reviewer can disagree with a row rather
//! than with a guess.

use std::path::Path;

use rk_ports::toolchain::Classification;

const GENERATED_DIRS: &[&str] = &["target", "node_modules", "dist", "build", ".git"];

const GENERATED_FILES: &[&str] = &[
    "Cargo.lock",
    "pnpm-lock.yaml",
    "package-lock.json",
    "yarn.lock",
    "poetry.lock",
    "Pipfile.lock",
    "go.sum",
];

const GENERATED_EXTS: &[&str] = &["d", "rlib", "rmeta", "pdb", "pyc"];

const BINARY_EXTS: &[&str] = &[
    "uasset", "umap", "upk", "pak", // Unreal (AC-497's examples; S-BIN's fixture)
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "tif", "tiff", "psd", // images
    "ttf", "otf", "woff", "woff2", "eot", // fonts
    "zip", "gz", "tgz", "bz2", "xz", "7z", "rar", "jar", // archives
    "exe", "dll", "so", "dylib", "a", "lib", "o", "obj", "bin", "wasm", "class", // objects
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", // documents
    "mp3", "mp4", "wav", "ogg", "flac", "avi", "mov", "mkv", // media
    "sqlite", "sqlite3", "db", // stores
];

const TEXT_EXTS: &[&str] = &[
    "rs", "toml", "md", "txt", "json", "yaml", "yml", "xml", "html", "css", "csv", "tsv", "svg",
    "py", "pyi", "ts", "tsx", "js", "jsx", "mjs", "cjs", "mts", "cts", "sh", "bash", "ps1", "cmd",
    "bat", "c", "h", "cc", "cpp", "hpp", "cs", "java", "go", "rb", "php", "sql", "ini", "cfg",
    "conf", "env", "lock", "patch", "diff", "tex", "rst", "adoc", "org",
];

/// Classify `path` (relative to `root`, or absolute) — the module doc's rule.
#[must_use]
pub fn classify(root: &Path, path: &str) -> Classification {
    if let Some(by_path) = classify_by_path(path) {
        return by_path;
    }
    let normalised = path.replace('\\', "/");
    let candidate = Path::new(path);
    let on_disk = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(&normalised)
    };
    match std::fs::read(&on_disk) {
        Ok(bytes)
            if bytes
                .iter()
                .take(rk_git::diff::BINARY_SNIFF_BYTES)
                .any(|b| *b == 0) =>
        {
            Classification::Binary
        }
        _ => Classification::Text,
    }
}

/// The module doc's rule by the PATH alone, or `None` when only bytes can decide.
///
/// The `Generated`, `Binary` and `Text` tables decide here. `change_summary` calls this and lets the
/// range's blob mark (`RangeEntry::binary`) stand in for the working-tree byte read, so a
/// committed range's answer never moves with the checkout or a dirty tree. *(Added 2026-09-25,
/// s8a fix round 14: `change_summary` called `classify`, whose byte read of the working tree
/// refused a path that was text at both ends of the range.)*
#[must_use]
pub fn classify_by_path(path: &str) -> Option<Classification> {
    let normalised = path.replace('\\', "/");
    let components: Vec<&str> = normalised.split('/').filter(|c| !c.is_empty()).collect();
    let file = components.last().copied().unwrap_or("");
    let ext = Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if components
        .iter()
        .take(components.len().saturating_sub(1))
        .any(|c| GENERATED_DIRS.contains(c))
        || GENERATED_FILES.contains(&file)
        || GENERATED_EXTS.contains(&ext.as_str())
    {
        return Some(Classification::Generated);
    }
    if BINARY_EXTS.contains(&ext.as_str()) {
        return Some(Classification::Binary);
    }
    if TEXT_EXTS.contains(&ext.as_str()) {
        return Some(Classification::Text);
    }
    None
}
