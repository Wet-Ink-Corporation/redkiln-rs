//! **The question registry, and the one refusal that is the kernel's** (ADR-0129 decision 3,
//! ADR-0063; Step 7d Slice 3, kernel edits four and five).
//!
//! **Two records, two boundaries, and the split is the point.** ADR-0129 decision 3 puts the
//! unknown-option refusal *"at the CLI boundary … as a usage error with zero events"*. ADR-0063's
//! Decision puts a different refusal in the kernel: *"the kernel refuses to close the mark unless
//! that attachment is supplied."* One is about what may be said; the other about what must be
//! brought. Collapsing them into a single kernel check refuses every verdict the default question
//! does not name — measured here as four `gates.rs` tests going red on a verdict of `"proceed"`,
//! against §7d.15's *"their assertions stand unchanged"*.

use rk_kernel::workflow::gate::{self, AnswerMeta, Disposition, GateError};
use rk_kernel::workflow::question::{
    Attachment, DEFAULT_OPTIONS, DEFAULT_QUESTION, OptionError, Question, QuestionRegistry,
};

/// ADR-0129 decision 3.
///
/// Red by the default question carrying anything but `approve` / `changes-requested`, either with
/// `attaches`, or by a gate step naming no question not resolving to it.
#[test]
fn adr0129_question_registry_default_question_two_options() {
    let registry = QuestionRegistry::default();

    let default = registry
        .question(DEFAULT_QUESTION)
        .expect("the default question is built in");
    assert_eq!(
        default.options(),
        vec!["approve", "changes-requested"],
        "the default question declares EXACTLY the two options ADR-0129 d3 names"
    );
    assert_eq!(
        DEFAULT_OPTIONS.len(),
        2,
        "the exported constant and the built-in question must agree"
    );

    for option in DEFAULT_OPTIONS {
        assert_eq!(
            default.attaches(option),
            Some(&[][..]),
            "`{option}` attaches something; ADR-0129 d3 says neither option attaches anything"
        );
    }

    // "A gate step that names no question is the default question." An unbound gate kind — which
    // is every kind at 7d, because no pack declares a question until 7e — resolves to it.
    for kind in [
        "unknown_facet:criticality",
        "unknown_facet:blast_radius",
        "escalation",
    ] {
        assert_eq!(
            registry.question_for(kind),
            DEFAULT_QUESTION,
            "the gate kind `{kind}` names no question, so it is answered with the default one"
        );
    }

    // And the resolution seam works in the other direction — the control that `question_for` reads
    // the bindings at all, rather than returning a constant.
    let bound = QuestionRegistry::default()
        .with_question("decide", Question::new(["ship", "hold"]))
        .bind("unknown_facet:criticality", "decide");
    assert_eq!(
        bound.question_for("unknown_facet:criticality"),
        "decide",
        "a bound gate kind resolves to the question it was bound to — without this the assertions \
         above are true of a `question_for` that ignores its argument"
    );
    assert_eq!(
        bound.question_for("unknown_facet:novelty"),
        DEFAULT_QUESTION,
        "and an UNbound kind still falls back, with a non-empty binding map present"
    );
}

/// ADR-0063, ADR-0129 decision 4 — **the kernel's refusal**.
///
/// Red by `QuestionRegistry::validate` returning `Ok(())` with the declared kind absent, by
/// `gate::answer` returning `Applied` (or writing any event) over an otherwise-`Applied` state when
/// the attachment is missing, or by `validate` refusing the attachment when present.
#[test]
fn adr0063_option_with_attaches_refuses_close_without_attachment() {
    let registry = QuestionRegistry::default()
        .with_question(
            "decide",
            Question::new(["hold"]).attaching("ship", ["ReleaseNote"]),
        )
        .bind("unknown_facet:criticality", "decide");

    let note = Attachment {
        kind: "ReleaseNote".to_owned(),
        path: "notes/rel.md".to_owned(),
    };

    // ---- the registry, directly.
    assert_eq!(
        registry.validate("decide", "ship", &[]),
        Err(OptionError::AttachmentMissing {
            kind: "ReleaseNote".to_owned()
        }),
        "an option that attaches a kind is not satisfied by no attachment"
    );
    assert_eq!(
        registry.validate("decide", "ship", std::slice::from_ref(&note)),
        Ok(()),
        "and it IS satisfied when the declared kind is supplied — without this arm the refusal \
         above is true of a validate that refuses everything"
    );
    assert_eq!(
        registry.validate("decide", "hold", std::slice::from_ref(&note)),
        Err(OptionError::AttachmentNotAdmitted {
            kind: "ReleaseNote".to_owned()
        }),
        "an attachment no declared option admits is refused — and as NotAdmitted, never as \
         UnknownOption (AC-410 says so in terms)"
    );
    assert!(
        matches!(
            registry.validate("decide", "nonesuch", &[]),
            Err(OptionError::UnknownOption { .. })
        ),
        "an option the question does not declare is UnknownOption — the CLI's refusal, and the \
         variant the attachment arms above must be distinguishable from"
    );

    // ---- and through `gate::answer`, over an otherwise-APPLIED state. This is the half ADR-0063
    // actually legislates: the mark does not close.
    let kind = "unknown_facet:criticality";
    let mut journal: Vec<rk_types::event::Event> = Vec::new();
    let reached = gate::reach(
        &crate::gates::fold_all(&journal),
        kind,
        &crate::gates::ctx(crate::gates::TREE_A, 9_000),
    )
    .expect("the gate mints a token");
    journal.push(rk_kernel::runtime::gate_opened_event(&reached));
    let state = crate::gates::fold_all(&journal);
    let token = reached.token();

    let refused = gate::answer(
        &state,
        token,
        token,
        crate::gates::TREE_A,
        "ship",
        &registry,
        &[],
        &AnswerMeta::default(),
    );
    assert!(
        matches!(
            refused,
            Err(GateError::OptionRefused(
                OptionError::AttachmentMissing { .. }
            ))
        ),
        "the kernel closed a mark whose option attaches a kind that was not supplied \
         (ADR-0063): {refused:?}"
    );

    // The control: the SAME state, the SAME option, with the attachment — applies, and writes.
    let applied = gate::answer(
        &state,
        token,
        token,
        crate::gates::TREE_A,
        "ship",
        &registry,
        std::slice::from_ref(&note),
        &AnswerMeta::default(),
    )
    .expect("the answer applies when the declared attachment is supplied");
    assert_eq!(
        applied.disposition,
        Disposition::Applied,
        "without this control the refusal above is true of an `answer` that refuses every option"
    );
    assert!(
        !applied.events.is_empty(),
        "an applied answer writes its row; the refusal writes none"
    );
}

/// ADR-0129 decision 1 — **the members are added kernel-side**, and the three-argument constructor
/// keeps its arity.
///
/// Red by `gate::answer` returning a `gate_answered` row without the members when given an
/// `AnswerMeta` that carries them; by the CLI being able to produce the same row (the members must
/// come from the kernel's own constructor, not from a caller editing the returned `Event`); and by
/// a `None` member being written as an explicit `null`, which would move every recorded journal
/// that ever carries one.
///
/// **The arity assertion is not decoration.** §7d.15 reads as though `gate_answered_event` itself
/// should widen — but it has a caller in a CLOSED crate,
/// `crates/rk-adapter-sqlite/tests/it/replay.rs`, and §7d.15's own floor says the four adapters are
/// not edited. Widening it would have broken `cargo test --workspace --features corpus`, a line
/// Checkpoint 7b runs, from inside this slice's own mandated re-run. The three-argument call below
/// is that caller's shape, compiled here so the constraint is a test rather than a memory.
#[test]
fn adr0129_gate_answered_carries_answer_meta_kernel_side() {
    let kind = "unknown_facet:criticality";
    let mut journal: Vec<rk_types::event::Event> = Vec::new();
    let reached = gate::reach(
        &crate::gates::fold_all(&journal),
        kind,
        &crate::gates::ctx(crate::gates::TREE_A, 9_000),
    )
    .expect("the gate mints a token");
    journal.push(rk_kernel::runtime::gate_opened_event(&reached));
    let state = crate::gates::fold_all(&journal);
    let token = reached.token();

    let meta = AnswerMeta {
        explain_sufficiency: Some(gate::ExplainSufficiency {
            sufficed: "yes".to_owned(),
            toml_opened: "no".to_owned(),
        }),
        answerer: Some("a.operator".to_owned()),
    };
    let applied = gate::answer(
        &state,
        token,
        token,
        crate::gates::TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &meta,
    )
    .expect("the answer applies");
    assert_eq!(applied.disposition, Disposition::Applied);

    let row = applied
        .events
        .iter()
        .find(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
        .expect("an applied answer writes its gate_answered row");
    // ONE member carrying TWO data. Reading it `as_str()` was the shape this test pinned until
    // cycle 7, and it was wrong against §7d.6, which fixes the payload as
    // `explain_sufficiency: { sufficed: "yes", toml_opened: "yes" }` VERBATIM.
    let sufficiency = row
        .payload
        .get("explain_sufficiency")
        .expect("the row carries the answerer's sufficiency report (ADR-0129 d1)");
    assert_eq!(
        sufficiency.get("sufficed").and_then(|v| v.as_str()),
        Some("yes"),
        "`explain_sufficiency.sufficed` is not what the answerer reported: {:?}",
        row.payload
    );
    assert_eq!(
        sufficiency.get("toml_opened").and_then(|v| v.as_str()),
        Some("no"),
        "`explain_sufficiency.toml_opened` is missing -- it is the member a one-flag reading of \
         AC-435 drops, and SM-126's kill is the cell it carries: {:?}",
        row.payload
    );
    assert_eq!(
        row.payload.get("answerer").and_then(|v| v.as_str()),
        Some("a.operator"),
        "the row does not name who answered: {:?}",
        row.payload
    );

    // The OMISSION. An answer with no meta must produce a row byte-identical to the one the
    // three-argument constructor produces — not one carrying two nulls — or every recorded journal
    // that ever holds a `gate_answered` row moves the day this member lands.
    let bare = gate::answer(
        &state,
        token,
        token,
        crate::gates::TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("the answer applies");
    let bare_row = bare
        .events
        .iter()
        .find(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
        .expect("the row exists");

    // The three-argument constructor, at the arity the CLOSED crate calls it with.
    let legacy = gate::gate_answered_event(token, crate::gates::TREE_A, "approve");
    assert_eq!(
        bare_row.payload, legacy.payload,
        "an answer carrying no AnswerMeta must produce the payload the three-argument constructor \
         produces. A `null` member here moves the canonical bytes of every journal that records a \
         gate answer"
    );
    assert!(
        !bare_row
            .payload
            .as_object()
            .is_some_and(|o| o.contains_key("answerer")),
        "an absent member is OMITTED, not written as null: {:?}",
        bare_row.payload
    );
}
