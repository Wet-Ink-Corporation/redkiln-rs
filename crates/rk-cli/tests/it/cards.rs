//! **`s8h` §8h.2, §8h.3, §8h.9 and §8h.10 — the SHIPPED cards and packs, read by parsers.**
//! *(Added 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments; Record A = ADR-0197,
//! Record B = ADR-0198, both proposed.)*
//!
//! Every test here reads the repository's own `.redkiln/` (one level deep: `cards/*.json`,
//! `packs/<pack>.toml`, `catalog/*.toml`), never a fixture, and never a run worktree under
//! `.redkiln/worktrees/` (D21-44). JSON is read with `serde_json` and TOML with the `toml` crate, so
//! nothing here is satisfied by a string that merely appears in a comment, with one exception that
//! is the point of its test: `shipped_rt57_scalars_carry_no_no_record_marker` reads the COMMENTS,
//! because the `NO RECORD` markers it retires are comments.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use rk_cli::verbs::run::PackExtras;

/// The workspace root: `crates/rk-cli/../..`.
fn workspace_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// The two installed packs (ADR-0128 decision 3's lane names).
const PACKS: [&str; 2] = ["bug-standard", "story"];

fn pack_src(pack: &str) -> String {
    let path = workspace_root()
        .join(".redkiln")
        .join("packs")
        .join(format!("{pack}.toml"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn pack_doc(pack: &str) -> toml::Value {
    toml::from_str(&pack_src(pack)).unwrap_or_else(|e| panic!("{pack}.toml is not TOML: {e}"))
}

fn pack_extras(pack: &str) -> PackExtras {
    toml::from_str(&pack_src(pack))
        .unwrap_or_else(|e| panic!("{pack}.toml does not carry rk-cli's PackExtras: {e}"))
}

/// §8h.2's table: card -> (tools, `permission_mode`).
fn least_privilege_table() -> BTreeMap<&'static str, (BTreeSet<&'static str>, &'static str)> {
    let read = ["Read", "Glob", "Grep", "StructuredOutput"];
    BTreeMap::from([
        (
            "implementer",
            (
                BTreeSet::from(["Read", "Glob", "Grep", "Edit", "Write", "StructuredOutput"]),
                "acceptEdits",
            ),
        ),
        ("investigator", (BTreeSet::from(read), "default")),
        ("reviewer", (BTreeSet::from(read), "default")),
        ("architect", (BTreeSet::from(read), "default")),
        ("spec-author", (BTreeSet::from(read), "default")),
        ("planner", (BTreeSet::from(["StructuredOutput"]), "default")),
        (
            "releaser",
            (BTreeSet::from(["StructuredOutput"]), "default"),
        ),
        (
            "operator",
            (BTreeSet::from(["StructuredOutput"]), "default"),
        ),
    ])
}

/// **§8h.2: the eight shipped cards are the least-privilege table, exactly** — each card's tool
/// list (as a set, with no duplicate) and its `permission_mode`, read with `serde_json`; no card
/// carries a tier (AC-118); each card LOADS through the adapter's own `AuthoredCard::from_file`
/// (the closed-set lint included); and the `spec-author` card's prompt states that every criterion
/// carries an authored `verification` (D21-43 N24, §8h.4).
///
/// RED (observed 2026-10-02, through D21-47's wrapper, before the cards were written): every
/// card's tools were `["StructuredOutput"]` and none carried `permission_mode`.
#[test]
fn shipped_cards_are_the_least_privilege_table() {
    let dir = workspace_root().join(".redkiln").join("cards");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the shipped cards directory reads")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .map(|p| {
            p.file_stem()
                .expect("a card file has a stem")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    let table = least_privilege_table();
    let want: Vec<String> = table.keys().map(|k| (*k).to_owned()).collect();
    assert_eq!(names, want, "the shipped card set is §8h.2's eight");

    let mut bad: Vec<String> = Vec::new();
    for (name, (tools, mode)) in &table {
        let path = dir.join(format!("{name}.json"));
        let text = std::fs::read_to_string(&path).expect("the card reads");
        let card: serde_json::Value = serde_json::from_str(&text).expect("the card is JSON");
        let listed: Vec<&str> = card
            .get("tools")
            .and_then(serde_json::Value::as_array)
            .map(|a| a.iter().filter_map(serde_json::Value::as_str).collect())
            .unwrap_or_default();
        let as_set: BTreeSet<&str> = listed.iter().copied().collect();
        if as_set != *tools || as_set.len() != listed.len() {
            bad.push(format!("{name}: tools {listed:?}, want {tools:?}"));
        }
        let got = card
            .get("permission_mode")
            .and_then(serde_json::Value::as_str);
        if got != Some(*mode) {
            bad.push(format!("{name}: permission_mode {got:?}, want {mode:?}"));
        }
        for tier in ["model", "effort"] {
            if card.get(tier).is_some() {
                bad.push(format!(
                    "{name}: carries `{tier}` (AC-118: the tier is the role's)"
                ));
            }
        }
        if let Err(e) = rk_adapter_claude::card::AuthoredCard::from_file(name, &text) {
            bad.push(format!(
                "{name}: does not load through AuthoredCard::from_file: {e}"
            ));
        }
    }
    let spec_author =
        std::fs::read_to_string(dir.join("spec-author.json")).expect("the spec-author card reads");
    let prompt = serde_json::from_str::<serde_json::Value>(&spec_author)
        .expect("the spec-author card is JSON")
        .get("prompt")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned();
    if !(prompt.contains("every criterion") && prompt.contains("verification")) {
        bad.push(format!(
            "spec-author: the prompt does not state that every criterion carries an authored \
             verification (D21-43 N24; s8h §8h.4): {prompt:?}"
        ));
    }
    assert!(
        bad.is_empty(),
        "the shipped cards are not §8h.2's table:\n  {}",
        bad.join("\n  ")
    );
}

/// The three `[models]` rows of Record A decision 4: key -> (full id, `accepts_effort`).
fn models_table() -> BTreeMap<&'static str, (&'static str, bool)> {
    BTreeMap::from([
        ("haiku", ("claude-haiku-4-5-20251001", false)),
        ("sonnet", ("claude-sonnet-5", true)),
        ("opus", ("claude-opus-5-5", true)),
    ])
}

/// **§8h.3: each installed pack's `[models]` table pins exactly the three full ids**, read through
/// `rk-cli`'s own `PackExtras` (the type the binder is handed), so a table the binder cannot read
/// fails here too.
///
/// RED (observed 2026-10-02, before the tables were written): both packs read an EMPTY `models`.
#[test]
fn the_models_table_pins_the_three_full_ids() {
    let want = models_table();
    for pack in PACKS {
        let extras = pack_extras(pack);
        let got: BTreeMap<&str, (&str, bool)> = extras
            .models
            .iter()
            .map(|(k, row)| (k.as_str(), (row.id.as_str(), row.accepts_effort)))
            .collect();
        assert_eq!(
            got, want,
            "{pack}: [models] is Record A decision 4's three rows"
        );
    }
}

/// **§8h.3: the role rows resolve through the `[models]` table** — every role the shipped catalog
/// nominates has a row; its `model` is a KEY of the same pack's table (never a full id, never a
/// tier absent from it); its `effort` is D21-41 (g)'s; a haiku role carries no effort, because the
/// haiku row does not accept one; and `family = "claude"` stays (D-7's partition key).
///
/// RED (observed 2026-10-02, before the rows were written): every role row carried `family`
/// alone, so `model` was `None`.
#[test]
fn shipped_role_rows_resolve_through_the_models_table() {
    let want: BTreeMap<&str, (&str, Option<rk_adapter_claude::card::Effort>)> = {
        use rk_adapter_claude::card::Effort;
        BTreeMap::from([
            ("implementer", ("sonnet", Some(Effort::High))),
            ("investigator", ("sonnet", Some(Effort::High))),
            ("spec-author", ("sonnet", Some(Effort::Medium))),
            ("reviewer", ("opus", Some(Effort::High))),
            ("architect", ("opus", Some(Effort::Medium))),
            ("planner", ("haiku", None)),
            ("operator", ("haiku", None)),
            ("releaser", ("haiku", None)),
        ])
    };
    for pack in PACKS {
        let extras = pack_extras(pack);
        let roles: BTreeSet<&str> = extras.role.keys().map(String::as_str).collect();
        let expected: BTreeSet<&str> = want.keys().copied().collect();
        assert_eq!(
            roles, expected,
            "{pack}: the role table is the catalog's eight roles"
        );
        let mut bad: Vec<String> = Vec::new();
        for (role, (key, effort)) in &want {
            let row = &extras.role[*role];
            if row.family != "claude" {
                bad.push(format!("{role}: family {:?}", row.family));
            }
            if row.model.as_deref() != Some(*key) {
                bad.push(format!("{role}: model {:?}, want {key:?}", row.model));
            }
            if row.effort != *effort {
                bad.push(format!("{role}: effort {:?}, want {effort:?}", row.effort));
            }
            match row.model.as_deref().and_then(|k| extras.models.get(k)) {
                None => bad.push(format!(
                    "{role}: model key {:?} is absent from [models] — the binder refuses it",
                    row.model
                )),
                Some(m) if row.effort.is_some() && !m.accepts_effort => bad.push(format!(
                    "{role}: an effort on a row that does not accept one ({})",
                    m.id
                )),
                Some(_) => {}
            }
        }
        assert!(
            bad.is_empty(),
            "{pack}: role rows do not resolve:\n  {}",
            bad.join("\n  ")
        );
    }
}

/// Every `question = "…"` value anywhere in the shipped catalog (inline `escalate` tables too).
fn catalog_questions() -> BTreeSet<String> {
    fn walk(v: &toml::Value, out: &mut BTreeSet<String>) {
        match v {
            toml::Value::Table(t) => {
                for (k, x) in t {
                    if k == "question"
                        && let Some(s) = x.as_str()
                    {
                        out.insert(s.to_owned());
                    }
                    walk(x, out);
                }
            }
            toml::Value::Array(a) => a.iter().for_each(|x| walk(x, out)),
            _ => {}
        }
    }
    let dir = workspace_root().join(".redkiln").join("catalog");
    let mut out = BTreeSet::new();
    let mut n = 0_usize;
    for entry in std::fs::read_dir(&dir).expect("the shipped catalog reads") {
        let path = entry.expect("an entry").path();
        if path.extension().is_none_or(|x| x != "toml") {
            continue;
        }
        let doc: toml::Value = toml::from_str(&std::fs::read_to_string(&path).expect("reads"))
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        walk(&doc, &mut out);
        n += 1;
    }
    assert!(n > 0, "PREMISE: no catalog file was read");
    out
}

/// The facet roots a pack's `require` rows read, through the kernel's own predicate parser.
fn require_axes(pack: &str) -> BTreeSet<String> {
    fn paths(e: &rk_kernel::predicate::Expr, out: &mut BTreeSet<String>) {
        use rk_kernel::predicate::Expr;
        match e {
            Expr::Or(a, b) | Expr::And(a, b) => {
                paths(a, out);
                paths(b, out);
            }
            Expr::Not(a) | Expr::Group(a) | Expr::Len(a) => paths(a, out),
            Expr::Cmp { left, right, .. } => {
                paths(left, out);
                paths(right, out);
            }
            Expr::In { left, list } => {
                paths(left, out);
                paths(list, out);
            }
            Expr::Member { literal, set } => {
                paths(literal, out);
                paths(set, out);
            }
            Expr::Path(p) => {
                out.insert(p.split('.').next().unwrap_or(p).to_owned());
            }
            Expr::List(xs) => xs.iter().for_each(|x| paths(x, out)),
            Expr::Quant { source, body, .. } => {
                paths(source, out);
                paths(body, out);
            }
            Expr::Ref(_) | Expr::Lit(_) => {}
        }
    }
    let doc = pack_doc(pack);
    let mut out = BTreeSet::new();
    if let Some(stages) = doc.get("stage").and_then(toml::Value::as_table) {
        for spec in stages.values() {
            for row in spec
                .get("require")
                .and_then(toml::Value::as_array)
                .into_iter()
                .flatten()
            {
                let when = row.get("when").and_then(toml::Value::as_str).unwrap_or("");
                let parsed = rk_kernel::predicate::parse(when)
                    .unwrap_or_else(|e| panic!("{pack}: `{when}` does not parse: {e}"));
                paths(&parsed.expr, &mut out);
            }
        }
    }
    out
}

/// **§8h.10: both packs carry a fail-closed `[[gate_expiry]]` row for every reachable class, and
/// for nothing else** — the 45 catalog questions, `unknown_facet:<axis>` for every axis the pack's
/// `require` rows read (5 in `bug-standard`, 7 in `story`), `unknown_facet:change` by name (`s8d`'s
/// R6 triage gate), `escalation`, and the kernel-minted kinds READ FROM THEIR CONSTANTS. That is 54
/// and 56 rows, each `fail` (AC-276's draft; no exception is signed). The expected set is derived
/// here, and the counts are PREMISES, so a catalog change that moves the set fails by name.
///
/// RED (observed 2026-10-02, before the rows were written): 0 rows in each pack.
#[test]
fn shipped_gate_expiry_rows_are_fail_closed_for_every_reachable_class() {
    let questions = catalog_questions();
    assert_eq!(
        questions.len(),
        45,
        "PREMISE: 45 catalog question classes (§8h.10)"
    );
    let minted: BTreeSet<String> = [
        rk_kernel::runtime::tier_one::LANDING_REFUSED_KIND,
        rk_kernel::workflow::fsm::DRIFT_GATE_KIND,
        rk_kernel::verify::ac_gate::AC_GATE_KIND,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    let escalation = rk_kernel::workflow::gate::ESCALATION_CLASS;
    for (pack, axes_want, rows_want) in [("bug-standard", 5, 54), ("story", 7, 56)] {
        let axes = require_axes(pack);
        assert_eq!(axes.len(), axes_want, "PREMISE: {pack} reads {axes:?}");
        assert!(
            axes.contains("change"),
            "PREMISE: {pack}'s require rows read `change`"
        );
        let mut want: BTreeSet<String> = questions.clone();
        want.extend(axes.iter().map(|a| format!("unknown_facet:{a}")));
        want.insert("unknown_facet:change".to_owned());
        want.insert(escalation.to_owned());
        want.extend(minted.iter().cloned());
        assert_eq!(
            want.len(),
            rows_want,
            "PREMISE: {pack}'s reachable class count (§8h.10)"
        );

        let extras = pack_extras(pack);
        let got: Vec<&str> = extras
            .gate_expiry
            .iter()
            .map(|r| r.class.as_str())
            .collect();
        let got_set: BTreeSet<String> = got.iter().map(|s| (*s).to_owned()).collect();
        assert_eq!(
            got.len(),
            got_set.len(),
            "{pack}: a gate class has two rows"
        );
        let missing: Vec<&String> = want.difference(&got_set).collect();
        let extra: Vec<&String> = got_set.difference(&want).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "{pack}: {} rows; missing {missing:?}; extra {extra:?}",
            got.len()
        );
        let not_fail: Vec<(&str, &str)> = extras
            .gate_expiry
            .iter()
            .filter(|r| r.action != "fail")
            .map(|r| (r.class.as_str(), r.action.as_str()))
            .collect();
        assert!(
            not_fail.is_empty(),
            "{pack}: rows drafted other than `fail` with no signed exception (AC-276): {not_fail:?}"
        );
    }
}

/// **§8h.9, Record B decision 5: no `NO RECORD` marker remains, and Record B is named where each
/// one stood** — on (or in the comment block directly above) the `gate_deadline_secs`, K
/// (`consecutive_platform_failures`), `expiry_secs`, `[margins]` and `appetite` lines, in both
/// packs; and the values are Record B's (864000, 3, 600, 10 / 10). This test reads comments on
/// purpose: the markers it retires are comments.
///
/// RED (observed 2026-10-02, before the comments were rewritten): five `NO RECORD` markers in each
/// pack.
#[test]
fn shipped_rt57_scalars_carry_no_no_record_marker() {
    const RECORD_B: &str = "ADR-0198";
    for pack in PACKS {
        let src = pack_src(pack);
        assert_eq!(
            src.matches("NO RECORD").count(),
            0,
            "{pack} still carries a `NO RECORD` marker"
        );
        let lines: Vec<&str> = src.lines().collect();
        for key in [
            "gate_deadline_secs",
            "consecutive_platform_failures",
            "expiry_secs",
            "[margins]",
            "appetite",
        ] {
            let at: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, l)| {
                    let t = l.trim_start();
                    !t.starts_with('#') && (t.starts_with(&format!("{key} ")) || t == key)
                })
                .map(|(i, _)| i)
                .collect();
            assert_eq!(
                at.len(),
                1,
                "{pack}: `{key}` is declared on exactly one line"
            );
            let i = at[0];
            let mut j = i;
            while j > 0 && lines[j - 1].trim_start().starts_with('#') {
                j -= 1;
            }
            let block = lines[j..=i].join("\n");
            assert!(
                block.contains(RECORD_B),
                "{pack}: `{key}` is not named to {RECORD_B} on its line or in the comment block \
                 above it:\n{block}"
            );
        }
        let doc = pack_doc(pack);
        let int = |v: Option<&toml::Value>| v.and_then(toml::Value::as_integer);
        assert_eq!(int(doc.get("gate_deadline_secs")), Some(864_000), "{pack}");
        let breaker = doc.get("breaker");
        assert_eq!(
            int(breaker.and_then(|b| b.get("consecutive_platform_failures"))),
            Some(3),
            "{pack}"
        );
        assert_eq!(
            int(breaker.and_then(|b| b.get("expiry_secs"))),
            Some(600),
            "{pack}"
        );
        let margins = doc.get("margins");
        assert_eq!(
            int(margins.and_then(|m| m.get("five_hour"))),
            Some(10),
            "{pack}"
        );
        assert_eq!(
            int(margins.and_then(|m| m.get("seven_day"))),
            Some(10),
            "{pack}"
        );
    }
}
