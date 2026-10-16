//! **The `Binary` arm's human-facing rendering** (AC-154; ADR-0199 decisions 1 and 2, proposed;
//! `s8i` §8i.8's Rust table). *(Added 2026-10-14, s8i build, package S8I-C5-STORE.)*
//!
//! A `ChangeSummary` whose range carries a `Binary` path serialises as four members, exactly as
//! `rk-ports` writes them: `assets` (each `path`, `kind`, `action`, `size`, `lfs_oid`),
//! `captures` (each `path`, `viewport`, `note`), `tests: { report_ref }` and
//! `build: { log_ref }`. `rk-store` does not depend on `rk-ports`, so the renderer takes that
//! serialised value, reads it into a private typed view member by member, and renders from the
//! view alone.
//!
//! **No binary content is embedded, parsed or diffed as text** (AC-154's AND). The renderer is
//! given the typed value and nothing else: no bytes, no tree, no path it opens. A member the arm
//! does not carry — an asset's `content`, a `diff`, a `Text` arm's `sha` or `manifest` — is
//! refused by name before anything renders ([`BinaryRenderError::Unknown`]), so content cannot
//! ride into the rendering on an extra member. Every string renders JSON-quoted, so no field's
//! text can open a fence, a heading or a diff in the rendering.
//!
//! The rendering is LF only and a pure function of the value: two renderings of one value are
//! byte-identical (AC-121's property).

use serde_json::{Map, Value as Json};

/// The members of the arm's top level, in rendering order.
const TOP: [&str; 4] = ["assets", "captures", "tests", "build"];
/// The members of one `assets` row, in rendering order (ADR-0199 decision 1).
const ASSET: [&str; 5] = ["path", "kind", "action", "size", "lfs_oid"];
/// The members of one `captures` row, in rendering order.
const CAPTURE: [&str; 3] = ["path", "viewport", "note"];
/// The one member of `tests`.
const TESTS: [&str; 1] = ["report_ref"];
/// The one member of `build`.
const BUILD: [&str; 1] = ["log_ref"];

/// Why the `Binary` arm did not render. Every variant names where (`at`, a JSON pointer, `""`
/// for the top level) and which member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryRenderError {
    /// A member the arm carries is absent.
    Missing {
        /// The object it is absent from.
        at: String,
        /// The absent member.
        member: &'static str,
    },
    /// A member is present but not of its typed shape.
    Mistyped {
        /// The object it sits in.
        at: String,
        /// The member.
        member: &'static str,
        /// The shape the arm declares for it.
        expected: &'static str,
    },
    /// A member the arm does not carry: refused by name and never rendered, because an extra
    /// member is how content or a diff would ride into the rendering.
    Unknown {
        /// The object it sits in.
        at: String,
        /// The member, as the value spells it.
        member: String,
    },
}

impl std::fmt::Display for BinaryRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing { at, member } => write!(
                f,
                "the Binary arm at `{at}` has no `{member}`; every typed field is required \
                 (ADR-0199 decision 1)"
            ),
            Self::Mistyped {
                at,
                member,
                expected,
            } => write!(f, "the Binary arm's `{at}/{member}` is not {expected}"),
            Self::Unknown { at, member } => write!(
                f,
                "the Binary arm at `{at}` carries `{member}`, which is not one of its typed \
                 fields; no content, diff or other member is rendered (AC-154)"
            ),
        }
    }
}

impl std::error::Error for BinaryRenderError {}

/// How an asset changed over the range: `rk-ports`' `ChangeKind`, a closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Added,
    Deleted,
    Modified,
}

impl Action {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Deleted => "deleted",
            Self::Modified => "modified",
        }
    }
}

/// One `assets` row, typed.
struct Asset<'v> {
    path: &'v str,
    kind: &'v str,
    action: Action,
    size: u64,
    lfs_oid: &'v str,
}

/// One `captures` row, typed.
struct Capture<'v> {
    path: &'v str,
    viewport: &'v str,
    note: &'v str,
}

/// The arm, typed: what the rendering is a function of.
struct View<'v> {
    assets: Vec<Asset<'v>>,
    captures: Vec<Capture<'v>>,
    report_ref: &'v str,
    log_ref: &'v str,
}

/// `value` as an object whose members are exactly `allowed` (an unknown member is refused first,
/// by name; then a missing one).
fn exact<'v>(
    value: &'v Json,
    at: &str,
    member: &'static str,
    allowed: &[&'static str],
) -> Result<&'v Map<String, Json>, BinaryRenderError> {
    let map = value
        .as_object()
        .ok_or_else(|| BinaryRenderError::Mistyped {
            at: parent_of(at),
            member,
            expected: "an object",
        })?;
    if let Some(extra) = map.keys().find(|k| !allowed.contains(&k.as_str())) {
        return Err(BinaryRenderError::Unknown {
            at: at.to_owned(),
            member: extra.to_owned(),
        });
    }
    if let Some(absent) = allowed.iter().find(|m| !map.contains_key(**m)) {
        return Err(BinaryRenderError::Missing {
            at: at.to_owned(),
            member: absent,
        });
    }
    Ok(map)
}

/// The pointer of the object `at` sits in: `/assets` for `/assets/0`, `""` for `/tests`.
fn parent_of(at: &str) -> String {
    at.rsplit_once('/')
        .map_or_else(String::new, |(parent, _)| parent.to_owned())
}

/// The member `name` of `map`, refused by name when it is absent.
fn member<'v>(
    map: &'v Map<String, Json>,
    at: &str,
    name: &'static str,
) -> Result<&'v Json, BinaryRenderError> {
    map.get(name).ok_or_else(|| BinaryRenderError::Missing {
        at: at.to_owned(),
        member: name,
    })
}

fn text_of<'v>(
    map: &'v Map<String, Json>,
    at: &str,
    name: &'static str,
) -> Result<&'v str, BinaryRenderError> {
    member(map, at, name)?
        .as_str()
        .ok_or_else(|| BinaryRenderError::Mistyped {
            at: at.to_owned(),
            member: name,
            expected: "a string",
        })
}

fn list_of<'v>(
    map: &'v Map<String, Json>,
    name: &'static str,
) -> Result<&'v [Json], BinaryRenderError> {
    member(map, "", name)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| BinaryRenderError::Mistyped {
            at: String::new(),
            member: name,
            expected: "an array",
        })
}

fn asset<'v>(row: &'v Json, at: &str) -> Result<Asset<'v>, BinaryRenderError> {
    let map = exact(row, at, "assets", &ASSET)?;
    let action = match text_of(map, at, "action")? {
        "added" => Action::Added,
        "deleted" => Action::Deleted,
        "modified" => Action::Modified,
        _ => {
            return Err(BinaryRenderError::Mistyped {
                at: at.to_owned(),
                member: "action",
                expected: "one of `added`, `deleted`, `modified`",
            });
        }
    };
    let size = member(map, at, "size")?
        .as_u64()
        .ok_or_else(|| BinaryRenderError::Mistyped {
            at: at.to_owned(),
            member: "size",
            expected: "a whole number of bytes",
        })?;
    let lfs_oid = text_of(map, at, "lfs_oid")?;
    let is_oid = lfs_oid.len() == 64
        && lfs_oid
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if !is_oid {
        return Err(BinaryRenderError::Mistyped {
            at: at.to_owned(),
            member: "lfs_oid",
            expected: "64 lower-case hex",
        });
    }
    Ok(Asset {
        path: text_of(map, at, "path")?,
        kind: text_of(map, at, "kind")?,
        action,
        size,
        lfs_oid,
    })
}

fn capture<'v>(row: &'v Json, at: &str) -> Result<Capture<'v>, BinaryRenderError> {
    let map = exact(row, at, "captures", &CAPTURE)?;
    Ok(Capture {
        path: text_of(map, at, "path")?,
        viewport: text_of(map, at, "viewport")?,
        note: text_of(map, at, "note")?,
    })
}

/// The value, read into the typed view: the only place the value's members are looked at.
fn view(value: &Json) -> Result<View<'_>, BinaryRenderError> {
    let top = value
        .as_object()
        .ok_or_else(|| BinaryRenderError::Mistyped {
            at: String::new(),
            member: "assets",
            expected: "an object holding the Binary arm's members",
        })?;
    if let Some(extra) = top.keys().find(|k| !TOP.contains(&k.as_str())) {
        return Err(BinaryRenderError::Unknown {
            at: String::new(),
            member: extra.to_owned(),
        });
    }
    let assets = list_of(top, "assets")?
        .iter()
        .enumerate()
        .map(|(i, row)| asset(row, &format!("/assets/{i}")))
        .collect::<Result<Vec<_>, _>>()?;
    let captures = list_of(top, "captures")?
        .iter()
        .enumerate()
        .map(|(i, row)| capture(row, &format!("/captures/{i}")))
        .collect::<Result<Vec<_>, _>>()?;
    let tests = exact(member(top, "", "tests")?, "/tests", "tests", &TESTS)?;
    let build = exact(member(top, "", "build")?, "/build", "build", &BUILD)?;
    Ok(View {
        assets,
        captures,
        report_ref: text_of(tests, "/tests", "report_ref")?,
        log_ref: text_of(build, "/build", "log_ref")?,
    })
}

/// Appends `s` as a JSON string literal: quoted, with `"`, `\` and every control character escaped, so
/// no field's text can break a line or open markdown structure in the rendering.
fn push_quoted(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                // Every control character is at most U+009F, so four hex digits hold it.
                out.push_str("\\u");
                let n = u32::from(c);
                for shift in [12_u32, 8, 4, 0] {
                    out.push(char::from_digit((n >> shift) & 0xf, 16).unwrap_or('0'));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// `- <name>: <quoted text>` and a newline.
fn push_text_line(out: &mut String, name: &str, text: &str) {
    out.push_str("- ");
    out.push_str(name);
    out.push_str(": ");
    push_quoted(out, text);
    out.push('\n');
}

/// `- <name>: <bare token>` and a newline, for a value the view has already typed.
fn push_bare_line(out: &mut String, name: &str, token: &str) {
    out.push_str("- ");
    out.push_str(name);
    out.push_str(": ");
    out.push_str(token);
    out.push('\n');
}

/// `### <what> <n>` and a blank line.
fn push_entry_heading(out: &mut String, what: &str, n: usize) {
    out.push_str("### ");
    out.push_str(what);
    out.push(' ');
    out.push_str(&n.to_string());
    out.push_str("\n\n");
}

/// The rendering of the typed view: a pure function of it.
fn render_view(v: &View<'_>) -> String {
    let mut out = String::from(
        "# ChangeSummary (Binary)\n\nRendered from the typed fields of the `Binary` arm \
         (ADR-0199). No asset content is embedded, parsed or diffed as text.\n\n## assets\n\n",
    );
    if v.assets.is_empty() {
        out.push_str("_none_\n\n");
    }
    for (n, a) in (1_usize..).zip(&v.assets) {
        push_entry_heading(&mut out, "asset", n);
        push_text_line(&mut out, "path", a.path);
        push_text_line(&mut out, "kind", a.kind);
        push_bare_line(&mut out, "action", a.action.as_str());
        push_bare_line(&mut out, "size", &a.size.to_string());
        push_bare_line(&mut out, "lfs_oid", a.lfs_oid);
        out.push('\n');
    }
    out.push_str("## captures\n\n");
    if v.captures.is_empty() {
        out.push_str("_none_\n\n");
    }
    for (n, c) in (1_usize..).zip(&v.captures) {
        push_entry_heading(&mut out, "capture", n);
        push_text_line(&mut out, "path", c.path);
        push_text_line(&mut out, "viewport", c.viewport);
        push_text_line(&mut out, "note", c.note);
        out.push('\n');
    }
    out.push_str("## tests\n\n");
    push_text_line(&mut out, "report_ref", v.report_ref);
    out.push_str("\n## build\n\n");
    push_text_line(&mut out, "log_ref", v.log_ref);
    out
}

/// **The `Binary` arm's rendering** (AC-154): `value` is the arm's serialised form, read into its
/// typed fields, and the rendering is a function of those fields alone. LF only.
///
/// # Errors
///
/// [`BinaryRenderError`] naming the member: a typed field absent ([`BinaryRenderError::Missing`])
/// or of the wrong shape ([`BinaryRenderError::Mistyped`]), or a member the arm does not carry
/// ([`BinaryRenderError::Unknown`]). Nothing renders on a refusal.
pub fn render_binary_summary(value: &Json) -> Result<Vec<u8>, BinaryRenderError> {
    Ok(render_view(&view(value)?).into_bytes())
}
