//! **Isolation, adoption and the observed version — decided from the child's own `system/init`**
//! — AC-202, AC-204, AC-386, AC-226, AC-385 (s7c §7c.4, §7c.9, §7c.18). Every init here is
//! SCRIPTED (§7c.17: the pin retains no raw init; the first is the probe's) and replayed by the
//! fixture child through the real `dispatch`; the isolation test never reads the adapter's own
//! command line (AC-202's lint half is the block's ban over `init.rs`).

use rk_ports::fakes::VecStore;
use rk_ports::harness::Harness;
use rk_types::error::{IsolationSurface, RetryClass, StepError, TransportKind};
use rk_types::usage::CliVersion;

use rk_adapter_claude::config::{Config, MismatchPolicy, TESTED_CLI_VERSION};
use rk_adapter_claude::record::mismatch_warning;

use crate::support::{
    Markers, Part, bind_probe, completed_rows, dispatch_replay, harness, spawn_values, transcript,
};

fn with_init(init: &str) -> crate::support::Transcript {
    transcript(&[
        Part::File(&format!("streams/{init}")),
        Part::File("streams/result-ok.scripted.ndjson"),
    ])
}

/// AC-202: *"isolation is decided from the child's own reported tool/server list, and a list that
/// does not match the declared configuration fails the dispatch as `NotIsolated`"* — a scripted
/// init reporting 280 tools while argv carries the flag must fail, AND a scripted init with the
/// card's tool COUNT and one substituted name must fail (the same-count fixture, §7c.17 — a
/// count-based implementation is green on the first and red on the second); the instrument is a
/// SET DIGEST (SM-061). A non-empty `mcp_servers` list is `NotIsolated { surface: McpServers }`.
///
/// RED (observed 2026-09-18) against the skeleton: `280 tools while argv carries the flag must be
/// NotIsolated: got Err(Transport { kind: Spawn, … })`.
#[test]
fn ac202_isolation_from_system_init_not_argv() {
    let r = dispatch_replay(&with_init("init-280-tools.scripted.ndjson"), 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::NotIsolated {
                surface: IsolationSurface::Tools,
                ..
            })
        ),
        "280 tools while argv carries the flag must be NotIsolated: got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(
        &with_init("init-same-count-substituted.scripted.ndjson"),
        30,
    );
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest,
        }) => {
            // the digest is over the symmetric difference {Bash, StructuredOutput}, sorted
            let want = rk_adapter_claude::init::set_digest(
                &std::iter::once("Bash".to_owned()).collect(),
                &std::iter::once("StructuredOutput".to_owned()).collect(),
            );
            assert_eq!(
                diff_digest, &want,
                "a SET digest, not a count (SM-061; PIA-016)"
            );
        }
        other => panic!(
            "the card's tool COUNT with one substituted name must be NotIsolated — a count-based \
             check is green here (AC-202); got {other:?}"
        ),
    }
    let r = dispatch_replay(&with_init("init-mcp-servers.scripted.ndjson"), 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::NotIsolated {
                surface: IsolationSurface::McpServers,
                ..
            })
        ),
        "a reported MCP server under --strict-mcp-config is NotIsolated {{ surface: mcp_servers }}: \
         {:?}",
        r.outcome.result
    );
    // and the good init proceeds — the flag in argv was never the evidence, the init was
    let r = dispatch_replay(&with_init("init.scripted.ndjson"), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
}

/// AC-204: *"adoption holds only if that set is EQUAL to the card's declared tool set, and any
/// difference — extra or missing — fails the dispatch as `NotIsolated { surface: tools }`; a
/// superset is not adoption. `NotAdopted` is reserved for the wrong agent answering"* — four
/// cases: equal / subset / superset / a different agent.
///
/// RED (observed 2026-09-18) against the skeleton: `equal: expected Ok, got Err(Transport { kind:
/// Spawn, … })`.
/// **AC-204 at CLI 2.1.277 — the composite reading (RT-27; the operator's decision of 2026-09-18).**
/// The first live run's retained raw `system/init` carried NO `agent` key: at 2.1.277 the init
/// reports `agents` (every loadable agent, the `--agents` entry among them), `model` as the full
/// id, and `tools`. Adoption when `agent` is absent is therefore read as: `agents` contains the
/// expected name AND `model` belongs to the bound card's tier AND the tool set equals the declared
/// allowlist (step 3's own check). When `agent` IS present it must equal the expected name, as
/// before. RED before the reading: the good 2.1.277 init was `NotAdopted { observed: "<no agent
/// reported>" }` — the refusal that killed the probe at the init line.
///
/// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.3.)* "belongs to the
/// bound card's tier" is now EQUALITY with the requested full id on both readings (the segment match is
/// deleted); the three fixtures' verdicts do not move.
#[test]
fn ac204_adoption_at_2_1_277_is_read_from_agents_and_model() {
    let r = dispatch_replay(&with_init("init-2-1-277-shape.scripted.ndjson"), 30);
    assert!(
        r.outcome.result.is_ok(),
        "the 2.1.277 init (agents carries probe, model is the haiku id, tools equal): expected Ok, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(
        &with_init("init-2-1-277-agents-without-probe.scripted.ndjson"),
        30,
    );
    assert!(
        matches!(r.outcome.result, Err(StepError::NotAdopted { .. })),
        "agents without probe: expected NotAdopted, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init-2-1-277-wrong-model.scripted.ndjson"), 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::NotAdopted { .. })),
        "agents with probe but a sonnet model: the tier was not applied — expected NotAdopted, got {:?}",
        r.outcome.result
    );
}

#[test]
fn ac204_adoption_equal_subset_superset_wrong_agent() {
    let r = dispatch_replay(&with_init("init.scripted.ndjson"), 30);
    assert!(
        r.outcome.result.is_ok(),
        "equal: expected Ok, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init-subset.scripted.ndjson"), 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::NotIsolated {
                surface: IsolationSurface::Tools,
                ..
            })
        ),
        "subset: a SUBSET accepted is the red — expected NotIsolated {{ tools }}, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init-superset.scripted.ndjson"), 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::NotIsolated {
                surface: IsolationSurface::Tools,
                ..
            })
        ),
        "superset: a superset is not adoption — expected NotIsolated {{ tools }}, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init-wrong-agent.scripted.ndjson"), 30);
    match &r.outcome.result {
        Err(StepError::NotAdopted {
            expected_agent,
            observed,
        }) => {
            assert_eq!(expected_agent, "probe");
            assert_eq!(observed, "general-purpose");
        }
        other => panic!(
            "a different agent answering is NotAdopted {{ expected_agent, observed }}, never \
             NotIsolated (AC-204): got {other:?}"
        ),
    }
    let r = dispatch_replay(&with_init("init-no-agent.scripted.ndjson"), 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::NotAdopted { .. })),
        "no agent reported is NotAdopted: {:?}",
        r.outcome.result
    );
}

/// AC-386 (owner): *"a tool set that is not equal to the card's effective allowlist / THEN the
/// dispatch fails with `NotIsolated` and is never retried"* — extra tools classified `Ok` or
/// `NotAdopted` is the adapter's mutation; the zero-retry half reads
/// `rk_types::StepError::retry_class()` (Step 5's; no adapter mutation moves it).
///
/// RED (observed 2026-09-18) against the skeleton: `extra tools must be NotIsolated: got
/// Err(Transport { kind: Spawn, … })`.
#[test]
fn ac386_extra_tools_not_isolated_no_retry() {
    let r = dispatch_replay(&with_init("init-superset.scripted.ndjson"), 30);
    let Err(err) = &r.outcome.result else {
        panic!(
            "extra tools must be NotIsolated: got {:?}",
            r.outcome.result
        );
    };
    assert!(
        matches!(
            err,
            StepError::NotIsolated {
                surface: IsolationSurface::Tools,
                ..
            }
        ),
        "extra tools must be NotIsolated: got {err:?}"
    );
    assert_eq!(
        err.retry_class(),
        RetryClass::Never,
        "zero retry attempts: NotIsolated is Never in rk-types' table (ADR-0095)"
    );
    assert_eq!(
        r.markers.count(),
        1,
        "exactly one spawn — no retry of the adapter's own"
    );
}

fn config_with_policy(markers: &Markers, mode: &str, policy: MismatchPolicy) -> Config {
    let mut v = spawn_values(markers, mode);
    v.on_version_mismatch = policy;
    Config::new(v).expect("constructs")
}

/// AC-226: *"the configured outcome for a mismatch fires — either refusal, or a recorded
/// `Degraded`-style allowance … the test asserts the CONFIGURED one"* — a scripted init at a
/// higher and a lower version, run under both configurations, each asserting its own outcome
/// (four assertions): under `Refuse`, `Err(Transport { kind: Decode })` after the child is killed
/// through the group, the row carrying `Observed(v)` (RT-20 as amended); under `Degraded`, `Ok`
/// with the row carrying `Observed(v)` beside the pin and the accessor `Some` (AC-385).
///
/// RED (observed 2026-09-18) against the skeleton: `Refuse × higher: expected Transport { kind:
/// Decode }, got Err(Transport { kind: Spawn, … })`.
///
/// *(dated 2026-10-19, s8j build, the pin move to 2.1.303; D21-164)* The higher init moved
/// `2.1.300` → `2.1.400`: at the new pin `2.1.300` is lower, and AC-226 wants one of each.
#[test]
fn ac226_version_mismatch_configured_outcome() {
    for (init, observed) in [
        ("init-version-higher.scripted.ndjson", "2.1.400"),
        ("init-version-lower.scripted.ndjson", "2.1.259"),
    ] {
        let t = with_init(init);
        // Refuse
        let markers = Markers::new();
        let wt = tempfile::tempdir().expect("wt");
        let config = config_with_policy(
            &markers,
            &format!("replay:{}", t.path.display()),
            MismatchPolicy::Refuse,
        );
        let (config, spec) = bind_probe(config, 30, wt.path(), "x");
        let mut store = VecStore::default();
        let o = harness(config).dispatch(&spec, &mut store);
        assert!(
            matches!(
                o.result,
                Err(StepError::Transport {
                    kind: TransportKind::Decode,
                    ..
                })
            ),
            "Refuse × {observed}: expected Transport {{ kind: Decode }}, got {:?}",
            o.result
        );
        assert_eq!(
            o.record.cli_version,
            CliVersion::Observed(observed.to_owned()),
            "the refusal row carries Observed(v)"
        );
        assert_eq!(completed_rows(&store).len(), 1);
        // Degraded
        let markers = Markers::new();
        let wt = tempfile::tempdir().expect("wt");
        let config = config_with_policy(
            &markers,
            &format!("replay:{}", t.path.display()),
            MismatchPolicy::Degraded,
        );
        let (config, spec) = bind_probe(config, 30, wt.path(), "x");
        let mut store = VecStore::default();
        let o = harness(config).dispatch(&spec, &mut store);
        assert!(
            o.result.is_ok(),
            "Degraded × {observed}: the opted-in allowance proceeds to Ok, got {:?}",
            o.result
        );
        assert_eq!(
            o.record.cli_version,
            CliVersion::Observed(observed.to_owned())
        );
        assert_eq!(o.record.instrument_version, TESTED_CLI_VERSION);
        let w = mismatch_warning(&o.record).expect("the allowance is recorded as a warning");
        assert_eq!(w.observed, observed);
        assert_eq!(w.pinned, TESTED_CLI_VERSION);
    }
    // the pin itself, under Refuse: no mismatch, no refusal
    let r = dispatch_replay(&with_init("init.scripted.ndjson"), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(mismatch_warning(&r.outcome.record), None);
}

/// AC-385: *"the observed CLI version is recorded on the event, and a mismatch against
/// `TESTED_CLI_VERSION` is surfaced as a typed warning"* — the row carries `Observed(v)` under a
/// scripted foreign version AND the accessor `mismatch_warning(&record)` reads `Some`; red by the
/// accessor returning `None` while `v != instrument_version`, or by a pre-spawn row written
/// `Observed`.
///
/// RED (observed 2026-09-18) against the skeleton (the accessor derives nothing): `the row lacks
/// Observed(2.1.300): Pinned("2.1.278")`.
#[test]
fn ac385_version_on_event_and_mismatch_warning() {
    let t = with_init("init-version-higher.scripted.ndjson");
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let config = config_with_policy(
        &markers,
        &format!("replay:{}", t.path.display()),
        MismatchPolicy::Degraded,
    );
    let (config, spec) = bind_probe(config, 30, wt.path(), "x");
    let mut store = VecStore::default();
    let o = harness(config).dispatch(&spec, &mut store);
    assert_eq!(
        o.record.cli_version,
        CliVersion::Observed("2.1.400".to_owned()),
        "the row lacks Observed(2.1.400): {:?}",
        o.record.cli_version
    );
    let w = mismatch_warning(&o.record);
    assert_eq!(
        w,
        Some(rk_adapter_claude::record::VersionMismatch {
            observed: "2.1.400".to_owned(),
            pinned: TESTED_CLI_VERSION.to_owned(),
        }),
        "mismatch_warning is Some exactly when Observed(v) != instrument_version (AC-385)"
    );
    // a pre-spawn row is Pinned and warns about nothing
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let mut v = spawn_values(&markers, "exit:0");
    v.harness_binary = markers.path().join("absent.exe");
    let (config, spec) = bind_probe(Config::new(v).expect("constructs"), 30, wt.path(), "x");
    let mut store = VecStore::default();
    let o = harness(config).dispatch(&spec, &mut store);
    assert_eq!(
        o.record.cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
    );
    assert_eq!(
        mismatch_warning(&o.record),
        None,
        "no measurement, no warning"
    );
    // an init with NO version key: nothing measured, Pinned, no warning, no refusal
    let r = dispatch_replay(&with_init("init-no-version.scripted.ndjson"), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned()),
        "an init that names no version under any key measured nothing: Pinned (init.rs)"
    );
}

/// AC-202's MCP surface with NO `mcp_servers` / `mcpServers` key on the init (S5-08): the absent
/// list is the EMPTY set, which is EQUAL to the empty set `--strict-mcp-config` declares, so the
/// dispatch proceeds — the same rule as the tool surface (an absent `tools` key is the empty set,
/// which is UNEQUAL to any card's non-empty list and is refused); `init.rs`'s module doc states
/// the asymmetry. A refusal here would carry a digest over two empty sets — a row saying "not
/// isolated, differing in nothing".
#[test]
fn ac202_absent_mcp_key_is_the_empty_set() {
    let r = dispatch_replay(&with_init("init-no-mcp-key.scripted.ndjson"), 30);
    assert!(
        r.outcome.result.is_ok(),
        "an init with no MCP key reports the empty set, which equals the declared empty set: {:?}",
        r.outcome.result
    );
    assert_eq!(completed_rows(&r.store).len(), 1);
}

// ------------------------------------------------------------------------------------------------
// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* `s8h` §8h.5's init equality
// — three comparisons beside the tool-set and MCP checks, each read from the child's own report and
// never from the argv: `permissionMode` equals the CARD's mode (D21-48 X12: *"the init comparison
// reads `default`"*), `model` equals the requested FULL id (§8h.3; RT-27 settled by equality), and
// `apiKeySource` equals `"none"` (ADR-0010 checked in-band). N17's Q1 classes (§8h.0): a model
// mismatch is `NotAdopted`; a mode or `apiKeySource` mismatch, or an absent key, is
// `NotIsolated { surface: Env }`, with the digest over the reported value's one-member set (empty
// when absent) against the expected one.

fn one(s: &str) -> std::collections::BTreeSet<String> {
    std::iter::once(s.to_owned()).collect()
}

/// §8h.5 / N17 Q1 (iii): an init reporting `permissionMode` `acceptEdits` for a `default` card is
/// `NotIsolated { surface: Env }`, the digest over `{acceptEdits}` against `{default}`. And the
/// comparison reads the CARD's mode, never the argv's spelling (D21-48 X12): a `Config` pinned at
/// ~~`2.1.287`~~ `2.1.303` puts `--permission-mode manual` on the argv (§8h.2's row), and an init
/// reporting `default` for it is adopted. *(dated 2026-10-19, s8j build, the pin move to 2.1.303;
/// D21-164)* The `2.1.287` row was deleted by the pin move, so the arm and its fixture
/// (`init-2-1-303-default`, renamed from `init-2-1-287-default`) are at the new pin's one row.
#[test]
fn permission_mode_reported_unequal_to_the_card_is_refused() {
    let r = dispatch_replay(
        &with_init("init-permission-mode-unequal.scripted.ndjson"),
        30,
    );
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Env,
            diff_digest,
        }) => assert_eq!(
            diff_digest,
            &rk_adapter_claude::init::set_digest(&one("acceptEdits"), &one("default")),
            "the digest is over the reported mode against the card's (N17 Q1 (iii))"
        ),
        other => panic!(
            "an init reporting permissionMode acceptEdits for a `default` card must be NotIsolated \
             {{ surface: env }} (s8h §8h.5; N17 Q1 (iii)); got {other:?}"
        ),
    }
    assert_eq!(
        completed_rows(&r.store).len(),
        1,
        "the refusal carries its row (AC-360)"
    );
    // the card's `default`, never the argv's `manual` (X12)
    let t = with_init("init-2-1-303-default.scripted.ndjson");
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let config = crate::support::spawn_config(&markers, &format!("replay:{}", t.path.display()))
        .at_pin("2.1.303");
    let (config, spec) = bind_probe(config, 30, wt.path(), "x");
    let mut store = VecStore::default();
    let o = harness(config).dispatch(&spec, &mut store);
    let argv = &markers.read()[0].argv;
    let at = argv
        .iter()
        .position(|a| a == "--permission-mode")
        .expect("the argv carries --permission-mode");
    assert_eq!(
        argv[at + 1],
        "manual",
        "PREMISE: at 2.1.303 the argv carries the row's spelling of `default` (s8h §8h.2)"
    );
    assert!(
        o.result.is_ok(),
        "guard 2 adopts an init reporting `default` for a `default` card whose argv spelled it \
         `manual` (D21-48 X12: the init comparison reads `default`); got {:?}",
        o.result
    );
}

/// §8h.5: an ABSENT `permissionMode` is refused, never treated as equal — `NotIsolated { surface:
/// Env }`, the digest over the empty reported set against `{default}`.
#[test]
fn permission_mode_absent_from_init_is_refused() {
    let r = dispatch_replay(
        &with_init("init-permission-mode-absent.scripted.ndjson"),
        30,
    );
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Env,
            diff_digest,
        }) => assert_eq!(
            diff_digest,
            &rk_adapter_claude::init::set_digest(
                &std::collections::BTreeSet::new(),
                &one("default")
            ),
            "an absent key reports the empty set (N17 Q1 (iii))"
        ),
        other => panic!(
            "an init with no permissionMode must be NotIsolated {{ surface: env }} — absent is \
             refused, not treated as equal (s8h §8h.5); got {other:?}"
        ),
    }
}

/// §8h.3 / §8h.5 / N17 Q1 (i): requested `claude-sonnet-5` (the `sonnet` row's full id), reported
/// `claude-sonnet-4-6` — `NotAdopted`, even with the `agent` key naming the card. A different
/// spelling of the same model (the haiku id without its dated suffix) is a failure too (§8h.11:
/// the equality is never loosened in place), and so is an init that reports no model.
#[test]
fn model_reported_unequal_to_the_requested_id_is_refused() {
    let r = crate::support::dispatch_replay_as(
        &with_init("init-model-sonnet-4-6.scripted.ndjson"),
        "sonnet-high",
        30,
    );
    match &r.outcome.result {
        Err(StepError::NotAdopted {
            expected_agent,
            observed,
        }) => {
            assert_eq!(expected_agent, "probe");
            assert!(
                observed.contains("claude-sonnet-4-6"),
                "the refusal names the reported model: {observed}"
            );
        }
        other => panic!(
            "requested claude-sonnet-5, reported claude-sonnet-4-6: adoption is EXACT equality of \
             the model id (s8h §8h.3; N17 Q1 (i)) — expected NotAdopted, got {other:?}"
        ),
    }
    let r = dispatch_replay(&with_init("init-model-haiku-undated.scripted.ndjson"), 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::NotAdopted { .. })),
        "requested claude-haiku-4-5-20251001, reported claude-haiku-4-5: a different spelling of \
         the same model is a failure (s8h §8h.11) — expected NotAdopted, got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init-no-model.scripted.ndjson"), 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::NotAdopted { .. })),
        "an init reporting no model adopted nothing — expected NotAdopted, got {:?}",
        r.outcome.result
    );
}

/// The control: an init reporting exactly the requested full id proceeds — the sonnet row's id
/// under the `sonnet-high` role, and the haiku row's id under `haiku`.
#[test]
fn model_equal_to_the_requested_id_proceeds() {
    let r = crate::support::dispatch_replay_as(
        &with_init("init-model-sonnet-5.scripted.ndjson"),
        "sonnet-high",
        30,
    );
    assert!(
        r.outcome.result.is_ok(),
        "requested claude-sonnet-5, reported claude-sonnet-5: adopted — got {:?}",
        r.outcome.result
    );
    let r = dispatch_replay(&with_init("init.scripted.ndjson"), 30);
    assert!(
        r.outcome.result.is_ok(),
        "requested and reported claude-haiku-4-5-20251001: adopted — got {:?}",
        r.outcome.result
    );
}

/// §8h.5 / ADR-0010 in-band (*"The adapter must refuse to proceed (assert, not warn)"*) / N17 Q1
/// (ii): `apiKeySource` other than `"none"` is `NotIsolated { surface: Env }`, the digest over
/// `{ANTHROPIC_API_KEY}` against `{none}`.
#[test]
fn api_key_source_other_than_none_is_refused() {
    let r = dispatch_replay(&with_init("init-api-key-source-other.scripted.ndjson"), 30);
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Env,
            diff_digest,
        }) => assert_eq!(
            diff_digest,
            &rk_adapter_claude::init::set_digest(&one("ANTHROPIC_API_KEY"), &one("none"))
        ),
        other => panic!(
            "apiKeySource ANTHROPIC_API_KEY must be NotIsolated {{ surface: env }} (s8h §8h.5; \
             ADR-0010; N17 Q1 (ii)); got {other:?}"
        ),
    }
}

/// §8h.5 / N17 Q1 (ii): an ABSENT `apiKeySource` is refused, the digest over the empty set
/// against `{none}`.
#[test]
fn api_key_source_absent_is_refused() {
    let r = dispatch_replay(&with_init("init-api-key-source-absent.scripted.ndjson"), 30);
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Env,
            diff_digest,
        }) => assert_eq!(
            diff_digest,
            &rk_adapter_claude::init::set_digest(&std::collections::BTreeSet::new(), &one("none"))
        ),
        other => panic!(
            "an init with no apiKeySource must be NotIsolated {{ surface: env }} — absent is \
             refused (s8h §8h.5; N17 Q1 (ii)); got {other:?}"
        ),
    }
}
