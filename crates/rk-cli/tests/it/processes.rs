//! **The question registry's loader** — `.redkiln/processes/*.toml` into `QuestionRegistry`
//! (`s8d` §8d.6; R5 = ADR-0177, proposed). *(Added 2026-09-26, `s8d` build, package P7.)*
//!
//! Three properties, one test each: the shipped file declares the lanes' nine questions and N3's
//! `facet.change`, transcribed (not invented) from `spikes/p9/questions.toml` and `facets.toml`;
//! `answer` validates against the LOADED registry and never `QuestionRegistry::default()` (M16);
//! and `run` refuses, by name and before any dispatch, a catalog gate step whose question the
//! registry does not declare.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_kernel::workflow::question::DEFAULT_QUESTION;

use crate::answer::{BOTH_FLAGS, is_answerable_gate, spawn_answer};

/// The workspace root: `crates/rk-cli` is two levels below it.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The nine lane questions §8d.6's table enumerates, each transcribed from `spikes/p9`.
const LANE_QUESTIONS: [&str; 9] = [
    "repro.cannot-reproduce",
    "root-cause.no-hypothesis",
    "root-cause.accept",
    "draft.accept",
    "tdd.green-exhausted",
    "confirmation.not-fixed",
    "scan.accept-findings",
    "review.blocking",
    "closure.signoff",
];

/// A `[question."<id>"]` table of a TOML document: its text and its option ids, in file order.
fn question_of(doc: &toml::Value, id: &str) -> Option<(String, Vec<String>)> {
    let q = doc.get("question")?.get(id)?;
    let text = q.get("text")?.as_str()?.to_owned();
    let options = q
        .get("options")?
        .as_array()?
        .iter()
        .map(|o| {
            o.as_str()
                .map(str::to_owned)
                .or_else(|| o.get("id").and_then(toml::Value::as_str).map(str::to_owned))
        })
        .collect::<Option<Vec<_>>>()?;
    Some((text, options))
}

fn read_toml(path: &Path) -> toml::Value {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    toml::from_str(&text).unwrap_or_else(|e| panic!("{} parses: {e}", path.display()))
}

/// **The lanes' nine gate questions and N3's `facet.change` load from `.redkiln/processes/`.**
///
/// Each of the nine carries `spikes/p9/questions.toml`'s text and options VERBATIM (§8d.6: *"The
/// entries are transcribed, not invented, except one"*), each binds its own id as a gate kind (a
/// catalog `gate` step's kind is its `question`, ADR-0145), and `facet.change` answers the
/// `unknown_facet:change` gate with `facets.toml`'s `change` values in declaration order. The total
/// is NOT pinned at ten: `s8j` authors the human-root questions into the same directory, through
/// this loader (D21-43), and `spec.ambiguity` is asserted absent (N2).
#[test]
fn the_lanes_nine_gate_questions_and_the_change_question_load_from_redkiln_processes() {
    let root = workspace_root();
    let registry = rk_cli::processes::load(&root)
        .unwrap_or_else(|e| panic!("the shipped .redkiln/processes/ loads: {e}"));
    let p9 = read_toml(&root.join("spikes").join("p9").join("questions.toml"));

    let mut walked = 0_usize;
    for id in LANE_QUESTIONS {
        let (text, options) = question_of(&p9, id)
            .unwrap_or_else(|| panic!("PREMISE: spikes/p9/questions.toml declares `{id}`"));
        let loaded = registry.question(id).unwrap_or_else(|| {
            panic!("`{id}` is not declared under .redkiln/processes/ (§8d.6's nine lane questions)")
        });
        assert_eq!(
            loaded.text(),
            Some(text.as_str()),
            "`{id}`'s text is not spikes/p9's, verbatim"
        );
        let want: BTreeSet<&str> = options.iter().map(String::as_str).collect();
        let got: BTreeSet<&str> = loaded.options().into_iter().collect();
        assert_eq!(got, want, "`{id}`'s options are not spikes/p9's");
        assert_eq!(
            registry.question_for(id),
            id,
            "a catalog gate step asking `{id}` must resolve to `{id}` (ADR-0145: its kind is its \
             question), not to the default question"
        );
        assert!(
            registry.binds_kind(id),
            "`{id}` is not bound as a gate kind"
        );
        walked += 1;
    }
    assert_eq!(walked, 9, "the walk covered the nine lane questions");

    // N3's tenth: `facet.change`, bound to the `unknown_facet:change` gate kind, options transcribed
    // from facets.toml's `change` values, in that order.
    let facets = read_toml(&root.join(".redkiln").join("facets.toml"));
    let change_values: Vec<String> = facets["facet"]["change"]["values"]
        .as_array()
        .expect("PREMISE: facets.toml declares [facet.change] values")
        .iter()
        .map(|v| v.as_str().expect("a string value").to_owned())
        .collect();
    assert_eq!(
        change_values,
        ["text", "binary", "generated"],
        "PREMISE: facets.toml's change values"
    );
    assert_eq!(
        registry.question_for("unknown_facet:change"),
        "facet.change",
        "the `unknown_facet:change` gate (N3's triage gate) must be answered with `facet.change`, \
         never the default question"
    );
    let change = registry
        .question("facet.change")
        .expect("`facet.change` is declared");
    assert_eq!(
        change.options().into_iter().collect::<BTreeSet<_>>(),
        change_values
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        "`facet.change`'s options are facets.toml's change values"
    );
    let shipped = read_toml(
        &root
            .join(".redkiln")
            .join("processes")
            .join("questions.toml"),
    );
    let (_, declared_order) =
        question_of(&shipped, "facet.change").expect("the shipped file declares facet.change");
    assert_eq!(
        declared_order, change_values,
        "`facet.change`'s options are declared in facets.toml's order"
    );
    assert!(
        change.text().is_some_and(|t| !t.trim().is_empty()),
        "`facet.change` carries its text"
    );

    // N2: no lane selects `user-story-ac`, so its `spec.ambiguity` is not an entry.
    assert!(
        registry.question("spec.ambiguity").is_none(),
        "`spec.ambiguity` must NOT be declared under .redkiln/processes/: no lane selects the one activity asking it (N2)"
    );
    // Every other `unknown_facet:*` gate keeps the default question.
    assert_eq!(
        registry.question_for("unknown_facet:has_repro"),
        DEFAULT_QUESTION,
        "an unbound `unknown_facet:*` gate kind keeps the default question"
    );
    println!(
        "PROCESSES questions={} (the default included)",
        registry.question_ids().len()
    );

    // CONTROL: a root with no `.redkiln/processes/` loads the default question and nothing else,
    // so the assertions above are about the shipped files, not a built-in table.
    let empty = tempfile::tempdir().expect("a temp dir");
    let bare = rk_cli::processes::load(empty.path()).expect("an absent directory loads");
    assert_eq!(bare.question_ids(), [DEFAULT_QUESTION]);
    assert!(!bare.binds_kind("review.blocking"));
}

/// The `pack-min` gate fixture with a processes file that answers its abstention gate
/// (`unknown_facet:has_repro`) with a two-option question the default does not carry.
fn gate_repo_with_questions() -> tempfile::TempDir {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let dir = rk_cli::processes::processes_dir(repo.path());
    std::fs::create_dir_all(&dir).expect("the processes directory is created");
    std::fs::write(
        dir.join("questions.toml"),
        "[question.\"repro.decide\"]\n\
         text = \"Is there a reproduction?\"\n\
         options = [\"yes\", \"no\"]\n\
         binds = [\"unknown_facet:has_repro\"]\n",
    )
    .expect("the question file writes");
    repo
}

/// **`answer` validates against the LOADED registry, never `QuestionRegistry::default()` (M16).**
///
/// The gate's question is resolved from its kind (§7d.6), and the pack binds `unknown_facet:has_repro`
/// to `repro.decide` (`yes` / `no`). So `approve` — the default question's option — is a usage error
/// (exit 2, zero answer rows), and `yes` is applied. Against the default registry both would be the
/// other way round.
#[test]
fn the_answer_verb_validates_against_the_loaded_registry_not_the_default() {
    let repo = gate_repo_with_questions();
    let root = repo.path();
    let opened = crate::run::spawn_run(root, "gate-1", None);
    assert_eq!(
        opened.status.code(),
        Some(0),
        "PREMISE: the abstention gate opens. stderr:\n{}",
        String::from_utf8_lossy(&opened.stderr)
    );
    assert_eq!(
        crate::support::store_rows(root)
            .iter()
            .filter(|e| is_answerable_gate(e))
            .count(),
        1,
        "PREMISE: exactly one answerable gate is open"
    );

    let before = crate::support::store_rows(root).len();
    let out = spawn_answer(root, "gate-1", "approve", &BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(2),
        "`approve` is the DEFAULT question's option; the loaded registry answers this gate with \
         `repro.decide`, which does not declare it, so the answer is a usage error (ADR-0129 \
         decision 3). stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("repro.decide"),
        "the refusal names the loaded question. stderr:\n{stderr}"
    );
    let after: Vec<_> = crate::support::store_rows(root);
    assert!(
        after[before..]
            .iter()
            .all(|e| e.kind != rk_types::event_kind::EventKind::GateAnswered),
        "the refused answer wrote a `gate_answered` row"
    );

    let out = spawn_answer(root, "gate-1", "yes", &BOTH_FLAGS);
    assert_eq!(
        out.status.code(),
        Some(0),
        "`yes` is `repro.decide`'s option and is applied. stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let answered: Vec<String> = crate::support::store_rows(root)
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
        .filter_map(|e| {
            e.payload
                .get("verdict")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .collect();
    assert_eq!(answered, ["yes"], "one `gate_answered` row, verdict `yes`");
}

/// Rewrite `pack-min`'s one activity so a `gate` step asking `question` precedes its agent step.
fn with_gate_step(root: &Path, question: &str) {
    let path = root
        .join(".redkiln")
        .join("catalog")
        .join("emit-report.toml");
    let src = std::fs::read_to_string(&path).expect("emit-report.toml is readable");
    let anchor = "[[step]]\nid      = \"check\"";
    assert_eq!(
        src.matches(anchor).count(),
        1,
        "PREMISE: emit-report.toml's agent step anchor"
    );
    let out = src.replacen(
        anchor,
        &format!("[[step]]\nid       = \"probe\"\nkind     = \"gate\"\nquestion = \"{question}\"\n\n{anchor}"),
        1,
    );
    std::fs::write(&path, out).expect("emit-report.toml writes");
    // *(dated 2026-10-02, s8h build, package P7; `s8h` §8h.10.)* The catalog now reaches
    // `question`, so the packs carry its fail-closed row: ADR-0146's load-time lint would otherwise
    // refuse the run at step (7), before the binding pass whose refusal these tests read.
    crate::support::add_gate_expiry_row(root, question);
}

/// **A catalog gate step whose question the registry does not declare refuses `run` by name.**
///
/// §8d.6: *"a gate step's kind must be declared, or the load refuses by name"*. The refusal is at
/// the binding pass, before any dispatch or gate row: exit 3, stderr naming the question and the
/// activity, and no runtime `gate_opened` and no `activity_committed`. CONTROL: the same repository
/// with the question declared under `.redkiln/processes/` opens that gate instead.
#[test]
fn a_gate_kind_the_registry_does_not_declare_refuses_by_name() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    with_gate_step(root, "probe.accept");

    let out = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a gate step asking a question no pack file declares must refuse the run. stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("probe.accept") && stderr.contains("emit-report"),
        "the refusal names the undeclared question and its activity. stderr:\n{stderr}"
    );
    let rows = crate::support::store_rows(root);
    assert!(
        !rows.iter().any(is_answerable_gate),
        "a gate was opened for an undeclared question"
    );
    assert!(
        !rows
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted),
        "the refusal must precede every dispatch"
    );

    // CONTROL: declared, the same step opens its gate.
    let control = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let croot = control.path();
    with_gate_step(croot, "probe.accept");
    let dir = rk_cli::processes::processes_dir(croot);
    std::fs::create_dir_all(&dir).expect("the processes directory is created");
    std::fs::write(
        dir.join("questions.toml"),
        "[question.\"probe.accept\"]\ntext = \"Accept the probe?\"\noptions = [\"accept\", \"reject\"]\n",
    )
    .expect("the question file writes");
    let out = crate::run::spawn_run(croot, "bug-1", None);
    assert_eq!(
        out.status.code(),
        Some(0),
        "CONTROL: a declared gate kind opens its gate and the run exits 0 at it. stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let kinds: Vec<String> = crate::support::store_rows(croot)
        .iter()
        .filter(|e| is_answerable_gate(e))
        .filter_map(|e| {
            e.payload
                .get("kind")
                .and_then(|k| k.as_str())
                .map(str::to_owned)
        })
        .collect();
    assert_eq!(
        kinds,
        ["probe.accept"],
        "CONTROL: the gate step's gate is open"
    );
}
