//! **The command builder and its lints** — AC-211 (PROVISIONAL) / AC-383's golden argv per profile,
//! AC-523's argv cap, AC-203, AC-205/AC-417, AC-214 (unit half), AC-216 (the matrix), AC-511 (the
//! lint with its control), AC-209, AC-132 (s7c §7c.5).
//!
//! One builder assembles every argv (`rk_adapter_claude::command::assemble`); the binder calls it
//! at bind time, so every refusal below is also `Binder::bind`'s. **The marker-count belt is Slice
//! 5's** (§7c.18, §7c.20 item 4); these are the `is_err()` halves.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use proptest::prelude::*;
use rk_types::digest::sha256;

use rk_adapter_claude::card::{AgentCard, CacheProfile, Effort, Tier};
use rk_adapter_claude::command::{
    ARGV_CAP_BYTES, Assembled, CommandSpec, assemble, command_line_units,
};
use rk_adapter_claude::config::{Config, ConfigError, MismatchPolicy};
use rk_adapter_claude::env_denylist::is_denylisted;

use crate::support::{
    Markers, belt_dispatch_agent, belt_dispatch_authored, catalog_card, config, fixture, schema,
    spawn_values, values,
};

/// A bound card the builder takes: the catalog card with the tier injected by hand (the binder's
/// job, tested in `binder.rs`; here the builder is driven directly).
fn bound(stem: &str, tier: Tier, effort: Option<Effort>) -> AgentCard {
    let c = catalog_card(stem);
    AgentCard {
        name: c.name,
        description: c.description,
        prompt: c.prompt,
        tools: c.tools,
        model: tier.as_str().to_owned(),
        effort: effort.map(|e| e.as_str().to_owned()),
        permission_mode: c.permission_mode,
        cache_profile: c.cache_profile,
    }
}

const fn spec<'a>(
    card: &'a AgentCard,
    schema_json: &'a serde_json::Value,
    user_message: &'a str,
) -> CommandSpec<'a> {
    CommandSpec {
        card,
        schema: schema_json,
        adopt_agent: true,
        strict_tools: &[],
        user_message,
    }
}

/// The `flags` arrays of every dispatch row in an extracted recorded file (§7c.17).
fn recorded_flags(file: &str) -> Vec<Vec<String>> {
    fixture(&format!("recorded/{file}"))
        .lines()
        .map(|line| {
            let row: serde_json::Value = serde_json::from_str(line).expect("a recorded row parses");
            serde_json::from_value(row["flags"].clone()).expect("the row carries a flags array")
        })
        .collect()
}

/// The pinned golden vector of a fixture file under `tests/fixtures/argv/`.
fn golden(file: &str) -> Vec<String> {
    serde_json::from_str(&fixture(&format!("argv/{file}"))).expect("a golden argv fixture parses")
}

/// AC-211 (PROVISIONAL on S-H1c, built, not claimed) / AC-383 (golden-argv half): ONE element
/// order for both profiles — the lifted `build_args` order with `--strict-mcp-config` after
/// `--permission-mode`, then `--permission-prompts none`, and under `static` only `--system-prompt
/// <card body>` LAST (§7c.5; TD3-2; O-5: both `--system-prompt` and `--agents`/`--agent` are
/// passed). The measured prefix is the corpus's: h1-gap's six dispatch rows (`default`) and
/// h1-sysprompt's two (`static`), read from the extracted rows with the element set and the order
/// of the first six asserted against them. BOTH append flags are asserted absent from both vectors,
/// and the body asserts both profiles were exercised — a count of 2.
///
/// RED (observed 2026-09-18) against the skeleton (the runner's order — `--system-prompt` before
/// `--strict-mcp-config`, no `--permission-prompts none`): `assertion left == right failed:
/// default: the argv is the ONE stated order (§7c.5)` — the built vector ending
/// `"--permission-mode", "default", "--strict-mcp-config"` against the golden one ending
/// `…, "--strict-mcp-config", "--permission-prompts", "none"`.
#[test]
fn golden_argv_per_profile() {
    let cfg = config();
    let schema = schema();
    let profiles = [
        (
            "golden-default",
            CacheProfile::Default,
            "golden-default.argv.json",
            "h1-gap.dispatch-rows.jsonl",
        ),
        (
            "golden-static",
            CacheProfile::Static,
            "golden-static.argv.json",
            "h1-sysprompt.dispatch-rows.jsonl",
        ),
    ];
    let mut exercised: BTreeSet<CacheProfile> = BTreeSet::new();
    // The built vectors, per profile, for the snapshot belt below: a second observation of the
    // BUILDER, so a builder change moves the review-visible `.snap` even where the fixture equality
    // above it is edited to match.
    let mut built_by_profile: BTreeMap<&str, (Vec<String>, String)> = BTreeMap::new();
    for (stem, profile, golden_file, recorded_file) in profiles {
        let card = bound(stem, Tier::Haiku, None);
        assert_eq!(
            card.cache_profile, profile,
            "PREMISE: the fixture card carries the profile"
        );
        let built = assemble(&cfg, &spec(&card, &schema.json, "Review the diff."))
            .expect("the golden card builds");
        let want = golden(golden_file);
        assert_eq!(
            built.argv,
            want,
            "{}: the argv is the ONE stated order (§7c.5)",
            profile.as_str()
        );

        // The corpus-measured element set and the order of the first six (§7c.5, §7c.17).
        let rows = recorded_flags(recorded_file);
        assert!(
            !rows.is_empty(),
            "PREMISE: {recorded_file} carries dispatch rows"
        );
        for flags in &rows {
            assert_eq!(
                &flags[..6],
                &built.argv[..6],
                "the first six elements are the recorded ones"
            );
            assert!(flags.contains(&"--strict-mcp-config".to_owned()));
            assert!(built.argv.contains(&"--strict-mcp-config".to_owned()));
        }
        // The two profiles differ by exactly the `--system-prompt` pair, LAST under static.
        match profile {
            CacheProfile::Static => {
                let n = built.argv.len();
                assert_eq!(
                    built.argv[n - 2],
                    "--system-prompt",
                    "static: --system-prompt is last (h1-sysprompt's position)"
                );
                assert_eq!(
                    built.argv[n - 1],
                    card.prompt,
                    "static: the element is the card body, verbatim"
                );
                for flags in &rows {
                    let m = flags.len();
                    assert_eq!(
                        flags[m - 2],
                        "--system-prompt",
                        "recorded: the static arm put it last"
                    );
                    assert_eq!(flags[m - 1].len(), 45, "recorded: the 45-byte prompt");
                }
                assert_eq!(
                    built.stdin, "Review the diff.",
                    "static: the user message alone on stdin"
                );
            }
            CacheProfile::Default => {
                assert!(
                    !built.argv.iter().any(|a| a == "--system-prompt"),
                    "default: --system-prompt is absent (AC-383: F28's measured arm)"
                );
                assert!(
                    built.stdin.starts_with(&card.prompt),
                    "default: the card body is at the HEAD of the user message (AC-383)"
                );
                assert!(
                    built.stdin.ends_with("Review the diff."),
                    "default: then the user message"
                );
                for flags in &rows {
                    assert!(
                        !flags.iter().any(|a| a == "--system-prompt"),
                        "recorded: no --system-prompt on the default arm"
                    );
                }
            }
        }
        // AC-211: BOTH append flags absent (the 211-line file grepped only one — §7c.5).
        for flag in [
            "--append-system-prompt",
            "--exclude-dynamic-system-prompt-sections",
        ] {
            assert!(
                !built.argv.iter().any(|a| a == flag),
                "{}: {flag} must be absent from argv (AC-211)",
                profile.as_str()
            );
        }
        // The authored elements, each warranted (AC-203, AC-205, AC-214, AC-215).
        let pos = |e: &str| {
            built
                .argv
                .iter()
                .position(|a| a == e)
                .unwrap_or_else(|| panic!("{e} absent"))
        };
        assert_eq!(built.argv[pos("--agent") + 1], stem);
        assert!(built.argv[pos("--agents") + 1].starts_with(&format!("{{\"{stem}\":")));
        assert_eq!(
            built.argv[pos("--json-schema") + 1],
            schema.json.to_string()
        );
        assert_eq!(
            built.argv[pos("--permission-mode") + 1],
            card.permission_mode
        );
        assert_eq!(built.argv[pos("--permission-prompts") + 1], "none");
        assert!(
            pos("--permission-mode") < pos("--strict-mcp-config"),
            "strict AFTER permission-mode"
        );
        assert!(pos("--strict-mcp-config") < pos("--permission-prompts"));
        assert_eq!(built.program, Path::new("claude"));
        exercised.insert(profile);
        built_by_profile.insert(profile.as_str(), (built.argv.clone(), built.stdin.clone()));
    }
    assert_eq!(
        exercised.len(),
        2,
        "BOTH profiles were exercised (§7c.5: a count of 2)"
    );

    // The one optional element: `--effort <e>` directly after `--model <tier>` on a tier that
    // accepts it (the lifted order), pinned on the reviewer card under sonnet/high.
    let card = bound("reviewer", Tier::Sonnet, Some(Effort::High));
    let built = assemble(&cfg, &spec(&card, &schema.json, "")).expect("builds");
    assert_eq!(built.argv, golden("reviewer-sonnet-high.argv.json"));
    assert_eq!(&built.argv[4..8], ["--model", "sonnet", "--effort", "high"]);

    // The snapshot belt: the BUILT argv and stdin per profile, review-visible — what the builder
    // produced, not the fixture read back (a snapshot of the fixture against itself observed
    // nothing the builder did).
    let (argv, stdin) = &built_by_profile["default"];
    insta::assert_json_snapshot!(
        "golden_argv_default",
        serde_json::json!({ "argv": argv, "stdin": stdin })
    );
    let (argv, stdin) = &built_by_profile["static"];
    insta::assert_json_snapshot!(
        "golden_argv_static",
        serde_json::json!({ "argv": argv, "stdin": stdin })
    );
}

/// AC-523's argv-cap instrument (decision #18 of 2026-09-11; RT-1: 32,767): *"the command builder
/// refuses before spawn with a typed configuration error at the cap; fixture at the cap"* — the
/// fixture card measures EXACTLY `ARGV_CAP_BYTES` on the command line the builder would hand
/// `CreateProcessW` (the test asserts the measure, so the fixture is verified at the cap and not
/// merely believed to be), and is refused; one unit under it is not (§7c.5).
///
/// RED (observed 2026-09-18) twice: against the skeleton's byte-sum measure, `one byte under the
/// cap measures cap - 1: left: 32702 right: 32766` (a measure that is not the platform's is red
/// before the refusal is); then, with the Windows measure in and the cap check disabled (a
/// mutant), `the at-the-cap fixture must be refused before spawn (AC-523; RT-1); the builder
/// produced 17 elements measuring 32767`.
#[test]
fn argv_cap_refuses_before_spawn() {
    let cfg = config();
    let schema = schema();
    let text = fixture("argv/at-the-cap.card.json");
    let authored = rk_adapter_claude::card::AuthoredCard::from_file("at-the-cap", &text)
        .expect("the at-the-cap card loads (it fails only the cap)");
    let card = AgentCard {
        name: authored.name.clone(),
        description: authored.description.clone(),
        prompt: authored.prompt.clone(),
        tools: authored.tools.clone(),
        model: Tier::Haiku.as_str().to_owned(),
        effort: None,
        permission_mode: authored.permission_mode.clone(),
        cache_profile: authored.cache_profile,
    };
    // The fixture IS at the cap: the measure over the vector the builder would produce.
    let dry = {
        let mut c = card.clone();
        c.prompt.pop();
        assemble(&cfg, &spec(&c, &schema.json, "")).expect("one under builds")
    };
    assert_eq!(
        dry.command_line_units,
        ARGV_CAP_BYTES - 1,
        "one byte under the cap measures cap - 1"
    );
    assert_eq!(ARGV_CAP_BYTES, 32_767, "RT-1: the routed value");
    // The measure's rules, pinned on a hand-built vector (MSVCRT quoting as std applies it): the
    // program in quotes (8), then ` "a b"` (1 + 5), ` x\"y` (1 + 4: the quote is escaped, the
    // element is not wrapped because it carries no space), the terminating null (1).
    assert_eq!(
        command_line_units(Path::new("claude"), &["a b".to_owned(), "x\"y".to_owned()]),
        8 + 6 + 5 + 1,
        "the command-line measure is the Windows one (§7c.5; RT-1)"
    );

    match assemble(&cfg, &spec(&card, &schema.json, "")) {
        Err(ConfigError::ArgvAtOrOverCap { units, cap }) => {
            assert_eq!((units, cap), (ARGV_CAP_BYTES, ARGV_CAP_BYTES));
        }
        Err(other) => panic!("the refusal is the typed cap error; got {other:?}"),
        Ok(built) => {
            // The marker-count belt (§7c.18; Slice 5), made to measure the SAME vector as the
            // refusal above (the test-discrimination review of Slice 5, F5): the belt's program
            // is the fixture child's ABSOLUTE path, not `claude`, and `assemble` measures the
            // program in `command_line_units` — so the at-the-cap card is dozens of units OVER
            // under the belt's own program and a "refuse only when over" mutant would refuse it
            // at bind, count 0 for the wrong reason. The belt card is therefore re-trimmed so the
            // vector measures EXACTLY the cap under the fixture child, then dispatched once.
            // RED (belt half, observed 2026-09-18) against `assemble` mutated to refuse only one
            // unit OVER the cap (`units > ARGV_CAP_BYTES`): `belt: the at-the-cap vector bound
            // under the belt's own program and a marker file appeared (AC-523; RT-1)`: left `1`,
            // right `0`.
            let belt_cfg = Config::new(spawn_values(&Markers::new(), "exit:0")).expect("belt cfg");
            let belt_card = trimmed_to_cap(&card, &belt_cfg);
            assert_eq!(
                belt_dispatch_agent(&belt_card),
                0,
                "belt: the at-the-cap vector bound under the belt's own program and a marker file \
                 appeared (AC-523; RT-1)"
            );
            panic!(
                "the at-the-cap fixture must be refused before spawn (AC-523; RT-1); the builder produced {} elements measuring {}",
                built.argv.len(),
                built.command_line_units
            )
        }
    }
    // A weaker red refuses only one byte OVER: one over is refused too.
    let mut over = card.clone();
    over.prompt.push('x');
    assert!(matches!(
        assemble(&cfg, &spec(&over, &schema.json, "")),
        Err(ConfigError::ArgvAtOrOverCap { .. })
    ));
    // The user message is on stdin, not argv: it never moves the measure.
    let big = "y".repeat(100_000);
    let built =
        assemble(&cfg, &spec(&dry_card(&card), &schema.json, &big)).expect("stdin is not argv");
    assert_eq!(built.command_line_units, ARGV_CAP_BYTES - 1);
    assert_eq!(built.stdin.len(), card.prompt.len() - 1 + 2 + big.len());
}

/// `card` with its prompt shortened so the vector `assemble` builds measures EXACTLY
/// `ARGV_CAP_BYTES` under `cfg`'s program (F5: the belt's program is the fixture child's absolute
/// path, so the fixture that is at the cap under `claude` is over it here). The measure is read
/// off `assemble` itself — the refusal carries `units`, a build carries `command_line_units` —
/// and asserted at the cap after the trim.
fn trimmed_to_cap(card: &AgentCard, cfg: &Config) -> AgentCard {
    let sch = schema();
    let units = |c: &AgentCard| match assemble(cfg, &spec(c, &sch.json, "")) {
        Ok(b) => b.command_line_units,
        Err(ConfigError::ArgvAtOrOverCap { units, .. }) => units,
        Err(other) => panic!("the at-the-cap card fails only the cap: {other:?}"),
    };
    let mut c = card.clone();
    let over = units(&c);
    assert!(
        over >= ARGV_CAP_BYTES,
        "the belt's program is longer than `claude`: {over} units"
    );
    for _ in 0..(over - ARGV_CAP_BYTES) {
        c.prompt.pop();
    }
    assert_eq!(
        units(&c),
        ARGV_CAP_BYTES,
        "the belt card measures exactly the cap under the belt's own program"
    );
    c
}

fn dry_card(card: &AgentCard) -> AgentCard {
    let mut c = card.clone();
    c.prompt.pop();
    c
}

/// AC-205 / AC-417 (card-validator half): *"a fixture card lacking `StructuredOutput` that must
/// fail"* — refused at configuration time, on the argv the builder assembles (ADR-0108: F20's exit
/// 0 is why this cannot be a runtime check).
///
/// RED (observed 2026-09-18) against the lint-free skeleton: `a card whose tools omit
/// StructuredOutput must be refused before spawn (AC-205)`.
#[test]
fn structured_output_required() {
    let cfg = config();
    let schema = schema();
    let red =
        crate::support::red_card("no-structured-output").expect("the card loads at the card level");
    let card = AgentCard {
        name: red.name.clone(),
        description: red.description.clone(),
        prompt: red.prompt.clone(),
        tools: red.tools.clone(),
        model: "haiku".to_owned(),
        effort: None,
        permission_mode: red.permission_mode.clone(),
        cache_profile: red.cache_profile,
    };
    assert!(
        !card.tools.iter().any(|t| t == "StructuredOutput"),
        "PREMISE: the red card omits it"
    );
    match assemble(&cfg, &spec(&card, &schema.json, "")) {
        Err(ConfigError::StructuredOutputAbsent { card: named }) => {
            assert_eq!(named, "no-structured-output");
        }
        Err(other) => {
            panic!("the refusal is StructuredOutputAbsent naming the card; got {other:?}")
        }
        Ok(_) => {
            // The marker-count belt (§7c.18; Slice 5): F20 is why a runtime `Schema` error is the
            // weaker red — the card must never reach a spawn.
            // RED (belt half, observed 2026-09-18) against `assemble` mutated to warn instead of
            // refuse: `belt: a card without StructuredOutput bound and a marker file appeared
            // (AC-205; dispatch count must be 0)`: left `1`, right `0`.
            assert_eq!(
                belt_dispatch_agent(&card),
                0,
                "belt: a card without StructuredOutput bound and a marker file appeared (AC-205; \
                 dispatch count must be 0)"
            );
            panic!("a card whose tools omit StructuredOutput must be refused before spawn (AC-205)")
        }
    }
    // The tool set is the card's declared list and nothing else (ADR-0108 d2): a card that carries
    // it builds, and the `--agents` JSON carries exactly the declared list.
    let good = bound("golden-static", Tier::Haiku, None);
    let built = assemble(&cfg, &spec(&good, &schema.json, "")).expect("builds");
    let agents: serde_json::Value =
        serde_json::from_str(&built.argv[7]).expect("the --agents element is JSON");
    assert_eq!(
        agents["golden-static"]["tools"],
        serde_json::json!(["Read", "StructuredOutput"])
    );
}

/// AC-203: *"a dispatch specification carrying `--agents` with no `--agent <name>` … refused as a
/// configuration error before any process is spawned, because `--agents` is inert alone"* (F7).
///
/// RED (observed 2026-09-18) against the skeleton that emitted `--agents` without `--agent`:
/// `--agents without --agent must be refused (AC-203; F7)`.
#[test]
fn ac203_agents_without_agent_refused() {
    let cfg = config();
    let schema = schema();
    let card = bound("golden-static", Tier::Haiku, None);
    let define_only = CommandSpec {
        adopt_agent: false,
        ..spec(&card, &schema.json, "")
    };
    match assemble(&cfg, &define_only) {
        Err(ConfigError::AgentsWithoutAgent { card: named }) => assert_eq!(named, "golden-static"),
        Err(other) => panic!("the refusal is AgentsWithoutAgent; got {other:?}"),
        Ok(built) => {
            // The marker-count belt (§7c.18; Slice 5): an implementation that lets the define-only
            // form through binds and spawns; the marker file is the red.
            // RED (belt half, observed 2026-09-18) against `assemble` mutated to emit `--agents`
            // without `--agent`: `belt: --agents without --agent bound and a marker file appeared
            // (AC-203)`: left `1`, right `0`.
            assert_eq!(
                belt_dispatch_agent(&card),
                0,
                "belt: --agents without --agent bound and a marker file appeared (AC-203)"
            );
            panic!(
                "--agents without --agent must be refused (AC-203; F7); built {:?}",
                built.argv
            )
        }
    }
    // The adopting form carries both, `--agent` directly after the `--agents` pair.
    let built = assemble(&cfg, &spec(&card, &schema.json, "")).expect("builds");
    let i = built
        .argv
        .iter()
        .position(|a| a == "--agents")
        .expect("--agents");
    assert_eq!(built.argv[i + 2], "--agent");
    assert_eq!(built.argv[i + 3], "golden-static");
}

/// AC-214 (unit half; the lint half is ARMED — §7c.5): *"`--permission-mode` is present with an
/// explicit value taken from the card or pack; there is no implicit default"* — a silent card is
/// refused at load, and every built argv carries the flag with the card's value.
///
/// RED (observed 2026-09-18) against the skeleton that defaulted a silent card to `default`:
/// `a card silent on permission_mode must be refused (AC-214: no implicit default)`.
#[test]
fn ac214_permission_mode_explicit() {
    match crate::support::red_card("no-permission-mode") {
        Err(ConfigError::PermissionModeAbsent { card }) => assert_eq!(card, "no-permission-mode"),
        Err(other) => panic!("the refusal is PermissionModeAbsent; got {other:?}"),
        Ok(card) => {
            // The marker-count belt (§7c.18; Slice 5): a silent card that loads binds and spawns
            // with a defaulted mode; the marker file is the red.
            // RED (belt half, observed 2026-09-18) against `AuthoredCard::from_file` mutated to
            // default a silent card to `default`: `belt: a card silent on permission_mode loaded
            // and a marker file appeared (AC-214)`: left `1`, right `0`.
            assert_eq!(
                belt_dispatch_authored(&card, Tier::Haiku, None),
                0,
                "belt: a card silent on permission_mode loaded and a marker file appeared (AC-214)"
            );
            panic!(
                "a card silent on permission_mode must be refused (AC-214: no implicit default); loaded with {:?}",
                card.permission_mode
            )
        }
    }
    let cfg = config();
    let schema = schema();
    for (stem, mode) in [
        ("golden-static", "default"),
        ("reviewer", "bypassPermissions"),
        ("probe", "default"),
    ] {
        let card = bound(stem, Tier::Sonnet, Some(Effort::Low));
        let built = assemble(&cfg, &spec(&card, &schema.json, "")).expect("builds");
        let i = built
            .argv
            .iter()
            .position(|a| a == "--permission-mode")
            .expect("the flag is unconditional");
        assert_eq!(built.argv[i + 1], mode, "{stem}: the value is the card's");
        assert_eq!(
            built
                .argv
                .iter()
                .filter(|a| *a == "--permission-mode")
                .count(),
            1
        );
    }
}

/// AC-215 (argv half): `--permission-prompts none` on every argv, unconditionally; the stdin
/// shutdown half is Slice 5's (`ac215_permission_prompts_none_stdin_closed`).
#[test]
fn permission_prompts_none_on_every_argv() {
    let cfg = config();
    let schema = schema();
    for stem in ["golden-static", "golden-default", "reviewer", "probe"] {
        let card = bound(stem, Tier::Haiku, None);
        let built = assemble(&cfg, &spec(&card, &schema.json, "")).expect("builds");
        let i = built
            .argv
            .iter()
            .position(|a| a == "--permission-prompts")
            .expect("present");
        assert_eq!(built.argv[i + 1], "none");
    }
}

/// The closed set of flags the builder may emit (§7c.5's ONE order): every element of a built argv
/// that starts with `-` is one of these, whatever spelled it — so a flag assembled by
/// concatenation, or pushed on a Config axis, fails the property without being named.
const KNOWN_FLAGS: &[&str] = &[
    "-p",
    "--output-format",
    "--verbose",
    "--model",
    "--effort",
    "--agents",
    "--agent",
    "--json-schema",
    "--permission-mode",
    "--strict-mcp-config",
    "--permission-prompts",
    "--system-prompt",
];

/// One point of the configuration axis: the values a `Config` is built from inside the property.
#[derive(Debug, Clone)]
struct ConfigPoint {
    captured_env: Vec<(String, String)>,
    ttl: Option<String>,
    harness_binary: PathBuf,
    on_version_mismatch: MismatchPolicy,
}

/// The configuration matrix AC-216's property test walks — *"under any card, pack or
/// configuration"*: every tier, effort or none, both profiles, a free permission mode (the CLI's
/// four and any other spelling a card may carry), a tool list that carries StructuredOutput, a
/// prompt and a user message — AND the `Config` axis: both mismatch policies, a captured map of
/// random non-denylisted names (`CLAUDE_CONFIG_DIR` among the candidates), a configured TTL pair
/// or the record's, and a harness path — every argv the builder CAN produce, from every `Config`
/// it can be handed.
fn matrix() -> impl Strategy<
    Value = (
        Tier,
        Option<Effort>,
        CacheProfile,
        String,
        Vec<String>,
        String,
        String,
        ConfigPoint,
    ),
> {
    let tier = prop_oneof![Just(Tier::Haiku), Just(Tier::Sonnet), Just(Tier::Opus)];
    let effort = prop_oneof![
        Just(None),
        Just(Some(Effort::Low)),
        Just(Some(Effort::Medium)),
        Just(Some(Effort::High)),
        Just(Some(Effort::Xhigh)),
        Just(Some(Effort::Max)),
    ];
    let profile = prop_oneof![Just(CacheProfile::Static), Just(CacheProfile::Default)];
    let mode = prop_oneof![
        Just("default".to_owned()),
        Just("acceptEdits".to_owned()),
        Just("bypassPermissions".to_owned()),
        Just("plan".to_owned()),
        Just("dontAsk".to_owned()),
        "[A-Za-z]{1,20}".prop_map(|s| s),
    ];
    let tools = proptest::collection::vec("[A-Za-z_]{1,12}", 0..6).prop_map(|mut v| {
        // A random name may spell the answer path (`answer`, `mint`, `resolve` as a segment) and
        // the card lint would refuse it before the argv exists — that is AC-269's, not AC-216's.
        v.retain(|t| !rk_adapter_claude::card::exposes_answer_api(t));
        v.push("StructuredOutput".to_owned());
        v
    });
    let prompt = "[ -~]{0,200}";
    let user = "[ -~\\n]{0,200}";
    let env_key = prop_oneof![
        3 => "[A-Z_]{1,16}".prop_map(|s| s),
        1 => Just("CLAUDE_CONFIG_DIR".to_owned()),
        1 => Just("CLAUDE_CODE_MAX_OUTPUT_TOKENS".to_owned()),
        1 => Just("ANTHROPIC_MODEL".to_owned()),
    ]
    .prop_filter("a denylisted name never becomes a Config (AC-217)", |k| {
        !is_denylisted(k)
    });
    let captured_env = proptest::collection::vec((env_key, "[ -~]{0,24}"), 0..6);
    let ttl = prop_oneof![
        Just(None),
        Just(Some("1h".to_owned())),
        Just(Some("5m".to_owned()))
    ];
    let harness = prop_oneof![
        Just(PathBuf::from("claude")),
        Just(PathBuf::from("claude.exe")),
        Just(PathBuf::from("C:/Users/x/.local/bin/claude.exe")),
        Just(PathBuf::from("/usr/local/bin/claude")),
    ];
    let policy = prop_oneof![Just(MismatchPolicy::Refuse), Just(MismatchPolicy::Degraded)];
    let point = (captured_env, ttl, harness, policy).prop_map(
        |(captured_env, ttl, harness_binary, on_version_mismatch)| ConfigPoint {
            captured_env,
            ttl,
            harness_binary,
            on_version_mismatch,
        },
    );
    (tier, effort, profile, mode, tools, prompt, user, point)
}

/// The `Config` a matrix point names, built through `Config::new` (its lints pass by
/// construction of the point: no denylisted name, a TTL pair or neither).
fn config_at(point: &ConfigPoint) -> Config {
    let mut v = values();
    v.captured_env = point.captured_env.iter().cloned().collect();
    v.prompt_cache_ttl = point.ttl.clone();
    v.subagent_prompt_cache_ttl = point.ttl.clone();
    v.harness_binary = point.harness_binary.clone();
    v.on_version_mismatch = point.on_version_mismatch;
    Config::new(v).expect("a matrix point's Config constructs")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// AC-216 (twin of owner AC-511): *"a property test asserting the token is absent from every
    /// argv the builder can produce across the configuration matrix (the matrix, not a grep, is the
    /// instrument — a plain lint over today's source passes vacuously)"*.
    ///
    /// RED (observed 2026-09-18) against a mutant builder that pushed the flag on one matrix point
    /// (`effort == "max"`): `Test failed: --bare on argv (AC-216): ["-p", …, "--effort", "max",
    /// "--bare", "--agents", …]`, the minimal failing input proptest shrank to
    /// `(Sonnet, Some(Max), Static, "default", ["StructuredOutput"], "", "")`. RED again
    /// (observed 2026-09-18, the review's three mutants, each against the matrix WITH the Config
    /// axis and the free mode): M10 — the flag spelt `"--ba".to_owned() + "re"` on
    /// `MismatchPolicy::Degraded` — `Test failed: --bare on argv (AC-216): ["-p", …, "--permission-
    /// prompts", "none", "--bare"]`; M11 — on `captured_env` carrying `CLAUDE_CONFIG_DIR` — the
    /// same; M12 — on `permission_mode == "dontAsk"` — the same. (The matrix without the Config
    /// axis and with four literal modes was green on all three.) The closed-set assertion below is
    /// the belt for a spelling the two `--bare` assertions do not name.
    #[test]
    fn ac216_bare_absent_across_matrix((tier, effort, profile, mode, tools, prompt, user, point) in matrix()) {
        let cfg = config_at(&point);
        let schema = schema();
        let effort = if tier.accepts_effort() { effort } else { None };
        let card = AgentCard {
            name: "m".to_owned(),
            description: "matrix".to_owned(),
            prompt,
            tools,
            model: tier.as_str().to_owned(),
            effort: effort.map(|e| e.as_str().to_owned()),
            permission_mode: mode,
            cache_profile: profile,
        };
        let built: Assembled = assemble(&cfg, &spec(&card, &schema.json, &user)).expect("every matrix point builds");
        prop_assert!(!built.argv.iter().any(|a| a == "--bare"), "--bare on argv (AC-216): {:?}", built.argv);
        prop_assert!(!built.argv.iter().any(|a| a.starts_with("--bare")), "no --bare-prefixed element either");
        // And the two append flags, on every point (AC-211's ban, as a property).
        prop_assert!(!built.argv.iter().any(|a| a == "--append-system-prompt" || a == "--exclude-dynamic-system-prompt-sections"));
        // The closed set: walking the vector flag by flag (each value-taking flag's value is
        // skipped — the card body under `static` may start with `-`), every element in FLAG
        // position is one of `KNOWN_FLAGS`. An unknown flag fails whatever spelled it, and an
        // element inserted anywhere shifts a value into flag position and fails there.
        let mut i = 0;
        while i < built.argv.len() {
            let a = &built.argv[i];
            prop_assert!(KNOWN_FLAGS.contains(&a.as_str()), "an element outside the closed flag set (AC-216; §7c.5's ONE order): {a:?} in {:?}", built.argv);
            // `-p`, `--verbose` and `--strict-mcp-config` take no value; every other flag takes one.
            i += if a == "-p" || a == "--verbose" || a == "--strict-mcp-config" { 1 } else { 2 };
        }
    }
}

/// AC-511 (the lint half, with its control): *"a source lint over argument-vector construction …
/// a fixture code path that adds the flag, which must fail the lint naming the call site"*. The lint
/// is the block's `rk_absent '"--bare"'` in its own shape — every non-comment line of `src/` — and
/// its control is `tests/fixtures/lint-controls/ac511_bare_flag_control.rs`, which the lint must NAME
/// by file and line; red by the control not being named (a lint that matches nothing).
///
/// RED (observed 2026-09-18) against a mutant control whose `push("--bare")` sat on a `//` line
/// (the token the ban's comment filter drops): `the lint names no call site in the control file —
/// the lint is vacuous (AC-511)`; and, with the control file absent, on the read.
#[test]
fn ac511_bare_never_in_argv_lint_with_control() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lint = |path: &Path| -> Vec<String> {
        let text = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        text.lines()
            .enumerate()
            .filter(|(_, l)| {
                let t = l.trim_start();
                !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
            })
            .filter(|(_, l)| l.contains("\"--bare\""))
            .map(|(i, l)| format!("{}:{}: {}", path.display(), i + 1, l.trim()))
            .collect()
    };
    // Over src/, RECURSIVELY (the block's `grep -rnE`; §7c.1 leaves a `src/<module>/mod.rs`
    // layout to the implementer, and the test lints the same files the block does): nothing.
    let files = rs_files(&root.join("src"));
    assert!(
        files.len() >= 7,
        "PREMISE: the walk found the crate's source files (got {}: {files:?}) — an empty walk is never green",
        files.len()
    );
    let mut src_hits = Vec::new();
    for p in &files {
        src_hits.extend(lint(p));
    }
    assert!(
        src_hits.is_empty(),
        "--bare in an argv construction site (AC-511): {src_hits:?}"
    );
    // Over the control: the call site is NAMED.
    let control = root.join("tests/fixtures/lint-controls/ac511_bare_flag_control.rs");
    let named = lint(&control);
    assert!(
        !named.is_empty(),
        "the lint names no call site in the control file — the lint is vacuous (AC-511)"
    );
    assert!(
        named[0].contains("ac511_bare_flag_control.rs:4:"),
        "named by file and line: {named:?}"
    );
}

/// Every `.rs` file under `dir`, at any depth (a small stack over `read_dir`), sorted.
fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).unwrap_or_else(|e| panic!("read {}: {e}", d.display())) {
            let p = entry.expect("entry").path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// AC-209 (static profile): *"the `--system-prompt` bytes are byte-identical between [two
/// dispatches of the same card version with different volatile inputs]"* — a golden test hashing
/// the assembled system prompt across a matrix of volatile inputs (all hashes equal).
///
/// RED (observed 2026-09-18) against a mutant builder that appended the first four characters of
/// the user message to the `--system-prompt` element: `assertion left == right failed: the
/// --system-prompt bytes are invariant across volatile inputs (AC-209): left: 4 right: 1`. (A
/// mutant that appended the whole message went red on the cap instead — the 50,000-byte input
/// pushed the argv over it — which is the same fact by another assertion.)
#[test]
fn ac209_system_prompt_hash_invariant() {
    let cfg = config();
    let schema = schema();
    let card = bound("golden-static", Tier::Haiku, None);
    let volatile: &[&str] = &[
        "",
        "Review the diff.",
        "Review the diff for issue-81 in D:\\rk\\worktrees\\run-1 at 2026-09-18T10:00:00Z",
        "attempt 2 of 3; previous verdict: changes-requested",
        &"z".repeat(50_000),
    ];
    let hashes: BTreeSet<String> = volatile
        .iter()
        .map(|user| {
            let built = assemble(&cfg, &spec(&card, &schema.json, user)).expect("builds");
            let i = built
                .argv
                .iter()
                .position(|a| a == "--system-prompt")
                .expect("static carries it");
            sha256(built.argv[i + 1].as_bytes()).as_str().to_owned()
        })
        .collect();
    assert_eq!(
        hashes.len(),
        1,
        "the --system-prompt bytes are invariant across volatile inputs (AC-209)"
    );
    assert_eq!(
        hashes.iter().next().expect("one"),
        sha256(card.prompt.as_bytes()).as_str(),
        "and they are the card body's bytes, exactly"
    );
    // The whole argv is invariant too: nothing volatile reaches any element.
    let argvs: BTreeSet<Vec<String>> = volatile
        .iter()
        .map(|user| {
            assemble(&cfg, &spec(&card, &schema.json, user))
                .expect("builds")
                .argv
        })
        .collect();
    assert_eq!(argvs.len(), 1, "no volatile input reaches the argv");
}

/// AC-132 (builder-lint half): the precondition *"one output schema is sent per dispatch and no
/// strict user-defined tool is on the adapter's command line"* is linted on the argv — *"a fixture
/// that adds a strict tool and must re-open the budget explicitly"* is refused, and exactly one
/// `--json-schema` element is ever emitted.
///
/// RED (observed 2026-09-18) against the lint-free skeleton: `a strict tool on the command line
/// must be refused — the 24-optional budget is per request (AC-132; ADR-0108 d4)`.
#[test]
fn ac132_builder_lints_one_schema_no_strict_tools() {
    let cfg = config();
    let schema = schema();
    let card = bound("golden-static", Tier::Haiku, None);
    let strict = vec!["lookup_ticket".to_owned()];
    let with_strict = CommandSpec {
        strict_tools: &strict,
        ..spec(&card, &schema.json, "")
    };
    match assemble(&cfg, &with_strict) {
        Err(ConfigError::StrictToolOnArgv { tool }) => assert_eq!(tool, "lookup_ticket"),
        Err(other) => panic!("the refusal names the strict tool; got {other:?}"),
        Ok(_) => panic!(
            "a strict tool on the command line must be refused — the 24-optional budget is per request (AC-132; ADR-0108 d4)"
        ),
    }
    let built = assemble(&cfg, &spec(&card, &schema.json, "")).expect("builds");
    assert_eq!(
        built.argv.iter().filter(|a| *a == "--json-schema").count(),
        1,
        "exactly one output schema per dispatch"
    );
    assert!(
        !built
            .argv
            .iter()
            .any(|a| a == "--tools" || a == "--strict-tools" || a == "--mcp-config")
    );
}
