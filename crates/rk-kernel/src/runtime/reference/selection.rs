//! **The ONE selection parser** (`s8d` §8d.3; ADR-0173; ADR-0204 decision 6, proposed).
//!
//! *(dated 2026-11-08, s8 repro-pair route.)* The 2026-11-08 relaunch 3 stopped at exit 14 because
//! an investigator's prose reached `toolchain:test` as a command, and the old parser split it after
//! ` -- ` into libtest arguments. [`parse_selection`] refuses by name any command that is not plain:
//! a newline, a control character, shell syntax, or no filter. [`selection_of`] keeps its signature
//! and meaning for its callers, and is exactly `parse_selection(command).ok()`.
//!
//! The kernel stays language-free: the check refuses shell syntax, not a non-cargo head, so
//! `pnpm test -- name` still parses.

use std::collections::BTreeMap;
use std::fmt;

use super::{Member, Seg, Source, at_path, parse_input, render_path};
use crate::catalog::{Step, StepKind};

/// The characters a plain command never holds: each is shell syntax, so a command holding one is
/// prose or a pipeline, and never a test selection.
const SHELL_SYNTAX: &[char] = &[
    '`', '$', ';', '|', '&', '<', '>', '(', ')', '{', '}', '"', '\'', '\\',
];

/// The separator between a command's head and its test filter.
const SEPARATOR: &str = " -- ";

/// Why a command, or one named test, is not a test selection (refused by name, before the port).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionRefusal {
    /// The command holds a line break, so it is more than one command.
    Multiline,
    /// The command holds a control character at byte offset `at`.
    Control {
        /// The byte offset of the first control character.
        at: usize,
    },
    /// The command holds the shell character `found` at byte offset `at`.
    ShellSyntax {
        /// The first shell character.
        found: char,
        /// Its byte offset.
        at: usize,
    },
    /// The command has no ` -- ` separator, so it names no test filter.
    NoFilter,
    /// The command's ` -- ` filter is empty, and an empty selection is never run (ADR-0148).
    EmptyFilter,
    /// One element of a `${…}` list is empty, or holds whitespace, a control character or shell
    /// syntax (ADR-0204 decision 6; Decision D5).
    BadToken {
        /// The element as resolved.
        token: String,
    },
}

impl fmt::Display for SelectionRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Multiline => f.write_str("Multiline: the command holds a line break"),
            Self::Control { at } => {
                write!(
                    f,
                    "Control: the command holds a control character at byte {at}"
                )
            }
            Self::ShellSyntax { found, at } => write!(
                f,
                "ShellSyntax: the command holds the shell character `{found}` at byte {at}, so it \
                 is not a plain command"
            ),
            Self::NoFilter => f.write_str("NoFilter: the command names no `--` test filter"),
            Self::EmptyFilter => {
                f.write_str("EmptyFilter: the command's `--` test filter is empty")
            }
            Self::BadToken { token } => write!(
                f,
                "BadToken: `{token}` is not one plain test name (it is empty, or holds \
                 whitespace, a control character or shell syntax)"
            ),
        }
    }
}

impl std::error::Error for SelectionRefusal {}

/// The plain tokens of `command` after its ` -- ` separator.
///
/// **The ONE selection parser** (`s8d` §8d.3): every agent-fed and repro-fed selection passes
/// through it (both lanes, `bisect`, `confirmation-test`, `review_pass`). A second parser is
/// forbidden (ADR-0173).
///
/// # Errors
/// [`SelectionRefusal`] naming the first reason the command is not plain, checked in this order: a
/// line break, a control character, a shell character, no filter, an empty filter.
pub fn parse_selection(command: &str) -> Result<Vec<String>, SelectionRefusal> {
    if command.contains(['\n', '\r']) {
        return Err(SelectionRefusal::Multiline);
    }
    if let Some((at, _)) = command.char_indices().find(|(_, c)| c.is_control()) {
        return Err(SelectionRefusal::Control { at });
    }
    if let Some((at, found)) = command
        .char_indices()
        .find(|(_, c)| SHELL_SYNTAX.contains(c))
    {
        return Err(SelectionRefusal::ShellSyntax { found, at });
    }
    let Some((_, tail)) = command.split_once(SEPARATOR) else {
        return Err(SelectionRefusal::NoFilter);
    };
    let selection: Vec<String> = tail.split_whitespace().map(str::to_owned).collect();
    if selection.is_empty() {
        return Err(SelectionRefusal::EmptyFilter);
    }
    Ok(selection)
}

/// The selection a `run` repro names: the tokens of `command` after a `--` separator.
///
/// **The ONE selection parser's `Option` face** (`s8d` §8d.3): moved here from
/// `rk_cli::review_pass`, which re-exports it with `pub use` so its path and its tests are
/// unchanged. `rk-kernel` cannot depend on `rk-cli`, and a second parser in the kernel is forbidden
/// (ADR-0173). *(dated 2026-11-08, ADR-0204 decision 6.)* It is exactly
/// `parse_selection(command).ok()`.
///
/// 🛑 **`None` rather than a guess, and `None` rather than an empty selection.** `Repro.command` is
/// a shell string and `Toolchain::test` takes a selection, and no record maps one to the other; the
/// narrowest rule that makes AC-288's four cells computable from what a `Repro` actually carries is
/// the explicit `--` filter. A command without one is refused into `Unverifiable { cannot_run }`,
/// which AC-287 forbids from blocking — and an empty selection is refused outright, because
/// `ac332_empty_selection_is_never_green` is the standing finding that an empty suite is not a pass.
#[must_use]
pub fn selection_of(command: &str) -> Option<Vec<String>> {
    parse_selection(command).ok()
}

/// One name from a `${…}` list (`tests_added`): non-empty, not led by `-`, with no whitespace, no
/// control character and no shell syntax.
///
/// A `-`-led token is a libtest flag (`--include-ignored`, `--ignored`, `--list`), not a name, so a
/// list cannot widen or replace the run it selects.
///
/// # Errors
/// [`SelectionRefusal::BadToken`] carrying the token.
pub fn check_selection_token(token: &str) -> Result<(), SelectionRefusal> {
    let bad = token.is_empty()
        || token.starts_with('-')
        || token
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || SHELL_SYNTAX.contains(&c));
    if bad {
        return Err(SelectionRefusal::BadToken {
            token: token.to_owned(),
        });
    }
    Ok(())
}

// =================================================================================================
// The agent-step feed check (ADR-0205, proposed; D21-177; plan s8-repro-aware-implement Decision
// D5 option i).
// =================================================================================================

/// The `run` of the step kind whose selection a feed reaches.
const TEST_VERB: &str = "toolchain:test";

/// One agent-step output field that a `toolchain:test` step of the same activity selects with.
///
/// The reader names it as `${<step>.outputs.<path>}` or `${<step>.outputs[*].<field>}`; it is
/// derived from the catalog at walk start by [`selection_feeds`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionFeed {
    /// The path inside ONE dispatch's value (for `[*]`, the element's field).
    path: Vec<Seg>,
    /// The `toolchain:test` step that reads it, for the refusal's text.
    reader: String,
}

impl SelectionFeed {
    /// The path inside one dispatch's value.
    #[must_use]
    pub fn path(&self) -> &[Seg] {
        &self.path
    }

    /// The `toolchain:test` step that selects with the field.
    #[must_use]
    pub fn reader(&self) -> &str {
        &self.reader
    }
}

/// Every feed of every agent step in `steps` (one activity's steps), keyed by agent step id.
///
/// Only a mandatory, plain `${<agent>.outputs…}` selection is a feed: an optional ref, a
/// coalesce, a literal (`affected`) and an `artifact:` selection read no fresh dispatch. A plain
/// `${s.outputs.p}` on a `for_each` agent step reads the element list, not one dispatch, so it
/// yields no per-dispatch feed (the tool step's own refusal stays the net for it); a `[*]`
/// projection on a step that is not a `for_each` reads a list output, and yields none either.
#[must_use]
pub fn selection_feeds(steps: &[Step]) -> BTreeMap<String, Vec<SelectionFeed>> {
    let mut feeds: BTreeMap<String, Vec<SelectionFeed>> = BTreeMap::new();
    for reader in steps
        .iter()
        .filter(|s| s.kind == StepKind::Tool && s.run.as_deref() == Some(TEST_VERB))
    {
        let Some(Ok(Source::StepRef {
            step,
            member: Member::Outputs,
            path,
            star,
            optional: false,
        })) = reader.inputs.get("selection").map(parse_input)
        else {
            continue;
        };
        let Some(agent) = steps
            .iter()
            .find(|s| s.id == step && s.kind == StepKind::Agent)
        else {
            continue;
        };
        let per_dispatch = match (star, agent.for_each.is_some()) {
            (Some(field), true) => field,
            (None, false) => path,
            (Some(_), false) | (None, true) => continue,
        };
        feeds.entry(step).or_default().push(SelectionFeed {
            path: per_dispatch,
            reader: reader.id.clone(),
        });
    }
    feeds
}

/// Why a dispatched value fails a feed (refused at the agent step as `StepError::Schema`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedRefusal {
    /// The path is absent from the value, or is neither a string nor a list of strings.
    NotASelection {
        /// The field path, rendered (`.tests_added`).
        path: String,
        /// The `toolchain:test` step that selects with it.
        reader: String,
    },
    /// A string feed that [`parse_selection`] refuses.
    Command {
        /// The field path, rendered.
        path: String,
        /// The `toolchain:test` step that selects with it.
        reader: String,
        /// The string as dispatched.
        value: String,
        /// The parser's refusal.
        why: SelectionRefusal,
    },
    /// One element of a list feed that [`check_selection_token`] refuses.
    Token {
        /// The field path, rendered.
        path: String,
        /// The `toolchain:test` step that selects with it.
        reader: String,
        /// The element's index in the list.
        index: usize,
        /// The token rule's refusal.
        why: SelectionRefusal,
    },
}

impl fmt::Display for FeedRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotASelection { path, reader } => write!(
                f,
                "the field `{path}` feeds `{reader}`'s selection and is absent, or is neither a \
                 string nor a list of strings"
            ),
            Self::Command {
                path,
                reader,
                value,
                why,
            } => write!(
                f,
                "the field `{path}` feeds `{reader}`'s selection and its value `{value}` is \
                 refused: {why}"
            ),
            Self::Token {
                path,
                reader,
                index,
                why,
            } => write!(
                f,
                "the field `{path}` feeds `{reader}`'s selection and its element [{index}] is \
                 refused: {why}"
            ),
        }
    }
}

impl std::error::Error for FeedRefusal {}

/// Check one dispatched value against its step's feeds.
///
/// An empty list passes: one element may name no test, and `tdd`'s `red-named` judges the
/// aggregate.
///
/// # Errors
/// The first [`FeedRefusal`], in feed order then element order.
pub fn check_feeds(value: &serde_json::Value, feeds: &[SelectionFeed]) -> Result<(), FeedRefusal> {
    feeds.iter().try_for_each(|feed| check_feed(value, feed))
}

fn check_feed(value: &serde_json::Value, feed: &SelectionFeed) -> Result<(), FeedRefusal> {
    let path = || render_path(&feed.path);
    let reader = || feed.reader.clone();
    let not_a_selection = || FeedRefusal::NotASelection {
        path: path(),
        reader: reader(),
    };
    match at_path(value, &feed.path) {
        Some(serde_json::Value::String(command)) => {
            parse_selection(command)
                .map(drop)
                .map_err(|why| FeedRefusal::Command {
                    path: path(),
                    reader: reader(),
                    value: command.clone(),
                    why,
                })
        }
        Some(serde_json::Value::Array(names)) => {
            for (index, name) in names.iter().enumerate() {
                let name = name.as_str().ok_or_else(not_a_selection)?;
                check_selection_token(name).map_err(|why| FeedRefusal::Token {
                    path: path(),
                    reader: reader(),
                    index,
                    why,
                })?;
            }
            Ok(())
        }
        _ => Err(not_a_selection()),
    }
}
