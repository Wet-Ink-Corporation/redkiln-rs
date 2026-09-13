//! The port-inventory denylists — AC-483's git surface and AC-528's `rk-ports` clause.
//!
//! Both run over the **same source parse** the parity test uses, so a trait the parity count sees
//! is a trait these rules see, and the three assertions cannot disagree about what the crate
//! contains.
//!
//! **Each carries its own positive control, inline**, because the pass condition of a denylist is
//! the *absence* of output — Halt 8 shape (g) — and a rule that has never been shown able to
//! reject anything is not a rule. The controls run the identical predicate over a synthetic
//! offending set, so a predicate that silently stops matching fails these tests rather than
//! passing them.

use std::collections::BTreeSet;

use crate::source_parse::Source;

/// The git surface, as AC-483's own enumeration spells it (register :6957): *"trailers,
/// worktrees, clone, `reconcile`"*, plus the `GitPort` name its fixture declares.
const GIT_SURFACE: [&str; 5] = ["git", "trailer", "worktree", "clone", "reconcile"];

/// Realistic offending DECLARATIONS the future-shape predicate must reject, written out as
/// literals rather than derived from the rule under test.
///
/// **A control that builds its probe by interpolating the rule into it is a tautology** and cannot
/// fail for any value of the rule; that is the defect this array replaces. Each line here is a
/// declaration a port trait could plausibly carry, and the last two are the ordinary spellings the
/// previous substring list missed.
const OFFENDING_DECLARATIONS: [&str; 5] = [
    "    async fn probe(&self);",
    "    fn probe(&self) -> impl Future<Output = ()>;",
    "    fn probe(&self) -> Pin<Box<dyn Future<Output = ()>>>;",
    "    fn probe(&self) -> Box<dyn std::future::Future<Output = ()>>;",
    "    fn probe(&self) -> BoxFuture<'_, ()>;",
];

/// Declarations this crate actually carries, which the predicate must NOT reject. Without this
/// half the control is one-sided: a predicate that answered `true` for every line would satisfy
/// the offending set and silently red-flag the whole crate.
const PERMITTED_DECLARATIONS: [&str; 3] = [
    "    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome;",
    "    fn query(&self, by: Query) -> Result<Vec<Event>, StepError>;",
    "    fn drift(&self, changed_sources: &[String]) -> Result<Vec<Drift>, StepError>;",
];

/// Which future-shaped token, if any, a line carries — **the same alternation Checkpoint 7a and
/// `ci.yml` grep for**, `async fn|impl Future|-> .*Future|BoxFuture`, as a predicate.
///
/// It is a predicate and not a substring list because the two disagreed: a list carrying
/// `-> Pin<Box<dyn Future` is strictly narrower than the greps' `-> .*Future`, so the ordinary
/// spelling `-> Box<dyn Future<…>>` was caught by the checkpoint and missed by the test named for
/// the rule [measured: adding that method to `Tracker` left this test green and printed two
/// checkpoint hits]. A test and a grep that disagree about what a rule forbids are two rules.
fn future_shape_hit(line: &str) -> Option<&'static str> {
    for token in ["async fn", "impl Future", "BoxFuture"] {
        if line.contains(token) {
            return Some(token);
        }
    }
    // The greps' `-> .*Future` half: anything future-shaped in a RETURN position, whatever the
    // wrapper. Marginally wider than the regex (it does not require the space after `->`), and
    // wider is the safe direction here — the dangerous direction is the one measured above, a test
    // narrower than the gate it is half of.
    if let Some((_, returns)) = line.split_once("->")
        && returns.contains("Future")
    {
        return Some("-> …Future");
    }
    None
}

/// Which of `names` name the git surface, and on which token.
fn git_surface_hits<'a>(
    names: impl IntoIterator<Item = &'a String>,
    what: &str,
) -> BTreeSet<String> {
    let mut hits = BTreeSet::new();
    for name in names {
        let lower = name.to_lowercase();
        for token in GIT_SURFACE {
            if lower.contains(token) {
                hits.insert(format!("{what} `{name}` names the git surface (`{token}`)"));
            }
        }
    }
    hits
}

/// AC-483's `rk-ports` half: no trait here abstracts git, and a trait whose **name or methods**
/// match the git surface is rejected.
///
/// `rk-git` is a concrete crate the kernel names and its `GitOps` seam is a struct (ADR-0105
/// clause 1). The compile-fail half is `tests/ui/git_port_is_not_a_port.rs`, which shows that a
/// declared `GitPort` has nowhere in the port set to go.
#[test]
fn git_surface_denylist_rejects_git_port() {
    let source = Source::parse();
    // THE POSITIVE CONTROL, and it is the same predicate: the shape AC-483's fixture declares
    // must be rejected, or the two assertions below are vacuous.
    let fixture_traits = vec!["GitPort".to_owned()];
    let fixture_methods = vec![
        "trailer".to_owned(),
        "worktree".to_owned(),
        "clone_repo".to_owned(),
        "reconcile".to_owned(),
    ];
    let control = git_surface_hits(&fixture_traits, "trait");
    assert_eq!(
        control.len(),
        1,
        "the denylist does not reject `trait GitPort` — it is inert"
    );
    assert_eq!(
        git_surface_hits(&fixture_methods, "method").len(),
        4,
        "the denylist does not reject the git surface's method names — it is inert"
    );

    let offending: BTreeSet<String> = git_surface_hits(&source.traits, "port trait")
        .union(&git_surface_hits(&source.methods, "port method"))
        .cloned()
        .collect();
    assert!(
        offending.is_empty(),
        "git is deliberately not a port (AC-483; ADR-0003:44, :57-58): {offending:?}"
    );
    assert_eq!(
        source.traits.len(),
        5,
        "ADR-0003:34 — exactly five ports, no sixth: {:?}",
        source.traits
    );
}

/// AC-528's `rk-ports` clause: no port trait declares an `async fn` or a future-returning method.
///
/// The checkpoint's grep over `crates/rk-ports/src/` is the other half of this; this one runs off
/// the same parse as the parity count and carries the positive control the grep cannot.
#[test]
fn no_async_in_port_traits() {
    let source = Source::parse();

    // THE POSITIVE CONTROL, and it runs the REAL predicate over hard-coded offending text — the
    // shape `git_surface_hits`' control already had. Nothing here is derived from the predicate,
    // so a predicate that stops matching fails this test instead of passing it.
    for declaration in OFFENDING_DECLARATIONS {
        assert!(
            future_shape_hit(declaration).is_some(),
            "the future-shape denylist does not reject `{declaration}` — it is inert"
        );
    }
    // ...and the negative half, so an always-true predicate cannot satisfy the control either.
    for declaration in PERMITTED_DECLARATIONS {
        assert!(
            future_shape_hit(declaration).is_none(),
            "the future-shape denylist rejects a SYNC declaration this crate carries, \
             `{declaration}` — it is over-wide, which fails a correct crate (golden constraint 9)"
        );
    }

    let mut offending: Vec<String> = Vec::new();
    for (file, number, text) in &source.lines {
        if let Some(token) = future_shape_hit(text) {
            offending.push(format!("{file}:{number}: {} (`{token}`)", text.trim()));
        }
    }
    assert!(
        offending.is_empty(),
        "a port trait declares an async or future-returning method (ADR-0076; AC-528, register \
         :7159-7160). Port traits are sync because `async fn` in trait is still not \
         dyn-compatible on the pin: {offending:?}"
    );
}
