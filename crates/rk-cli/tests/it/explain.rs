//! **`explain <item>`** — AC-430 to AC-434, over committed run-record fixtures.
//!
//! Every case here reads a directory in `replay::write_run`'s layout under
//! `tests/fixtures/explain/`, because §7d.7 fixes the recorded resolution as *"the persisted run
//! record, not the event log alone"* — no event carries AC-430's row families or AC-431's fields.
//!
//! **Three of these ids are not discharged whole and each says so in its own doc comment.**
//! AC-431 and AC-433 are PROVISIONAL at 7d (§7d.18): the binder's binding is never logged, so
//! skills, fillers and tier are on no recorded value, and no admissibility exclusion is recorded
//! either — `dag_assembled.skipped[].reason` carries only `forbidden_stage` and
//! `forbidden_stage_via_requires`. A test that asserted the unrecorded halves would be red for a
//! reason the record cannot answer, which is *wrong* rather than merely weak.

use crate::{code_lines, fixtures_dir, run_init, src_dir, temp_repo};

/// A temp repository whose real store has been seeded from `explain/<fixture>/events.jsonl`.
///
/// `rk_types::event::Event` derives no `Deserialize` — its `payload` is deliberately an untyped
/// `serde_json::Value` and the typed-payload record is OWED — so the rows are rebuilt through
/// `EventKind::from_wire`, which is the same closed-table lookup `read_run` refuses on.
fn repo_seeded_from(fixture: &str) -> tempfile::TempDir {
    let repo = temp_repo();
    assert_eq!(
        run_init(repo.path()).status.code(),
        Some(0),
        "the fixture repository initialises"
    );
    let discovered =
        rk_cli::repo::discover(Some(repo.path())).expect("the temp repository discovers");
    let mut store = rk_adapter_sqlite::SqliteStore::open(&discovered.exec, &discovered.worktree)
        .expect("the store opens over the fixture repository");

    let path = fixtures_dir()
        .join("explain")
        .join(fixture)
        .join("events.jsonl");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    let mut seeded = 0_usize;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let row: serde_json::Value =
            serde_json::from_str(line).unwrap_or_else(|e| panic!("{line}: {e}"));
        let wire = row["kind"]
            .as_str()
            .unwrap_or_else(|| panic!("{line}: no kind"));
        let kind = rk_types::event_kind::EventKind::from_wire(wire)
            .unwrap_or_else(|| panic!("`{wire}` is not a kind in the closed EventKind table"));
        rk_ports::event_store::EventStore::append(
            &mut store,
            rk_types::event::Event::new(kind, row["payload"].clone()),
        )
        .unwrap_or_else(|e| panic!("the seeded row appends: {e:?}"));
        seeded += 1;
    }
    assert!(
        seeded > 0,
        "{} seeded no rows, so every assertion over this repository would be over an EMPTY store \
         — which is a different case from the one this fixture exists to be",
        path.display()
    );
    repo
}

/// Copy `src` into `dst`, recursively. `README.md` is left behind — it documents the fixture and is
/// not part of `write_run`'s layout, and `read_run` refuses a stray file under `inputs/pack/`.
fn copy_tree(src: &std::path::Path, dst: &std::path::Path) {
    std::fs::create_dir_all(dst).expect("the destination directory is created");
    for entry in std::fs::read_dir(src).expect("the fixture directory reads") {
        let entry = entry.expect("the fixture entry reads");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else if from.file_name().is_some_and(|n| n != "README.md") {
            std::fs::copy(&from, &to).unwrap_or_else(|e| panic!("{}: {e}", from.display()));
        }
    }
}

/// A temp repository carrying `explain/<fixture>/` **both** as a run record under
/// `<common-dir>/redkiln/runs/<run_id>/` and as the journal its store holds.
///
/// Both halves are needed and they are different things: the store is what the item→run_id resolver
/// reads (only `activity_committed` carries both `item` and `run_id`), and the directory is what
/// `read_run` then parses. A repository with one and not the other exercises a path no real run
/// produces.
fn repo_with_record(fixture: &str) -> (tempfile::TempDir, String) {
    let repo = repo_seeded_from(fixture);
    let dir = fixtures_dir().join("explain").join(fixture);

    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("run.json")).expect("run.json"))
            .expect("run.json parses");
    let run_id = manifest["run_id"]
        .as_str()
        .expect("run.json names its run_id");

    let common = crate::common_dir(repo.path());
    copy_tree(&dir, &common.join("redkiln").join("runs").join(run_id));

    // The item is read off the record rather than passed in, so a fixture and its test can never
    // disagree about which item they are about.
    let text = std::fs::read_to_string(dir.join("events.jsonl")).expect("events.jsonl");
    let item = text
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|r| r["kind"] == "activity_committed")
        .find_map(|r| r["payload"]["item"].as_str().map(str::to_owned))
        .unwrap_or_else(|| panic!("{fixture} carries no activity_committed row naming its item"));
    (repo, item)
}

/// Render `explain/<fixture>/` through the real inner verb, and return its stdout.
fn rendered(fixture: &str) -> String {
    let (repo, item) = repo_with_record(fixture);
    let discovered =
        rk_cli::repo::discover(Some(repo.path())).expect("the fixture repository discovers");
    let opened = rk_adapter_sqlite::SqliteStore::open(&discovered.exec, &discovered.worktree)
        .expect("the store opens");
    let store: std::sync::Arc<std::sync::Mutex<dyn rk_ports::event_store::EventStore>> =
        std::sync::Arc::new(std::sync::Mutex::new(opened));
    let mut out: Vec<u8> = Vec::new();
    let exit = rk_cli::verbs::explain::run_with(
        &rk_cli::verbs::RealReconciler,
        &discovered.exec,
        &discovered.worktree,
        &discovered.common_dir,
        &store,
        &item,
        &mut out,
    )
    .expect("explain does not halt");
    assert_eq!(
        exit.code(),
        0,
        "explain refused `{item}` over {fixture}, so there is no rendering to read"
    );
    String::from_utf8(out).expect("the rendering is utf-8")
}

/// The per-node blocks of a rendering, keyed by node id.
fn node_blocks(text: &str) -> std::collections::BTreeMap<String, String> {
    let mut blocks = std::collections::BTreeMap::new();
    for chunk in text.split("\nnode: ").skip(1) {
        let id = chunk.lines().next().unwrap_or_default().trim().to_owned();
        blocks.insert(id, chunk.to_owned());
    }
    blocks
}

/// The value of `  <label>: ` inside one node block.
fn field<'a>(block: &'a str, label: &str) -> &'a str {
    block
        .lines()
        .find_map(|l| l.trim_start().strip_prefix(&format!("{label}: ")))
        .unwrap_or_else(|| panic!("no `{label}:` line in block:\n{block}"))
        .trim()
}

/// Spawn `redkiln -C <repo> explain <item>` with stdin closed (D-15).
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS, which is what carries its stderr — golden \
              constraint 4's 'a stub never replaced'; run::spawn_run_in_mode's form"
)]
fn spawn_explain(repo: &std::path::Path, item: &str) -> std::process::Output {
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(repo).arg("explain").arg(item);
    crate::support::output_bounded(&mut cmd)
}

/// **AC-432 — `explain` prints from the record ONLY, and an item without one is an ERROR.**
///
/// Register AC-432's THEN: *"the output is derived from the recorded resolution only; `explain`
/// performs no selection, and an item with no recorded resolution is an error rather than a
/// recomputation."* F36 is the ground — recomputing selection returned the same modal `R` on only
/// 5/20, 3/20, 2/20 and 2/20 draws, so *"a recomputation at explain time would print something that
/// never ran"*.
///
/// # 🛑 BOTH HALVES ARE IN THIS TEST, AND THE SECOND IS A SOURCE SCAN
///
/// §7d.7 asks for the exit-`5` refusal **and** *"a source scan that `crates/rk-cli/src/verbs/
/// explain.rs` names no `rk_kernel::select` path"*. The refusal alone is satisfiable by a verb that
/// recomputes everything and merely happens to refuse this one item; the scan is what makes
/// "performs no selection" structural rather than incidental.
///
/// # 🛑 THE SCAN IS SELF-CONTROLLED, AND OVER CODE LINES ONLY
///
/// §7d.14: *"it runs its scan over its control and asserts ≥ 1 hit **before** asserting zero hits
/// over `src/`"*. The control here is `src/verbs/run.rs`, which legitimately names
/// `rk_kernel::select::Item` at step (7) — the same control-against-a-real-file form §7d.7 uses for
/// the kernel's AC-436 lint, so no nineteenth `lint-controls/` file is owed. The comment filter is
/// itself controlled below: three guards this stage has already shipped were satisfied by the prose
/// explaining them, so a scan that counted a comment would make every zero here vacuous.
///
/// # 🛑 THE STORE IS NOT EMPTY, AND THAT IS THE POINT
///
/// `explain/no-resolution/` seeds two rows for the item — a `reconcile_started` and a
/// `stage_transition` — and neither is a `dag_assembled` or an `activity_committed`. An empty store
/// also yields exit `5`, so a test seeded from one could not tell *"no resolution was recorded"*
/// from *"the store holds nothing"*, and only the first is AC-432's THEN.
#[test]
fn ac432_explain_from_record_only_and_errors_without_resolution() {
    // ---- (1) THE REFUSAL, as a process, so its stderr is readable. -----------------------------
    let repo = repo_seeded_from("no-resolution");
    let out = spawn_explain(repo.path(), "explain-none");
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(5),
        "an item with no recorded resolution is `Exit::NoRecord` = 5, not a recomputation and not \
         the `3` every unbuilt verb returns. stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("no recorded resolution"),
        "the refusal does not NAME the missing resolution. Exit 5 alone would also be produced by \
         a verb that refused for some other reason, so the wording is what makes this AC-432's \
         outcome. stderr:\n{stderr}"
    );

    // ---- (2) THE UNREACHABILITY SCAN, self-controlled FIRST. -----------------------------------
    let needle = "rk_kernel::select";

    let control_path = src_dir().join("verbs").join("run.rs");
    let control_src = std::fs::read_to_string(&control_path).expect("the control file is readable");
    let control_hits = code_lines(&control_src)
        .into_iter()
        .filter(|(_, line)| line.contains(needle))
        .count();
    assert!(
        control_hits >= 1,
        "the scan found no `{needle}` on a code line of {}, which legitimately names it at step \
         (7) — the scan itself is broken and the zero below would be vacuous",
        control_path.display()
    );

    let explain_path = src_dir().join("verbs").join("explain.rs");
    let explain_src =
        std::fs::read_to_string(&explain_path).expect("crates/rk-cli/src/verbs/explain.rs exists");
    let hits: Vec<(usize, String)> = code_lines(&explain_src)
        .into_iter()
        .filter(|(_, line)| line.contains(needle))
        .collect();
    assert!(
        hits.is_empty(),
        "{} names `{needle}` on a code line: {hits:?}. AC-432 makes `explain` a read of the \
         record; a reachable selection path means it can print something that never ran (F36)",
        explain_path.display()
    );

    // ---- (3) THE COMMENT FILTER IS CONTROLLED, NOT ASSUMED. ------------------------------------
    //
    // `explain.rs`'s own header names `rk_kernel::select` several times in prose, so without this
    // the zero above is satisfied by a file whose every mention happens to be a comment — which is
    // exactly the shape of the three shipped guards this stage has already had to correct.
    let only_comments = format!("//! a header naming {needle}\n// and a line comment: {needle}\n");
    assert_eq!(
        code_lines(&only_comments)
            .into_iter()
            .filter(|(_, line)| line.contains(needle))
            .count(),
        0,
        "the comment filter counted a comment, so every zero above is satisfiable by prose"
    );
    let one_code_line = format!("{only_comments}let _ = {needle}::Item::new(\"i\", \"bug\");\n");
    assert_eq!(
        code_lines(&one_code_line)
            .into_iter()
            .filter(|(_, line)| line.contains(needle))
            .count(),
        1,
        "the comment filter dropped a CODE line, so the zero above is unfalsifiable"
    );
}

/// **AC-430 — for EVERY included activity, BOTH its admissibility row and the pack rows that put
/// it in the DAG.**
///
/// Register AC-430's THEN: *"the output prints, for every included activity, BOTH its admissibility
/// row from the activity (`provides`, `requires`, `applicable_if`, `method_rank`, `min_rigor`) AND
/// the requirement rows from the process pack (`baseline` and each matching `require` row) that put
/// it in the DAG."* *Verified by:* *"a snapshot test on a fixture item asserting both row families
/// for every included node, **and failing if either family is empty while the other is
/// populated**."*
///
/// # 🛑 THE SNAPSHOT IS NOT THE INSTRUMENT — THE BOTH-OR-NEITHER ASSERTION IS
///
/// A snapshot pins whatever is rendered, including a rendering that dropped one family: the first
/// `cargo insta accept` would bless the defect and every later run would agree with it. The
/// register names the discriminating property separately and so does this test, per node, before
/// the snapshot is taken. The snapshot's job is to catch a change nobody intended; the assertion's
/// job is to catch the one the register names.
///
/// `two-activities/` is built so the two nodes arrive by DIFFERENT routes — `ImplReport` through
/// `baseline`, `TestReport` through the one `require` row, which the recorded `has_repro = true`
/// fires. A fixture whose nodes all arrived the same way could not fail the assertion at all.
#[test]
fn ac430_explain_prints_both_row_families() {
    let text = rendered("two-activities");
    let blocks = node_blocks(&text);

    assert_eq!(
        blocks.len(),
        2,
        "`two-activities/` records exactly two nodes and the rendering shows {}. \
         Rendering:\n{text}",
        blocks.len()
    );

    for (node, block) in &blocks {
        // FAMILY A — the activity's own admissibility row, all five fields the register names.
        let family_a: Vec<(&str, &str)> = [
            "provides",
            "requires",
            "applicable_if",
            "method_rank",
            "min_rigor",
        ]
        .into_iter()
        .map(|label| (label, field(block, label)))
        .collect();
        let a_populated = family_a.iter().any(|(_, v)| *v != "-");

        // FAMILY B — the pack rows that put it there.
        let baseline = field(block, "baseline");
        let require = field(block, "require");
        let b_populated = baseline != "-" || require != "-";

        assert_eq!(
            a_populated, b_populated,
            "node `{node}` prints one row family and not the other — activity row {family_a:?}, \
             baseline {baseline:?}, require {require:?}. AC-430's Verified-by fails *\"if either \
             family is empty while the other is populated\"*, because a rendering that joined only \
             one side would still look complete.\nRendering:\n{text}"
        );
        assert!(
            a_populated && b_populated,
            "node `{node}` prints NEITHER row family, which satisfies the equality above \
             vacuously. Rendering:\n{text}"
        );
    }

    // The two nodes arrived by different routes, which is what makes the equality above capable of
    // failing: if both had come through `baseline`, a renderer that never joined the `require` rows
    // would still print a populated family B for both.
    let by_baseline = blocks
        .values()
        .filter(|b| field(b, "baseline") != "-")
        .count();
    let by_require = blocks
        .values()
        .filter(|b| field(b, "require") != "-")
        .count();
    assert_eq!(
        (by_baseline, by_require),
        (1, 1),
        "PREMISE: `two-activities/` must put one node in the DAG through `baseline` and the other \
         through a fired `require` row; got {by_baseline} by baseline and {by_require} by require. \
         Rendering:\n{text}"
    );

    // Both digests are PRINTED and LABELLED (ADR-0128 decision 4, RT-52). The inputs digest is
    // never asserted — a resumed run that re-derives its DAG over a tree a committed step changed
    // may legitimately differ there — but `output_digest`'s LABEL is load-bearing: s8's exit check
    // greps for that exact token.
    assert!(
        text.contains("inputs_digest: "),
        "the inputs digest is not printed, labelled. Rendering:\n{text}"
    );
    for (node, block) in &blocks {
        assert_ne!(
            field(block, "output_digest"),
            "not recorded",
            "node `{node}` has a committed result in the record and no `output_digest` was \
             derived from it. Rendering:\n{text}"
        );
    }

    insta::assert_snapshot!(text);
}

/// **AC-431 — the printed resolution names the CARD.** PROVISIONAL at 7d.
///
/// Register AC-431's THEN: *"the printed resolution names the card, the attached skills, the
/// inserted slot fillers and the effective tier for that activity."*
///
/// # 🛑 THIS TEST ASSERTS ONE OF THOSE FOUR, AND THE OTHER THREE ARE RAISED
///
/// **Skills, fillers and tier are on no recorded value.** `RecordedSpec` carries `card_digest`,
/// `tools`, `schema`, `timeout_secs` and `inputs` — the binder's binding is never logged
/// [measured 2026-09-18] — so there is nothing in the record for `explain` to read and nothing for
/// a test to compare against. Asserting them would be red for a reason the record cannot answer,
/// which is **wrong** rather than merely weak, and §7d.18 places this id in the PROVISIONAL
/// sub-list for exactly that reason. It is discharged whole only once the binder's binding is
/// recorded.
///
/// The renderer prints `skills: not recorded` / `fillers: not recorded` / `tier: not recorded` —
/// the operator's choice between that and printing nothing — so the artifact discloses its own gap.
/// **This test does not assert those lines**: pinning them would freeze a choice §7 U-4 leaves open,
/// and the moment the binding is recorded they must change.
///
/// # 🛑 THE CARD IS COMPARED WITH THE RECORD, NOT MERELY FOUND NON-EMPTY
///
/// *"red by dropping it"* is satisfied by any assertion that notices the line vanish; it is not
/// satisfied by a renderer that prints *some* digest. The recorded `inputs/specs.json` is read here
/// and the printed value must equal the `card_digest` it carries for that node, so a card taken
/// from the wrong node — or recomputed — is red too.
#[test]
fn ac431_explain_names_card_skills_fillers_tier() {
    let specs: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            fixtures_dir()
                .join("explain")
                .join("two-activities")
                .join("inputs")
                .join("specs.json"),
        )
        .expect("the recorded specs.json is readable"),
    )
    .expect("specs.json parses");

    let text = rendered("two-activities");
    let blocks = node_blocks(&text);
    assert!(
        !blocks.is_empty(),
        "no node blocks to read. Rendering:\n{text}"
    );

    for (node, block) in &blocks {
        let recorded = specs[node]["card_digest"]
            .as_str()
            .unwrap_or_else(|| panic!("the record carries no card_digest for `{node}`"));
        assert_eq!(
            field(block, "card"),
            recorded,
            "node `{node}` prints a card that is not the one its `RecordedSpec` carries. AC-431's \
             card is the RECORDED `card_digest`; a renderer that printed any other digest, or the \
             digest of another node, would still look like it had named a card.\nRendering:\n{text}"
        );
    }
}

/// The `excluded: <id>` blocks of a rendering, keyed by activity id.
fn excluded_blocks(text: &str) -> std::collections::BTreeMap<String, String> {
    let mut blocks = std::collections::BTreeMap::new();
    for chunk in text.split("\nexcluded: ").skip(1) {
        let id = chunk.lines().next().unwrap_or_default().trim().to_owned();
        blocks.insert(id, chunk.to_owned());
    }
    blocks
}

/// **AC-433 — the output NAMES the excluding clause, and never says `not selected`.**
/// PROVISIONAL at 7d.
///
/// Register AC-433's THEN: *"the output names the specific clause that excluded it — the false
/// `applicable_if` sub-predicate, the unmet `min_rigor`, or the absent `require` row — rather than
/// reporting 'not selected'."*
///
/// # 🛑 WHY THIS IS PROVISIONAL, STATED HERE RATHER THAN INFERRED FROM THE ASSERTIONS
///
/// **The excluding clause is not on the record.** `dag_assembled.skipped[].reason` is only ever
/// `forbidden_stage` or `forbidden_stage_via_requires` — `SkipReason` has exactly two stage
/// variants [measured 2026-09-18] — so none of AC-433's three clause kinds is recorded anywhere.
/// What `explain` prints is a **re-reading of recorded inputs**: the recorded `applicable_if`
/// evaluated through `rk_kernel::predicate` over the recorded `context.json`, the recorded
/// `min_rigor` against the recorded pack's stage rigor, and the recorded `require` rows. That is a
/// read of two logged inputs and not a selection — AC-432's unreachability scan still holds — but
/// it is not the register's *"from the record only"* in full, and the id is discharged whole only
/// when a kernel-recorded `excluded` row lands.
///
/// # 🛑 `predicate::on_false` IS NOT THE SUB-PREDICATE NAMER
///
/// §7d.7 calls the naming *"its `on_false` naming"*. `crates/rk-kernel/src/predicate/on_false.rs`
/// is ADR-0103 clause 6's closed form set for a step's `on_false` FIELD —
/// `OnFalse{Continue, Fail, LoopBack, Deviate}` and `classify_on_false` — and has nothing to do
/// with naming a false conjunct. The prose is wrong and is RAISED; the renderer walks the parsed
/// `Expr` with `predicate::{parse, evaluate}` instead.
///
/// # 🛑 BOTH HALVES
///
/// The bare string is asserted ABSENT and each clause asserted PRESENT. Absence alone is satisfied
/// by a renderer that prints nothing about exclusions at all.
#[test]
fn ac433_explain_names_excluding_clause_never_not_selected() {
    let text = rendered("three-exclusions");

    // ---- (1) THE BARE STRING IS ABSENT. -------------------------------------------------------
    assert!(
        !text.contains("not selected"),
        "the rendering reports the literal `not selected`, which is the exact phrasing AC-433 \
         exists to replace.\nRendering:\n{text}"
    );

    // ---- (2) AND EACH CLAUSE KIND IS NAMED. ---------------------------------------------------
    let blocks = excluded_blocks(&text);
    for id in ["excluded-predicate", "excluded-rigor", "excluded-norow"] {
        assert!(
            blocks.contains_key(id),
            "`{id}` is in the recorded catalog and not in `dag_assembled.nodes`, and the \
             rendering says nothing about it. Absence of `not selected` is satisfied by a \
             rendering that is simply silent.\nRendering:\n{text}"
        );
    }

    let predicate_clause = field(&blocks["excluded-predicate"], "clause");
    assert!(
        predicate_clause.contains("market_facing"),
        "the false `applicable_if` sub-predicate is not NAMED: the recorded predicate is \
         `kind == bug and market_facing` and only its right conjunct is false, so naming the whole \
         string would not be naming the clause. clause: {predicate_clause:?}\nRendering:\n{text}"
    );
    assert!(
        !predicate_clause.contains("kind == bug and market_facing"),
        "the whole predicate was echoed rather than the false conjunct identified. AC-433 asks for \
         *the specific clause*. clause: {predicate_clause:?}"
    );

    let rigor_clause = field(&blocks["excluded-rigor"], "clause");
    assert!(
        rigor_clause.contains("min_rigor") && rigor_clause.contains("mandatory"),
        "the unmet `min_rigor` is not named with the value that was not met. clause: \
         {rigor_clause:?}\nRendering:\n{text}"
    );

    let norow_clause = field(&blocks["excluded-norow"], "clause");
    assert!(
        norow_clause.contains("IncidentLog"),
        "the absent `require` row is not named with the artifact kind nothing asked for. clause: \
         {norow_clause:?}\nRendering:\n{text}"
    );

    insta::assert_snapshot!(text);
}

/// **AC-434 — every `unknown` facet renders as `unknown` WITH its position-derived treatment, and
/// never as a band value.**
///
/// Register AC-434's THEN: *"each `unknown` facet is rendered as `unknown` with its
/// position-derived treatment stated, and is never rendered as a measured band value."* Display
/// only — the selection engine's block is untouched, and this is not a determinism claim.
///
/// # 🛑 BOTH HALVES, AND THE POSITIVE ONE IS WHAT MAKES IT NON-VACUOUS
///
/// §7d.7 is explicit: *"the positive half — a renderer that prints nothing for an `unknown` facet
/// passes the band check alone"*. So each of the four axes must be found rendering the literal
/// `unknown` **and** stating its treatment, before anything is asserted about bands.
///
/// # 🛑 `unknown` IS THE ORDINAL DOMAIN MEMBER HERE, NOT JSON `null`
///
/// `FacetValue` is `{Null, Bool, Str, Int, Ordinal{domain,index}, Set}` and **`Null` carries no
/// position**, so against a `null` the register's *"position-derived treatment"* would have nothing
/// to state — and a null read mints `gate_opened { reason: unknown_facet, axis }` at selection step
/// 2, so the axis would open an abstention gate rather than appear in an admissibility row at all.
/// The fixture records `{"domain": ["low","med","high","unknown"], "index": 3}` per axis, which is
/// the live encoding the committed kernel log already uses, and the domain is recorded beside the
/// index precisely because the index is what "position-derived" means.
#[test]
fn ac434_unknown_facet_rendered_unknown() {
    const AXES: [&str; 4] = ["blast_radius", "criticality", "novelty", "size"];
    const BANDS: [&str; 3] = ["low", "med", "high"];

    let text = rendered("unknown-four-axes");

    for axis in AXES {
        // THE POSITIVE HALF: it is rendered, it renders as `unknown`, and a treatment is stated.
        let line = text
            .lines()
            .find(|l| {
                l.trim_start()
                    .starts_with(&format!("unknown_facet: {axis} ="))
            })
            .unwrap_or_else(|| {
                panic!(
                    "`{axis}` is `unknown` in the recorded context and an admissibility row read \
                     it, and the rendering says nothing about it. A renderer that prints nothing \
                     for an unknown facet passes the band check below on its own.\n\
                     Rendering:\n{text}"
                )
            });
        assert!(
            line.contains("= unknown"),
            "`{axis}` does not render as the literal `unknown`. line: {line:?}"
        );
        assert!(
            line.contains("position"),
            "`{axis}` renders `unknown` without stating its position-derived treatment, which is \
             the half of AC-434's THEN that says what the reader should do with it. line: {line:?}"
        );

        // THE OMISSION HALF: it is never rendered AS a measured band value.
        for band in BANDS {
            assert!(
                !text.contains(&format!("unknown_facet: {axis} = {band}")),
                "`{axis}` renders as the band value `{band}`. `unknown` is an abstention and a \
                 band is a measurement; printing one as the other is the substitution AC-434 \
                 exists to forbid.\nRendering:\n{text}"
            );
        }
    }

    insta::assert_snapshot!(text);
}

/// **AC-432's second half — `explain` over a REPLAYED log is byte-identical to `explain` at run
/// time.**
///
/// Register AC-432's Verified-by: *"a replay assertion — `explain` over a replayed event log is
/// byte-identical to `explain` at run time — plus a kernel test asserting the selection engine is
/// not reachable from the `explain` code path."* The second half is
/// `ac432_explain_from_record_only_and_errors_without_resolution`'s scan; this is the first.
///
/// # 🛑 SIDE A IS THE IN-PROCESS VALUE, NOT A RE-READ OF THE DIRECTORY
///
/// §7d.14 names the weaker red in terms: *"a weaker red renders the same record twice and calls it
/// a replay"*. So side A is the `RecordedRun` **`replay::resume` returned**, rendered before and
/// independently of `write_run`; side B is what comes back after the record has been through
/// `replay()` and `read_run()`. Rendering the directory twice would compare a file with itself and
/// pass against any renderer whatever.
///
/// `replay()` returns a `ReplayReport`, **not** a `RecordedRun` — re-reading the directory with
/// `read_run` after the replay has verified it is the only public path to the value, and that is
/// why side B is spelled in two steps.
///
/// # 🛑 THE REPORT'S THREE COUNTERS ARE ASSERTED BESIDE THE BYTES
///
/// A replay that silently dispatched, or wrote through `rk_git`, or diverged, could still produce
/// identical bytes — the render would just be rendering a record that had been rebuilt by doing the
/// work again. `divergences`, `rk_git_writes` and `harness_dispatches` are all read, so "it
/// replayed" is a measurement rather than an assumption.
///
/// # 🛑 THE DECLARED CLOCK/ENV MUTANT IS TIME-DEPENDENT, AND THIS TEST SAYS SO
///
/// §7d.7 declares the mutant as *"a render that reads the clock or the process environment"*. Its
/// honest limit: a `Clock::now()` inserted into the render differs between the two renders **only
/// if the wall clock ticks between them**, and an env read such as `%USERNAME%` is *identical* in
/// one process, so it would not move these bytes at all. What actually holds that property is
/// structural — the `std::env::var(` and `SystemTime::now()` absence bans over `crates/rk-cli/src`,
/// each with its own `lint-controls/` positive control — and not this comparison. Recorded here
/// rather than left for a reader to discover by running the mutant and finding it green.
///
/// # 🛑 THIS RECORD CARRIES NO `Config`, AND A RECORD `run` WRITES CANNOT BE REPLAYED AT ALL
///
/// Measured while authoring this test. `RunInputs::to_json` — which is exactly what `run_id`
/// hashes — inserts a `config` key when one was threaded in (`replay.rs`, Slice 2's ADR-0083 /
/// D18-14 edit), and **`write_run` persists no config while `read_run` restores none**. So a
/// config-carrying record re-derives a different `run_id` from its own written inputs, and
/// `replay()`, which re-derives and compares, refuses it:
///
/// ```text
/// Divergent(HiddenInput { … the recorded run_id is implement-61b5dd057fefc7fe and the logged
/// input set hashes to implement-ff18038f5d185916 — some input was edited after the run was
/// recorded … })
/// ```
///
/// `run` threads a config into every record it writes, so **no record the shipped binary produces
/// is replayable**, and AC-042's *"re-derives the DAG from the logged inputs alone"* does not hold
/// for them. The kernel's own replay suite does not catch this because its committed golden log
/// pre-dates the config edit and carries none. This test therefore drives a **config-less** input
/// set, which is the narrowest thing that lets the byte-identity property be exercised at all.
/// **RAISED** — the fix is `write_run`/`read_run` persisting the config, which is a kernel edit
/// and re-opens 7b, so it is not taken here.
#[test]
fn ac432_explain_over_replayed_log_is_byte_identical() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
    let (_with_config, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
    // 🛑 WITHOUT THE CONFIG, AND THAT IS A RAISED KERNEL DEFECT RATHER THAN A TEST CONVENIENCE.
    // `RunInputs::to_json` hashes a `config` key into `run_id` while `write_run` persists no config
    // and `read_run` restores none, so a config-carrying record re-derives a different `run_id` and
    // `replay()` refuses it outright. Every record `run` writes carries a config, so no record this
    // binary produces can be replayed at all. See `support::inputs_over_pack_min_without_config`.
    let inputs = crate::support::inputs_over_pack_min_without_config(&resolved, &tree_hash);
    let inputs = inputs.with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::supplied(
            "emit-report@1",
            crate::support::any_activity_spec(),
        ),
    );

    // ---- THE RUN, IN PROCESS. Its dispatch must COMMIT, or there is no `activity_committed` row
    //      and `output_digest` would render `not recorded` on both sides — identical, and vacuous.
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = std::sync::Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    let ports = crate::support::ports(
        std::sync::Arc::clone(&harness) as std::sync::Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let mut supply = crate::support::no_spec();
    let recorded = rk_kernel::replay::resume(&inputs, ports, &[], &mut supply)
        .expect("the recorded run is produced in process");
    assert_eq!(
        harness.dispatches(),
        1,
        "the in-process run reached the harness {} times, not once — with no committed dispatch \
         there is no recorded result for either side to render",
        harness.dispatches()
    );
    assert!(
        recorded
            .events
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted),
        "the in-process run committed no activity, so both renderings would agree on an absence"
    );

    // SIDE A — the value `resume` returned, rendered directly.
    let side_a = rk_cli::verbs::explain::render(&recorded).expect("the in-process record renders");

    // ---- THE RECORD ON DISK, AND THE REPLAY OVER IT. -------------------------------------------
    let runs = crate::common_dir(repo.path()).join("redkiln").join("runs");
    let dir = rk_kernel::replay::write_run(&runs, &recorded).expect("the run record is written");

    let replay_harness = crate::support::ScriptedHarness::scripted(Vec::new());
    let mut replay_store = rk_ports::fakes::VecStore::default();
    let mut journal = rk_git::gitops::VecJournal::default();
    let replay_repo =
        rk_cli::repo::discover(Some(repo.path())).expect("the fixture repository discovers");
    let mut git =
        rk_git::gitops::GitOps::new(replay_repo.exec, &mut journal, rk_git::gitops::Mode::Replay);
    let mut replay_ports = rk_kernel::replay::ReplayPorts {
        harness: &replay_harness,
        dispatches: replay_harness.counter(),
        store: &mut replay_store,
        git: &mut git,
    };
    let report = rk_kernel::replay::replay(&dir, &mut replay_ports).expect("the record replays");

    assert_eq!(
        report.divergences, 0,
        "the replay diverged from the record, so side B is not a replay of side A"
    );
    assert_eq!(
        report.rk_git_writes, 0,
        "the replay wrote through `rk_git`; a replay performs no git write"
    );
    assert_eq!(
        report.harness_dispatches, 0,
        "the replay reached the harness; a replay re-serves recorded results and dispatches nothing"
    );

    // SIDE B — the record after the replay has verified it, re-read and rendered.
    let reread = rk_kernel::replay::read_run(&dir).expect("the written record reads back");
    let side_b = rk_cli::verbs::explain::render(&reread).expect("the replayed record renders");

    assert_eq!(
        side_a.as_bytes(),
        side_b.as_bytes(),
        "`explain` at run time and `explain` over the replayed log differ.\n\
         run time:\n{side_a}\n---\nreplayed:\n{side_b}"
    );
}
