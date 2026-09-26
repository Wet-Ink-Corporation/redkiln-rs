//! `explain <item>` — **the recorded resolution, printed from the record and nothing else.**
//!
//! §7d.7: *"It prints **from the recorded resolution only**, and the recorded resolution is the
//! persisted run record, not the event log alone"* — no event carries AC-430's rows or AC-431's
//! fields. `run` persists the record through `replay::write_run`, and this verb reads it back
//! through `replay::read_run`, rendered by a function that takes `&RecordedRun` and nothing else.
//!
//! # 🛑 THE SELECTION ENGINE IS NOT REACHABLE FROM HERE, AND THAT IS ASSERTED
//!
//! AC-432's whole content is that `explain` performs **no selection**: F36 measured that
//! recomputing it returns the same modal `R` on only 5/20, 3/20, 2/20 and 2/20 draws, so *"a
//! recomputation at explain time would print something that never ran"*. This module therefore
//! names no `rk_kernel::select` path at any depth, and
//! `ac432_explain_from_record_only_and_errors_without_resolution` scans for it — self-controlled
//! against `verbs/run.rs`, which legitimately names `rk_kernel::select::Item`.
//!
//! Evaluating a **recorded** predicate through `rk_kernel::predicate` for AC-433's and AC-434's
//! display is expressly permitted and is not selection: it is a read of two logged inputs.

use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_ports::event_store::EventStore;

use crate::cli::{Cli, ExplainArgs};
use crate::exit::{Exit, Halt};
use crate::verbs::{EntryReconciler, Startup};
use crate::{repo, store};

/// The outer verb: discover the repository, open the real store, delegate.
///
/// # Errors
/// [`Halt::PoisonedLock`] from [`crate::store::acquire`]. Every repository and store refusal is
/// exit `3` and is returned as a value, not a halt.
pub fn run(cli: &Cli, args: &ExplainArgs, startup: Startup<'_>) -> Result<Exit, Halt> {
    run_to(cli, args, startup, &mut std::io::stdout())
}

/// *(dated 2026-09-26, s8c build; D21-64 ruling 3.)* [`run`] with the writer handed in; an
/// in-process caller passes its own, so nothing it renders reaches libtest's stdout.
///
/// # Errors
/// As [`run`].
pub fn run_to(
    cli: &Cli,
    args: &ExplainArgs,
    _startup: Startup<'_>,
    out: &mut dyn Write,
) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    // AC-413's identity check lives inside `open`: a store belonging to another repository is
    // refused here, before a single row is read.
    let opened = match rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
    run_with(
        &crate::verbs::RealReconciler,
        &repo.exec,
        &repo.worktree,
        &repo.common_dir,
        &store,
        &args.item,
        out,
    )
}

/// The inner verb — the in-process twin every Slice-3 test drives.
///
/// # 🛑 IT TAKES `reconciler`, `exec` AND `worktree` IT DOES NOT USE, AND THAT IS THE POINT
///
/// ADR-0105 clause 8: *"`status` and `explain` are read-only and do not reconcile"*. Those three
/// arguments are exactly what `rk_git::reconciled::reconcile_at_entry` needs, and this function
/// takes **all three** and calls nothing — a verb that *could not* reconcile would make
/// `ac402_status_and_explain_do_not_reconcile`'s recorder zero true of every possible body, and
/// therefore evidence of nothing. `status::run_with` carries the same three for the same reason.
///
/// The type is the second guard and it is `rk_git`'s: `Entry` is exactly `{ Run, Answer, Doctor }`,
/// so there is no `Entry::Explain` to pass.
///
/// # Errors
/// [`Halt::PoisonedLock`] naming this verb.
#[expect(
    clippy::too_many_arguments,
    reason = "three arguments are the unused reconcile seam AC-402 needs present to be falsifiable, \
              and `common_dir` is where `write_run` put the record this verb reads"
)]
pub fn run_with(
    reconciler: &dyn EntryReconciler,
    exec: &GitExec,
    worktree: &WorktreePath,
    common_dir: &Path,
    store: &Arc<Mutex<dyn EventStore>>,
    item: &str,
    out: &mut dyn Write,
) -> Result<Exit, Halt> {
    let (_, _, _) = (reconciler, exec, worktree);

    let found = match recorded_run_id(store, item)? {
        Ok(found) => found,
        Err(exit) => return Ok(exit),
    };
    let Some(run_id) = found else {
        eprintln!("redkiln: no recorded resolution for `{item}`; explain does not recompute");
        return Ok(Exit::NoRecord);
    };

    let dir = common_dir.join("redkiln").join("runs").join(&run_id);
    let recorded = match rk_kernel::replay::read_run(&dir) {
        Ok(run) => run,
        Err(e) => {
            eprintln!("redkiln: the run record `{run_id}` could not be read: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    let rendered = match render(&recorded) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("redkiln: the recorded inputs of `{run_id}` did not rebuild: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    if let Err(e) = out.write_all(rendered.as_bytes()) {
        eprintln!("redkiln: the resolution could not be written: {e}");
        return Ok(Exit::ConfigError);
    }
    Ok(Exit::Success)
}

/// The item's most recent run id, off the store.
///
/// # 🛑 ONLY `activity_committed` CARRIES BOTH `item` AND `run_id`
///
/// §7d.7 says the run id comes from *"the store's last `activity_committed` / `dag_assembled` row's
/// `run_id`"*, and the second half of that is not implementable: `Dag::to_json` emits
/// `{inputs_digest, nodes, edges, rows_fired, skipped, gated, appetite, cost}` — **no `run_id` and
/// no `item`** (GAP 10, measured; the same bullet enumerates those members three lines earlier and
/// contradicts itself). So the resolver reads `activity_committed` and nothing else, and an item
/// with none has no recorded resolution and exits `5`. The `dag_assembled … run_id` phrase is
/// RAISED, not edited.
///
/// The selector is inert — `SqliteStore::query` is `SELECT kind, payload FROM t2_event ORDER BY id`
/// with no `WHERE` — so the `item` match is made here, on the payload, rather than trusted to the
/// query.
///
/// # 🛑 A STORE THAT WILL NOT READ IS NOT "NO RESOLUTION"
///
/// The three outcomes are kept apart deliberately: the outer `Result` is the halt a poisoned lock
/// raises, the inner `Err(Exit)` is a store that refused — exit `3` — and `Ok(None)` is the one
/// AC-432 is about, an item for which nothing was ever recorded. Folding the middle one into
/// `Ok(None)` would report a broken store as `5`, which reads as a statement about the ITEM.
///
/// # Errors
/// [`Halt::PoisonedLock`] naming this verb.
fn recorded_run_id(
    store: &Arc<Mutex<dyn EventStore>>,
    item: &str,
) -> Result<Result<Option<String>, Exit>, Halt> {
    let guard = store::acquire(store, "explain")?;
    let read = guard.query(rk_kernel::runtime::journal_query(item));
    drop(guard);
    let rows = match read {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("redkiln: the store could not be read: {e:?}");
            return Ok(Err(Exit::ConfigError));
        }
    };
    Ok(Ok(rows
        .iter()
        .rev()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
        .filter(|e| e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item))
        .find_map(|e| {
            e.payload
                .get("run_id")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })))
}

/// The whole rendering, from a `&RecordedRun` and **nothing else**.
///
/// §7d.7 fixes this shape: *"rendered by a function in `crates/rk-cli/src/verbs/explain.rs` that
/// takes `&RecordedRun` and nothing else"*. It is what makes
/// `ac432_explain_over_replayed_log_is_byte_identical` possible at all — a render reading anything
/// outside the record could differ between two renders of the same record. It reads **no clock, no
/// `std::env::var` and no `%USERNAME%`**, and the absence bans with their `lint-controls/` positive
/// controls are what actually hold that: the byte comparison alone is time-dependent.
///
/// The `Result` is about the RECORD, not about the environment: `RunInputs::build()` re-reads the
/// logged pack, catalog, artifact registry and facet registry, and a fixture that does not rebuild
/// is a broken fixture rather than a rendering choice.
///
/// # 🛑 THE STAGE COMES OFF THE `run_id`, AND NO KERNEL EDIT IS OWED FOR IT
///
/// `RunInputs.stage` is private with no accessor, which two readers called blocking and recommended
/// a fifth kernel edit for. It is not: `RecordedRun::run_id()` is `"{stage}-{16 hex}"`, so the
/// recorded stage is the run id minus its trailing 17 bytes. `Pack::stage`, `Pack::stage_specs` and
/// `Pack::rigor` are all public, so nothing else is needed either.
///
/// # Errors
/// [`rk_kernel::replay::ReplayError`] when the logged inputs will not rebuild.
pub fn render(
    run: &rk_kernel::replay::RecordedRun,
) -> Result<String, rk_kernel::replay::ReplayError> {
    // The four registries, rebuilt FROM THE LOGGED INPUTS ALONE. `build()` is the sanctioned route
    // and is why no `rk_kernel::select` path is named anywhere in this file: the `Item` arrives as a
    // value, and its type is never spelled.
    let (pack, catalog, _artifacts, registry, item) = run.inputs.build()?;
    let _ = &registry;
    // Taken ONCE, off the rebuilt item. Its type is never spelled: it arrives as a value from
    // `build()`, which is what keeps this file free of the path AC-432's scan forbids.
    let ctx = item.context();
    // The recorded context vector as the log holds it. `recorded_json` is public precisely
    // because AC-042 needs the logged inputs to be sufficient to rebuild the item, and reading
    // it here keeps this file free of the `rk_kernel::select` path AC-432's scan forbids.
    let (context_json, _artifacts_json) = item.recorded_json();

    let run_id = run.run_id();
    let stage = recorded_stage(run_id);
    let dag = row_payload(run, rk_types::event_kind::EventKind::DagAssembled);

    let mut out = String::new();
    out.push_str(&format!("run: {run_id}\n"));
    out.push_str(&format!("stage: {stage}\n"));
    out.push_str(&format!("item: {}\n", item.id()));
    // 🛑 THE INPUTS DIGEST IS PRINTED AND NEVER ASSERTED. ADR-0128 decision 4 makes `output_digest`
    // the quantity s8's exit check greps; a resumed run that re-derives its DAG over a tree a
    // committed step changed may legitimately differ here, so pinning it would pin a legitimate
    // difference.
    out.push_str(&format!(
        "inputs_digest: {}\n",
        dag.as_ref()
            .and_then(|p| p.get("inputs_digest"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("not recorded")
    ));

    let spec_of: std::collections::BTreeMap<&str, &rk_kernel::replay::RecordedSpec> = run
        .inputs
        .specs()
        .iter()
        .map(|(node, spec)| (node.as_str(), spec))
        .collect();

    let nodes = dag
        .as_ref()
        .and_then(|p| p.get("nodes"))
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();

    for node in &nodes {
        let node_id = node
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("?");
        let activity_id = node
            .get("activity_id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("?");
        let instance_kind = node
            .get("instance")
            .and_then(|i| i.get("kind"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("?");

        out.push_str(&format!("\nnode: {node_id}\n"));
        out.push_str(&format!("  activity: {activity_id}\n"));

        // ---- FAMILY A: the admissibility row FROM THE ACTIVITY (AC-430's first half). ----------
        if let Some(a) = catalog.activities().iter().find(|a| a.id == activity_id) {
            out.push_str(&format!("  provides: {}\n", join(&a.provides)));
            out.push_str(&format!(
                "  requires: {}\n",
                join(
                    &a.requires
                        .iter()
                        .map(|r| if r.optional {
                            format!("{}?", r.kind)
                        } else {
                            r.kind.clone()
                        })
                        .collect::<Vec<_>>()
                )
            ));
            out.push_str(&format!(
                "  applicable_if: {}\n",
                a.applicable_if.as_deref().unwrap_or("-")
            ));
            out.push_str(&format!("  method_rank: {}\n", a.method_rank));
            out.push_str(&format!("  min_rigor: {}\n", a.min_rigor));
        } else {
            out.push_str("  provides: -\n  requires: -\n  applicable_if: -\n  method_rank: -\n  min_rigor: -\n");
        }

        // ---- FAMILY B: the requirement rows FROM THE PACK that put it in the DAG. --------------
        let spec = pack.stage(&stage);
        let baseline = spec
            .filter(|s| s.baseline.iter().any(|k| k == instance_kind))
            .map(|_| instance_kind.to_owned());
        out.push_str(&format!(
            "  baseline: {}\n",
            baseline.as_deref().unwrap_or("-")
        ));
        let rows: Vec<String> = spec
            .map(|s| {
                s.require
                    .iter()
                    .filter(|r| r.artifacts.iter().any(|k| k == instance_kind))
                    .filter(|r| predicate_is_true(&r.when, &ctx))
                    .map(|r| format!("{} -> {}", r.when, join(&r.artifacts)))
                    .collect()
            })
            .unwrap_or_default();
        out.push_str(&format!("  require: {}\n", join(&rows)));

        // ---- AC-434: the `unknown` axes THIS node's admissibility row CONSUMED. -----------
        //
        // The GIVEN is *"the admissibility rows that consumed them"*, and `Outcome.read_facets`
        // is exactly that — every context-bound path the evaluation actually read, once each,
        // in byte order. An axis that no included predicate reads is one this rendering could
        // say nothing honest about, which is why the fixture makes one node read all four.
        for (axis, domain, index) in unknown_axes_read(
            catalog
                .activities()
                .iter()
                .find(|a| a.id == activity_id)
                .and_then(|a| a.applicable_if.as_deref()),
            &ctx,
            &context_json,
        ) {
            out.push_str(&format!(
                "  unknown_facet: {axis} = unknown (position {} of {} in the recorded domain \
                 [{}]; an ordered comparison ranks it at the TOP of the band, so a row that \
                 passed on it was not satisfied by evidence)\n",
                index + 1,
                domain.len(),
                domain.join(", ")
            ));
        }

        // ---- AC-431: THE CARD, AND THE THREE FIELDS NO RECORDED VALUE CARRIES. ----------------
        //
        // `RecordedSpec` carries `card_digest`, `tools`, `schema`, `timeout_secs` and `inputs` —
        // the binder's binding is never logged. Printing `skills: []` would be FABRICATION: an
        // empty list the record does not carry reads as "none were attached". The labelled
        // `not recorded` line discloses the gap instead, and the test asserts only the card.
        out.push_str(&format!(
            "  card: {}\n",
            spec_of.get(node_id).map_or_else(
                || "not recorded".to_owned(),
                |s| s.spec().card_digest.as_str().to_owned()
            )
        ));
        out.push_str("  skills: not recorded\n");
        out.push_str("  fillers: not recorded\n");
        out.push_str("  tier: not recorded\n");

        // ---- AC-430's second digest. DERIVED, and disclosed as derived. -----------------------
        //
        // `output_digest` exists nowhere in the workspace (GAP 9: zero hits), so it is computed
        // here over the recorded `activity_committed.result` with `rk_store::canon::canonical_digest`
        // — a pure function of recorded bytes, which is what keeps the byte-identity test honest.
        // The alternative is a kernel-recorded member on `activity_committed`, which would move
        // every recorded journal's bytes and re-open 7b. The LABEL SPELLING is load-bearing: s8's
        // exit check greps for `output_digest`.
        out.push_str(&format!(
            "  output_digest: {}\n",
            committed_result(run, activity_id)
                .as_ref()
                .and_then(|v| rk_store::canon::canonical_digest(v).ok())
                .map_or_else(|| "not recorded".to_owned(), |d| d.as_str().to_owned())
        ));
    }

    // ---- AC-433: WHAT IS IN THE RECORDED CATALOG AND NOT IN THE RECORDED DAG. -----------------
    //
    // 🛑 THE EXCLUDING CLAUSE IS NOT ON THE RECORD, AND THIS IS DISCLOSED AS A RE-READING.
    // `dag_assembled.skipped[].reason` is only ever `forbidden_stage` or
    // `forbidden_stage_via_requires` — `SkipReason` has exactly two stage variants — so none of
    // AC-433's three clause kinds was ever recorded. Each is recovered below by re-reading two
    // LOGGED inputs, the recorded catalog beside the recorded `context.json` and pack. That is why
    // the id is PROVISIONAL, and why the header line says so in the output itself.
    let in_dag: std::collections::BTreeSet<&str> = nodes
        .iter()
        .filter_map(|n| n.get("activity_id").and_then(serde_json::Value::as_str))
        .collect();
    let stage_spec = pack.stage(&stage);
    let asked: std::collections::BTreeSet<String> =
        stage_spec.map_or_else(std::collections::BTreeSet::new, |s| {
            s.baseline
                .iter()
                .cloned()
                .chain(
                    s.require
                        .iter()
                        .filter(|r| predicate_is_true(&r.when, &ctx))
                        .flat_map(|r| r.artifacts.iter().cloned()),
                )
                .collect()
        });
    let stage_rigor = pack.rigor(&stage);
    for a in catalog.activities() {
        if a.stage != stage || in_dag.contains(a.id.as_str()) {
            continue;
        }
        out.push_str(&format!("\nexcluded: {}\n", a.id));
        out.push_str(&format!(
            "  clause: {}\n",
            excluding_clause(a, &ctx, &asked, stage_rigor)
        ));
    }

    Ok(out)
}

/// Which recorded clause kept `a` out of the recorded DAG — never the bare string `not selected`.
///
/// The three kinds are tried in the order AC-433's THEN names them. Each is a re-reading of a
/// logged input, and the wording says which input, so the output cannot be mistaken for a recorded
/// exclusion.
fn excluding_clause(
    a: &rk_kernel::catalog::Activity,
    ctx: &rk_kernel::predicate::Context,
    asked: &std::collections::BTreeSet<String>,
    stage_rigor: Option<rk_kernel::rigor::Rigor>,
) -> String {
    // (1) THE FALSE `applicable_if` SUB-PREDICATE.
    if let Some(src) = a.applicable_if.as_deref()
        && !predicate_is_true(src, ctx)
        && let Some(sub) = false_conjunct(src, ctx)
    {
        return format!(
            "the `applicable_if` sub-predicate `{sub}` is false over the recorded context"
        );
    }

    // (2) THE UNMET `min_rigor`.
    if let Some(min) = rk_kernel::rigor::Rigor::from_token(&a.min_rigor)
        && let Some(stage) = stage_rigor
        && min > stage
    {
        return format!(
            "its `min_rigor` is `{}` and the recorded pack sets this stage to `{}`",
            a.min_rigor,
            stage.as_str()
        );
    }

    // (3) THE ABSENT `require` ROW. `asked` is every kind the recorded pack's `baseline` names
    // plus every kind a FIRED `require` row names — computed by the caller, where the pack's
    // type is inferred rather than spelled.
    let _ = ctx;
    if let Some(kind) = a.provides.iter().find(|k| !asked.contains(*k)) {
        return format!(
            "no `baseline` entry and no fired `require` row of the recorded pack names `{kind}`"
        );
    }

    "the recorded inputs name no clause that excluded it".to_owned()
}

/// The first top-level `and` conjunct of `src` that is not true over `ctx`.
///
/// # 🛑 ONLY A TOP-LEVEL CONJUNCTION IS DECOMPOSED, AND THAT LIMIT IS STATED
///
/// AC-433 asks for *the specific clause*, and for a conjunction the specific clause is the conjunct
/// that failed. The split is over top-level ` and ` only, tracking bracket depth so a parenthesised
/// group is never cut in half; a predicate that is not a conjunction has exactly one conjunct and
/// is reported whole, which is correct for it. A disjunction is **not** decomposed — no single
/// disjunct of a false `or` is "the" reason, so naming one would be inventing a cause.
fn false_conjunct(src: &str, ctx: &rk_kernel::predicate::Context) -> Option<String> {
    let parts = top_level_conjuncts(src);
    if parts.len() > 1
        && let Some(part) = parts.into_iter().find(|p| !predicate_is_true(p, ctx))
    {
        return Some(part);
    }
    Some(src.trim().to_owned())
}

/// `src` split on top-level ` and `, with bracket depth tracked.
fn top_level_conjuncts(src: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0_i32;
    let mut start = 0_usize;
    let bytes = src.as_bytes();
    let mut i = 0_usize;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            _ => {}
        }
        if depth == 0 && src[i..].starts_with(" and ") {
            parts.push(src[start..i].trim().to_owned());
            i += " and ".len();
            start = i;
            continue;
        }
        i += 1;
    }
    parts.push(src[start..].trim().to_owned());
    parts.retain(|p| !p.is_empty());
    parts
}

/// The `unknown` axes an admissibility row actually READ, with the domain position that says
/// how an ordered comparison treated each.
///
/// # 🛑 DETECTED BY POSITION, NOT BY THE TOKEN
///
/// The settled encoding is the ordinal domain member, so an axis is `unknown` exactly when the
/// recorded `index` points at the domain entry spelled `unknown`. Matching the recorded VALUE
/// against the string would miss a domain that spells its abstention differently, and matching
/// `FacetValue::Null` would miss this encoding entirely — which is what contract §2.7's
/// mechanism did, and why CONTRADICTION 1 replaced it.
fn unknown_axes_read(
    applicable_if: Option<&str>,
    ctx: &rk_kernel::predicate::Context,
    context_json: &serde_json::Value,
) -> Vec<(String, Vec<String>, usize)> {
    let Some(src) = applicable_if else {
        return Vec::new();
    };
    let Ok(parsed) = rk_kernel::predicate::parse(src) else {
        return Vec::new();
    };
    let Ok(outcome) = rk_kernel::predicate::evaluate(&parsed.expr, ctx) else {
        return Vec::new();
    };
    let facets = &context_json["facets"];
    outcome
        .read_facets
        .iter()
        .filter_map(|path| {
            let name = path.split('.').next().unwrap_or(path);
            let recorded = facets.get(name)?;
            let domain: Vec<String> = recorded
                .get("domain")?
                .as_array()?
                .iter()
                .filter_map(|d| d.as_str().map(str::to_owned))
                .collect();
            let index = usize::try_from(recorded.get("index")?.as_u64()?).ok()?;
            (domain.get(index).map(String::as_str) == Some("unknown"))
                .then(|| (name.to_owned(), domain, index))
        })
        .collect()
}

/// `"{stage}-{16 hex}"` minus its trailing 17 bytes.
fn recorded_stage(run_id: &str) -> String {
    run_id
        .rsplit_once('-')
        .map_or_else(|| run_id.to_owned(), |(stage, _)| stage.to_owned())
}

/// The payload of the LAST row of `kind`, if the record carries one.
fn row_payload(
    run: &rk_kernel::replay::RecordedRun,
    kind: rk_types::event_kind::EventKind,
) -> Option<serde_json::Value> {
    run.events
        .iter()
        .rev()
        .find(|e| e.kind == kind)
        .map(|e| e.payload.clone())
}

/// The recorded `result` of the `activity_committed` row for `activity_id`.
fn committed_result(
    run: &rk_kernel::replay::RecordedRun,
    activity_id: &str,
) -> Option<serde_json::Value> {
    run.events
        .iter()
        .rev()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
        .find(|e| {
            e.payload
                .get("activity_id_at_version")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|a| a.split('@').next() == Some(activity_id))
        })
        .and_then(|e| e.payload.get("result").cloned())
}

/// Evaluate a RECORDED predicate over the RECORDED context — a display read, never a selection.
///
/// §7d.7 permits this in terms: *"a read of two logged inputs through the predicate module, never
/// `select`"*. `Tri::True` is the only admitted answer; `False` and `Null` both mean the row did not
/// put this node in the DAG.
fn predicate_is_true(src: &str, ctx: &rk_kernel::predicate::Context) -> bool {
    rk_kernel::predicate::parse(src).is_ok_and(|parsed| {
        rk_kernel::predicate::evaluate(&parsed.expr, ctx)
            .is_ok_and(|outcome| outcome.value == rk_kernel::predicate::Tri::True)
    })
}

/// `", "`-joined, or `-` when empty — so an absent family is visibly absent rather than blank.
fn join(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.join(", ")
    }
}
