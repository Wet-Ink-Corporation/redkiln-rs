//! The seven verb-output structs (s8c §8c.4 "Typed outputs"; record V = ADR-0170, proposed, and
//! record T = ADR-0169 decision 8 for `BuildReport`): `BuildReport`, `SmokeResult`, `ScanReport`,
//! `BuildArtifact`, `SetupResult`, `ItemRefs` and `ReleaseTag`. Each is `deny_unknown_fields`, has
//! no `Default`, requires every member, is NOT in `EXPORTED` (which kinds an agent emits is `s8h`'s,
//! D21-41 g) and is `oneOf`-free anyway (Checkpoint 5 greps for it).
//!
//! *(Added 2026-09-26, s8c.)*

use rk_types::verb_output::{
    BuildArtifact, BuildReport, ItemRefs, ReleaseTag, ScanReport, SetupResult, SmokeResult,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// A valid sample of each struct, keyed by the struct's name. Every member is present, and every
/// nested row carries every member too, so the removal loop below can name each one.
fn samples() -> Vec<(&'static str, Value)> {
    vec![
        (
            "BuildReport",
            json!({
                "argv": ["cargo", "build", "--workspace"],
                "toolchain_version": "rustc 1.98.1",
                "wall_secs": 3,
                "errors": 0,
                "status": "passing"
            }),
        ),
        (
            "SmokeResult",
            json!({
                "rows": [{
                    "id": "help",
                    "argv": ["redkiln", "--help"],
                    "cwd": "scratch",
                    "expect_exit": 0,
                    "exit": 0,
                    "predicate": "contains:Usage",
                    "predicate_held": true,
                    "passed": true
                }],
                "passed": 1,
                "failed": 0,
                "status": "passing"
            }),
        ),
        (
            "ScanReport",
            json!({
                "critical": 0,
                "high": 1,
                "medium": 0,
                "low": 0,
                "unrated": 1,
                "advisories": [
                    {"id": "RUSTSEC-0000-0001", "package": "p", "version": "1.0.0", "severity": "high"},
                    {"id": "RUSTSEC-0000-0002", "package": "q", "version": "2.0.0", "severity": null}
                ],
                "tools": [{
                    "name": "cargo-audit",
                    "version": "0.22.2",
                    "db_path": "C:/db",
                    "db_commit": "ef8244d2ef8244d2ef8244d2ef8244d2ef8244d2"
                }]
            }),
        ),
        (
            "BuildArtifact",
            json!({
                "profile": "release",
                "binaries": [{"name": "redkiln", "path": "target/release/redkiln.exe", "sha256": "ab"}],
                "argv": ["cargo", "build", "--release", "--locked"],
                "toolchain_version": "rustc 1.98.1",
                "sha": "0123456789012345678901234567890123456789",
                "head": "0123456789012345678901234567890123456789"
            }),
        ),
        (
            "SetupResult",
            json!({
                "toolchain": "rust",
                "projects": [{
                    "path": "crates/x/project",
                    "argv": ["pnpm", "install", "--frozen-lockfile", "--prefer-offline"],
                    "lockfile_sha256": "cd"
                }]
            }),
        ),
        ("ItemRefs", json!({"ids": ["debt-1"]})),
        (
            "ReleaseTag",
            json!({
                "tag": "rk-release/0.1.0",
                "target": "0123456789012345678901234567890123456789",
                "rollback_target": "none"
            }),
        ),
    ]
}

/// Parse `value` as `T` and say whether it parsed.
fn parses<T: DeserializeOwned>(value: &Value) -> Result<(), String> {
    serde_json::from_value::<T>(value.clone())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Dispatch on the struct's name, so one loop drives all seven.
fn parse_as(name: &str, value: &Value) -> Result<(), String> {
    match name {
        "BuildReport" => parses::<BuildReport>(value),
        "SmokeResult" => parses::<SmokeResult>(value),
        "ScanReport" => parses::<ScanReport>(value),
        "BuildArtifact" => parses::<BuildArtifact>(value),
        "SetupResult" => parses::<SetupResult>(value),
        "ItemRefs" => parses::<ItemRefs>(value),
        "ReleaseTag" => parses::<ReleaseTag>(value),
        other => panic!("no such verb output `{other}` in this test's table"),
    }
}

/// Every object reachable in `value`, with a path: the top level and each object inside an array.
fn objects(value: &Value, path: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            out.push(path.to_owned());
            for (k, v) in map {
                objects(v, &format!("{path}/{k}"), out);
            }
        }
        Value::Array(items) => {
            for (i, v) in items.iter().enumerate() {
                objects(v, &format!("{path}/{i}"), out);
            }
        }
        _ => {}
    }
}

/// The object at a `/`-separated path, mutably.
fn at<'a>(value: &'a mut Value, path: &str) -> &'a mut serde_json::Map<String, Value> {
    let mut cur = value;
    for seg in path.split('/').filter(|s| !s.is_empty()) {
        cur = match cur {
            Value::Object(m) => m.get_mut(seg).expect("the path names a member"),
            Value::Array(a) => &mut a[seg.parse::<usize>().expect("an index")],
            _ => panic!("the path runs through a scalar"),
        };
    }
    cur.as_object_mut().expect("the path names an object")
}

/// **Every verb output refuses an unknown field, requires every member, and has no `Default`.**
/// For each of the seven: the sample parses; an unknown member at every object refuses; every
/// member removed at every object refuses naming it (an `Option` member included, so a `null`
/// severity must be WRITTEN, never inferred from absence); no `Default` is named on a code line of
/// the module; none is in `EXPORTED`; and the derived schema carries no `oneOf`.
#[test]
fn every_verb_output_refuses_unknown_fields_and_has_no_default() {
    let all = samples();
    assert_eq!(all.len(), 7, "the seven verb outputs of s8c §8c.4");
    for (name, sample) in &all {
        parse_as(name, sample).unwrap_or_else(|e| panic!("{name}'s sample must parse: {e}"));
        let mut paths = Vec::new();
        objects(sample, "", &mut paths);
        for path in &paths {
            let mut widened = sample.clone();
            at(&mut widened, path).insert("zz_unknown".to_owned(), json!(1));
            let err = parse_as(name, &widened)
                .expect_err(&format!("{name}{path}: an unknown member must refuse"));
            assert!(
                err.contains("zz_unknown"),
                "{name}{path}: the refusal names the unknown member: {err}"
            );
            let members: Vec<String> = at(&mut sample.clone(), path).keys().cloned().collect();
            for member in members {
                let mut narrowed = sample.clone();
                at(&mut narrowed, path).remove(&member);
                let err = parse_as(name, &narrowed).expect_err(&format!(
                    "{name}{path}: the absent member `{member}` must refuse (every member required)"
                ));
                assert!(
                    err.contains(&member),
                    "{name}{path}: the refusal names the absent `{member}`: {err}"
                );
            }
        }
    }

    // No `Default` on a code line of the module (AC-324's shape: an output cannot be conjured).
    let src = include_str!("../../src/verb_output.rs");
    for (i, line) in src.lines().enumerate() {
        let code = line.split("//").next().unwrap_or("");
        assert!(
            !code.contains("Default"),
            "verb_output.rs:{}: a verb output has no `Default`: {line}",
            i + 1
        );
    }

    // Not exported: which kinds an agent emits is s8h's (D21-41 g).
    let exported: Vec<&str> = rk_types::registry::EXPORTED
        .iter()
        .map(|(n, _)| *n)
        .collect();
    for (name, _) in &all {
        assert!(
            !exported.contains(name),
            "{name} is in EXPORTED; the verb outputs are not presented to agents at s8c"
        );
    }

    // oneOf-free anyway (Checkpoint 5 greps the snapshot directory for it).
    let schemas = [
        ("BuildReport", schemars::schema_for!(BuildReport)),
        ("SmokeResult", schemars::schema_for!(SmokeResult)),
        ("ScanReport", schemars::schema_for!(ScanReport)),
        ("BuildArtifact", schemars::schema_for!(BuildArtifact)),
        ("SetupResult", schemars::schema_for!(SetupResult)),
        ("ItemRefs", schemars::schema_for!(ItemRefs)),
        ("ReleaseTag", schemars::schema_for!(ReleaseTag)),
    ];
    for (name, schema) in schemas {
        let text = serde_json::to_string(&schema).expect("the schema serialises");
        assert!(
            !text.contains("oneOf"),
            "{name}'s schema carries a oneOf: {text}"
        );
    }
}

/// **`ScanReport.unrated` is a required member** (D21-43 N25: *"`sca-scan` requires `unrated == 0`
/// or goes to `scan.accept-findings`"*). A report without it refuses by its name; it is never read
/// as zero. An advisory with no severity is written `null` and counted there, never as `low`.
#[test]
fn scan_report_carries_unrated_as_a_required_member() {
    let (_, sample) = samples()
        .into_iter()
        .find(|(n, _)| *n == "ScanReport")
        .expect("the ScanReport sample");
    let report: ScanReport = serde_json::from_value(sample.clone()).expect("parses");
    assert_eq!(
        report.unrated, 1,
        "the sample's unrated advisory is counted"
    );
    assert_eq!(
        report
            .advisories
            .iter()
            .filter(|a| a.severity.is_none())
            .count(),
        1,
        "the unrated advisory is carried with no severity, not dropped"
    );
    let back = serde_json::to_value(&report).expect("serialises");
    assert_eq!(back["unrated"], json!(1), "unrated round-trips");
    assert_eq!(back["advisories"][1]["severity"], Value::Null);

    let mut without = sample;
    without.as_object_mut().expect("object").remove("unrated");
    let err = serde_json::from_value::<ScanReport>(without)
        .expect_err("a ScanReport without `unrated` must refuse, never default to zero");
    assert!(
        err.to_string().contains("unrated"),
        "the refusal names `unrated`: {err}"
    );
}
