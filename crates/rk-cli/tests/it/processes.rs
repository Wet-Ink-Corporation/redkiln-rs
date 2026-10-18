//! **The question registry's loader** — `.redkiln/processes/*.toml` into `QuestionRegistry`
//! (`s8d` §8d.6; R5 = ADR-0177, proposed). *(Added 2026-09-26, `s8d` build, package P7.)*
//!
//! Three properties, one test each: the shipped file declares the lanes' nine questions and N3's
//! `facet.change`, transcribed (not invented) from `spikes/p9/questions.toml` and `facets.toml`;
//! `answer` validates against the LOADED registry and never `QuestionRegistry::default()` (M16);
//! and `run` refuses, by name and before any dispatch, a catalog gate step whose question the
//! registry does not declare.
//!
//! *(dated 2026-10-18, `s8j` build, package S8J-C2-ROOT-QUESTIONS; `s8j` §8j.2; ADR-0200,
//! proposed.)* Three more, one test each, on the two human roots: their questions load and each
//! attaches its kind; every human kind a lane demands is attached by exactly one question; and the
//! step-4 gate on a human kind opens as `human_root:<Kind>`, answered by its question.

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

// ---- s8j §8j.2: the human roots (ADR-0200, proposed) -------------------------------------------
// *(Added 2026-10-18, `s8j` build, package S8J-C2-ROOT-QUESTIONS.)*

/// `(question id, the kind its one option attaches, the gate kind it answers)`, as ADR-0200
/// decisions 1 and 2 spell them. Literals, not the kernel's constants: the constants are under test.
const HUMAN_ROOTS: [(&str, &str, &str); 2] = [
    ("root.bug-report", "BugReport", "human_root:BugReport"),
    (
        "root.intake-record",
        "IntakeRecord",
        "human_root:IntakeRecord",
    ),
];

/// **The two human-root questions load from `.redkiln/processes/`, and each one's one option
/// attaches exactly its kind** (`s8j` §8j.2 item 1; ADR-0200 decision 1).
///
/// Each is declared with text, has the one option `attach` whose `attaches` names its kind and
/// nothing else, and answers the `human_root:<Kind>` gate through the loader's `binds` (decision 2),
/// never the default question. The kernel's two minted spellings are those gate kinds, and its one
/// mint, `human_root::gate_kind`, spells them. `M-attach-dropped` (the BugReport option loses its
/// `attaches`) fails the `attaches` assertion here.
#[test]
fn the_two_human_root_questions_load_and_each_attaches_its_kind() {
    use rk_kernel::workflow::human_root;
    let registry = rk_cli::processes::load(&workspace_root())
        .unwrap_or_else(|e| panic!("the shipped .redkiln/processes/ loads: {e}"));
    for (id, kind, gate) in HUMAN_ROOTS {
        let question = registry.question(id).unwrap_or_else(|| {
            panic!(
                "`{id}` is not declared under .redkiln/processes/ (s8j §8j.2; ADR-0200 decision 1)"
            )
        });
        assert!(
            question.text().is_some_and(|t| !t.trim().is_empty()),
            "`{id}` carries its text"
        );
        assert_eq!(
            question.options(),
            ["attach"],
            "`{id}` has the one option `attach`"
        );
        assert_eq!(
            question.attaches("attach"),
            Some([kind.to_owned()].as_slice()),
            "`{id}`'s `attach` option must attach exactly `{kind}` (ADR-0129 decision 4: the \
             attachment is validated against the option, so this is what the gate admits)"
        );
        assert_eq!(
            registry.question_for(gate),
            id,
            "the `{gate}` gate must be answered with `{id}` (ADR-0200 decision 2), not the default \
             question, whose options attach nothing"
        );
        assert_eq!(
            human_root::gate_kind(kind),
            gate,
            "the kernel's mint for `{kind}`"
        );
    }
    assert_eq!(human_root::BUG_REPORT_KIND, "human_root:BugReport");
    assert_eq!(human_root::INTAKE_RECORD_KIND, "human_root:IntakeRecord");
}

/// The shipped-lane fixtures the three human-root tests share (TST-14: a test helper module).
#[cfg(test)]
mod lanes {
    use std::path::Path;

    use super::{read_toml, workspace_root};

    /// The two lanes: the shipped pack and the shipped item record each lane's real run is pointed at.
    pub(super) const LANES: [(&str, &str); 2] = [
        ("bug-standard", "rk-fix-doctor-test-count"),
        ("story", "rk-story-codex-adapter"),
    ];

    /// The shipped catalog, registries and one lane's pack, read off `.redkiln/` (never a fixture copy).
    pub(super) struct Shipped {
        pub(super) catalog: rk_kernel::catalog::Catalog,
        pub(super) artifacts: rk_kernel::select::ArtifactRegistry,
        pub(super) facets: rk_kernel::registry::Registry,
        pub(super) pack: rk_kernel::select::Pack,
    }

    fn read_text(path: &Path) -> String {
        std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()))
    }

    /// `artifacts_src` replaces the shipped `artifacts.toml`, for a control that re-registers a kind.
    pub(super) fn shipped(pack: &str, artifacts_src: &str) -> Shipped {
        let redkiln = workspace_root().join(".redkiln");
        let pack_file = format!("{pack}.toml");
        Shipped {
            catalog: rk_kernel::catalog::Catalog::from_dir(&redkiln.join("catalog"))
                .unwrap_or_else(|e| panic!("the shipped catalog loads: {e:?}")),
            artifacts: rk_kernel::select::ArtifactRegistry::from_toml(
                "artifacts.toml",
                artifacts_src,
            )
            .unwrap_or_else(|e| panic!("the artifact registry loads: {e:?}")),
            facets: rk_kernel::registry::Registry::from_toml(&read_text(
                &redkiln.join("facets.toml"),
            ))
            .unwrap_or_else(|e| panic!("the shipped facet registry loads: {e:?}")),
            pack: rk_kernel::select::Pack::from_toml(
                &pack_file,
                &read_text(&redkiln.join("packs").join(&pack_file)),
            )
            .unwrap_or_else(|e| panic!("the shipped pack {pack_file} loads: {e}")),
        }
    }

    pub(super) fn shipped_artifacts_src() -> String {
        read_text(&workspace_root().join(".redkiln").join("artifacts.toml"))
    }

    /// The shipped item record past triage, bound as `rk-kernel`'s `lane_walk` binds a lane's item:
    /// every declared facet the record does not state (save `kind` and the kernel's own) is an explicit
    /// `null`, the record's `[facets]` are bound (an ordered facet as a member of its domain),
    /// `toolchain` is `["rust"]`, and `change` is `text`, the triage gate's answer (D21-43 N3) that the
    /// fix lane's analysis producers' `change:text` tag reads.
    pub(super) fn shipped_item(
        facets: &rk_kernel::registry::Registry,
        id: &str,
    ) -> rk_kernel::select::Item {
        let record = read_toml(
            &workspace_root()
                .join(".redkiln")
                .join("items")
                .join(format!("{id}.toml")),
        );
        let kind = record
            .get("kind")
            .and_then(toml::Value::as_str)
            .unwrap_or_else(|| panic!("PREMISE: {id}'s record carries a `kind`"));
        let stated = record.get("facets").and_then(toml::Value::as_table);
        let mut item = rk_kernel::select::Item::new(id, kind);
        for facet in facets.facets() {
            let ours = facet.name == "kind" || facet.source == rk_kernel::registry::Source::Kernel;
            if !ours && !stated.is_some_and(|t| t.contains_key(&facet.name)) {
                item = item.with_facet_null(&facet.name);
            }
        }
        for (name, value) in stated.into_iter().flatten() {
            let domain: Option<Vec<&str>> = facets
                .facet(name)
                .filter(|f| f.ordered)
                .map(|f| f.values.iter().filter_map(|v| v.as_str()).collect());
            item = match (value, domain) {
                (toml::Value::Boolean(b), _) => item.with_facet_bool(name, *b),
                (toml::Value::String(s), Some(domain)) => item.with_facet_ordinal(name, &domain, s),
                (toml::Value::String(s), None) => item.with_facet_str(name, s),
                (other, _) => panic!("PREMISE: {id}'s `{name}` is a string or a bool, not {other}"),
            };
        }
        item.with_facet_set("toolchain", ["rust"])
            .with_toolchain("rust", ["text"])
    }

    /// The kernel's next action over a fresh journal for `item` at `stage`, and the selection it read.
    pub(super) fn first_action(
        s: &Shipped,
        stage: &str,
        item: &rk_kernel::select::Item,
    ) -> (
        rk_kernel::select::Selection,
        rk_kernel::workflow::fsm::NextAction,
    ) {
        use rk_kernel::workflow::fsm;
        let selection =
            rk_kernel::select::select(&s.pack, &s.catalog, &s.artifacts, &s.facets, stage, item)
                .unwrap_or_else(|e| panic!("PREMISE: the {stage} stage selects: {e}"));
        let state = fsm::fold(item.id(), stage, &[]).expect("an empty journal folds");
        let deadline = rk_types::time::Timestamp::from_unix_seconds(1_800_000_000);
        let ctx = fsm::FsmContext::supplied(item.id(), "tree-0", deadline, None);
        let action = fsm::next_action(&selection, &state, &ctx)
            .unwrap_or_else(|e| panic!("the next action at {stage} is computed: {e}"));
        (selection, action)
    }
}

use lanes::{LANES, first_action, shipped, shipped_artifacts_src, shipped_item};

/// **Every `source = "human"` kind a selected activity of either lane requires without `?` is
/// attached by exactly one question, and every question's `attaches` names a `source = "human"`
/// kind** (`s8j` §8j.2's both directions; ADR-0200 decision 1).
///
/// The demand is MEASURED, not listed: a real `select` over each shipped pack's whole
/// `stage_order` for the lane's shipped item, and the mandatory `requires` of every selected
/// producer. A stage whose selection refuses selects nothing and demands nothing; the count is
/// printed. A later human kind a lane demands fails here for its missing question (ADR-0200's
/// Negative consequences), and a question that attached an activity kind fails the second half.
#[test]
fn every_human_kind_a_lane_demands_is_attached_by_exactly_one_question() {
    let registry = rk_cli::processes::load(&workspace_root())
        .unwrap_or_else(|e| panic!("the shipped .redkiln/processes/ loads: {e}"));
    let artifacts_src = shipped_artifacts_src();

    let mut demanded: BTreeSet<String> = BTreeSet::new();
    let mut refused = 0_usize;
    for (pack, item_id) in LANES {
        let s = shipped(pack, &artifacts_src);
        let item = shipped_item(&s.facets, item_id);
        for stage in s.pack.stage_order() {
            let selection = match rk_kernel::select::select(
                &s.pack,
                &s.catalog,
                &s.artifacts,
                &s.facets,
                stage,
                &item,
            ) {
                Ok(selection) => selection,
                Err(e) => {
                    println!("HUMAN_ROOTS {pack}/{stage} refuses: {e}");
                    refused += 1;
                    continue;
                }
            };
            for producer in selection.producers() {
                let activity = s
                    .catalog
                    .activities()
                    .iter()
                    .find(|a| a.id == producer.activity_id)
                    .expect("a selected producer is in the catalog");
                demanded.extend(
                    activity
                        .requires
                        .iter()
                        .filter(|r| !r.optional && s.artifacts.is_human_sourced(&r.kind))
                        .map(|r| r.kind.clone()),
                );
            }
        }
    }
    println!("HUMAN_ROOTS demanded={demanded:?} refused_stages={refused}");
    assert!(
        demanded.contains("BugReport") && demanded.contains("IntakeRecord"),
        "PREMISE: the fix lane's analysis demands BugReport and the story lane's specification \
         IntakeRecord (s8j §8j.2, measured there); measured {demanded:?}"
    );

    // Every option's `attaches`, by question.
    let artifacts =
        rk_kernel::select::ArtifactRegistry::from_toml("artifacts.toml", &artifacts_src)
            .expect("the shipped artifact registry loads");
    let mut attached: Vec<(String, String)> = Vec::new();
    for id in registry.question_ids() {
        let question = registry.question(id).expect("a listed id is declared");
        for option in question.options() {
            for kind in question.attaches(option).into_iter().flatten() {
                assert!(
                    artifacts.is_human_sourced(kind),
                    "`{id}`'s option `{option}` attaches `{kind}`, which .redkiln/artifacts.toml \
                     does not declare `source = \"human\"`: an answer may supply only a human root"
                );
                attached.push((kind.clone(), id.to_owned()));
            }
        }
    }
    for kind in &demanded {
        let by: Vec<&str> = attached
            .iter()
            .filter(|(k, _)| k == kind)
            .map(|(_, q)| q.as_str())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(
            by.len(),
            1,
            "the lanes demand the human kind `{kind}`, which must be attached by exactly one \
             question (ADR-0200 decision 1); attached by {by:?}"
        );
    }
}

/// **Selection's step-4 `Unsatisfiable` on a `source = "human"` kind opens a durable gate of kind
/// `human_root:<Kind>`, and the loaded registry answers that gate with the question whose one
/// option attaches the kind** (`s8j` §8j.2 item 2; ADR-0200 decision 2; AC-054's *"opens a gate"*).
///
/// Over the shipped pack, catalog and registries, the fix lane's analysis stage (whose
/// `minimal-repro` requires `BugReport`) and the story lane's specification stage (whose
/// `bdd-scenarios` requires `IntakeRecord`) each report the root `Unsatisfiable {
/// NoAdmissibleProducer }`, and the kernel's next action on a fresh journal is `OpenGate` of that
/// gate kind, before any dispatch. CONTROL (*"Not widened"*): the same stage with the kind
/// re-registered `source = "activity"` reports the same `Unsatisfiable` and opens no
/// `human_root:` gate, so the gate is the registry's source and not the kind's name.
#[test]
fn the_step_four_gate_on_a_human_kind_is_bound_to_its_question() {
    use rk_kernel::select::UnsatReason;
    use rk_kernel::workflow::fsm::NextAction;
    let registry = rk_cli::processes::load(&workspace_root())
        .unwrap_or_else(|e| panic!("the shipped .redkiln/processes/ loads: {e}"));
    let artifacts_src = shipped_artifacts_src();
    let stages = ["analysis", "specification"];
    for (((pack, item_id), stage), (id, kind, gate)) in
        LANES.into_iter().zip(stages).zip(HUMAN_ROOTS)
    {
        let unsat = |selection: &rk_kernel::select::Selection| {
            selection
                .unsatisfiable()
                .iter()
                .any(|u| u.kind == kind && u.reason == UnsatReason::NoAdmissibleProducer)
        };
        let s = shipped(pack, &artifacts_src);
        let item = shipped_item(&s.facets, item_id);
        let (selection, action) = first_action(&s, stage, &item);
        assert!(
            unsat(&selection),
            "PREMISE: {pack}/{stage} reports `{kind}` Unsatisfiable {{ NoAdmissibleProducer }}"
        );
        let NextAction::OpenGate(record) = &action else {
            panic!(
                "{pack}/{stage}: with `{kind}` unattached the next action must open its gate \
                 (ADR-0200 decision 2), not {action:?}"
            );
        };
        assert_eq!(record.kind(), gate, "{pack}/{stage}: the gate's kind");
        assert_eq!(record.reach(), 1, "a gate not yet reached opens at reach 1");
        assert_eq!(
            registry.question_for(record.kind()),
            id,
            "{pack}/{stage}: the `{gate}` gate is answered with `{id}`"
        );
        let question = registry
            .question(id)
            .expect("the bound question is declared");
        assert_eq!(
            question.attaches("attach"),
            Some([kind.to_owned()].as_slice()),
            "{pack}/{stage}: the gate's question admits exactly `{kind}`"
        );

        // CONTROL: the kind registered `source = "activity"` is unsatisfiable the same way and
        // opens no `human_root:` gate.
        let human = format!("[artifact.{kind}]\nsource = \"human\"");
        assert_eq!(
            artifacts_src.matches(&human).count(),
            1,
            "PREMISE: artifacts.toml registers `{kind}` human"
        );
        let as_activity = artifacts_src.replacen(
            &human,
            &format!("[artifact.{kind}]\nsource = \"activity\""),
            1,
        );
        let control = shipped(pack, &as_activity);
        let (selection, action) = first_action(&control, stage, &item);
        assert!(
            unsat(&selection),
            "PREMISE (control): `{kind}` stays Unsatisfiable when registered `activity`"
        );
        assert!(
            !matches!(&action, NextAction::OpenGate(r) if r.kind().starts_with("human_root:")),
            "CONTROL: an activity-sourced Unsatisfiable must open no human_root gate (ADR-0200 \
             decision 2, not widened); got {action:?}"
        );
    }
}
