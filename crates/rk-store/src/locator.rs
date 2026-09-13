//! `store::locator` — the **one** module allowed to look at markdown structure (AC-120).
//!
//! It answers exactly one question: where, in bytes, is the section under this anchor? It returns a
//! byte range and nothing derived from the content, because *"the section's content reaches the
//! rest of the store only as an uninterpreted byte slice"* (AC-149) and no typed field is ever
//! populated from anything inside it.
//!
//! **The end rule, as amended (PIA-033; design §5.1 row 4).** An anchor is a heading at the
//! template's declared level for that section. The slice runs from the line after the anchor to the
//! next heading **at that level or shallower**, or to end of file. **Deeper headings inside the
//! slice are prose bytes and not anchors** — that last clause is what makes a `###` inside a `##`
//! section part of the prose a human may rewrite freely (AC-152) rather than a boundary.
//!
//! **Decoys are not anchors** (AC-150). A heading-looking line inside a fenced code block, an
//! indented block, an HTML comment or a blockquote is content. So is a genuine heading at a deeper
//! level inside another section's slice. Each has a fixture.
//!
//! **Zero or more than one genuine occurrence is a named error with the count** (AC-151) — never a
//! first-match, nearest-match or fuzzy-match range. The mutation test for this criterion changes the
//! error path to first-match and must fail the suite.

use std::ops::Range;

/// A heading anchor: its text and the level the template declares for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    pub text: String,
    pub level: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocatorError {
    /// The anchor occurs zero times, or more than once as a genuine heading at the declared level.
    /// **The count is part of the error**: "not found" and "found four" are different situations
    /// and an operator needs to know which.
    Occurrences {
        anchor: String,
        level: usize,
        count: usize,
    },
}

impl std::fmt::Display for LocatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Occurrences {
                anchor,
                level,
                count,
            } => write!(
                f,
                "anchor `{anchor}` at level {level} occurs {count} times; exactly one genuine \
                 heading is required. No first-match, nearest-match or fuzzy-match range is \
                 returned (AC-151)."
            ),
        }
    }
}

/// Resolve `anchor` in `doc`, returning the byte range of its section's content.
///
/// # Errors
/// [`LocatorError::Occurrences`] when the anchor does not occur exactly once as a genuine heading
/// at its declared level.
pub fn resolve(doc: &str, anchor: &Anchor) -> Result<Range<usize>, LocatorError> {
    let lines = scan(doc);

    let matches: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| {
            l.heading
                .as_ref()
                .is_some_and(|h| h.level == anchor.level && h.text == anchor.text)
        })
        .map(|(i, _)| i)
        .collect();

    if matches.len() != 1 {
        return Err(LocatorError::Occurrences {
            anchor: anchor.text.clone(),
            level: anchor.level,
            count: matches.len(),
        });
    }

    let start_line = matches[0];
    let start = lines[start_line].end; // the line AFTER the anchor
    let mut end = doc.len();
    for l in lines.iter().skip(start_line + 1) {
        // The end rule: the next heading at that level OR SHALLOWER. A deeper heading is prose.
        if l.heading.as_ref().is_some_and(|h| h.level <= anchor.level) {
            end = l.start;
            break;
        }
    }
    Ok(start..end)
}

struct Line {
    start: usize,
    end: usize,
    heading: Option<Heading>,
}

struct Heading {
    level: usize,
    text: String,
}

/// Scan the document once, marking which lines are genuine headings.
///
/// The state this carries is exactly what tells a heading from a decoy: whether we are inside a
/// fenced code block, and whether we are inside an HTML comment. Indentation and blockquote markers
/// are decided per line.
fn scan(doc: &str) -> Vec<Line> {
    let mut out = Vec::new();
    let mut in_fence = false;
    let mut fence_marker = String::new();
    let mut in_html_comment = false;
    let mut offset = 0usize;

    for raw in doc.split_inclusive('\n') {
        let start = offset;
        offset += raw.len();
        let line = raw.trim_end_matches(['\n', '\r']);
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        // Fenced code blocks, ``` or ~~~. A fence closes only on its own marker kind.
        let is_fence = trimmed.starts_with("```") || trimmed.starts_with("~~~");
        if is_fence {
            let marker: String = trimmed
                .chars()
                .take_while(|c| *c == '`' || *c == '~')
                .collect();
            if in_fence {
                if marker.starts_with(fence_marker.chars().next().unwrap_or('`'))
                    && marker.len() >= fence_marker.len()
                {
                    in_fence = false;
                    fence_marker.clear();
                }
            } else {
                in_fence = true;
                fence_marker = marker;
            }
            out.push(Line {
                start,
                end: offset,
                heading: None,
            });
            continue;
        }

        // HTML comments may span lines.
        if !in_fence {
            if in_html_comment {
                if trimmed.contains("-->") {
                    in_html_comment = false;
                }
                out.push(Line {
                    start,
                    end: offset,
                    heading: None,
                });
                continue;
            }
            if trimmed.starts_with("<!--") && !trimmed.contains("-->") {
                in_html_comment = true;
                out.push(Line {
                    start,
                    end: offset,
                    heading: None,
                });
                continue;
            }
        }

        let heading = if in_fence || in_html_comment {
            None
        } else if indent >= 4 {
            // An indented block is code, not a heading.
            None
        } else if trimmed.starts_with('>') {
            // A blockquote's contents are quoted, not structural.
            None
        } else {
            parse_heading(trimmed)
        };

        out.push(Line {
            start,
            end: offset,
            heading,
        });
    }
    out
}

fn parse_heading(trimmed: &str) -> Option<Heading> {
    if !trimmed.starts_with('#') {
        return None;
    }
    let level = trimmed.chars().take_while(|c| *c == '#').count();
    let rest = &trimmed[level..];
    // ATX headings require a space after the hashes; `#hashtag` is not a heading.
    if !rest.starts_with(' ') {
        return None;
    }
    Some(Heading {
        level,
        text: rest.trim().trim_end_matches('#').trim().to_string(),
    })
}
