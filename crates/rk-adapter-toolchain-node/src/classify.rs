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
//! - **Text** — an extension known to be source or prose (`.ts`, `.js`, `.json`, `.md`, `.rs`,
//!   `.py`, `.yaml`, `.txt`, `.sh`, `.ps1`, `.cmd`, …) — **honestly for non-Node paths**: a
//!   `spikes/*.py` or a `crates/*.rs` classifies `Text`, which says nothing about Node and is the
//!   reading ADR-0029 (i) asks for;
//! - otherwise, when the file exists under the project root, **its bytes**: a NUL within the
//!   first 8 KiB is git's own binary heuristic and answers `Binary`; no NUL, or no file (an
//!   affected path may be a deletion), answers `Text`.
//!
//! No spawn: the classification is a property of the path and its bytes, and `change_summary` —
//! the method that would read a range — is S-BIN's and answers `Unsupported` (D17-9). The tables
//! are the implementer's (`s7c` §7c.1: no record fixes them), the same rows as the Rust crate's
//! because the rule is toolchain-agnostic by construction (ADR-0029 i: one `classify` per
//! adapter over the SAME affected paths of a mixed item), and are stated here so a reviewer can
//! disagree with a row rather than with a guess.

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
        return Classification::Generated;
    }
    if BINARY_EXTS.contains(&ext.as_str()) {
        return Classification::Binary;
    }
    if TEXT_EXTS.contains(&ext.as_str()) {
        return Classification::Text;
    }
    let candidate = Path::new(path);
    let on_disk = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(&normalised)
    };
    match std::fs::read(&on_disk) {
        Ok(bytes) if bytes.iter().take(8192).any(|b| *b == 0) => Classification::Binary,
        _ => Classification::Text,
    }
}
