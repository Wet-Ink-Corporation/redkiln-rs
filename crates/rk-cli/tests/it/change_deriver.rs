//! **The `change` deriver in `build_item`, and `change` at triage (N3)** (`s8d` §8d.7; R6 =
//! ADR-0178, proposed). *(Added 2026-09-26, `s8d` build, package P7.)*
//!
//! `R-change-extension-and-numstat-marker`, ported from `spikes/p1/derive-facets.py`'s `derive` and
//! read through `rk-git`: over the item's changeset (`HEAD..rk-run/<item>`), `text` is a path that
//! is neither generated nor marked binary, `binary` is `--numstat`'s `-\t-` marker or the `unreal`
//! table, and `generated` is the generated table. An empty range ABSTAINS (`null`), a record-supplied
//! `change` is refused by name, and a `null` `change` at the lane's first stage opens AC-079's
//! `unknown_facet` gate once, whose answer binds `change` as a one-element set, folded from the
//! journal and never written to the record.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_cli::change_deriver::{DERIVER, classify, derive};
use rk_kernel::select::{FacetValue, Item};

use crate::answer::{BOTH_FLAGS, is_answerable_gate, spawn_answer};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The shipped `[facet.change]` table, verbatim: the deriver it names is what `build_item` wires.
fn shipped_change_facet() -> String {
    let text = std::fs::read_to_string(workspace_root().join(".redkiln").join("facets.toml"))
        .expect("the shipped facets.toml reads");
    let start = text
        .find("[facet.change]")
        .expect("PREMISE: the shipped facets.toml declares [facet.change]");
    let rest = &text[start..];
    let end = rest[1..].find("\n[").map_or(rest.len(), |i| i + 1);
    let table = rest[..end].trim_end().to_owned();
    assert!(
        table.contains("source = \"derived\"") && table.contains(DERIVER),
        "PREMISE: [facet.change] is derived by {DERIVER}: {table}"
    );
    table
}

/// `pack-min`, plus the shipped `change` facet, a declared lane order (so the lane's first stage
/// is `triage`), the shipped question files, and two items: `chg-1` at `triage` and `chg-rec`,
/// whose record supplies `change` itself.
fn change_repo() -> tempfile::TempDir {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let rk = root.join(".redkiln");

    let facets = rk.join("facets.toml");
    let mut src = std::fs::read_to_string(&facets).expect("facets.toml reads");
    src.push_str("\n\n");
    src.push_str(&shipped_change_facet());
    src.push('\n');
    std::fs::write(&facets, src).expect("facets.toml writes");

    for pack in ["bug-standard.toml", "story.toml"] {
        let path = rk.join("packs").join(pack);
        let src = std::fs::read_to_string(&path).expect("the pack reads");
        let anchor = "[policy]\nstages = { triage = \"mandatory\", implement = \"recommended\", review = \"mandatory\" }\n";
        assert_eq!(
            src.matches(anchor).count(),
            1,
            "PREMISE: pack-min's [policy] table"
        );
        let out = src.replacen(
            anchor,
            &format!("{anchor}stage_order = [\"triage\", \"implement\", \"review\"]\n"),
            1,
        );
        std::fs::write(&path, out).expect("the pack writes");
    }

    // The lane advances out of `triage` once the gate is answered, and an advance writes a
    // `Story:` trailer, whose value needs `[project] name` (ADR-0159).
    // *(dated 2026-09-28, `s8e` build, package S8E-P2.)* ~~This fixture appended its own
    // `[project] name = "chg-fixture"`.~~ `pack-min/config.toml` now declares `[project] name` for
    // every fixture repository a lane runs in (`s8e` §8e.11), so a second `[project]` table would be
    // a duplicate key; the shared one is asserted instead.
    let config = rk.join("config.toml");
    let cfg = std::fs::read_to_string(&config).expect("config.toml reads");
    assert_eq!(
        cfg.lines().filter(|l| l.trim() == "[project]").count(),
        1,
        "PREMISE: the fixture configuration declares exactly one `[project]` table"
    );

    let processes = rk_cli::processes::processes_dir(root);
    std::fs::create_dir_all(&processes).expect("the processes directory is created");
    std::fs::copy(
        rk_cli::processes::processes_dir(&workspace_root()).join("questions.toml"),
        processes.join("questions.toml"),
    )
    .expect("the shipped question file installs");

    std::fs::write(
        rk.join("items").join("chg-1.toml"),
        "id    = \"chg-1\"\nkind  = \"bug\"\nstage = \"triage\"\n\n[facets]\nhas_repro = true\n",
    )
    .expect("chg-1 writes");
    std::fs::write(
        rk.join("items").join("chg-rec.toml"),
        "id    = \"chg-rec\"\nkind  = \"bug\"\nstage = \"implement\"\n\n[facets]\nhas_repro = true\nchange = [\"text\"]\n",
    )
    .expect("chg-rec writes");
    repo
}

fn git(cwd: &Path, args: &[&str]) {
    crate::git(cwd, args);
}

/// Put a changeset on `rk-run/<item>`: a text source, a binary blob (a NUL byte, so `--numstat`
/// marks it `-\t-`) and a generated lockfile. The checkout returns to where it was.
fn changeset_on_run_branch(root: &Path, item: &str) {
    git(
        root,
        &["checkout", "--quiet", "-b", &format!("rk-run/{item}")],
    );
    std::fs::create_dir_all(root.join("src")).expect("mkdir src");
    std::fs::write(root.join("src").join("lib.rs"), "pub fn f() {}\n").expect("lib.rs");
    std::fs::create_dir_all(root.join("assets")).expect("mkdir assets");
    std::fs::write(
        root.join("assets").join("logo.bin"),
        b"\x89PNG\x00\x01\x02\x00binary",
    )
    .expect("logo.bin");
    std::fs::write(root.join("Cargo.lock"), "# generated\n").expect("Cargo.lock");
    git(
        root,
        &["add", "src/lib.rs", "assets/logo.bin", "Cargo.lock"],
    );
    git(root, &["commit", "--quiet", "-m", "the item's changeset"]);
    git(root, &["checkout", "--quiet", "-"]);
}

fn repo_of(root: &Path) -> rk_cli::repo::Repo {
    rk_cli::repo::discover(Some(root)).expect("the fixture repository discovers")
}

fn change_of(item: &Item) -> Option<&FacetValue> {
    item.facet("change")
}

fn set(values: &[&str]) -> FacetValue {
    FacetValue::Set(
        values
            .iter()
            .map(|v| (*v).to_owned())
            .collect::<BTreeSet<_>>(),
    )
}

fn through_step_seven(
    root: &Path,
    item: &str,
) -> Result<rk_cli::verbs::run::Resolved, rk_cli::exit::Exit> {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::RunArgs {
        item: item.to_owned(),
        lane: None,
    };
    rk_cli::verbs::run::through_step_seven(
        &cli,
        &args,
        &crate::support::clean_env(),
        rk_cli::verbs::Startup::real(),
        rk_git::reconciled::Entry::Run,
    )
}

/// **`build_item` derives `change` over the recorded range.** The deriver reads `HEAD..rk-run/<item>`
/// through `rk-git` and finds all three values; `build_item` (steps (1)-(7)) carries exactly them.
/// The pure rule is the Python's: an unreal-table path that `--numstat` did not mark is `text`
/// AND `binary`, and a generated path is never `text`.
#[test]
fn build_item_derives_change_over_the_recorded_range() {
    let repo = change_repo();
    let root = repo.path();
    changeset_on_run_branch(root, "chg-1");

    let found = repo_of(root);
    let derived = derive(&found.exec, &found.worktree, "chg-1").expect("the range reads");
    assert_eq!(
        derived,
        Some(vec![
            "binary".to_owned(),
            "generated".to_owned(),
            "text".to_owned()
        ]),
        "the changeset carries a text source, a numstat-binary blob and a generated lockfile"
    );

    let resolved = through_step_seven(root, "chg-1").expect("steps (1)-(7) resolve");
    assert_eq!(
        change_of(&resolved.item),
        Some(&set(&["binary", "generated", "text"])),
        "build_item carries the derived `change`"
    );

    // The rule, as ported.
    let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
    assert_eq!(classify(&s(&["src/a.rs"]), &[]), s(&["text"]));
    assert_eq!(classify(&s(&["img.png"]), &s(&["img.png"])), s(&["binary"]));
    assert_eq!(classify(&s(&["pnpm-lock.yaml"]), &[]), s(&["generated"]));
    assert_eq!(
        classify(&s(&["target/x.o"]), &s(&["target/x.o"])),
        s(&["binary", "generated"])
    );
    assert_eq!(
        classify(&s(&["Game/Maps/Level.umap"]), &[]),
        s(&["binary", "text"]),
        "an unreal-table path numstat did not mark is `binary` by the table AND `text` (the Python's \
         rule, verbatim)"
    );
    assert_eq!(
        classify(&s(&["Config/Default.ini"]), &[]),
        s(&["binary", "text"])
    );
    assert_eq!(classify(&s(&["app.min.js"]), &[]), s(&["generated"]));
}

/// **A record-supplied `change` is refused by name.** `facets.toml` declares `source = "derived"`,
/// so a `[facets] change` in the item record is not a human value on a derived facet (R6's reading
/// of ADR-0093 decision 1's source classes): steps (1)-(7) refuse it, and the binary exits 3
/// naming the facet and its deriver, having written only the reconcile row.
#[test]
fn a_record_supplied_value_for_a_derived_facet_is_refused() {
    let repo = change_repo();
    let root = repo.path();

    assert!(
        through_step_seven(root, "chg-1").is_ok(),
        "CONTROL: an item whose record omits `change` resolves"
    );
    assert!(
        through_step_seven(root, "chg-rec").is_err(),
        "a record-supplied `change` must be refused, never read as the facet's value (M17)"
    );

    let before = crate::support::store_rows(root).len();
    let out = crate::run::spawn_run(root, "chg-rec", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        stderr.contains("`change`") && stderr.contains("derived") && stderr.contains(DERIVER),
        "the refusal names the facet, its source class and its deriver. stderr:\n{stderr}"
    );
    let added: Vec<_> = crate::support::store_rows(root)[before..]
        .iter()
        .map(|e| e.kind)
        .collect();
    assert_eq!(
        added,
        [rk_types::event_kind::EventKind::ReconcileStarted],
        "the refusal writes only the invocation's reconcile row (AC-172)"
    );
}

/// **An empty range abstains and never defaults.** No `rk-run/<item>` ref, or one with nothing on
/// it past `HEAD`, is `null` — never `[]` and never `text` — and `build_item` carries `null`.
/// CONTROL first: a non-empty range derives a value, so the abstention is not the only answer.
#[test]
fn an_empty_range_abstains_and_never_defaults() {
    let control = change_repo();
    changeset_on_run_branch(control.path(), "chg-1");
    let found = repo_of(control.path());
    assert!(
        derive(&found.exec, &found.worktree, "chg-1")
            .expect("the range reads")
            .is_some_and(|v| !v.is_empty()),
        "CONTROL: a non-empty changeset derives a value"
    );

    let repo = change_repo();
    let root = repo.path();
    let found = repo_of(root);
    // No `rk-run/chg-1` ref at all: the item has no changeset.
    assert_eq!(
        derive(&found.exec, &found.worktree, "chg-1").expect("an absent ref is not an error"),
        None,
        "no changeset: the deriver abstains"
    );
    let resolved = through_step_seven(root, "chg-1").expect("steps (1)-(7) resolve");
    assert_eq!(
        change_of(&resolved.item),
        Some(&FacetValue::Null),
        "an abstention is `with_facet_null`, never `[]` and never `text`"
    );
    // The ref exists and carries nothing past HEAD: still an abstention.
    git(root, &["branch", "rk-run/chg-1"]);
    assert_eq!(
        derive(&found.exec, &found.worktree, "chg-1").expect("an empty range reads"),
        None,
        "an EMPTY range abstains"
    );
    let resolved = through_step_seven(root, "chg-1").expect("steps (1)-(7) resolve");
    assert_eq!(change_of(&resolved.item), Some(&FacetValue::Null));
    assert!(
        classify(&[], &[]).is_empty(),
        "the rule over no path is empty — which is why `derive` must never reach it for an empty range"
    );
}

/// The item context the LAST run record of `root` carries, rebuilt.
fn last_recorded_item(root: &Path) -> Item {
    let runs = crate::common_dir(root).join("redkiln").join("runs");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&runs)
        .unwrap_or_else(|e| panic!("{}: {e}", runs.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("inputs").join("context.json").is_file())
        .collect();
    dirs.sort_by_key(|p| {
        std::fs::metadata(p.join("run.json"))
            .and_then(|m| m.modified())
            .ok()
    });
    let last = dirs.last().expect("at least one run record");
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_str(
            &std::fs::read_to_string(last.join("inputs").join(name)).expect("a recorded input"),
        )
        .expect("recorded JSON")
    };
    Item::from_recorded(&read("context.json"), &read("artifact-state.json"))
        .expect("the recorded item rebuilds")
}

fn change_gates(root: &Path) -> Vec<serde_json::Value> {
    crate::support::store_rows(root)
        .iter()
        .filter(|e| is_answerable_gate(e))
        .filter(|e| e.payload.get("kind").and_then(|k| k.as_str()) == Some("unknown_facet:change"))
        .map(|e| e.payload.clone())
        .collect()
}

/// **A `null` `change` at triage opens the `unknown_facet` gate, and its answer binds the facet**
/// (D21-43 N3; AC-079).
///
/// The first `run` of `chg-1` (at `triage`, no changeset) opens exactly one answerable
/// `gate_opened { reason: unknown_facet, axis: "change" }` and exits 0 at it. `answer … text` is
/// validated against `facet.change`. The next `run` opens no second gate — AC-079's *"counted once
/// per read facet"* — and every later stage's item carries `change = ["text"]`, folded from the
/// `gate_answered` row; the item record is never rewritten (M22).
#[test]
fn a_null_change_at_triage_opens_the_unknown_facet_gate_and_its_answer_binds_the_facet() {
    let repo = change_repo();
    let root = repo.path();
    let record = root.join(".redkiln").join("items").join("chg-1.toml");
    let record_before = std::fs::read(&record).expect("the record reads");

    let opened = crate::run::spawn_run(root, "chg-1", None);
    assert_eq!(
        opened.status.code(),
        Some(0),
        "a null `change` at triage opens the gate and the run exits 0 at it (golden constraint 2). \
         stderr:\n{}",
        String::from_utf8_lossy(&opened.stderr)
    );
    let gates = change_gates(root);
    assert_eq!(
        gates.len(),
        1,
        "exactly one answerable `unknown_facet:change` gate at triage: {gates:?} (M22: without it \
         the deriver's null is kept in silence)"
    );
    assert_eq!(gates[0]["reason"], "unknown_facet");
    assert_eq!(gates[0]["axis"], "change");

    // The default question's option is refused: the gate asks `facet.change`.
    let refused = spawn_answer(root, "chg-1", "approve", &BOTH_FLAGS);
    assert_eq!(
        refused.status.code(),
        Some(2),
        "`approve` is not a `facet.change` option. stderr:\n{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    let answered = spawn_answer(root, "chg-1", "text", &BOTH_FLAGS);
    assert_eq!(
        answered.status.code(),
        Some(0),
        "the operator supplies `change`. stderr:\n{}",
        String::from_utf8_lossy(&answered.stderr)
    );

    let _ = crate::run::spawn_run(root, "chg-1", None);
    assert_eq!(
        change_gates(root).len(),
        1,
        "an answered axis opens no second gate (AC-079: counted once per read facet)"
    );
    assert_eq!(
        change_of(&last_recorded_item(root)),
        Some(&set(&["text"])),
        "the answer binds `change` as a one-element set, folded from `gate_answered`"
    );
    assert_eq!(
        std::fs::read(&record).expect("the record reads"),
        record_before,
        "the answer is never written back to the item record"
    );
}

// ---- D21-74 ruling 2 (R2 = ADR-0173 decision 9 as amended, proposed; `s8d` build, package
// F3-D2174) --------------------------------------------------------------------------------------

/// **The composition root's run-branch reader reads R6's range through `rk-git`**: the paths
/// `HEAD..rk-run/<item>` changes, the same range `derive` classifies. No ref, or a ref with nothing
/// past `HEAD`, is the empty range (`None`), and another item's branch is never read.
#[test]
fn the_run_branch_reader_reads_the_items_changeset_paths_through_rk_git() {
    use rk_kernel::runtime::reference::RunBranch;

    let repo = change_repo();
    let root = repo.path();
    let found = repo_of(root);
    let reader =
        rk_cli::change_deriver::RunBranchDiff::new(found.exec.clone(), found.worktree.clone());

    assert_eq!(
        reader.changed_paths("chg-1"),
        Ok(None),
        "no rk-run/chg-1 ref: the empty range"
    );
    git(root, &["branch", "rk-run/chg-1"]);
    assert_eq!(
        reader.changed_paths("chg-1"),
        Ok(None),
        "a ref with nothing past HEAD: the empty range"
    );
    git(root, &["branch", "--quiet", "-D", "rk-run/chg-1"]);

    changeset_on_run_branch(root, "chg-1");
    let mut paths = reader
        .changed_paths("chg-1")
        .expect("the range reads")
        .expect("the run branch carries a changeset");
    paths.sort();
    assert_eq!(
        paths,
        vec![
            "Cargo.lock".to_owned(),
            "assets/logo.bin".to_owned(),
            "src/lib.rs".to_owned()
        ],
        "every path the run branch changes, the binary blob and the generated lockfile included"
    );
    assert_eq!(
        derive(&found.exec, &found.worktree, "chg-1").expect("the range reads"),
        Some(vec![
            "binary".to_owned(),
            "generated".to_owned(),
            "text".to_owned()
        ]),
        "CONTROL: the same range `derive` classifies"
    );
    assert_eq!(
        reader.changed_paths("chg-rec"),
        Ok(None),
        "another item's run branch is not this item's changeset"
    );
}
