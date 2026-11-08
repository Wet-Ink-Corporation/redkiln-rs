//! **The repro pair**: `.redkiln/intake/<item>.repro.toml` and its `.repro.rs` (s8j §8j.5 step 3;
//! ADR-0204 decision 2, proposed).
//!
//! *(Added 2026-11-08, s8 repro-pair route.)* The fix lane reproduces from the item's committed
//! pair, not from agent prose. `rk-cli` reads both files from the run tree (the frozen commit's
//! bytes), parses the TOML into [`ReproPairSpec`](crate::repro_pair::ReproPairSpec), validates it
//! into [`ReproPair`](crate::repro_pair::ReproPair), and binds the result as the row input
//! `repro_place.pair` inside the [`ReproPairInput`](crate::repro_pair::ReproPairInput) envelope.
//! The envelope is always bound, so an absent or invalid pair is refused at `place`, by name, and
//! never at load for every item and stage.
//!
//! "Parse, don't validate" (API-03): a [`ReproPair`](crate::repro_pair::ReproPair) has private
//! fields and is built only through [`ReproPair::new`](crate::repro_pair::ReproPair::new) or a
//! deserialise that runs the same validation, so a decoded pair is never one `new` would refuse.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::digest::{Digest, sha256};

/// The cap on a pair's `.rs` and `.toml` read (H-13).
///
/// It is 64 KiB: the shipped `.repro.rs` is about 2 KiB, so the cap leaves a 30x margin and keeps
/// an untrusted file from becoming an unbounded prompt input.
pub const REPRO_PAIR_MAX_BYTES: usize = 64 * 1024;

/// `.redkiln/intake/<item>.repro.toml` as written (s8j §8j.5 step 3). A DTO: serde only, with no
/// rule checked; [`ReproPair::new`] checks them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReproPairSpec {
    /// `"it"` (appended to `crates/<crate>/tests/it/<module>.rs`) or `"unit"` (appended to
    /// `crates/<crate>/src/<file>`).
    pub kind: String,
    /// The package the repro belongs to.
    #[serde(rename = "crate")]
    pub krate: String,
    /// `it`: the one module of the `it` binary; `unit`: the appended module's full path.
    pub module: String,
    /// The test function's name.
    pub test: String,
    /// A substring of the red failure.
    pub expect: String,
    /// `unit` only: the file the repro is appended to, relative to `crates/<crate>/src/`.
    #[serde(default)]
    pub file: Option<String>,
}

/// A `unit` repro's target file, relative to `crates/<crate>/src/`: relative, made of plain
/// segments (no `..`, no empty segment, no backslash) and ending in `.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrcRelPath(String);

impl SrcRelPath {
    /// The path, `/`-separated.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Where the repro goes (API-02: a closed set, not a string plus an optional file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproTarget {
    /// Appended to `crates/<crate>/tests/it/<module>.rs`; `module` is one identifier.
    It,
    /// Appended to `crates/<crate>/src/<file>`; `module` is the appended module's full path.
    Unit {
        /// The target file, relative to `crates/<crate>/src/`.
        file: SrcRelPath,
    },
}

/// A validated pair: constructed only through [`ReproPair::new`] or a re-validating deserialise
/// (API-03).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ReproPairWire", into = "ReproPairWire")]
pub struct ReproPair {
    krate: String,
    module: String,
    test: String,
    expect: String,
    target: ReproTarget,
    source: String,
    source_digest: Digest,
    spec_digest: Digest,
}

/// The pair as it crosses a JSON boundary (the row input): the spec, the source, and the spec's
/// digest, which cannot be recomputed from the parsed spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReproPairWire {
    spec: ReproPairSpec,
    source: String,
    spec_sha256: Digest,
}

impl TryFrom<ReproPairWire> for ReproPair {
    type Error = ReproPairError;

    fn try_from(wire: ReproPairWire) -> Result<Self, Self::Error> {
        Self::validated(wire.spec, wire.source, wire.spec_sha256)
    }
}

impl From<ReproPair> for ReproPairWire {
    fn from(pair: ReproPair) -> Self {
        let (kind, file) = match pair.target {
            ReproTarget::It => ("it", None),
            ReproTarget::Unit { file } => ("unit", Some(file.0)),
        };
        Self {
            spec: ReproPairSpec {
                kind: kind.to_owned(),
                krate: pair.krate,
                module: pair.module,
                test: pair.test,
                expect: pair.expect,
                file,
            },
            source: pair.source,
            spec_sha256: pair.spec_digest,
        }
    }
}

impl ReproPair {
    /// Validate `spec` and `source`. `spec_bytes` is the `.repro.toml` as read, hashed so the
    /// journal names what ran.
    ///
    /// # Errors
    /// [`ReproPairError`] naming the first rule the pair breaks.
    pub fn new(
        spec: ReproPairSpec,
        source: String,
        spec_bytes: &[u8],
    ) -> Result<Self, ReproPairError> {
        Self::validated(spec, source, sha256(spec_bytes))
    }

    fn validated(
        spec: ReproPairSpec,
        source: String,
        spec_digest: Digest,
    ) -> Result<Self, ReproPairError> {
        let target = match (spec.kind.as_str(), spec.file) {
            ("it", None) => ReproTarget::It,
            ("it", Some(file)) => return Err(ReproPairError::ItTakesNoFile { file }),
            ("unit", None) => return Err(ReproPairError::UnitNeedsFile),
            ("unit", Some(file)) => ReproTarget::Unit {
                file: src_rel_path(file)?,
            },
            _ => return Err(ReproPairError::UnknownKind { found: spec.kind }),
        };
        if !is_crate_name(&spec.krate) {
            return Err(ReproPairError::BadCrate { found: spec.krate });
        }
        let module_ok = match target {
            ReproTarget::It => is_ident(&spec.module),
            ReproTarget::Unit { .. } => spec.module.split("::").all(is_ident),
        };
        if !module_ok {
            return Err(ReproPairError::BadModule { found: spec.module });
        }
        if !is_ident(&spec.test) {
            return Err(ReproPairError::BadTest { found: spec.test });
        }
        if spec.expect.trim().is_empty() {
            return Err(ReproPairError::EmptyExpect);
        }
        if source.len() > REPRO_PAIR_MAX_BYTES {
            return Err(ReproPairError::SourceTooLarge {
                bytes: source.len(),
                cap: REPRO_PAIR_MAX_BYTES,
            });
        }
        let found = source.matches(&format!("fn {}(", spec.test)).count();
        if found != 1 {
            return Err(ReproPairError::SourceLacksTest {
                test: spec.test,
                found,
            });
        }
        Ok(Self {
            krate: spec.krate,
            module: spec.module,
            test: spec.test,
            expect: spec.expect,
            target,
            source_digest: sha256(source.as_bytes()),
            source,
            spec_digest,
        })
    }

    /// The package the repro belongs to.
    #[must_use]
    pub fn krate(&self) -> &str {
        &self.krate
    }

    /// `it`: the `it` binary's module; `unit`: the appended module's full path.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// The test function's name.
    #[must_use]
    pub fn test(&self) -> &str {
        &self.test
    }

    /// The spec's `expect`: a substring of the red failure.
    #[must_use]
    pub fn expected(&self) -> &str {
        &self.expect
    }

    /// Where the repro goes.
    #[must_use]
    pub const fn target(&self) -> &ReproTarget {
        &self.target
    }

    /// The `.repro.rs` bytes, appended verbatim.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// sha256 of [`Self::source`].
    #[must_use]
    pub const fn source_digest(&self) -> &Digest {
        &self.source_digest
    }

    /// sha256 of the `.repro.toml` as read.
    #[must_use]
    pub const fn spec_digest(&self) -> &Digest {
        &self.spec_digest
    }
}

/// What `rk-cli` binds as `repro_place.pair`: an envelope, so absence refuses at `place`, by name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ReproPairInput {
    /// Both files read and the pair validated.
    Present(ReproPair),
    /// No `.repro.toml` for the item.
    Absent {
        /// The repo-relative paths looked for.
        looked_for: Vec<String>,
    },
    /// A read, parse or validation failure, rendered.
    Invalid {
        /// The repo-relative path that failed.
        path: String,
        /// Why it failed.
        why: String,
    },
}

/// Why a pair is refused (ERR-01..04). Manual `Display` and `Error`, as `acceptance.rs` does: this
/// crate takes no `thiserror`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproPairError {
    /// `kind` is neither `"it"` nor `"unit"`.
    UnknownKind {
        /// The kind as written.
        found: String,
    },
    /// A `unit` pair names no `file`.
    UnitNeedsFile,
    /// An `it` pair names a `file`.
    ItTakesNoFile {
        /// The file as written.
        file: String,
    },
    /// `crate` is empty or not `[a-z0-9_-]+`.
    BadCrate {
        /// The crate as written.
        found: String,
    },
    /// `module` is not one identifier (`it`) or not `ident(::ident)*` (`unit`).
    BadModule {
        /// The module as written.
        found: String,
    },
    /// `test` is not an identifier.
    BadTest {
        /// The test as written.
        found: String,
    },
    /// `file` is absolute, holds `..`, an empty segment or a backslash, or does not end in `.rs`.
    BadFile {
        /// The file as written.
        found: String,
    },
    /// `expect` is empty.
    EmptyExpect,
    /// The `.repro.rs` is over [`REPRO_PAIR_MAX_BYTES`].
    SourceTooLarge {
        /// Its length.
        bytes: usize,
        /// The cap.
        cap: usize,
    },
    /// `fn <test>(` does not occur exactly once in the `.repro.rs`.
    SourceLacksTest {
        /// The test name.
        test: String,
        /// How many times it occurs.
        found: usize,
    },
}

impl fmt::Display for ReproPairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKind { found } => {
                write!(f, "kind `{found}` is neither `it` nor `unit`")
            }
            Self::UnitNeedsFile => f.write_str("a `unit` pair needs a `file` under src/"),
            Self::ItTakesNoFile { file } => {
                write!(f, "an `it` pair takes no `file`, and names `{file}`")
            }
            Self::BadCrate { found } => {
                write!(f, "crate `{found}` is not a package name ([a-z0-9_-]+)")
            }
            Self::BadModule { found } => write!(
                f,
                "module `{found}` is not one identifier (`it`) or an identifier path (`unit`)"
            ),
            Self::BadTest { found } => write!(f, "test `{found}` is not an identifier"),
            Self::BadFile { found } => write!(
                f,
                "file `{found}` is not a relative `.rs` path of plain segments under src/"
            ),
            Self::EmptyExpect => f.write_str("expect is empty"),
            Self::SourceTooLarge { bytes, cap } => {
                write!(f, "the .repro.rs is {bytes} bytes, over the {cap}-byte cap")
            }
            Self::SourceLacksTest { test, found } => write!(
                f,
                "the .repro.rs holds `fn {test}(` {found} times, and it must hold it exactly once"
            ),
        }
    }
}

impl std::error::Error for ReproPairError {}

/// A Rust identifier as a test or module name: `[A-Za-z_][A-Za-z0-9_]*`, and not `_` alone.
fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && s != "_"
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// A package directory name: non-empty `[a-z0-9_-]+`.
fn is_crate_name(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// A `unit` target file: relative, plain `/`-separated segments, ending in `.rs`.
fn src_rel_path(file: String) -> Result<SrcRelPath, ReproPairError> {
    let plain_segment = |seg: &str| {
        !seg.is_empty()
            && seg != "."
            && seg != ".."
            && seg
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
    };
    let ok = file.ends_with(".rs") && file.split('/').all(plain_segment);
    if ok {
        Ok(SrcRelPath(file))
    } else {
        Err(ReproPairError::BadFile { found: file })
    }
}
