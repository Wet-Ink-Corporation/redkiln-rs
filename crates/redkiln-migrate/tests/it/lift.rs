//! The lift, over `legacy-mini` (§7d.11 step (4), §7d.13).

use redkiln_migrate::lift::{self, Lifted, LiftedFile, Natural};
use redkiln_migrate::report::Outcome;
use rk_types::companion::{CompanionKind, PHASE_2_KINDS};

use crate::legacy_mini_repo;

/// Lift the fixture in a temp repository, the only way anything here runs it.
fn lift_fixture() -> Lifted {
    let repo = legacy_mini_repo();
    let lifted = lift::run(repo.path(), &mut Natural).expect("the fixture lifts");
    // `repo` drops here; `Lifted` owns its bytes, so nothing reaches back into the temp dir.
    lifted
}

fn file_named<'a>(lifted: &'a Lifted, ends_with: &str) -> &'a LiftedFile {
    let matches: Vec<&LiftedFile> = lifted
        .files
        .iter()
        .filter(|f| {
            f.source
                .to_string_lossy()
                .replace('\\', "/")
                .ends_with(ends_with)
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "expected exactly one lifted file ending {ends_with}, found {}",
        matches.len()
    );
    matches[0]
}

/// AC-143's list, ADR-0075 clause 6's treatment, DECISIONS-2026-09-11 #3.
///
/// The six typed kinds lift typed; the eight Phase-2 kinds lift opaque. `legacy-mini` carries one
/// file per kind, so a fifteenth kind or a fourteenth-kind shortfall is red on the set, not on a
/// count alone.
///
/// DECISIONS-2026-09-11 #3 words the set as *"five plus `ledger`"* — the five sidecars and one bare
/// census kind — which the type spells as six variants because `_ledger` and `ledger` are distinct
/// wire names. **Noted, not relitigated.**
#[test]
fn adr0075_six_typed_kinds_eight_opaque() {
    let lifted = lift_fixture();

    let typed_wire: Vec<&str> = CompanionKind::ALL.iter().map(|k| k.wire_name()).collect();
    assert_eq!(
        typed_wire.len(),
        6,
        "CompanionKind::ALL is not six variants"
    );
    assert_eq!(PHASE_2_KINDS.len(), 8, "PHASE_2_KINDS is not eight");

    let mut seen: std::collections::BTreeMap<&str, (bool, usize)> =
        std::collections::BTreeMap::new();
    for f in &lifted.files {
        let e = seen.entry(f.kind.as_str()).or_insert((f.typed, 0));
        e.1 += 1;
        assert_eq!(
            e.0, f.typed,
            "kind {} lifted both typed and opaque across files",
            f.kind
        );
    }

    for wire in &typed_wire {
        let (typed, n) = *seen
            .get(wire)
            .unwrap_or_else(|| panic!("no file of companion kind {wire} in legacy-mini"));
        assert_eq!(
            n, 1,
            "legacy-mini carries {n} files of kind {wire}, not one"
        );
        assert!(typed, "the typed companion kind {wire} lifted OPAQUE");
    }

    for wire in PHASE_2_KINDS {
        let (typed, n) = *seen
            .get(wire)
            .unwrap_or_else(|| panic!("no file of Phase-2 kind {wire} in legacy-mini"));
        assert_eq!(
            n, 1,
            "legacy-mini carries {n} files of kind {wire}, not one"
        );
        assert!(
            !typed,
            "the Phase-2 kind {wire} lifted TYPED — ADR-0075 clause 6 carries it as opaque prose, \
             and rk-types refuses to write one against the Phase-1 registry"
        );
    }

    let companion_kinds: std::collections::BTreeSet<&str> = typed_wire
        .iter()
        .copied()
        .chain(PHASE_2_KINDS.iter().copied())
        .collect();
    assert_eq!(
        companion_kinds.len(),
        14,
        "the fourteen companion kinds are not fourteen"
    );

    // A fifteenth companion kind accepted is red: every kind the lift produced is either one of the
    // fourteen or one of the item/config row kinds §7d.11 step (4) names.
    let row_kinds = [
        "initiative",
        "project",
        "story",
        "spec",
        "report",
        "config",
        "process",
    ];
    for kind in seen.keys() {
        assert!(
            companion_kinds.contains(kind) || row_kinds.contains(kind),
            "the lift produced kind {kind}, which is neither a companion kind nor one of \
             §7d.11 step (4)'s item/report/config row kinds — a fifteenth kind was accepted"
        );
    }
}

/// AC-475's carry, claimed under **no id** (E13 leaves the loss-marker migration fixture to
/// Phase 2; this is built here as the migrator's carry, as `free_form_gate_result_…` is).
#[test]
fn diff_prose_carried_opaque_with_loss_marker() {
    let lifted = lift_fixture();
    let spec = file_named(&lifted, "story-2/spec.md");

    assert_eq!(
        spec.outcome,
        Outcome::LiftedWithLoss,
        "the diff-prose spec lifted {:?}, not LiftedWithLoss",
        spec.outcome
    );
    assert!(
        !spec.losses.is_empty(),
        "the diff-prose section was carried with NO loss marker — a section with no schema home \
         that lands silently is the failure AC-475's carry exists to catch"
    );
    let marker = spec
        .losses
        .iter()
        .find(|l| l.opaque.contains("```diff") || l.what.contains("diff"))
        .expect("a loss marker naming the diff-prose section");
    assert!(
        marker.opaque.contains("the new one walks it once"),
        "the diff prose was not carried VERBATIM in the marker — carrying a summary of it is not \
         carrying it opaque:\n{}",
        marker.opaque
    );
    assert!(
        !marker.why.trim().is_empty(),
        "the loss marker does not say why the section had no home"
    );
}

/// AC-477's carry, claimed under **no id** (AC-477 is ARMED whole — its WHEN needs a render path
/// `rk-store` does not carry at this pin, §7d.11).
///
/// S-MIG decision 2: a gate result is *"emitted by the toolchain and rendered, never parsed"*.
#[test]
fn free_form_gate_result_carried_opaque_with_loss_marker() {
    let lifted = lift_fixture();
    let report_md = file_named(&lifted, "story-2/report.md");

    assert_eq!(
        report_md.outcome,
        Outcome::LiftedWithLoss,
        "the free-form gate result lifted {:?}, not LiftedWithLoss",
        report_md.outcome
    );
    assert!(
        !report_md.typed,
        "the free-form gate result was parsed into a TYPED value — S-MIG decision 2 says a gate \
         result is emitted and rendered, never parsed, and reconstructing a typed verdict from \
         prose is what AC-478's second conjunct forbids"
    );
    assert!(
        !report_md.losses.is_empty(),
        "the free-form gate result was carried with no loss marker"
    );

    // The value must carry the prose, not a verdict derived from it.
    let text = serde_json::to_string(&report_md.value).expect("the artifact serialises");
    assert!(
        text.contains("accepted it on the third reading"),
        "the free-form result's prose is not in the artifact — it was summarised, not carried"
    );
    for derived in ["\"verdict\"", "\"approved\"", "\"changes-requested\""] {
        assert!(
            !text.contains(derived),
            "the artifact carries {derived}, so a typed verdict was reconstructed from prose"
        );
    }
}

/// Claimed under **no id** — AC-477 is ARMED whole (§7d.11). This is the shape the render
/// round-trip will extend when `rk-store` grows a render path.
#[test]
fn typed_gate_result_round_trips_through_canonical() {
    let lifted = lift_fixture();
    let review = file_named(&lifted, "story-1/_review.md");

    assert!(
        review.typed,
        "the review companion did not lift typed — it is one of CompanionKind::ALL's six"
    );

    // The typed value, read back out of the artifact and deserialised into the rk-types struct.
    let typed = review
        .value
        .get("typed")
        .expect("the artifact carries its typed value");
    let companion: rk_types::companion::ReviewCompanion = serde_json::from_value(typed.clone())
        .expect(
            "the typed value deserialises into ReviewCompanion — deny_unknown_fields means an \
             extra key is a hard error here, which is the point",
        );

    // Round trip: typed -> Value -> canonical bytes, twice, equal.
    let once = rk_store::canon::canonical(typed).expect("canonical");
    let reserialised = serde_json::to_value(&companion).expect("the typed value re-serialises");
    let twice = rk_store::canon::canonical(&reserialised).expect("canonical");
    assert_eq!(
        once,
        twice,
        "the typed gate result did not round-trip: canonical bytes differ after a \
         deserialise/serialise cycle.\n  once: {}\n twice: {}",
        String::from_utf8_lossy(&once),
        String::from_utf8_lossy(&twice)
    );

    // AC-477's "the value read back equals the value written" half, as far as it can be claimed
    // with no render path: the summary survives.
    assert!(
        !companion.summary.as_str().trim().is_empty(),
        "the round-tripped companion has an empty summary"
    );
}

/// AC-463 — the Phase-2 exit check's mechanical definition of *"green"*.
///
/// **Named, not run.** It is authored as a `fn`, compiled only under the `corpus` feature and
/// `#[ignore]`d even then; Checkpoint 7d asserts the `fn` exists by `grep` and never that it
/// passed. A comment or a TODO naming it would not be a test.
#[cfg(feature = "corpus")]
#[test]
#[ignore = "corpus-bound (AC-464/PIA-002): runs only on a box holding redkiln-corpus at CORPUS_PIN, as `cargo test -p redkiln-migrate --features corpus -- --ignored`"]
fn ac463_corpus_differential_runner_named() {
    let pin = redkiln_migrate::report::corpus_sha().expect("CORPUS_PIN carries a CORPUS_SHA= line");
    assert_eq!(pin.len(), 40, "the corpus pin is not a 40-hex sha");
    panic!(
        "AC-463's corpus differential runner is NAMED, not built: the Phase-2 acceptance set runs \
         the migrator over redkiln-corpus at {pin} and scores the differential report's lift \
         share per (kind, slot) against SM-133. Phase 1 owes the runner's NAME and this crate's \
         schema half only (E13)."
    );
}
