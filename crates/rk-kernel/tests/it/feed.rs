//! **The `toolchain:<feed>` producers after `s8f`** (`s8f` §8f.7; D21-46 R7; D21-106; the render
//! record ADR-0189 decision 12, proposed). *(Added 2026-09-28, s8f build, package
//! S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! D21-106: `Runtime::call_feed` gains exactly ONE shape, a `store` row with no inputs, served by
//! the ONE composition-root-bound store-route seam the step-grain `Route::Store` arm uses; the
//! seam's kernel default refuses by name and never answers empty; the result is still decoded
//! against the row's `outputs`; every other route keeps its by-name refusal, byte-unchanged.
//!
//! Each test drives a real [`Runtime`] and reads the step's verdict and the seam's own call log.

use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::reference::{Env, Scope, UnwiredReadSeam, resolve_text};
use rk_kernel::runtime::step::{StepVerdict, StoreCall, StoreRoute, ToolRouting};
use rk_kernel::workflow::memo::{SourceRefusal, SourceValue, ToolchainValues, VerbRegistry};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{Toolchain, VerbOutput};
use rk_store::backlog::ParsedRecord;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::reference_resolver::{check, root, step, with_inputs};
use crate::runtime::{
    AID, CountingHarness, CountingToolchain, ITEM, STAGE, counters, ports_with_toolchain, store,
};

type Json = serde_json::Value;

/// A store-route seam over `rk-store`'s backlog reader, rooted in a temp worktree, whose names
/// come from `rows`. It LOGS every call.
struct BacklogSeam {
    tree: tempfile::TempDir,
    rows: Vec<Event>,
    calls: Mutex<Vec<(String, usize)>>,
}

impl BacklogSeam {
    fn new(rows: Vec<Event>) -> Self {
        Self {
            tree: tempfile::tempdir().expect("a temp worktree"),
            rows,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn write(&self, id: &str) {
        let path = self.tree.path().join(format!(".redkiln/items/{id}.toml"));
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("the items dir");
        std::fs::write(
            path,
            format!("id = \"{id}\"\nkind = \"tech-debt\"\nstage = \"triage\"\n"),
        )
        .expect("the record writes");
    }

    fn calls(&self) -> Vec<(String, usize)> {
        self.calls.lock().expect("not poisoned").clone()
    }
}

/// The records these tests write: three top-level keys, one per line.
fn parse(bytes: &[u8]) -> Result<ParsedRecord, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let mut out = ParsedRecord::default();
    for line in text.lines() {
        let Some((k, v)) = line.split_once(" = ") else {
            continue;
        };
        let v = Some(v.trim_matches('"').to_owned());
        match k {
            "id" => out.id = v,
            "kind" => out.kind = v,
            "stage" => out.stage = v,
            _ => {}
        }
    }
    Ok(out)
}

impl StoreRoute for BacklogSeam {
    fn serve(&self, call: &StoreCall<'_>) -> Result<VerbOutput, StepError> {
        self.calls
            .lock()
            .expect("not poisoned")
            .push((call.verb.to_owned(), call.inputs.len()));
        let named = rk_store::backlog::names(&self.rows);
        let snap = rk_store::backlog::snapshot(self.tree.path(), &named, &parse).map_err(|e| {
            StepError::Unsupported {
                capability: e.to_string(),
            }
        })?;
        Ok(VerbOutput {
            kind: "BacklogSnapshot".to_owned(),
            value: rk_store::backlog::to_value(&snap).map_err(|e| StepError::Unsupported {
                capability: e.to_string(),
            })?,
        })
    }
}

fn boundary(item: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "review", "to": "closeout", "item": item }),
    )
    .with_item(item)
}

/// A runtime over `registry` with the feed table `table` and, when given, a bound store route.
fn runtime(
    registry: VerbRegistry,
    table: &str,
    route: Option<Arc<dyn StoreRoute>>,
) -> (Runtime, Arc<CountingToolchain>) {
    let values = registry.values_for(table).unwrap_or_default();
    let mut routing = ToolRouting::new(registry, values)
        .with_sources(Some(table.to_owned()), Some(root().join(".redkiln")));
    if let Some(route) = route {
        routing = routing.with_store_route(route);
    }
    let (q, a) = counters();
    let db = store(&q, &a);
    let tc = Arc::new(CountingToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        "run-feed",
        STAGE,
    )
    .with_tool_routing(routing);
    (rt, tc)
}

fn shipped_registry() -> VerbRegistry {
    VerbRegistry::load_file(&root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry loads")
}

fn shipped_catalog() -> Catalog {
    Catalog::from_dir(&root().join(".redkiln").join("catalog")).expect("the shipped catalog loads")
}

/// A `check` step reading `closure-record/audit`'s own `backlog` input, verbatim.
fn audit_backlog_input() -> String {
    let catalog = shipped_catalog();
    let activity = catalog
        .activities()
        .iter()
        .find(|a| a.id == "closure-record")
        .expect("the shipped catalog carries closure-record");
    let audit = activity
        .steps
        .iter()
        .find(|s| s.id == "audit")
        .expect("closure-record has an `audit` step");
    let backlog = audit
        .inputs
        .get("backlog")
        .and_then(rk_kernel::catalog::InputValue::as_text)
        .expect("audit declares `backlog`")
        .to_owned();
    assert_eq!(
        backlog, "toolchain:backlog",
        "PREMISE: `audit` reads the backlog feed as a MANDATORY input"
    );
    backlog
}

/// **R7, over the shipped catalog and registry**: `closure-record/audit`'s mandatory
/// `toolchain:backlog` resolves through the `backlog` verb's `store` route — the ONE store-route
/// seam, called once with no inputs — to a `BacklogSnapshot` that decodes and binds. Red while
/// `[feed.rust.backlog]` is `unprovided` (M34).
#[test]
fn closure_record_audit_resolves_toolchain_backlog_through_the_store_route() {
    let input = audit_backlog_input();
    let seam = Arc::new(BacklogSeam::new(vec![
        boundary("present-1"),
        boundary("gone-1"),
    ]));
    seam.write("present-1");
    let (rt, tc) = runtime(
        shipped_registry(),
        "rust",
        Some(Arc::clone(&seam) as Arc<dyn StoreRoute>),
    );
    let probe = check(
        "audit-probe",
        &[("backlog", input.as_str())],
        "backlog.items.len() == 1 and backlog.absent.len() == 1",
    );
    let steps = vec![probe.clone()];
    let verdict = rt
        .execute_step_in(
            &probe,
            &rk_kernel::predicate::Context::new(),
            &Scope::new(&steps),
        )
        .expect("a value");
    assert_eq!(
        verdict,
        StepVerdict::Continue,
        "`toolchain:backlog` did not resolve through the store route to the snapshot: {verdict:?}"
    );
    assert_eq!(
        seam.calls(),
        vec![("backlog".to_owned(), 0)],
        "the feed is produced by ONE store-route call of `backlog`, with no inputs"
    );
    assert_eq!(
        tc.calls(),
        0,
        "no Toolchain adapter is called: the store route is the kernel's"
    );
}

/// Every `toolchain:<feed>` reach in the shipped catalog's step inputs, as `(input, activity/step)`.
fn feed_reaches(catalog: &Catalog) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for a in catalog.activities() {
        for s in &a.steps {
            for v in s.inputs.values() {
                if let Some(t) = rk_kernel::catalog::InputValue::as_text(v)
                    && t.starts_with("toolchain:")
                {
                    out.push((t.to_owned(), format!("{}/{}", a.id, s.id)));
                }
            }
        }
    }
    out.sort();
    out
}

/// The reaches of an `unprovided` feed on `table`, each `(feed, activity/step, optional)`. A reach
/// is of an unprovided feed when the feed's MANDATORY resolution refuses as `Unprovided`.
fn unprovided_reaches(
    registry: &VerbRegistry,
    table: &str,
    reaches: &[(String, String)],
) -> Vec<(String, String, bool)> {
    reaches
        .iter()
        .filter_map(|(input, site)| {
            let bare = input.trim_end_matches('?');
            match registry.resolve_source(table, bare) {
                Err(SourceRefusal::Unprovided { .. }) => Some((
                    bare.trim_start_matches("toolchain:").to_owned(),
                    site.clone(),
                    input.ends_with('?'),
                )),
                _ => None,
            }
        })
        .collect()
}

/// **§8f.7's list, pinned**: after this stage every reach of an `unprovided` feed is OPTIONAL
/// (ADR-0032: `null`), and `backlog` is reached only through its verb. The red case is a synthetic
/// MANDATORY reach of an `unprovided` feed, which the resolver refuses by name, naming the feed and
/// the step — M35 resolves it to `null`.
#[test]
fn every_shipped_unprovided_feed_is_reached_only_as_optional() {
    let registry = shipped_registry();
    let reaches = feed_reaches(&shipped_catalog());
    for table in ["rust", "node"] {
        let unprovided = unprovided_reaches(&registry, table, &reaches);
        let want: Vec<(String, String, bool)> = [
            ("logs", "root-cause/hypothesise"),
            ("sbom", "dependency-update/rescan"),
            ("sbom", "exploitability-analysis/sca"),
            ("sbom", "sca-scan/scan"),
        ]
        .into_iter()
        .map(|(f, s)| (f.to_owned(), s.to_owned(), true))
        .collect();
        let mut got = unprovided.clone();
        got.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        assert_eq!(
            got, want,
            "[{table}] the reaches of an `unprovided` feed are §8f.7's four, each optional"
        );
        let mandatory: Vec<_> = unprovided.iter().filter(|(_, _, opt)| !opt).collect();
        assert!(
            mandatory.is_empty(),
            "[{table}] a MANDATORY reach of an `unprovided` feed stops its activity: {mandatory:?}"
        );
        for site in ["closure-record/audit", "problem-record/draft"] {
            assert!(
                reaches
                    .iter()
                    .any(|(i, s)| i == "toolchain:backlog" && s == site),
                "PREMISE: `{site}` reaches `toolchain:backlog` as a mandatory input"
            );
        }
        assert_eq!(
            registry.resolve_source(table, "toolchain:backlog"),
            Ok(SourceValue::FeedVerb("backlog".to_owned())),
            "[{table}] `backlog` is produced by the `backlog` verb"
        );
    }

    // ---- the RED case: a synthetic mandatory reach of an `unprovided` feed ----------------------
    let values = ToolchainValues::default();
    let feed = |verb: &str| -> Result<Json, String> { Err(format!("no producer for {verb}")) };
    let seam = UnwiredReadSeam;
    let sources = root().join(".redkiln");
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events: &[],
        values: &values,
        registry: Some(&registry),
        feed_table: Some("rust"),
        sources: Some(&sources),
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    let steps = vec![with_inputs(
        step("hypothesise", StepKind::Agent),
        &[("logs", "toolchain:logs")],
    )];
    let scope = Scope::new(&steps);
    let e = resolve_text(&env, &scope, "hypothesise", "logs", "toolchain:logs")
        .expect_err("a MANDATORY reach of an `unprovided` feed is refused by name, never `null`");
    let text = e.to_string();
    assert!(
        text.contains("feed.rust.logs")
            && text.contains("`hypothesise`")
            && text.contains("unprovided"),
        "the refusal names the feed, the step and why: {text}"
    );
    // ... and the census reads it as a mandatory reach, naming the feed and the activity/step.
    let synthetic = vec![(
        "toolchain:logs".to_owned(),
        "synthetic/hypothesise".to_owned(),
    )];
    assert_eq!(
        unprovided_reaches(&registry, "rust", &synthetic),
        vec![("logs".to_owned(), "synthetic/hypothesise".to_owned(), false)],
        "the census does not see a synthetic mandatory reach of `logs`"
    );
}

/// **D21-106 ruling 3: the kernel's default store route REFUSES BY NAME**, for the feed arm and for
/// the step-grain arm alike, and never serves an empty value.
#[test]
fn an_unbound_store_route_refuses_by_name_and_never_serves_empty() {
    let input = audit_backlog_input();
    let (rt, tc) = runtime(shipped_registry(), "rust", None);
    let probe = check(
        "audit-probe",
        &[("backlog", input.as_str())],
        "backlog.items.len() == 0",
    );
    let steps = vec![probe.clone()];
    let verdict = rt
        .execute_step_in(
            &probe,
            &rk_kernel::predicate::Context::new(),
            &Scope::new(&steps),
        )
        .expect("a value");
    let StepVerdict::Failed(StepError::Unsupported { capability }) = &verdict else {
        panic!("an unbound store route served the feed: {verdict:?}");
    };
    assert!(
        capability.contains("no store-route handler is bound") && capability.contains("`backlog`"),
        "the feed's refusal does not name the unbound route: {capability}"
    );
    // The step-grain arm: `item_create`'s `store` row, through the same default.
    let mut file = with_inputs(step("file", StepKind::Tool), &[("items", "x")]);
    file.run = Some("toolchain:item_create".to_owned());
    let steps = vec![file.clone()];
    let verdict = rt
        .execute_step_in(
            &file,
            &rk_kernel::predicate::Context::new(),
            &Scope::new(&steps),
        )
        .expect("a value");
    let StepVerdict::Failed(StepError::Unsupported { capability }) = &verdict else {
        panic!("an unbound store route served `item_create`: {verdict:?}");
    };
    assert!(
        capability.contains("no store-route handler is bound")
            && capability.contains("`item_create`"),
        "the step's refusal does not name the unbound route: {capability}"
    );
    assert_eq!(tc.calls(), 0, "no Toolchain adapter is called");
}

/// **D21-106 ruling 1's negative route**: a `git`-route feed verb, and a `store`-route feed verb
/// that declares an input, still refuse with the existing message — and the bound store-route seam
/// is never called.
#[test]
fn a_git_route_feed_and_a_store_route_feed_with_an_input_still_refuse() {
    let registry = VerbRegistry::from_toml(
        "[verb]\ngitfeed = { outputs = \"ReleaseTag\", live = true, timeout = \"build_timeout\", \
         inputs = [], route = \"git\", side_effecting = false }\nstoreone = { outputs = \
         \"BacklogSnapshot\", live = true, timeout = \"build_timeout\", inputs = [\"x\"], route = \
         \"store\", side_effecting = false }\n\n[feed.t.viagit]\nverb = \"gitfeed\"\n\n\
         [feed.t.viastore]\nverb = \"storeone\"\n\n[toolchain.t]\nbuild_timeout = { secs = 60, \
         measured = [30], rule = \"r\", pin = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", source \
         = \"test\" }\n",
    )
    .expect("the test registry loads");
    let seam = Arc::new(BacklogSeam::new(Vec::new()));
    let (rt, tc) = runtime(
        registry,
        "t",
        Some(Arc::clone(&seam) as Arc<dyn StoreRoute>),
    );
    for (feed, verb, route, inputs) in [
        ("viagit", "gitfeed", "git", 0),
        ("viastore", "storeone", "store", 1),
    ] {
        let key = format!("toolchain:{feed}");
        let probe = check("probe", &[("f", key.as_str())], "f.items.len() == 0");
        let steps = vec![probe.clone()];
        let verdict = rt
            .execute_step_in(
                &probe,
                &rk_kernel::predicate::Context::new(),
                &Scope::new(&steps),
            )
            .expect("a value");
        let StepVerdict::Failed(StepError::Unsupported { capability }) = &verdict else {
            panic!("`{feed}` resolved: {verdict:?}");
        };
        let existing = format!(
            "the feed verb `{verb}` takes the `{route}` route with {inputs} declared input(s); a feed \
             is produced by a `toolchain` row with no inputs"
        );
        assert!(
            capability.contains(&existing),
            "`{feed}` did not refuse with the existing message:\n{capability}\nwanted:\n{existing}"
        );
    }
    assert!(
        seam.calls().is_empty(),
        "the store-route seam was called: {:?}",
        seam.calls()
    );
    assert_eq!(tc.calls(), 0, "no Toolchain adapter is called");
}
