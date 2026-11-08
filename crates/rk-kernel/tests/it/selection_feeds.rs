//! **The agent-step feed check** (ADR-0205, proposed; D21-177; plan
//! `spikes/plans/s8-repro-aware-implement-2026-11-08.md` section 2, Decision D5 option i).
//!
//! An agent step whose output a `toolchain:test` step of the same activity selects with is checked
//! before commit: a string feed through the ONE `parse_selection`, each element of a list feed
//! through `check_selection_token`. These tests pin the catalog side ([`selection_feeds`]) and the
//! value side ([`check_feeds`]); the activity loop's use of them is in `feed_check`.

use std::path::PathBuf;

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::runtime::reference::{
    FeedRefusal, Seg, SelectionRefusal, check_feeds, selection_feeds,
};

use crate::reference_resolver::{step, with_inputs};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn shipped_steps(id: &str) -> Vec<rk_kernel::catalog::Step> {
    let catalog = Catalog::from_dir(&root().join(".redkiln").join("catalog"))
        .expect("the shipped catalog loads");
    catalog
        .activities()
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("the shipped catalog carries {id}"))
        .steps
        .clone()
}

fn agent(id: &str, inputs: &[(&str, &str)]) -> rk_kernel::catalog::Step {
    with_inputs(step(id, StepKind::Agent), inputs)
}

fn test_tool(id: &str, selection: &str) -> rk_kernel::catalog::Step {
    let mut s = with_inputs(step(id, StepKind::Tool), &[("selection", selection)]);
    s.run = Some("toolchain:test".to_owned());
    s
}

fn field(name: &str) -> Vec<Seg> {
    vec![Seg::Field(name.to_owned())]
}

/// The one feed of a step's slice, asserted to be exactly one.
fn only_feed(
    feeds: &std::collections::BTreeMap<String, Vec<rk_kernel::runtime::reference::SelectionFeed>>,
    agent: &str,
) -> rk_kernel::runtime::reference::SelectionFeed {
    let of = feeds
        .get(agent)
        .unwrap_or_else(|| panic!("{agent} has feeds: {feeds:?}"));
    assert_eq!(of.len(), 1, "{of:?}");
    of.first().expect("one feed").clone()
}

/// **Test 1.** The shipped `tdd` gives `red`'s `tests_added`, read by `red-verify`, and nothing for
/// `green` or `refactor` (their outputs feed no selection).
#[test]
fn selection_feeds_of_tdd_red_is_the_tests_added_field() {
    let feeds = selection_feeds(&shipped_steps("tdd-red-green-refactor"));
    assert_eq!(feeds.keys().collect::<Vec<_>>(), vec!["red"], "{feeds:?}");
    let feed = only_feed(&feeds, "red");
    assert_eq!(feed.path(), field("tests_added").as_slice());
    assert_eq!(feed.reader(), "red-verify");
}

/// **Test 2.** A plain `${attempt.outputs.command}` on a single-dispatch agent step is a string
/// feed at `command`.
#[test]
fn selection_feeds_of_a_string_ref_is_its_path() {
    let steps = vec![
        agent("attempt", &[("spec", "artifact:Spec")]),
        test_tool("confirm", "${attempt.outputs.command}"),
    ];
    let feeds = selection_feeds(&steps);
    assert_eq!(feeds.len(), 1, "{feeds:?}");
    let feed = only_feed(&feeds, "attempt");
    assert_eq!(feed.path(), field("command").as_slice());
    assert_eq!(feed.reader(), "confirm");
}

/// **Test 3.** A plain `${s.outputs.p}` on a `for_each` agent step reads the element LIST, not one
/// dispatch, so it is no per-dispatch feed; a `[*]` projection on it is. A ref to a tool step, an
/// optional ref and a literal selection are no feed either.
#[test]
fn selection_feeds_skips_a_plain_ref_to_a_for_each_step() {
    let mut each = agent("each", &[("ac", "${item}")]);
    each.for_each = Some("artifact:Spec.acceptance_criteria".to_owned());
    let steps = vec![
        each,
        test_tool("plain", "${each.outputs.tests_added}"),
        test_tool("optional", "${single.outputs.command?}"),
        test_tool("affected", "affected"),
        agent("single", &[]),
        test_tool("of-tool", "${plain.outputs.command}"),
    ];
    assert!(
        selection_feeds(&steps).is_empty(),
        "{:?}",
        selection_feeds(&steps)
    );

    let mut each = agent("each", &[("ac", "${item}")]);
    each.for_each = Some("artifact:Spec.acceptance_criteria".to_owned());
    let star = vec![each, test_tool("verify", "${each.outputs[*].tests_added}")];
    let feed = only_feed(&selection_feeds(&star), "each");
    assert_eq!(feed.path(), field("tests_added").as_slice());
}

/// **Test 4.** A spaced token is refused, naming the field, the element's index and the reader.
#[test]
fn check_feeds_refuses_a_spaced_token_naming_field_index_and_reader() {
    let feeds = selection_feeds(&shipped_steps("tdd-red-green-refactor"));
    let red = feeds.get("red").expect("red has a feed");
    let refusal = check_feeds(
        &serde_json::json!({"tests_added": ["ok", "doctor test"]}),
        red,
    )
    .expect_err("a spaced token is refused");
    assert_eq!(
        refusal,
        FeedRefusal::Token {
            path: ".tests_added".to_owned(),
            reader: "red-verify".to_owned(),
            index: 1,
            why: SelectionRefusal::BadToken {
                token: "doctor test".to_owned()
            },
        }
    );
    let text = refusal.to_string();
    assert!(
        text.starts_with("the field `.tests_added` feeds `red-verify`'s selection and ")
            && text.contains("[1]")
            && text.contains("BadToken")
            && text.contains("doctor test"),
        "{text}"
    );
}

fn command_feed() -> Vec<rk_kernel::runtime::reference::SelectionFeed> {
    let steps = vec![
        agent("attempt", &[]),
        test_tool("confirm", "${attempt.outputs.command}"),
    ];
    selection_feeds(&steps)
        .remove("attempt")
        .expect("attempt has a feed")
}

/// **Test 5a.** A prose string feed is refused, naming the `parse_selection` variant.
#[test]
fn check_feeds_refuses_prose_naming_the_variant() {
    let value = serde_json::json!({"command": "run `cargo test` then check"});
    let refusal = check_feeds(&value, &command_feed()).expect_err("prose is refused");
    let FeedRefusal::Command {
        path, reader, why, ..
    } = &refusal
    else {
        panic!("a string feed refuses as Command: {refusal:?}");
    };
    assert_eq!((path.as_str(), reader.as_str()), (".command", "confirm"));
    assert_eq!(*why, SelectionRefusal::ShellSyntax { found: '`', at: 4 });
    assert!(refusal.to_string().contains("ShellSyntax"), "{refusal}");
}

/// **Test 5b.** A missing field, or one that is neither a string nor a list of strings, is
/// `NotASelection`.
#[test]
fn check_feeds_refuses_a_missing_or_non_string_field() {
    let feeds = command_feed();
    for value in [
        serde_json::json!({}),
        serde_json::json!({"command": 7}),
        serde_json::json!({"command": null}),
        serde_json::json!({"command": ["a", 3]}),
    ] {
        assert_eq!(
            check_feeds(&value, &feeds),
            Err(FeedRefusal::NotASelection {
                path: ".command".to_owned(),
                reader: "confirm".to_owned(),
            }),
            "{value}"
        );
    }
}

/// **Test 5c.** An empty list passes (an element may name no test; `tdd`'s `red-named` judges the
/// aggregate), as do plain tokens and a plain command.
#[test]
fn check_feeds_passes_an_empty_list_and_plain_tokens() {
    let feeds = selection_feeds(&shipped_steps("tdd-red-green-refactor"));
    let red = feeds.get("red").expect("red has a feed");
    for value in [
        serde_json::json!({"tests_added": []}),
        serde_json::json!({"tests_added": ["doctor::margin_absent", "b"]}),
    ] {
        assert_eq!(check_feeds(&value, red), Ok(()), "{value}");
    }
    let value = serde_json::json!({"command": "cargo test -p rk-cli -- doctor::x --exact"});
    assert_eq!(check_feeds(&value, &command_feed()), Ok(()));
    assert_eq!(check_feeds(&value, &[]), Ok(()), "no feed checks nothing");
}

mod check_feeds_properties {
    use proptest::prelude::*;
    use rk_kernel::catalog::StepKind;
    use rk_kernel::runtime::reference::{check_feeds, check_selection_token, selection_feeds};

    use crate::reference_resolver::{step, with_inputs};

    proptest! {
        /// **Test 6.** For every string `s`, `check_feeds({"f":[s]})` is `Ok` iff
        /// `check_selection_token(s)` is `Ok`: the list feed has no second token rule.
        #[test]
        fn a_list_feed_agrees_with_check_selection_token(s in "\\PC{0,24}|[ a-z:_-]{0,12}") {
            let mut verify = with_inputs(step("v", StepKind::Tool), &[("selection", "${a.outputs.f}")]);
            verify.run = Some("toolchain:test".to_owned());
            let steps = vec![step("a", StepKind::Agent), verify];
            let feeds = selection_feeds(&steps).remove("a").expect("a has a feed");
            let value = serde_json::json!({ "f": [s.as_str()] });
            prop_assert_eq!(check_feeds(&value, &feeds).is_ok(), check_selection_token(&s).is_ok());
        }
    }
}
