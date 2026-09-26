//! **AC-495** — pack validation over two fixture packs, asserting the **span location** on the
//! failing one and clean validation on the routed one.
//!
//! *(dated 2026-09-25, s8b refuting round 4)* **Not AC-495 only since `s8b`.** Two catalog tests sit
//! here too, and neither reads a pack: `tdd_red_verify_projects_tests_added_per_adr_0035` (D21-43
//! **N13**) and `sca_scan_blocks_on_unrated_or_routes_to_accept_findings` (D21-43 **N25**). Each
//! reads the edited activity from `.redkiln/catalog`, versioned under **ADR-0072**. The N13 test
//! also reads the working tree's `spikes/p9/schemas.toml` as the schema stand-in. That read is live,
//! not a pin to `P9_PIN`: it measures no historical p9 figure, and `s8b` §8b.3 inventories it as a
//! non-pin reader. The AC-495 tables below cover the AC-495 tests only.
//!
//! > **AC-495** — GIVEN a process pack whose only route through a stage requires a capability no
//! > configured adapter claims / WHEN the pack is validated at startup / THEN validation fails with
//! > a span pointing at the requiring row, before any activity runs / AND a pack that declares an
//! > alternative route for that capability validates instead of failing.
//! > *Verified by:* a pack-validation test with spans over **two fixture packs (one unroutable, one
//! > routed)**, asserting the **span location** on the failing one and **clean validation** on the
//! > routed one.
//!
//! # 🛑 THE NOTE, AND WHAT THIS FILE THEREFORE DOES NOT ASSERT
//!
//! AC-495's *Note*: the earlier draft also asserted that the alternative branch is **taken at
//! runtime**; that clause was **trimmed** because *"it is already AC-500's subject"*. Nothing here
//! dispatches, nothing here chooses a route, and **AC-500 is ARMED at this stage and is not
//! claimed** (`s7b`:1791).
//!
//! # 🛑 WHAT IS MEASURED AND WHAT IS NOT
//!
//! | clause | state | why |
//! | --- | --- | --- |
//! | *"a span pointing at the requiring row"* | **measured twice over** — the line is asserted as a literal **and** the fixture's own text at that line is read back and matched | a rule returning a plausible-but-wrong line passes one of those and fails the other |
//! | *"an alternative route … validates instead of failing"* | **measured, and shown to depend on the row** — the routed fixture's alternative block is deleted from its text in-test and the rule is re-run, which must then fail | a green over a second file proves nothing about *why* it is green |
//! | *"before any activity runs"* | **a type fact, and an ARMED half** — the rule binds no `Ports`, no `Runtime` and no `Harness`, so nothing it can reach dispatches; the **ordering** inside a live run needs a composition root and is `rk-cli`'s, **Step 7d** | `src/pack_validation.rs`'s header carries the same table |
//! | the `capability` key's **spelling** on a `[[stage.<name>.require]]` row | **a READING**, and the same one `crate::select::pack` already records for `[retry] attempts_cap`, `[breaker] consecutive_platform_failures`, `appetite.tokens` and `[degradation] permit_substitution` | no record states it: `grep -rn capabilit spikes/p9/packs/` → **0** hits across all twelve shipped packs [measured 2026-09-14]. A record naming it is **OWED**, and it is named here rather than smoothed over |
//! | AC-029's / AC-033's pack-validation halves | **not touched** — `Pack::require_attempt_cap` and `require_breaker_threshold` are Slice 4b's and this rule neither calls nor duplicates them | |

use rk_kernel::pack_validation::{RoutingViolation, Span, pack_capability_routing_rule};
use rk_kernel::predicate::{Context, Expr, Tri, Value as PValue, evaluate, parse};
use rk_kernel::resolver::{CapabilityClaim, ClaimedCapabilities};
use rk_kernel::toml_subset;

/// The unroutable pack: one route to `AffectedTestReport`, needing a token nothing claims.
const UNROUTABLE: &str = include_str!("../fixtures/packs/ac495-unroutable.toml");

/// The routed pack: the same file, plus one alternative row, plus a different `id`.
const ROUTED: &str = include_str!("../fixtures/packs/ac495-routed.toml");

const UNROUTABLE_NAME: &str = "ac495-unroutable.toml";
const ROUTED_NAME: &str = "ac495-routed.toml";

/// The token design:1031 names as the one a toolchain may not claim without a probe.
const NEEDED: &str = "affected-selection";

/// The token design:1031 says the pack routes to instead.
const ALTERNATIVE: &str = "whole-suite";

/// The artifact the two fixtures' `verify` stage requires.
const ARTIFACT: &str = "AffectedTestReport";

/// **The span the criterion names**, as a literal. `[[stage.verify.require]]` in the unroutable
/// fixture.
const ROW_LINE: usize = 34;

/// Its `capability = "affected-selection"` key.
const CAPABILITY_LINE: usize = 37;

/// The four lines `ac495-routed.toml` adds and `ac495-unroutable.toml` does not have.
const ALTERNATIVE_BLOCK: [&str; 4] = [
    "[[stage.verify.require]]",
    "when = \"changed_paths > 0\"",
    "artifacts = [\"AffectedTestReport\"]",
    "capability = \"whole-suite\"",
];

/// A registry that claims the alternative and omits the one the first row needs.
///
/// **Not empty**: a rule that refused every pack would pass against a registry that claimed nothing.
fn configured_adapter() -> ClaimedCapabilities {
    ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new(ALTERNATIVE, "2026-09-14", "toolchain:whole-suite@1").expect("a row"),
        CapabilityClaim::new("json-schema", "2026-09-14", "harness:schema-probe@3").expect("a row"),
    ])
    .expect("two distinct tokens")
}

/// The same registry plus the token the first row needs.
fn adapter_claiming_everything() -> ClaimedCapabilities {
    ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new(ALTERNATIVE, "2026-09-14", "toolchain:whole-suite@1").expect("a row"),
        CapabilityClaim::new("json-schema", "2026-09-14", "harness:schema-probe@3").expect("a row"),
        CapabilityClaim::new(NEEDED, "2026-09-14", "toolchain:affected@2").expect("a row"),
    ])
    .expect("three distinct tokens")
}

/// The 1-based line of a source, for reading a span back against the file it points into.
fn line_at(src: &str, line: usize) -> &str {
    src.lines()
        .nth(line - 1)
        .unwrap_or_else(|| panic!("line {line} is past the end of this fixture"))
}

/// Every non-comment, non-blank line, which is the pack's **body** — what the loader sees.
fn body(src: &str) -> Vec<&str> {
    src.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// The single `StageUnroutable` a violation list is expected to hold.
fn the_unroutable(violations: &[RoutingViolation]) -> &RoutingViolation {
    let found: Vec<&RoutingViolation> = violations
        .iter()
        .filter(|v| v.as_str() == "stage_unroutable")
        .collect();
    assert_eq!(
        found.len(),
        1,
        "exactly one stage is unroutable in this fixture; got {violations:?}"
    );
    found[0]
}

// ---- the criterion ------------------------------------------------------------------------------

/// **AC-495.** Two fixture packs, the span location on the failing one, clean validation on the
/// routed one.
///
/// **What would make this vacuous, and what stops each:**
///
/// - *A rule that fails every pack* passes the first half. The routed fixture must validate
///   **clean**, and its report's counts are read so *"clean"* is not *"walked nothing"*.
/// - *A rule that passes every pack* passes the second half. The unroutable fixture must fail.
/// - *A rule whose span is a constant, or the file's first line, or the last* passes a bare
///   *"it reported a span"*. The line is asserted as a **literal** and the fixture's own text at
///   that line is read back: the row span must be the `[[stage.verify.require]]` header and the
///   capability span its `capability = "…"` key.
/// - *Two fixtures that differ in some other way* would make the green mean nothing. Their bodies
///   are compared — the routed one is the unroutable one plus four lines and a different `id` — and
///   then the four lines are **deleted in-test** and the rule re-run, which must fail.
/// - *A rule that ignores the claimed set* passes both halves. The unroutable fixture is validated a
///   second time against a registry that claims the token, and must come out clean.
/// - *A rule that refuses stages rather than routes* would also refuse `[stage.review]`, which
///   mentions no capability at all. The violation list is asserted to name `verify` and nothing else.
#[test]
fn ac495_pack_validation() {
    let claimed = configured_adapter();

    // PREMISE — the configured adapter claims the alternative and omits the token the first row
    // needs. Both halves, because a refusal against an adapter claiming nothing is not the GIVEN.
    assert!(!claimed.claims_token(NEEDED), "the token must be unclaimed");
    assert!(
        claimed.claims_token(ALTERNATIVE),
        "and the alternative must be claimed, or the routed pack is routed by nothing"
    );

    // ---- THE FAILING PACK — "validation fails with a span pointing at the requiring row" --------
    let violations = pack_capability_routing_rule(UNROUTABLE_NAME, UNROUTABLE, &claimed)
        .expect_err(
            "AC-495: a pack whose only route needs an unclaimed capability does not validate",
        );

    let violation = the_unroutable(&violations);
    let RoutingViolation::StageUnroutable {
        stage,
        artifact,
        capability,
        site,
        row,
        capability_span,
        routes_considered,
    } = violation
    else {
        panic!("the_unroutable returned something else: {violation:?}");
    };

    assert_eq!(stage, "verify", "the stage with no route");
    assert_eq!(artifact, ARTIFACT, "the artifact with no route");
    assert_eq!(
        capability, NEEDED,
        "and the token the row needs — the one the run would fail `Unsupported` on (AC-494)"
    );
    assert_eq!(
        site, "ac495-unroutable.toml: stage.verify.require[0]",
        "the loader's own site vocabulary, so a reader can find the row by path as well as by line"
    );
    assert_eq!(
        *routes_considered, 1,
        "this artifact has exactly one route, which is what makes it the ONLY route"
    );

    // THE SPAN LOCATION — asserted as a literal AND read back against the fixture's own text.
    assert_eq!(
        row,
        &Span::new(UNROUTABLE_NAME, ROW_LINE),
        "AC-495: the span points at the REQUIRING ROW"
    );
    assert_eq!(
        line_at(UNROUTABLE, row.line()).trim(),
        "[[stage.verify.require]]",
        "and the line it points at IS the requiring row's header — a literal alone would still pass \
         if the fixture were renumbered under it"
    );
    assert_eq!(
        capability_span,
        &Span::new(UNROUTABLE_NAME, CAPABILITY_LINE)
    );
    assert_eq!(
        line_at(UNROUTABLE, capability_span.line()).trim(),
        "capability = \"affected-selection\"",
        "and the capability span points at the key that declares the requirement"
    );
    assert_eq!(
        row.source(),
        UNROUTABLE_NAME,
        "the span names the file the caller named"
    );
    assert_eq!(
        row.to_string(),
        format!("{UNROUTABLE_NAME}:{ROW_LINE}"),
        "and it renders `file:line`, leading with the location"
    );

    // The message names the stage, the artifact and the capability — a reader can act on it without
    // opening the file.
    let rendered = violation.to_string();
    for needle in [UNROUTABLE_NAME, "verify", ARTIFACT, NEEDED] {
        assert!(
            rendered.contains(needle),
            "the message names {needle:?}: {rendered}"
        );
    }

    // ONLY `verify` IS REFUSED. `[stage.review]` mentions no capability and must stay routable, or
    // the rule is refusing stages rather than refusing routes.
    assert_eq!(
        violations.len(),
        1,
        "one violation, and `[stage.review]` is not one of them: {violations:?}"
    );

    // ---- THE ROUTED PACK — "validates instead of failing" ---------------------------------------
    let report = pack_capability_routing_rule(ROUTED_NAME, ROUTED, &claimed)
        .expect("AC-495: a pack that declares an alternative route validates");

    // CLEAN, and not "walked nothing": every count is read.
    assert_eq!(report.stages_walked(), 2, "review and verify");
    assert_eq!(
        report.rows_walked(),
        2,
        "the original row and the alternative"
    );
    assert_eq!(report.rows_without_artifacts(), 0);
    assert_eq!(
        report.artifacts_checked(),
        3,
        "ReviewVerdict, ConfirmationResult and AffectedTestReport"
    );
    assert_eq!(
        report.routes_checked(),
        4,
        "two baseline entries and two require-row routes"
    );
    assert_eq!(
        report.capabilities_required(),
        [NEEDED.to_owned(), ALTERNATIVE.to_owned()],
        "both tokens are still REQUIRED by the routed pack — the alternative does not delete the \
         first row's requirement, it routes around it"
    );
    assert_eq!(report.claims_walked(), 2, "against the two-row registry");

    // ---- THE TWO FIXTURES DIFFER BY THE ROUTE AND BY NOTHING ELSE -------------------------------
    let un = body(UNROUTABLE);
    let ro = body(ROUTED);
    assert_eq!(
        ro.len(),
        un.len() + ALTERNATIVE_BLOCK.len(),
        "the routed pack's body is the unroutable pack's plus the alternative block"
    );
    assert_eq!(
        &ro[..un.len()],
        &un.iter()
            .map(|l| if l.starts_with("id = ") {
                "id = \"ac495-routed\""
            } else {
                l
            })
            .collect::<Vec<_>>()[..],
        "and they are otherwise line-for-line identical apart from the `id`"
    );
    assert_eq!(
        &ro[un.len()..],
        &ALTERNATIVE_BLOCK[..],
        "the added block is the alternative route and nothing else"
    );

    // ---- AND THE GREEN DEPENDS ON THAT BLOCK. Delete it and re-run: the routed pack must fail
    // exactly as the unroutable one does. Without this, "the routed pack validates" could be true
    // for any reason at all.
    let without: String = {
        let mut kept: Vec<&str> = Vec::new();
        let mut dropped = 0_usize;
        for line in ROUTED.lines() {
            if dropped < ALTERNATIVE_BLOCK.len()
                && line.trim() == ALTERNATIVE_BLOCK[dropped]
                && kept
                    .iter()
                    .any(|l| l.trim() == "capability = \"affected-selection\"")
            {
                dropped += 1;
                continue;
            }
            kept.push(line);
        }
        assert_eq!(
            dropped,
            ALTERNATIVE_BLOCK.len(),
            "the four-line alternative block must be the one removed"
        );
        kept.join("\n")
    };
    let regressed = pack_capability_routing_rule(ROUTED_NAME, &without, &claimed)
        .expect_err("with the alternative route deleted, the routed pack is the unroutable one");
    let RoutingViolation::StageUnroutable {
        artifact,
        capability,
        routes_considered,
        ..
    } = the_unroutable(&regressed)
    else {
        unreachable!("the_unroutable filters on the tag")
    };
    assert_eq!((artifact.as_str(), capability.as_str()), (ARTIFACT, NEEDED));
    assert_eq!(
        *routes_considered, 1,
        "one route again, because the alternative is gone"
    );

    // ---- THE CLAIMED SET IS READ. The same unroutable pack, against an adapter that claims the
    // token, validates — so the refusal is a function of the registry and not of the pack alone.
    let report =
        pack_capability_routing_rule(UNROUTABLE_NAME, UNROUTABLE, &adapter_claiming_everything())
            .expect("the pack is routable for an adapter that claims the capability");
    assert_eq!(report.rows_walked(), 1);
    assert_eq!(report.claims_walked(), 3);
    assert_eq!(report.capabilities_required(), [NEEDED.to_owned()]);
}

// ---- the span comes from the loader, not from a second scan --------------------------------------

/// The spans are the **parser's**, and `parse` is defined as a projection of `parse_spans`.
///
/// That is what stops a span pointing at a line the value did not come from: there is one loop, so
/// the line recorded for `stage.verify.require[0].capability` is the line the scanner read that key
/// on. A second scan — a `lines().position(|l| l.contains("capability"))` in the rule — would agree
/// with the parser on these fixtures and disagree on a pack with a `capability` inside a comment or
/// a string, which is precisely the class of bug a hand-rolled locator introduces.
#[test]
fn the_span_of_a_require_row_is_the_parsers_own_reading() {
    let (table, spans) =
        toml_subset::parse_spans(UNROUTABLE).expect("the fixture is in the subset");

    assert_eq!(
        toml_subset::parse(UNROUTABLE).expect("the same source"),
        table,
        "`parse` is `parse_spans` with the spans dropped — one parser, or a span can point at a \
         line the value did not come from"
    );

    assert_eq!(
        spans.get("stage.verify.require[0]"),
        Some(&ROW_LINE),
        "the array-of-tables header's own line, keyed by the SAME path vocabulary the pack loader \
         builds `RequireRow::site` from"
    );
    assert_eq!(
        spans.get("stage.verify.require[0].capability"),
        Some(&CAPABILITY_LINE),
        "and the key inside it"
    );
    assert_eq!(
        spans.get("stage.verify"),
        Some(&27),
        "a plain [table] header is spanned too"
    );
    assert_eq!(spans.get("pack.id"), Some(&14), "and a root-level key");

    // The routed fixture's two rows get two DIFFERENT spans — an index that did not advance would
    // give both rows the same line and the rule would report the wrong one.
    let (_, routed) = toml_subset::parse_spans(ROUTED).expect("the fixture is in the subset");
    assert_eq!(routed.get("stage.verify.require[0]"), Some(&32));
    assert_eq!(routed.get("stage.verify.require[1]"), Some(&37));
    assert_eq!(routed.get("stage.verify.require[1].capability"), Some(&40));
    assert_eq!(
        line_at(ROUTED, 40).trim(),
        "capability = \"whole-suite\"",
        "read back against the file"
    );

    // A `#` inside a string is not a comment, and a span must survive it — the hand-rolled locator
    // this test exists to forbid would find the word on the wrong line.
    let (_, tricky) = toml_subset::parse_spans(
        "[pack]\nid = \"p\"\napplies = \"kind == story # not a comment\"\n[[stage.s.require]]\nwhen = \"true\"\ncapability = \"c\"\n",
    )
    .expect("in the subset");
    assert_eq!(tricky.get("stage.s.require[0]"), Some(&4));
    assert_eq!(tricky.get("stage.s.require[0].capability"), Some(&6));
}

// ---- the refusals that are not the criterion's, and are still refusals ---------------------------

/// A pack that does not load is a violation, never a silent pass.
///
/// `crate::toml_subset`'s header states the rule this follows: *"a reader that silently ignores what
/// it cannot parse turns a malformed registry into a passing lint"*. A routing rule that returned
/// `Ok` for a file it could not read would be exactly that, one layer up.
#[test]
fn a_pack_that_does_not_load_is_a_violation_and_never_a_clean_validation() {
    let claimed = configured_adapter();

    for (name, src, why) in [
        (
            "no-pack-table.toml",
            "[policy]\nstages = { a = \"optional\" }\n",
            "no [pack] table",
        ),
        ("not-toml.toml", "[pack\nid = \"x\"\n", "unterminated"),
        (
            "discouraged-here.toml",
            "[pack]\nid = \"x\"\n[policy]\nstages = { verify = \"discouraged\" }\n",
            "RESERVED",
        ),
    ] {
        let violations = pack_capability_routing_rule(name, src, &claimed)
            .expect_err("a pack that does not load never validates clean");
        assert_eq!(violations.len(), 1, "one violation: {violations:?}");
        assert_eq!(
            violations[0].as_str(),
            "pack_does_not_load",
            "{name} is a load failure and is reported as one"
        );
        let rendered = violations[0].to_string();
        assert!(
            rendered.contains(why) && rendered.contains(name),
            "the refusal carries the loader's own words and names the file: {rendered}"
        );
    }

    // A `capability` that is not a non-empty string is refused rather than ignored: an ignored
    // malformed key is a requirement that silently stops being one.
    for (src, needle) in [
        (
            "[pack]\nid = \"x\"\n[stage.s]\nbaseline = [\"A\"]\n[[stage.s.require]]\nwhen = \"true\"\nartifacts = [\"B\"]\ncapability = 3\n",
            "not a non-empty string",
        ),
        (
            "[pack]\nid = \"x\"\n[stage.s]\nbaseline = [\"A\"]\n[[stage.s.require]]\nwhen = \"true\"\nartifacts = [\"B\"]\ncapability = \"\"\n",
            "not a non-empty string",
        ),
    ] {
        let violations = pack_capability_routing_rule("malformed.toml", src, &claimed)
            .expect_err("a malformed capability key is refused");
        assert_eq!(violations.len(), 1, "one violation: {violations:?}");
        assert_eq!(violations[0].as_str(), "capability_not_a_token");
        assert_eq!(
            violations[0].row().map(Span::line),
            Some(5),
            "and it points at the requiring row"
        );
        assert!(
            violations[0].to_string().contains(needle),
            "{}",
            violations[0]
        );
    }
}

/// A row that declares a capability and names **no artifact** routes nothing, so it cannot be a
/// stage's only route — and the reading is a number rather than a sentence.
///
/// The shipped loader gives such a row `artifacts: []` (the key is optional), so this shape is
/// reachable and the rule has to have an answer for it. The answer is stated in
/// `src/pack_validation.rs`'s header and counted here.
#[test]
fn a_require_row_with_no_artifacts_routes_nothing_and_is_counted() {
    let claimed = configured_adapter();
    let src = "[pack]\nid = \"x\"\n[stage.s]\nbaseline = [\"A\"]\n[[stage.s.require]]\nwhen = \"true\"\ncapability = \"nobody-claims-this\"\n";

    let report = pack_capability_routing_rule("no-artifacts.toml", src, &claimed)
        .expect("a route to nothing cannot be a stage's ONLY route");
    assert_eq!(report.rows_walked(), 1);
    assert_eq!(
        report.rows_without_artifacts(),
        1,
        "counted, so the reading is readable off the report"
    );
    assert_eq!(report.artifacts_checked(), 1, "the baseline's `A`");
    assert_eq!(
        report.capabilities_required(),
        ["nobody-claims-this".to_owned()],
        "the requirement is still REPORTED even though it routes nothing"
    );
}

/// A stage whose artifact has **two** unclaimed routes is still unroutable, and the violation says
/// how many were considered.
///
/// This is the difference between *"the only route"* and *"no route"*: AC-495's GIVEN is the first,
/// and a rule that fired only when a stage had exactly one row would miss a pack with two dead
/// alternatives. The count in the violation is what tells a reader which case they are in.
#[test]
fn every_alternative_is_considered_before_a_stage_is_called_unroutable() {
    let claimed = configured_adapter();
    let src = "[pack]\nid = \"x\"\n[stage.s]\nbaseline = [\"A\"]\n[[stage.s.require]]\nwhen = \"true\"\nartifacts = [\"B\"]\ncapability = \"dead-one\"\n[[stage.s.require]]\nwhen = \"true\"\nartifacts = [\"B\"]\ncapability = \"dead-two\"\n";

    let violations = pack_capability_routing_rule("two-dead.toml", src, &claimed)
        .expect_err("two dead routes are still no route");
    let RoutingViolation::StageUnroutable {
        capability,
        row,
        routes_considered,
        ..
    } = the_unroutable(&violations)
    else {
        unreachable!("the_unroutable filters on the tag")
    };
    assert_eq!(
        *routes_considered, 2,
        "both alternatives were considered before the stage was called unroutable"
    );
    assert_eq!(
        capability, "dead-one",
        "and the row reported is the FIRST in file order, so the span is stable"
    );
    assert_eq!(row.line(), 5);

    // One claimed alternative is enough — that is the AND, at its smallest.
    let routed = src.replace("dead-two", ALTERNATIVE);
    let report = pack_capability_routing_rule("one-live.toml", &routed, &claimed)
        .expect("one claimed alternative routes the stage");
    assert_eq!(report.rows_walked(), 2);
    assert_eq!(
        report.capabilities_required(),
        ["dead-one".to_owned(), ALTERNATIVE.to_owned()]
    );
}

/// A baseline entry is a route with no capability, so an artifact the baseline names is never
/// unroutable — *"the **only** route"* is false as soon as the stage lists it unconditionally.
#[test]
fn a_baseline_entry_is_a_route_that_needs_no_capability() {
    let claimed = configured_adapter();
    let src = "[pack]\nid = \"x\"\n[stage.s]\nbaseline = [\"B\"]\n[[stage.s.require]]\nwhen = \"true\"\nartifacts = [\"B\"]\ncapability = \"nobody-claims-this\"\n";

    let report = pack_capability_routing_rule("baselined.toml", src, &claimed)
        .expect("the baseline already routes B unconditionally");
    assert_eq!(report.routes_checked(), 2, "the baseline entry and the row");
    assert_eq!(report.artifacts_checked(), 1);

    // And with the baseline naming something else, the same pack is unroutable — so the green above
    // is the baseline's doing and not the rule ignoring rows.
    let elsewhere = src.replace("baseline = [\"B\"]", "baseline = [\"A\"]");
    let violations = pack_capability_routing_rule("not-baselined.toml", &elsewhere, &claimed)
        .expect_err("with B off the baseline, the row is its only route");
    assert_eq!(the_unroutable(&violations).as_str(), "stage_unroutable");
}

/// **The shipped packs are not disturbed.** Every pack in `spikes/p9/packs/` validates clean against
/// any registry, because none of them declares a `capability` at all.
///
/// This is the *"no shipped artefact is edited to make a red case"* half of §7b.12, asserted rather
/// than promised: if a later pass adds a `capability` row to a corpus pack, this goes red and
/// whoever added it has to say so.
#[test]
fn no_shipped_pack_declares_a_capability_and_every_one_of_them_routes() {
    let claimed = ClaimedCapabilities::claims_nothing();
    for (name, src) in [
        (
            "discouraged.toml",
            include_str!("../fixtures/packs/discouraged.toml"),
        ),
        (
            "incident-appetite.toml",
            include_str!("../fixtures/packs/incident-appetite.toml"),
        ),
        (
            "incident-deviation.toml",
            include_str!("../fixtures/packs/incident-deviation.toml"),
        ),
        (
            "incident-excludes.toml",
            include_str!("../fixtures/packs/incident-excludes.toml"),
        ),
        (
            "incident-forbidden.toml",
            include_str!("../fixtures/packs/incident-forbidden.toml"),
        ),
        (
            "incident-no-appetite.toml",
            include_str!("../fixtures/packs/incident-no-appetite.toml"),
        ),
        (
            "runtime-degrade-permitted.toml",
            include_str!("../fixtures/packs/runtime-degrade-permitted.toml"),
        ),
        (
            "runtime-envelope.toml",
            include_str!("../fixtures/packs/runtime-envelope.toml"),
        ),
        (
            "runtime-no-appetite.toml",
            include_str!("../fixtures/packs/runtime-no-appetite.toml"),
        ),
        (
            "runtime-no-degrade.toml",
            include_str!("../fixtures/packs/runtime-no-degrade.toml"),
        ),
        (
            "runtime-no-k.toml",
            include_str!("../fixtures/packs/runtime-no-k.toml"),
        ),
        (
            "runtime-no-n.toml",
            include_str!("../fixtures/packs/runtime-no-n.toml"),
        ),
    ] {
        // `discouraged.toml` is AC-050's must-fail fixture, so it is expected NOT to load — and the
        // rule must say so rather than pass it.
        let verdict = pack_capability_routing_rule(name, src, &claimed);
        match verdict {
            Ok(report) => assert_eq!(
                report.capabilities_required(),
                &[] as &[String],
                "{name} declares a capability and the fixture set says none of them does"
            ),
            Err(violations) => assert!(
                violations
                    .iter()
                    .all(|v| v.as_str() == "pack_does_not_load"),
                "{name} is a must-fail loader fixture, not a routing failure: {violations:?}"
            ),
        }
    }
}

// ---- s8b §8b.5 items 2 and 3: two ADR-0072 catalog edits over `.redkiln/catalog` ---------------------

/// One shipped activity's TOML, read raw. `rk_kernel::catalog::Step` carries neither `inputs` nor
/// `on_false` (and derives the `inputs/specs.json` transcription the replay digests hash), so these
/// two tests read the file through `toml_subset` rather than widen that type.
fn shipped_activity(id: &str) -> toml_subset::Table {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.redkiln/catalog")
        .join(format!("{id}.toml"));
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    toml_subset::parse(&src).unwrap_or_else(|e| panic!("{id}.toml parses: {e:?}"))
}

/// The `[[step]]` table whose `id` is `step`.
fn shipped_step<'a>(activity: &'a toml_subset::Table, step: &str) -> &'a toml_subset::Table {
    activity
        .get("step")
        .and_then(toml_subset::Value::as_array)
        .expect("the activity declares [[step]] tables")
        .iter()
        .filter_map(toml_subset::Value::as_table)
        .find(|t| t.get("id").and_then(toml_subset::Value::as_str) == Some(step))
        .unwrap_or_else(|| panic!("no step `{step}`"))
}

/// **D21-43 N13, under ADR-0072:** *"A catalog edit to the existing `[*].tests_added` projection"*.
/// ADR-0035: *"Fan-out outputs are projected with `${step.outputs[*].field}`."* `red` fans out over the
/// Spec's acceptance criteria, so `red-verify` selects the concatenation of every iteration's
/// `tests_added`, in element order (D21-48 X5: *"Concatenation in element order. Nothing is flattened
/// implicitly anywhere else."*). Resolving the projection at run time is `s8d`'s resolver; this is the
/// catalog text, and the two premises that make it well-formed.
#[test]
fn tdd_red_verify_projects_tests_added_per_adr_0035() {
    let tdd = shipped_activity("tdd-red-green-refactor");
    let verify = shipped_step(&tdd, "red-verify");
    let selection = verify
        .get("inputs")
        .and_then(toml_subset::Value::as_table)
        .and_then(|i| i.get("selection"))
        .and_then(toml_subset::Value::as_str);
    assert_eq!(
        selection,
        Some("${red.outputs[*].tests_added}"),
        "red-verify must select the [*].tests_added projection, not the unprojected list of \
         ImplReports that its old comment said the tool would flatten"
    );
    let red = shipped_step(&tdd, "red");
    assert!(
        red.get("for_each")
            .and_then(toml_subset::Value::as_str)
            .is_some_and(|f| !f.is_empty()),
        "the projection's source step must fan out — `[*]` over a single output projects nothing"
    );
    let schemas = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/schemas.toml"),
    )
    .expect("the schema stand-in is readable");
    let schemas = toml_subset::parse(&schemas).expect("the schema stand-in parses");
    let lists: Vec<&str> = schemas
        .get("value")
        .and_then(toml_subset::Value::as_table)
        .and_then(|v| v.get("ImplReport"))
        .and_then(toml_subset::Value::as_table)
        .and_then(|t| t.get("lists"))
        .and_then(toml_subset::Value::as_array)
        .expect("ImplReport's stand-in declares `lists`")
        .iter()
        .filter_map(toml_subset::Value::as_str)
        .collect();
    assert!(
        lists.contains(&"tests_added"),
        "`tests_added` must be a LIST field of ImplReport for the projection to concatenate: {lists:?}"
    );
}

/// `${a.b}` / `${a.b?}` references become the paths they name. Reference resolution is `s8d`'s
/// resolver (ADR-0136 evaluates `when` through `predicate::evaluate` after it); here the scan's
/// three counts are bound by path, and nothing else is rewritten.
fn refs_to_paths(e: Expr) -> Expr {
    let b = |x: Box<Expr>| Box::new(refs_to_paths(*x));
    match e {
        Expr::Ref(r) => Expr::Path(
            r.trim_start_matches("${")
                .trim_end_matches('}')
                .trim_end_matches('?')
                .to_owned(),
        ),
        Expr::Or(l, r) => Expr::Or(b(l), b(r)),
        Expr::And(l, r) => Expr::And(b(l), b(r)),
        Expr::Not(x) => Expr::Not(b(x)),
        Expr::Group(x) => Expr::Group(b(x)),
        Expr::Cmp { op, left, right } => Expr::Cmp {
            op,
            left: b(left),
            right: b(right),
        },
        other => other,
    }
}

/// **D21-43 N25:** *"`sca-scan` requires `unrated == 0` or goes to `scan.accept-findings`."* Both
/// predicates are evaluated through `rk_kernel::predicate` over three reports — all zero, one unrated
/// advisory alone, one high alone — and `clean` must be true, and the `exception` gate's `when` false,
/// for the first report ONLY. Before the edit the unrated report passed `clean` and never opened the
/// gate, which is the defect: an advisory nobody rated was passed silently.
#[test]
fn sca_scan_blocks_on_unrated_or_routes_to_accept_findings() {
    let sca = shipped_activity("sca-scan");
    let clean = shipped_step(&sca, "clean")
        .get("expr")
        .and_then(toml_subset::Value::as_str)
        .expect("`clean` carries an expr");
    let exception = shipped_step(&sca, "exception");
    assert_eq!(
        exception
            .get("question")
            .and_then(toml_subset::Value::as_str),
        Some("scan.accept-findings"),
        "the gate an unrated advisory goes to"
    );
    let when = exception
        .get("when")
        .and_then(toml_subset::Value::as_str)
        .expect("the exception gate is guarded");
    let clean = parse(clean).expect("`clean` parses").expr;
    let when = refs_to_paths(parse(when).expect("`when` parses").expr);
    let reports = [
        ("all zero", 0, 0, 0),
        ("unrated alone", 0, 0, 1),
        ("high alone", 0, 1, 0),
    ];
    assert_eq!(reports.len(), 3, "three reports are walked");
    for (i, (label, critical, high, unrated)) in reports.into_iter().enumerate() {
        let mut ctx = Context::new();
        for root in ["report", "scan.outputs"] {
            ctx.bind(&format!("{root}.critical"), PValue::Int(critical));
            ctx.bind(&format!("{root}.high"), PValue::Int(high));
            ctx.bind(&format!("{root}.unrated"), PValue::Int(unrated));
        }
        let c = evaluate(&clean, &ctx).unwrap_or_else(|e| panic!("{label}: clean errs: {e}"));
        let w = evaluate(&when, &ctx).unwrap_or_else(|e| panic!("{label}: when errs: {e}"));
        let first = i == 0;
        assert_eq!(
            c.value,
            if first { Tri::True } else { Tri::False },
            "{label}: `clean` must pass the all-zero report and no other"
        );
        assert_eq!(
            w.value,
            if first { Tri::False } else { Tri::True },
            "{label}: the exception gate must open on every report but the all-zero one"
        );
    }
}
