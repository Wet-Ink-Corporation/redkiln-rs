//! **`RK-D-ARTIFACT-MUTATION` — AC-159's unrecorded artifact mutation** (`s8f` §8f.9; the render
//! record ADR-0189 decision 13, proposed; D21-43 N12). *(Added 2026-09-29, s8f build, package
//! S8F-P8-DOCTOR-RULES-AND-NOTES.)*
//!
//! AC-159: *"GIVEN a managed artifact whose **structured-section or frontmatter digest** differs
//! from the store's last recorded rendering (a prose-section edit accepted by `reconcile()` under
//! AC-152 is not a mutation; the reconcile that accepted it counts as the write record …) WHEN
//! `doctor` runs and no store write record accounts for the difference THEN the file is reported
//! as an unrecorded artifact mutation, naming artifact, both digests and the last recorded
//! write."*
//!
//! Every managed file here is written by `rk-store`'s OWN writer (`ArtifactWriter`) into a
//! run-owned tree, and every row the rule reads is a row that writer (or `reconcile_entry`, or the
//! vouched pass) appended — so a control is "rewritten through the store" by the store itself, not
//! by a test's imitation of it. The rule runs over `Ctx::build`, the verb's one builder, with the
//! journal named as the fixture source (`events.jsonl`, the rows dumped verbatim).
//!
//! These are NOT in `tests/it/doctor.rs`: D21-42 18 keeps `rk-fix-doctor-test-count` as *"the
//! untouched fixture item, prose defect included"* (§8f.9).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_cli::doctor::rules::RULES;
use rk_cli::doctor::{Ctx, Finding, JournalSource};
use rk_store::render::{self, ArtifactWriter, Journal, ManagedDir, StageRun};
use rk_types::digest::sha256;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::support;

const RULE: &str = "RK-D-ARTIFACT-MUTATION";
const ITEM: &str = "RK-S0042";
const NOW: i64 = 1_800_000_000;

type Rows = Arc<Mutex<Vec<Event>>>;

struct Pinned;

impl rk_cli::doctor::probe::VersionProbe for Pinned {
    fn version(&self, _program: &str) -> Result<String, rk_cli::doctor::probe::ProbeError> {
        Ok(rk_adapter_claude::config::TESTED_CLI_VERSION.to_owned())
    }
}

/// A checkout (the `clean` doctor fixture, so the whole `ConfigFile` loads) and its run-owned tree
/// at `.redkiln/worktrees/<item>/`, with the journal the store's writer appends to.
struct Repo {
    checkout: tempfile::TempDir,
    rows: Rows,
}

impl Repo {
    fn new() -> Self {
        Self {
            checkout: crate::doctor::materialise("clean"),
            rows: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn root(&self) -> &Path {
        self.checkout.path()
    }

    /// The run-owned tree of `item`, where `s8e` puts it: rk-git's ONE derivation of the root
    /// (`crate::run_tree`; Checkpoint 8d-ii (3), D21-44). *(dated 2026-09-29, s8f build, package
    /// R1FIX-8f-A-CODE: it joined the literal before.)*
    fn tree(&self, item: &str) -> PathBuf {
        crate::run_tree(self.root(), item).path
    }

    /// Append the `worktree_assigned` row a `run` writes, through the kernel's own constructor.
    fn assign(&self, item: &str) {
        let tree = self.tree(item);
        std::fs::create_dir_all(&tree).expect("the run tree exists");
        self.rows.lock().expect("rows").push(
            rk_kernel::runtime::tier_one::worktree_assigned_event(
                item,
                &tree,
                &format!("rk-run/{item}"),
                false,
                &[],
            ),
        );
    }

    /// `rk-store`'s writer over `item`'s run tree, bound to `stage`'s run.
    fn writer(&self, item: &str, stage: &str) -> ArtifactWriter {
        let append = Arc::clone(&self.rows);
        let read = Arc::clone(&self.rows);
        ArtifactWriter::new(
            &self.tree(item),
            StageRun {
                item: item.to_owned(),
                stage: stage.to_owned(),
                run: format!("{stage}-0123456789abcdef"),
            },
            std::collections::BTreeMap::new(),
            Journal::new(
                move |e| {
                    append.lock().map_err(|e| e.to_string())?.push(e);
                    Ok(())
                },
                move || Ok(read.lock().map_err(|e| e.to_string())?.clone()),
            ),
        )
    }

    fn rendering(&self, item: &str) -> PathBuf {
        ManagedDir::of(&self.tree(item), item).abs("Spec.md")
    }

    /// Replace `from` with `to` in `item`'s `Spec.md`, out-of-band, asserting it was there once.
    fn edit(&self, item: &str, from: &str, to: &str) {
        let md = self.rendering(item);
        let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering exists");
        assert_eq!(text.matches(from).count(), 1, "PREMISE: `{from}` once");
        std::fs::write(&md, text.replace(from, to)).expect("the edit is written");
    }

    /// The rule's findings over the journal as it stands, dumped to `events.jsonl` verbatim.
    fn fire(&self) -> Vec<Finding> {
        let rows = self.rows.lock().expect("rows").clone();
        let mut jsonl = String::new();
        for e in &rows {
            jsonl.push_str(
                &serde_json::json!({ "kind": e.kind.wire_name(), "payload": e.payload })
                    .to_string(),
            );
            jsonl.push('\n');
        }
        std::fs::write(self.root().join("events.jsonl"), jsonl).expect("events.jsonl");
        fire(&ctx(self.root()))
    }

    fn count(&self, kind: EventKind) -> usize {
        self.rows
            .lock()
            .expect("rows")
            .iter()
            .filter(|e| e.kind == kind)
            .count()
    }
}

/// `Ctx::build` over `root`, the verb's one builder, the journal named as the fixture source.
fn ctx(root: &Path) -> Ctx {
    let config_src = std::fs::read_to_string(root.join(".redkiln").join("config.toml"))
        .expect("the repository carries a .redkiln/config.toml");
    let config = rk_cli::config::parse(&config_src).expect("the ConfigFile parses");
    let repo = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    Ctx::build(
        root.to_path_buf(),
        repo.common_dir,
        config,
        support::clean_env(),
        None,
        Box::new(Pinned),
        NOW,
        JournalSource::FixtureEventsJsonl,
    )
}

fn fire(ctx: &Ctx) -> Vec<Finding> {
    let rule = RULES
        .iter()
        .find(|r| r.id == RULE)
        .unwrap_or_else(|| panic!("{RULE} is in RULES"));
    (rule.check)(ctx)
}

/// A `Spec` value: frontmatter `id`/`status`/`title`, structured `acceptance` and `scope`, prose
/// `context` and `notes`.
fn spec(acceptance: &[&str]) -> serde_json::Value {
    serde_json::json!({
        "acceptance": acceptance,
        "context": "Why the spec exists.",
        "id": "RK-S0042-spec",
        "notes": "",
        "scope": { "in": "the doctor rule" },
        "status": "draft",
        "title": "A spec",
    })
}

fn assert_clean(found: &[Finding], why: &str) {
    assert!(found.is_empty(), "{why}: {found:#?}");
}

// ------------------------------------------------------------------------------- the tests

/// **AC-159's rejecting case**: ONE managed file edited out-of-band, table-driven over a
/// structured section and the frontmatter (one arm each, so no arm hides behind the other). Each
/// arm reports exactly one finding, of this rule and at `Error`, naming the artifact, the anchor,
/// BOTH digests (the rendering's and the bytes on disk, each recomputed here through `rk-store`'s
/// own renderer and locator) and the last recorded write. The committed fixture
/// `doctor/RK-D-ARTIFACT-MUTATION/` is run too, and names exactly its one edited section — so every
/// other byte of it is the renderer's.
///
/// Red before the rule: no finding.
#[test]
fn an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation() {
    let value = spec(&["one", "two"]);
    let template = render::template_of("Spec").expect("Spec is human-facing");
    let arms: [(&str, &str, &str, Vec<u8>); 2] = [
        (
            "acceptance",
            r#"["one","two"]"#,
            r#"["one","two","three"]"#,
            render::section_slice(template, "acceptance", &value).expect("the slice"),
        ),
        (
            "frontmatter",
            r#"title: "A spec""#,
            r#"title: "A spec, retitled by hand""#,
            render::frontmatter_block(template, &value).expect("the frontmatter"),
        ),
    ];
    for (anchor, from, to, rendered) in arms {
        let repo = Repo::new();
        repo.assign(ITEM);
        let w = repo.writer(ITEM, "implement");
        let written = w.write("Spec", &value).expect("the write");
        assert_clean(
            &repo.fire(),
            "PREMISE: a tree the store just wrote is clean",
        );

        repo.edit(ITEM, from, to);
        let disk = std::fs::read_to_string(repo.rendering(ITEM)).expect("the rendering");
        let range = if anchor == "frontmatter" {
            rk_store::locator::frontmatter(&disk).expect("the frontmatter locates")
        } else {
            rk_store::locator::resolve(
                &disk,
                &rk_store::locator::Anchor {
                    text: anchor.to_owned(),
                    level: render::SECTION_LEVEL,
                },
            )
            .expect("the section locates")
        };
        let observed = sha256(&disk.as_bytes()[range]);
        let rendered = sha256(&rendered);
        assert_ne!(observed, rendered, "PREMISE: the edit moved the {anchor}");

        let found = repo.fire();
        assert_eq!(
            found.len(),
            1,
            "one finding for the {anchor} arm: {found:#?}"
        );
        let f = &found[0];
        assert_eq!(f.id, RULE);
        assert_eq!(f.severity, rk_cli::doctor::Severity::Error);
        for (what, needle) in [
            ("the artifact", format!(".redkiln/artifacts/{ITEM}/Spec.md")),
            ("the anchor", format!("`{anchor}`")),
            ("the rendering's digest", rendered.as_str().to_owned()),
            ("the digest on disk", observed.as_str().to_owned()),
            (
                "the last recorded write",
                written.digest.as_str().to_owned(),
            ),
        ] {
            assert!(
                f.detail.contains(&needle),
                "the {anchor} arm's finding does not name {what} ({needle}): {}",
                f.detail
            );
        }
    }

    // ---- the committed rejecting fixture: exactly its one edited section.
    let dir = crate::doctor::materialise(RULE);
    let found = fire(&ctx(dir.path()));
    assert_eq!(found.len(), 1, "the committed fixture: {found:#?}");
    assert!(
        found[0].detail.contains("`acceptance`")
            && found[0]
                .detail
                .contains(".redkiln/artifacts/RK-S0001/Spec.md"),
        "the committed fixture names its edited section: {}",
        found[0].detail
    );
}

/// **AC-159's control**: *"a control fixture whose file is rewritten **through** the store,
/// asserted not to be reported"*. Two arms: (1) the value is rewritten by the writer (a second
/// `artifact.write`, the rendering's bytes moved); (2) an out-of-band structured edit that the
/// entry pass surfaced as `structured_drift` and RESTORED through the writer. Neither is reported.
///
/// Red by `M27` (the rule ignores the store's write record — it takes the FIRST recorded write for
/// the sidecar, so the rewrite's own row accounts for nothing).
#[test]
fn a_rewrite_through_the_store_is_not_reported() {
    let repo = Repo::new();
    repo.assign(ITEM);
    let w = repo.writer(ITEM, "implement");
    w.write("Spec", &spec(&["one"])).expect("the first write");
    let before = std::fs::read(repo.rendering(ITEM)).expect("the rendering");
    let second = repo.writer(ITEM, "review");
    second
        .write("Spec", &spec(&["one", "two", "three"]))
        .expect("the rewrite");
    assert_ne!(
        std::fs::read(repo.rendering(ITEM)).expect("the rendering"),
        before,
        "PREMISE: the rewrite moved the rendering's bytes"
    );
    assert_eq!(
        repo.count(EventKind::ArtifactWrite),
        2,
        "PREMISE: two writes"
    );
    assert_clean(
        &repo.fire(),
        "a file rewritten through the store is reported",
    );

    // ---- (2) out-of-band, then restored through the store by the entry pass.
    repo.edit(
        ITEM,
        r#"{"in":"the doctor rule"}"#,
        r#"{"in":"a hand edit"}"#,
    );
    assert_eq!(
        repo.fire().len(),
        1,
        "PREMISE: the edit is a mutation before the entry pass"
    );
    let done = render::reconcile_entry(&second, &|_: &str| Ok(None)).expect("the entry pass");
    assert_eq!(done.drifts().len(), 1, "PREMISE: one structured_drift");
    assert_eq!(
        repo.count(EventKind::StructuredDrift),
        1,
        "PREMISE: its row"
    );
    assert_clean(
        &repo.fire(),
        "a file restored through the store by the entry pass is reported",
    );
}

/// **AC-152 / AC-159's parenthetical**: *"a prose-section edit accepted by `reconcile()` under
/// AC-152 is not a mutation; the reconcile that accepted it counts as the write record"*. Two arms:
/// (1) the prose edit on disk, before any entry pass — a prose-only edit is not a mutation, and no
/// whole-file digest is the instrument (PIA-043's amendment); (2) the entry pass accepts it into
/// the value through the writer. Neither is reported.
///
/// Red by `M26` (the rule compares a whole-file digest: arm (1)'s rendering differs from the
/// value's as a whole).
#[test]
fn a_prose_edit_accepted_by_reconcile_is_not_reported() {
    let repo = Repo::new();
    repo.assign(ITEM);
    let w = repo.writer(ITEM, "implement");
    w.write("Spec", &spec(&["one"])).expect("the write");
    repo.edit(
        ITEM,
        "Why the spec exists.",
        "Rewritten entirely by a human, in two paragraphs.\n\nThe second one.",
    );
    let disk = std::fs::read(repo.rendering(ITEM)).expect("the rendering");
    let sidecar = ManagedDir::of(&repo.tree(ITEM), ITEM)
        .read("Spec.json")
        .expect("the sidecar");
    let value: serde_json::Value = serde_json::from_slice(&sidecar).expect("the sidecar parses");
    let whole = render::render(render::template_of("Spec").expect("Spec"), &value)
        .expect("the rendering of the value");
    assert_ne!(
        sha256(&disk),
        sha256(&whole),
        "PREMISE: as a WHOLE FILE the rendering now differs from the value's"
    );
    assert_clean(&repo.fire(), "a prose-only edit on disk is reported");

    let done = render::reconcile_entry(&w, &|_: &str| Ok(None)).expect("the entry pass");
    assert!(
        !done.accepted().is_empty(),
        "PREMISE: the entry pass accepted the prose edit"
    );
    assert_eq!(
        repo.count(EventKind::StructuredDrift),
        0,
        "PREMISE: no drift"
    );
    assert_clean(&repo.fire(), "a prose edit reconcile accepted is reported");
}

/// **D21-44's `worktrees/` exclusion, by construction**: the rule reads the run trees the store
/// NAMES (`worktree_assigned`) and the checkout's own managed trees, and never walks `.redkiln/`.
/// A mutated copy under `.redkiln/worktrees/x/` that no `worktree_assigned` row names is never
/// read. The SYNTHETIC CONTROL is the same tree once a row names it: then it is reported, so the
/// silence above is the exclusion and not a rule that reads nothing.
#[test]
fn an_unassigned_copy_under_redkiln_worktrees_is_never_read() {
    let repo = Repo::new();
    let w = repo.writer("x", "implement");
    w.write("Spec", &spec(&["one"])).expect("the write");
    repo.edit("x", r#"["one"]"#, r#"["one","a hand edit"]"#);
    assert_eq!(
        repo.count(EventKind::WorktreeAssigned),
        0,
        "PREMISE: no row names the tree"
    );
    assert!(
        repo.tree("x")
            .join(".redkiln")
            .join("artifacts")
            .join("x")
            .join("Spec.md")
            .is_file(),
        "PREMISE: the mutated copy is in the run tree rk-git derives for `x`"
    );
    assert_clean(&repo.fire(), "an unassigned copy was read");

    repo.assign("x");
    let found = repo.fire();
    assert_eq!(
        found.len(),
        1,
        "CONTROL: once a worktree_assigned row names the tree, its mutation is reported: {found:#?}"
    );
}

/// **A vouched root is out of scope** (§8f.9; ADR-0189 decisions 10 and 13): it is not layout C, and
/// its change is `drift_advisory`'s. A stored root (`<Kind>.root<ext>`, stored by the vouched pass
/// through the writer) edited on disk is NOT reported. In the same tree a `Spec` structured edit IS
/// — so the rule ran over the tree, and the one finding names `Spec`, never the root.
///
/// Red by `M43` (the rule reads vouched roots).
#[test]
fn a_vouched_root_edit_is_not_reported_as_an_unrecorded_mutation() {
    let repo = Repo::new();
    repo.assign(ITEM);
    let vouched = b"{\"items\":[]}\n";
    std::fs::write(repo.root().join("findings.json"), vouched).expect("the attached file");
    repo.rows.lock().expect("rows").push(
        Event::new(
            EventKind::ArtifactWrite,
            serde_json::json!({
                "attached_path": "findings.json",
                "digest": sha256(vouched).as_str(),
                "item": ITEM,
                "kind": "Findings",
                "run": "implement-fedcba9876543210",
                "schema_version": null,
                "stage": "implement",
            }),
        )
        .with_item(ITEM),
    );
    let w = repo.writer(ITEM, "implement");
    let pass = rk_store::vouched::vouched_pass(&w, repo.root(), &mut |_| {
        Err("PREMISE: nothing is advised at the storing entry".to_owned())
    })
    .expect("the root is stored");
    assert_eq!(pass.stored().len(), 1, "PREMISE: the root is stored");
    let root = ManagedDir::of(&repo.tree(ITEM), ITEM).abs("Findings.root.json");
    std::fs::write(&root, b"{\"items\":[\"added after vouching\"]}\n").expect("the root edit");

    w.write("Spec", &spec(&["one"])).expect("the Spec write");
    assert_clean(&repo.fire(), "a vouched root's edit is reported");

    repo.edit(ITEM, r#"["one"]"#, r#"["one","a hand edit"]"#);
    let found = repo.fire();
    assert_eq!(
        found.len(),
        1,
        "CONTROL: the Spec edit in the same tree: {found:#?}"
    );
    assert!(
        found[0].detail.contains("Spec.md") && !found[0].detail.contains("Findings"),
        "the one finding is the Spec's, never the root's: {}",
        found[0].detail
    );
}
