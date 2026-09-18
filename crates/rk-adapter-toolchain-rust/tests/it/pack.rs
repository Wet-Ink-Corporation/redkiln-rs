//! **The Rust technology pack fixture** (ADR-0094 decision 1: *"A Rust technology pack binds the
//! adapter's artifact kinds (`TestReport`, `Changeset`) per ADR-0029"*; s7c §7c.14), read through
//! `rk_kernel::toml_subset` over the dev edge — the one in-tree TOML reader; the adapter's library
//! parses no pack and the runtime reader is 7d/7e's.
//!
//! What is asserted, clause by clause: (i) the facts table names `classify` per item; (ii) a
//! producer for each of the two ADR-0094 kinds, **keyed to `rust`** — and both kinds are
//! `keyed_by = "toolchain"` in the catalog's `artifacts.toml`, so amendment 43's per-toolchain
//! instance rule (ADR-0073) is what applies, not the withdrawn tie-break; (iii) a contributed kind
//! and an additive `contribute` row at a declared stage; (iv) an override row; ADR-0029's forbids
//! (no stage, no baseline removal, no `require` row) hold on the fixture; AC-337's `[mutation]`
//! spelling reaches `MutationFeasibility::declared`; AC-213's `test_timeout` slot is a HUMAN
//! input with a `gate_record` placeholder and no number.

use std::path::Path;

use rk_kernel::toml_subset::{Table, Value, parse};
use rk_ports::toolchain::MutationFeasibility;

use crate::support::pack_fixture;

fn table<'a>(t: &'a Table, key: &str) -> &'a Table {
    t.get(key)
        .and_then(Value::as_table)
        .unwrap_or_else(|| panic!("the pack carries a `[{key}]` table"))
}

fn rows<'a>(t: &'a Table, key: &str) -> &'a [Value] {
    t.get(key)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("the pack carries `[[{key}]]` rows"))
}

/// Does any table under `v`, at any depth, carry `key`?
fn names_key(v: &Value, key: &str) -> bool {
    match v {
        Value::Table(t) => t.contains_key(key) || t.values().any(|c| names_key(c, key)),
        Value::Array(items) => items.iter().any(|c| names_key(c, key)),
        _ => false,
    }
}

fn str_of<'a>(t: &'a Table, key: &str) -> &'a str {
    t.get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("`{key}` is a string"))
}

/// **ADR-0094 d1 / ADR-0029 i-iv / AC-337 / AC-213** over the fixture.
#[test]
fn rust_pack_fixture_binds_kinds_per_adr0029() {
    let text = std::fs::read_to_string(pack_fixture()).expect("the pack fixture is readable");
    let pack = parse(&text).expect("the pack is inside toml_subset's measured subset");

    let head = table(&pack, "pack");
    assert_eq!(str_of(head, "kind"), "technology");
    assert_eq!(str_of(head, "toolchain"), "rust");
    assert_eq!(str_of(head, "adapter"), "rk-adapter-toolchain-rust");
    assert_eq!(str_of(head, "build"), "cargo build --workspace");
    assert_eq!(str_of(head, "test"), "cargo test --workspace");

    // (i) facts per item via classify
    let facts = table(&pack, "facts");
    assert!(
        str_of(facts, "toolchains").contains("Toolchain::classify"),
        "facts come from Toolchain::classify per item (ADR-0029 i)"
    );
    let kinds: Vec<&str> = rows(facts, "change_kinds")
        .iter()
        .map(|v| v.as_str().expect("a kind"))
        .collect();
    assert_eq!(kinds, ["text", "binary", "generated"]);

    // (ii) producers for the two existing kinds, keyed to rust
    let producers = rows(&pack, "producer");
    for kind in ["TestReport", "Changeset"] {
        let row = producers
            .iter()
            .filter_map(Value::as_table)
            .find(|r| r.get("kind").and_then(Value::as_str) == Some(kind))
            .unwrap_or_else(|| panic!("a producer for {kind} (ADR-0094 d1)"));
        assert_eq!(
            str_of(row, "toolchain"),
            "rust",
            "the {kind} producer is keyed to `rust` (ADR-0029 ii)"
        );
    }
    let test_report = producers
        .iter()
        .filter_map(Value::as_table)
        .find(|r| r.get("kind").and_then(Value::as_str) == Some("TestReport"))
        .expect("present");
    assert_eq!(
        str_of(test_report, "type"),
        "rk_types::verify::VerifyReport",
        "the Rust TYPE behind TestReport is VerifyReport, never a TestReport struct (§7c.14)"
    );
    // …and both kinds are toolchain-keyed in the catalog, so the per-toolchain instance rule applies.
    let artifacts_toml =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/artifacts.toml");
    let artifacts = parse(&std::fs::read_to_string(&artifacts_toml).expect("artifacts.toml"))
        .expect("artifacts.toml parses");
    let artifact = table(&artifacts, "artifact");
    for kind in ["TestReport", "Changeset"] {
        assert_eq!(
            str_of(table(artifact, kind), "keyed_by"),
            "toolchain",
            "{kind} is keyed_by toolchain in the catalog (amendment 43 / ADR-0073)"
        );
    }

    // (iii) a contributed kind and an additive contribute row at a declared stage
    let contributed = rows(&pack, "artifact");
    assert!(
        contributed
            .iter()
            .filter_map(Value::as_table)
            .any(|r| r.get("kind").and_then(Value::as_str) == Some("CargoMetadata")),
        "a contributed kind (ADR-0029 iii)"
    );
    let contribute = table(&pack, "contribute");
    let verify_rows = rows(contribute, "verify");
    let row = verify_rows[0].as_table().expect("a table");
    assert!(str_of(row, "when").contains("'rust' in toolchains"));

    // (iv) overrides
    let overrides = rows(&pack, "override");
    let over = overrides[0].as_table().expect("a table");
    assert_eq!(str_of(over, "activity"), "verify");
    assert!(over.get("timeout_secs").and_then(Value::as_int).is_some());
    assert!(over.get("cost_class").and_then(Value::as_str).is_some());

    // ADR-0029's forbids, on the fixture: no stage, no require row, no baseline removal.
    assert!(
        !pack.contains_key("stage"),
        "a technology pack adds no stage"
    );
    assert!(
        !pack.contains_key("policy"),
        "a technology pack owns no policy"
    );
    assert!(
        !names_key(&Value::Table(pack.clone()), "require"),
        "a technology pack edits no `require` row"
    );

    // AC-337: the [mutation] spelling reaches MutationFeasibility::declared (D17-3: a reading).
    let mutation = table(&pack, "mutation");
    let feasible = mutation.get("feasible").and_then(Value::as_bool);
    let reason = mutation.get("reason").and_then(Value::as_str);
    assert_eq!(
        MutationFeasibility::declared(feasible, reason).expect("a well-formed declaration"),
        MutationFeasibility::Feasible,
        "Rust declares mutation feasible, with no reason"
    );

    // AC-213: the human input has a slot and a gate_record placeholder, and is not a number.
    let toolchain = table(&pack, "toolchain");
    let slot = str_of(toolchain, "test_timeout");
    assert!(
        slot.contains("gate_record"),
        "test_timeout is a placeholder naming the gate record, never a number: {slot}"
    );
    assert!(
        toolchain
            .get("test_timeout")
            .and_then(Value::as_int)
            .is_none()
    );
    assert!(str_of(toolchain, "gate_record").contains("OWED"));
}
