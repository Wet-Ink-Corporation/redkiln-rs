//! The `Binary` arm's payload (ADR-0199, proposed): [`BinaryChangeSummary`], its rows
//! [`BinaryAsset`] and [`BinaryCapture`], and why one is refused. Re-exported from
//! [`crate::toolchain`], whose paths are the public ones.

use super::ChangeKind;

mod text_patch;

use text_patch::TextPatched;

/// The `Binary` arm's payload, as S-BIN fixed it (ADR-0199, proposed; F43, proposed): the four
/// fields S-BIN's reviewers cited, and no other.
///
/// - `assets` -- one [`BinaryAsset`] per changed path the range classifies `Binary`, its `size`
///   and `lfs_oid` read from the git-lfs pointer;
/// - `captures` -- one [`BinaryCapture`] per rendered capture;
/// - `tests.report_ref` -- the raw test report's reference;
/// - `build.log_ref` -- the build log's reference.
///
/// It serialises those four members, `tests` and `build` as `{ "report_ref" }` and
/// `{ "log_ref" }`, and none of the `Text` arm's (`sha`, `diff`, `files`, `manifest`).
///
/// **The construction rule** (ADR-0199 decision 2): the fields are private, and the payload is
/// built only through [`Self::new`], which refuses a text patch of a `Binary`-classified path
/// (AC-497's AND). It has no `Default`, no `From` and no `Deserialize`;
/// `tests/ui/change_summary_binary_construction_rule.rs` pins each refused route. **Binary
/// producers in Phase 1: none** (ADR-0199 decision 4): no Phase-1 adapter calls the constructor,
/// and [`crate::fakes::FixtureToolchain`] still refuses a range carrying a `Binary` path by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChangeSummary {
    assets: Vec<BinaryAsset>,
    captures: Vec<BinaryCapture>,
    tests_report_ref: String,
    build_log_ref: String,
}

impl BinaryChangeSummary {
    /// The recorded constructor (ADR-0199 decision 2): the four fields, checked against the
    /// range's textual diff as the producer read it.
    ///
    /// `range_diff` is that diff, or `""` when the producer read none. It is never stored: the
    /// value carries no diff. It is read only to refuse an input that carries a TEXT patch of a
    /// path in `assets` -- a `diff --git` section naming `<p>` on its a-side or b-side (a rename's
    /// or copy's `from`/`to` lines included) with a hunk or a `GIT binary patch`, or a
    /// `--- a/<p>` or `+++ b/<p>` header outside a hunk. Names are read as git writes them:
    /// C-quoted names are unquoted, and the TAB git writes after an unquoted `---`/`+++` name ends
    /// it. git's content-free `Binary files … differ` record of the path, a patch of a path the
    /// value does not carry, and a hunk line that only looks like a header are admitted.
    ///
    /// **The producer contract:** `range_diff` is git's diff with its default `a/` and `b/`
    /// prefixes (not `--no-prefix`, `--src-prefix`/`--dst-prefix`, `diff.noprefix` or
    /// `diff.mnemonicPrefix`), and each prefix is stripped once; a name under any other prefix is
    /// not read as naming an asset. A quoted name the check cannot read (its quote never closes,
    /// or it holds an escape git does not write) counts as naming every asset, so a malformed
    /// diff refuses more, never less. The diff is read once, whatever the number of assets.
    ///
    /// # Errors
    ///
    /// [`BinaryChangeSummaryError::TextPatchForBinaryPath`] naming the first asset path the diff
    /// patches as text (AC-497's AND: *"no textual diff of the binary path is assembled or
    /// dispatched"*). The refused input is not handed back (an ERR-14 deviation, ADR-0199's
    /// consequences): an input that patches a binary path as text is invalid, and no caller can
    /// retry with it.
    pub fn new(
        mut assets: Vec<BinaryAsset>,
        captures: Vec<BinaryCapture>,
        tests_report_ref: String,
        build_log_ref: String,
        range_diff: &str,
    ) -> Result<Self, BinaryChangeSummaryError> {
        let text_patched = TextPatched::read(range_diff);
        if let Some(patched) = assets
            .iter()
            .position(|asset| text_patched.covers(&asset.path))
        {
            return Err(BinaryChangeSummaryError::TextPatchForBinaryPath {
                path: assets.swap_remove(patched).path,
            });
        }
        Ok(Self {
            assets,
            captures,
            tests_report_ref,
            build_log_ref,
        })
    }

    /// One row per changed `Binary` path.
    #[must_use]
    pub fn assets(&self) -> &[BinaryAsset] {
        &self.assets
    }

    /// One row per rendered capture.
    #[must_use]
    pub fn captures(&self) -> &[BinaryCapture] {
        &self.captures
    }

    /// The raw test report's reference (`tests.report_ref`).
    #[must_use]
    pub fn tests_report_ref(&self) -> &str {
        &self.tests_report_ref
    }

    /// The build log's reference (`build.log_ref`).
    #[must_use]
    pub fn build_log_ref(&self) -> &str {
        &self.build_log_ref
    }
}

impl serde::Serialize for BinaryChangeSummary {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("BinaryChangeSummary", 4)?;
        s.serialize_field("assets", &self.assets)?;
        s.serialize_field("captures", &self.captures)?;
        s.serialize_field(
            "tests",
            &ReportRef {
                report_ref: &self.tests_report_ref,
            },
        )?;
        s.serialize_field(
            "build",
            &LogRef {
                log_ref: &self.build_log_ref,
            },
        )?;
        s.end()
    }
}

/// `tests: { report_ref }`, the serialised form of `tests.report_ref`.
#[derive(serde::Serialize)]
struct ReportRef<'a> {
    report_ref: &'a str,
}

/// `build: { log_ref }`, the serialised form of `build.log_ref`.
#[derive(serde::Serialize)]
struct LogRef<'a> {
    log_ref: &'a str,
}

/// One `assets` row (ADR-0199 decision 1): a changed path the range classifies `Binary`, LFS
/// pointer paths included.
///
/// `size` and `lfs_oid` are READ from the git-lfs pointer, never inferred from whether the
/// pointer's size line moved (`s8i` §8i.2 item 6). The fields are private and the one constructor
/// is [`Self::from_pointer`], so no row carries a size the pointer does not state. The pointer is
/// the blob at the end of the range where the path exists (the head end, or the base end for a
/// deletion); which blob that is, is the producer's to read.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BinaryAsset {
    path: String,
    kind: String,
    action: ChangeKind,
    size: u64,
    lfs_oid: String,
}

/// The git-lfs pointer's first line (the git-lfs v1 spec).
const LFS_POINTER_VERSION: &str = "version https://git-lfs.github.com/spec/v1";
/// A pointer is under 1024 bytes (the git-lfs v1 spec); a longer blob is not one, and is never
/// scanned.
const LFS_POINTER_MAX_BYTES: usize = 1024;

impl BinaryAsset {
    /// The row for `path`, its `size` and `lfs_oid` read from `pointer`, the path's git-lfs
    /// pointer blob.
    ///
    /// `kind` is the asset's type as the producer names it (S-BIN used the extension, `uasset`),
    /// and `action` how the path changed over the range.
    ///
    /// # Errors
    ///
    /// [`BinaryChangeSummaryError::NotAnLfsPointer`] naming the path and the defect when `pointer`
    /// is 1024 bytes or more, is not UTF-8, does not open with the spec's version line, or holds
    /// no single `oid sha256:<64 hex>` line or no single `size <n>` line (`<n>` ASCII digits).
    /// `kind` is not handed back (an ERR-14 deviation, ADR-0199's consequences): a blob that is not
    /// a pointer is invalid input, and no caller retries with it.
    pub fn from_pointer(
        path: String,
        kind: String,
        action: ChangeKind,
        pointer: &[u8],
    ) -> Result<Self, BinaryChangeSummaryError> {
        match read_pointer(pointer) {
            Ok((lfs_oid, size)) => Ok(Self {
                path,
                kind,
                action,
                size,
                lfs_oid,
            }),
            Err(defect) => Err(BinaryChangeSummaryError::NotAnLfsPointer { path, defect }),
        }
    }

    /// The path, relative to the repository root, `/`-separated.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The asset's type, as the producer names it.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// How the path changed over the range.
    #[must_use]
    pub const fn action(&self) -> ChangeKind {
        self.action
    }

    /// The asset's size in bytes, read from the pointer's `size` line.
    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }

    /// The asset's sha256, read from the pointer's `oid sha256:` line: 64 lower-case hex.
    #[must_use]
    pub fn lfs_oid(&self) -> &str {
        &self.lfs_oid
    }
}

/// The pointer's `oid` and `size`, or the first defect found.
fn read_pointer(pointer: &[u8]) -> Result<(String, u64), PointerDefect> {
    if pointer.len() >= LFS_POINTER_MAX_BYTES {
        return Err(PointerDefect::TooLong);
    }
    let Ok(text) = std::str::from_utf8(pointer) else {
        return Err(PointerDefect::NotUtf8);
    };
    // `lines` splits on `\n` and on `\r\n`.
    let mut lines = text.lines();
    if lines.next() != Some(LFS_POINTER_VERSION) {
        return Err(PointerDefect::NoVersionLine);
    }
    let (mut oid, mut size) = (None, None);
    for line in lines {
        if let Some(hex) = line.strip_prefix("oid sha256:") {
            let well_formed = hex.len() == 64
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
            if !well_formed || oid.replace(hex).is_some() {
                return Err(PointerDefect::Oid);
            }
        } else if let Some(n) = line.strip_prefix("size ") {
            // ASCII digits only: `u64::from_str` also takes a leading `+`, which the spec's
            // `size <n>` does not.
            let digits = !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit());
            let Some(parsed) = digits.then(|| n.parse::<u64>()).and_then(Result::ok) else {
                return Err(PointerDefect::Size);
            };
            if size.replace(parsed).is_some() {
                return Err(PointerDefect::Size);
            }
        }
    }
    match (oid, size) {
        (Some(oid), Some(size)) => Ok((oid.to_owned(), size)),
        (None, _) => Err(PointerDefect::Oid),
        (Some(_), None) => Err(PointerDefect::Size),
    }
}

/// One `captures` row (ADR-0199 decision 1): a rendered capture of a changed asset.
///
/// Public members, as [`super::ManifestEntry`]'s are: a capture carries no value the producer could get
/// wrong by construction.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct BinaryCapture {
    /// The capture's path, relative to the tree the reviewer reads.
    pub path: String,
    /// The viewport or procedure that took it.
    pub viewport: String,
    /// What it shows (S-BIN: the asset's object path).
    pub note: String,
}

/// What is wrong with a git-lfs pointer blob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerDefect {
    /// The blob is 1024 bytes or more.
    TooLong,
    /// The blob is not UTF-8.
    NotUtf8,
    /// The first line is not the spec's version line.
    NoVersionLine,
    /// No single well-formed `oid sha256:<64 hex>` line.
    Oid,
    /// No single `size <n>` line with a decimal `u64`.
    Size,
}

impl std::fmt::Display for PointerDefect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooLong => "it is 1024 bytes or more",
            Self::NotUtf8 => "it is not UTF-8",
            Self::NoVersionLine => {
                "its first line is not `version https://git-lfs.github.com/spec/v1`"
            }
            Self::Oid => "it holds no single `oid sha256:<64 lower-case hex>` line",
            Self::Size => "it holds no single `size <n>` line",
        })
    }
}

/// Why the `Binary` payload, or one of its rows, was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryChangeSummaryError {
    /// The input carries a text patch of a `Binary`-classified path (AC-497's AND).
    TextPatchForBinaryPath {
        /// The asset path the diff patches as text.
        path: String,
    },
    /// An asset's blob is not a git-lfs pointer, so its `size` and `lfs_oid` cannot be read.
    NotAnLfsPointer {
        /// The asset path.
        path: String,
        /// What is wrong with the blob.
        defect: PointerDefect,
    },
}

impl std::fmt::Display for BinaryChangeSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextPatchForBinaryPath { path } => write!(
                f,
                "the range diff carries a text patch of {path}, which classifies Binary: no \
                 textual diff of a binary path is assembled or dispatched (AC-497; ADR-0199 \
                 decision 2)"
            ),
            Self::NotAnLfsPointer { path, defect } => write!(
                f,
                "{path}: the blob is not a git-lfs pointer ({defect}), so its size and lfs_oid \
                 cannot be read from it (ADR-0199 decision 1)"
            ),
        }
    }
}

impl std::error::Error for BinaryChangeSummaryError {}
