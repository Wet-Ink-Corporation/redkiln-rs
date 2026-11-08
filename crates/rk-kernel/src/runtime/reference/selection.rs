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

use std::fmt;

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

/// One name from a `${…}` list (`tests_added`): non-empty, with no whitespace, no control
/// character and no shell syntax.
///
/// # Errors
/// [`SelectionRefusal::BadToken`] carrying the token.
pub fn check_selection_token(token: &str) -> Result<(), SelectionRefusal> {
    let bad = token.is_empty()
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
