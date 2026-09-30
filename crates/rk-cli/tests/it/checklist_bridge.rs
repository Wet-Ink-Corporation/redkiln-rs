//! **The checklist-envelope bridge** (`s8g` B1; D21-124, TAKEN PROVISIONALLY under D21-105; R4 =
//! ADR-0194 decision 2's dated note, proposed). *(Added 2026-09-29, `s8g` build, package
//! R-D2124-D2127-8g.)*
//!
//! ADR-0173's resolver hands a `checklist:` input to the port as its envelope, `{ key, path,
//! content, digest }`; ADR-0170's Rust `smoke` handler reads `checklist` as a LIST of items. The
//! bridge is a `Toolchain` decorator in `rk-cli`'s composition root
//! ([`rk_cli::verbs::run::bridged`]), applied where `choose_toolchain` constructs each adapter, so
//! every composition path (`real_toolchain`, the verify and review passes, `doctor`) gets it. It
//! unwraps ONLY a `checklist` input that is an object whose `key` starts `checklist:` and which
//! carries a string `content`, parsing the content with `rk_kernel::toml_subset` into the `[[item]]`
//! rows as a JSON array; every other shape but an array refuses `input-malformed`, naming the key.
//! The kernel's journalled input keeps its envelope and digest: the unwrap is at the port.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_cli::env::Env;
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, Toolchain, VerbCall,
    VerbOutput,
};
use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::BuildReport;

type Json = serde_json::Value;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// The resolver's envelope of the SHIPPED `.redkiln/checklists/smoke.toml`, in ADR-0173's form
/// (`demands::smoke_result_is_produced_through_the_registry_verb` asserts the resolver hands the
/// port exactly this `path` and `digest`).
fn shipped_smoke_envelope() -> Json {
    let bytes = std::fs::read(workspace_root().join(".redkiln/checklists/smoke.toml"))
        .expect("the shipped smoke checklist reads");
    serde_json::json!({
        "key": "checklist:smoke",
        "path": "checklists/smoke.toml",
        "content": String::from_utf8(bytes.clone()).expect("the checklist is UTF-8"),
        "digest": rk_types::digest::sha256(&bytes).as_str(),
    })
}

fn inputs_with(checklist: Json) -> BTreeMap<String, Json> {
    BTreeMap::from([("checklist".to_owned(), checklist)])
}

fn bound() -> StepBound {
    StepBound::secs(300).expect("a non-zero bound")
}

/// The detail of an `input-malformed` refusal (`Transport { Decode }`), or a panic naming what came.
fn malformed_detail(got: &Result<VerbOutput, StepError>) -> String {
    match got {
        Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        }) if detail.contains("(input-malformed)") => detail.clone(),
        other => panic!("expected an input-malformed refusal, got {other:?}"),
    }
}

/// A `Toolchain` that records every `run_verb` input map it is handed and answers `Echo`.
#[derive(Default)]
struct Recorder {
    seen: Mutex<Vec<BTreeMap<String, Json>>>,
}

impl Recorder {
    fn seen(&self) -> Vec<BTreeMap<String, Json>> {
        self.seen.lock().expect("unpoisoned").clone()
    }
}

impl Toolchain for Recorder {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        Err(StepError::Unsupported {
            capability: "build".to_owned(),
        })
    }
    fn test(&self, _selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        Err(StepError::Unsupported {
            capability: "test".to_owned(),
        })
    }
    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "lint".to_owned(),
        })
    }
    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Ok(Classification::Text)
    }
    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        Err(StepError::Unsupported {
            capability: "change_summary".to_owned(),
        })
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.seen
            .lock()
            .expect("unpoisoned")
            .push(call.inputs.clone());
        Ok(VerbOutput {
            kind: "Echo".to_owned(),
            value: Json::Null,
        })
    }
}

fn call(
    tc: &dyn Toolchain,
    verb: &str,
    inputs: &BTreeMap<String, Json>,
) -> Result<VerbOutput, StepError> {
    tc.run_verb(&VerbCall {
        verb,
        inputs,
        bound: bound(),
    })
}

/// **(i) The real `RustToolchain`, reached through `real_toolchain`, gets past input parsing on the
/// shipped checklist's envelope** (D21-124 item 5 (i)).
///
/// The scratch root is a repository carrying a `Cargo.toml` with no target, so the Rust adapter
/// detects it (a `Cargo.toml` and `cargo --version`) and the `smoke` handler's build then fails
/// fast. The assertion is only that the refusal, whatever it is, is NOT the `checklist` input's
/// `input-malformed`: before the bridge the same call answered *"smoke refused (input-malformed):
/// the `checklist` input: invalid type: map, expected a sequence"*.
#[test]
fn the_real_rust_smoke_gets_past_input_parsing_through_real_toolchain() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    std::fs::write(
        repo.path().join("Cargo.toml"),
        "[package]\nname = \"bridge-fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("the fixture manifest is writable");
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
    let env = Env::from_map(std::env::vars().collect());
    let toolchain = rk_cli::verbs::run::real_toolchain(&resolved, &env, repo.path())
        .unwrap_or_else(|e| panic!("real_toolchain over the fixture: {e:?}"));
    assert_eq!(
        toolchain.detect().ok(),
        Some(true),
        "PREMISE: a toolchain adapter detects the fixture (the Rust adapter over its Cargo.toml), \
         so the call below reaches the real handler and not the undetected fallback"
    );
    let got = call(
        toolchain.as_ref(),
        "smoke",
        &inputs_with(shipped_smoke_envelope()),
    );
    println!("BRIDGE_REAL_SMOKE {got:?}");
    let text = format!("{got:?}");
    assert!(
        !(text.contains("input-malformed") && text.contains("`checklist` input")),
        "the real Rust `smoke` handler, reached through real_toolchain, still refuses the shipped \
         checklist's envelope at input parsing (D21-124): {got:?}"
    );
    assert!(
        !text.contains("Unsupported"),
        "the call did not reach the Rust `smoke` handler: {got:?}"
    );
}

/// **(ii) The undecorated adapter still refuses the envelope** (D21-124 item 5 (ii)): the pin that
/// makes the decorator, and nothing in the adapter, the fix.
#[test]
fn the_undecorated_rust_smoke_still_refuses_the_envelope() {
    let root = tempfile::tempdir().expect("a temp root");
    let scratch = tempfile::tempdir().expect("a temp scratch");
    let cfg = rk_adapter_toolchain_rust::Config::new(
        PathBuf::from(rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
        std::time::Duration::from_secs(60),
        root.path(),
        std::env::vars().collect(),
        scratch.path().to_path_buf(),
    )
    .expect("the Rust adapter's configuration over this process's environment");
    let raw = rk_adapter_toolchain_rust::RustToolchain::new(cfg).expect("the Rust adapter builds");
    let got = call(&raw, "smoke", &inputs_with(shipped_smoke_envelope()));
    let detail = malformed_detail(&got);
    assert!(
        detail.contains("the `checklist` input: invalid type: map, expected a sequence"),
        "the undecorated handler's refusal moved (ADR-0170's `smoke` reads a list): {detail}"
    );
}

/// **(iii) Every shape the bridge does not unwrap refuses by name, and nothing is forwarded**
/// (D21-124 item 4).
#[test]
fn a_malformed_checklist_envelope_refuses_by_name() {
    let env = |key: &str, content: Json| serde_json::json!({ "key": key, "path": "checklists/x.toml", "content": content, "digest": "d" });
    let cases: Vec<(&str, Json, &str)> = vec![
        (
            "unparseable content",
            env(
                "checklist:broken",
                Json::from("[[item]]\nid = \"a\nargv = [\n"),
            ),
            "checklist:broken",
        ),
        (
            "no `item` array",
            env("checklist:noitems", Json::from("source = \"x\"\n")),
            "checklist:noitems",
        ),
        (
            "an empty `item` array",
            env("checklist:empty", Json::from("item = []\n")),
            "checklist:empty",
        ),
        (
            "an object whose key is not `checklist:`",
            env("kb:smoke", Json::from("[[item]]\nid = \"a\"\n")),
            "kb:smoke",
        ),
        (
            "an object with no string `content`",
            serde_json::json!({ "key": "checklist:nocontent", "path": "p", "digest": "d" }),
            "checklist:nocontent",
        ),
        (
            "an object with no `key`",
            serde_json::json!({ "content": "[[item]]\nid = \"a\"\n" }),
            "`checklist`",
        ),
        ("a string", Json::from("checklist:smoke"), "`checklist`"),
        ("a number", Json::from(7), "`checklist`"),
        ("null", Json::Null, "`checklist`"),
    ];
    let mut wrong = Vec::new();
    for (what, value, names) in &cases {
        let inner = Arc::new(Recorder::default());
        let bridged = rk_cli::verbs::run::bridged(Arc::clone(&inner) as Arc<dyn Toolchain>);
        let got = call(bridged.as_ref(), "smoke", &inputs_with(value.clone()));
        match &got {
            Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail,
            }) if detail
                .starts_with("smoke refused (input-malformed): the `checklist` input: ")
                && detail.contains(names) => {}
            other => wrong.push(format!(
                "{what}: not refused input-malformed naming {names}: {other:?}"
            )),
        }
        if !inner.seen().is_empty() {
            wrong.push(format!(
                "{what}: the raw value was forwarded to the adapter: {:?}",
                inner.seen()
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the bridge fails open (D21-124 item 4):\n  {}",
        wrong.join("\n  ")
    );
}

/// **The unwrap, and what passes unchanged** (D21-124 items 2 and 3): an envelope becomes its
/// `[[item]]` rows as a JSON array with field names and types kept (the shipped smoke checklist's
/// twelve rows); an input that is already an array, a call with no `checklist`, and every other
/// input pass through byte for byte.
#[test]
fn an_envelope_is_unwrapped_to_its_items_and_everything_else_passes_unchanged() {
    let inner = Arc::new(Recorder::default());
    let bridged = rk_cli::verbs::run::bridged(Arc::clone(&inner) as Arc<dyn Toolchain>);

    // The envelope, beside another input that must not move.
    let mut inputs = inputs_with(shipped_smoke_envelope());
    inputs.insert(
        "other".to_owned(),
        serde_json::json!({ "key": "checklist:not-me" }),
    );
    call(bridged.as_ref(), "smoke", &inputs).expect("the recorder answers");
    // An array already.
    let array =
        serde_json::json!([{ "id": "a", "argv": ["redkiln", "--version"], "expect_exit": 0 }]);
    call(bridged.as_ref(), "smoke", &inputs_with(array.clone())).expect("the recorder answers");
    // No checklist at all.
    let plain = BTreeMap::from([("binaries".to_owned(), serde_json::json!(["redkiln"]))]);
    call(bridged.as_ref(), "package", &plain).expect("the recorder answers");

    let seen = inner.seen();
    assert_eq!(
        seen.len(),
        3,
        "each call reached the adapter once: {seen:?}"
    );

    let items = seen[0]
        .get("checklist")
        .and_then(Json::as_array)
        .unwrap_or_else(|| panic!("the envelope was not unwrapped to a list: {:?}", seen[0]));
    assert_eq!(
        items.len(),
        12,
        "the shipped checklist's twelve rows: {items:?}"
    );
    let first = &items[0];
    assert_eq!(first.get("id"), Some(&Json::from("redkiln-version")));
    assert_eq!(
        first.get("argv"),
        Some(&serde_json::json!(["redkiln", "--version"]))
    );
    assert_eq!(first.get("cwd"), Some(&Json::from("scratch")));
    assert_eq!(first.get("expect_exit"), Some(&Json::from(0)));
    assert_eq!(
        first.get("stdout_contains"),
        Some(&Json::from("redkiln 0.0.0\n"))
    );
    assert_eq!(
        seen[0].get("other"),
        Some(&serde_json::json!({ "key": "checklist:not-me" })),
        "an input other than `checklist` moved"
    );
    assert_eq!(
        seen[1],
        inputs_with(array),
        "an array did not pass unchanged"
    );
    assert_eq!(
        seen[2], plain,
        "a call with no `checklist` did not pass unchanged"
    );
}

/// Every adapter-constructor site in `sources`, and those whose `Ok` arm is not `bridged(...)`.
fn constructor_sites(sources: &[(String, String)]) -> (Vec<String>, Vec<String>) {
    const CTORS: [&str; 4] = [
        "RustToolchain::new(",
        "RustToolchain::new_over_registry(",
        "NodeToolchain::new(",
        "NodeToolchain::new_over_registry(",
    ];
    let mut sites = Vec::new();
    let mut undecorated = Vec::new();
    for (name, text) in sources {
        let lines = crate::code_lines(text);
        for (i, (n, line)) in lines.iter().enumerate() {
            if !CTORS.iter().any(|c| line.contains(c)) {
                continue;
            }
            let site = format!("{name}:{n}: {}", line.trim());
            // The match's `Ok` arm, within the next three code lines, must wrap the built adapter.
            let decorated = lines
                .iter()
                .skip(i + 1)
                .take(3)
                .any(|(_, l)| l.contains("Ok(built) => bridged(Arc::new(built))"));
            if !decorated {
                undecorated.push(site.clone());
            }
            sites.push(site);
        }
    }
    (sites, undecorated)
}

/// **No toolchain adapter constructor in `rk-cli`'s `src` is reachable undecorated** (D21-124
/// item 1). A code-lines-only lint (comments cannot satisfy it): every `RustToolchain::new` /
/// `NodeToolchain::new` site's `Ok` arm is `bridged(Arc::new(built))`, and the sites are the two
/// `choose_toolchain` arms, through which `real_toolchain`, `doctor` and the passes all build.
/// A floor beneath the behavioural test (i) and the decorator-removal mutant.
#[test]
fn no_adapter_constructor_in_rk_cli_src_is_reachable_undecorated() {
    let sources: Vec<(String, String)> = crate::rs_files(&crate::src_dir())
        .into_iter()
        .map(|p| {
            let text = std::fs::read_to_string(&p).expect("a source file reads");
            (p.display().to_string(), text)
        })
        .collect();
    let (sites, undecorated) = constructor_sites(&sources);
    println!("BRIDGE_SITES {} {sites:?}", sites.len());
    assert_eq!(
        sites.len(),
        2,
        "the adapter constructor sites in rk-cli src are the two `choose_toolchain` arms: {sites:?}"
    );
    // CONTROL: the same lint flags an undecorated arm, and a comment naming the constructor is no site.
    let control = (
        "control.rs".to_owned(),
        concat!(
            "// RustToolchain::new(cfg) in a comment is not a site\n",
            "match rk_adapter_toolchain_rust::RustToolchain::new(cfg) {\n",
            "    Ok(built) => Arc::new(built),\n",
            "    Err(e) => return Err(e),\n",
            "}\n",
        )
        .to_owned(),
    );
    let (control_sites, control_bad) = constructor_sites(&[control]);
    assert_eq!(
        (control_sites.len(), control_bad.len()),
        (1, 1),
        "CONTROL: the lint does not flag an undecorated constructor arm: {control_sites:?}"
    );
    assert!(
        undecorated.is_empty(),
        "a toolchain adapter is built in rk-cli src without the checklist bridge (D21-124): \
         {undecorated:?}"
    );
}
