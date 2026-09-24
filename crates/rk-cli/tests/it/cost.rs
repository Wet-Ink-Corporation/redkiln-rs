//! **`redkiln cost <ITEM>`** — ADR-0163 decisions 1, 4 and 6; ADR-0110; AC-373, AC-395; `s7i`
//! §7i.2–§7i.3.
//!
//! Three instruments, and each covers what the others cannot:
//!
//! - **A real run, then a real `cost`** (`cost_after_a_real_run_renders_low_sample_through_the_binary`)
//!   — the producer, the journal and the verb end to end, through the binary, over the synthetic
//!   transcript `tests/fixtures/cost/run-frames.jsonl`. It reaches exactly one rendering, `n=1 …
//!   low_sample`, because one run is one dispatch.
//! - **A seeded corpus** (`cost_fixture_corpus_binary_arm`) — the renderings one run cannot reach
//!   (`n >= 10`, a reset crossing, interleaving, `unkeyed`, a flat reading, `non_monotonic`), each
//!   asserted byte-for-byte against an `expected.stdout` computed by hand.
//! - **In-process `render` / `run_with`** — AC-395's no-dispatch property under a harness that
//!   panics, the empty case, and the closed grammar of every line.
//!
//! The shipped store is not read here: it holds zero `dispatch_completed` rows (ADR-0163 decision
//! 6), and Checkpoint 7i asserts its empty case directly.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_ports::event_store::EventStore;
use rk_ports::fakes::VecStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use rk_cli::exit::Exit;
use rk_cli::verbs::cost;

use crate::support::{
    PackVariant, cost_rows, output_bounded, pack_min_repo, seed_store, store_rows,
};
use crate::{fixtures_dir, temp_repo};

/// Every seeded case, in directory byte order — asserted equal to the on-disk set both ways.
const CASES: [&str; 23] = [
    "empty",
    "empty-counts-dispatch-rows-only",
    "flat-reading",
    "flat-unwindowed",
    "flat-unwindowed-other-type",
    "foreign-adjacent-to-span",
    "foreign-dispatch-no-reading",
    "foreign-only",
    "foreign-outside-span",
    "interleaved",
    "malformed-foreign-in-interleaved-span",
    "malformed-owned-in-interleaved-span",
    "malformed-owned-missing-member",
    "n10-10-10",
    "n10-10-11",
    "n10-10-13",
    "n9-low-sample",
    "non-monotonic",
    "null-resets-at",
    "null-splits-a-batch",
    "own-noise-kinds",
    "reset-crossing",
    "single-owned",
];

/// The five-hour `resetsAt` the run transcript's `unifiedWindows.five_hour` carries. Its top-level
/// `resetsAt` is `1788300000`, deliberately different, so a producer that journalled the top-level
/// value would render a different key.
const RUN_FIVE_HOUR_RESETS_AT: i64 = 1_788_370_800;

fn cost_dir() -> PathBuf {
    fixtures_dir().join("cost")
}

/// The exit code a case expects: its `expected.exit` file when present (one integer, trailing
/// newline allowed), `0` when absent — the rendering cases carry none.
fn expected_exit(case_dir: &Path) -> i32 {
    let path = case_dir.join("expected.exit");
    if !path.is_file() {
        return 0;
    }
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    text.trim_end_matches('\n')
        .parse()
        .unwrap_or_else(|e| panic!("{} is not one integer: {e}", path.display()))
}

/// The empty rendering for `item` over `dispatches` of its kernel `dispatch` rows.
fn nothing_line(item: &str, dispatches: usize) -> String {
    format!(
        "cost {item}: nothing to attribute \u{2014} no rate_limit_reading row in this store names \
         this item ({dispatches} dispatch rows)\n"
    )
}

/// Spawn `redkiln -C <root> cost <item>`.
///
/// `RK_FIXTURE_MODE` is set to `exit:0` although `cost` dispatches nothing: left unset, an
/// UNEXPECTED dispatch would park for 600 s in the fixture child's `hold` default instead of
/// failing — the reason `answer`'s spawner carries it too.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS — the binary arm of ADR-0163 decision 6"
)]
fn spawn_cost(root: &Path, item: &str) -> std::process::Output {
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C")
        .arg(root)
        .arg("cost")
        .arg(item)
        .env("RK_FIXTURE_MODE", "exit:0");
    output_bounded(&mut cmd)
}

/// **The dispatched case, through the binary** — Checkpoint 7i's *"`redkiln cost <item>` exists
/// and exits 0 on an item with dispatches"*, on the only route this tree has (ADR-0163 decision 6).
///
/// `redkiln run bug-1` over `pack-min`'s base variant, with the fixture child replaying
/// `cost/run-frames.jsonl` — the init and result frames of `exit/schema-invalid-structured-output`
/// with a `rate_limit_event` between them — mirrored from AC-438's binary arm. The result frame's
/// `structured_output` fails `ImplReport.json`, so the run exits `11`, and the reading is on a
/// **failed** dispatch: ADR-0163 decision 2's *"whether or not the dispatch succeeds"*.
///
/// # 🛑 THE PREMISE IS ASSERTED BEFORE THE RENDERING, AND IT IS NOT WEAKENED
///
/// `k >= 1` owned readings in the store the run wrote. Zero would mean the dispatch never reached
/// `ClaudeHarness` or the producer did not fire, and every `cost` assertion after it would be over
/// the empty case — the vacuous green §7i.2 forbids. `k` must also equal the `dispatch_completed`
/// count (one reading per dispatch through the child), and each reading's `resets_at` members must
/// be JSON integers (ADR-0163 decision 3's spelling).
///
/// Red by: the producer's append removed (k = 0); the reading keyed by the top-level `resetsAt`
/// (the key line reads `1788300000`); `cost` quoting a delta at `n < 10`; `cost` rendering a
/// sampled shape for an item that never ran.
#[test]
#[expect(
    clippy::disallowed_types,
    reason = "the run is driven AS A PROCESS, exactly as AC-438's binary arm drives it"
)]
fn cost_after_a_real_run_renders_low_sample_through_the_binary() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();
    let frames = cost_dir().join("run-frames.jsonl");
    assert!(
        frames.is_file(),
        "the run transcript {} is absent",
        frames.display()
    );

    // ---- (1) THE RUN, mirrored from `ac438_table_driven_exit_codes_over_fixtures_binary_arm`.
    let markers = root.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(root).arg("run").arg("bug-1");
    cmd.env("RK_FIXTURE_MODE", format!("replay:{}", frames.display()))
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    let ran = output_bounded(&mut cmd);
    assert_eq!(
        ran.status.code(),
        Some(Exit::Schema.code()),
        "`redkiln run bug-1` over the cost transcript is the schema-invalid exit fixture plus one \
         rate_limit_event, so it exits 11.\nstderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&ran.stderr),
        String::from_utf8_lossy(&ran.stdout)
    );

    // ---- (2) THE PREMISE: the producer journalled a reading, owned by the item, per dispatch.
    let rows = store_rows(root);
    let kinds: Vec<EventKind> = rows.iter().map(|e| e.kind).collect();
    let owned: Vec<&Event> = rows
        .iter()
        .filter(|e| e.kind == EventKind::RateLimitReading)
        .filter(|e| e.payload["item"].as_str() == Some("bug-1"))
        .collect();
    let k = owned.len();
    assert!(
        k >= 1,
        "the run wrote NO rate_limit_reading owned by bug-1 — the route is vacuous and every \
         assertion below would read the empty case. Rows: {kinds:?}"
    );
    let completed = rows
        .iter()
        .filter(|e| e.kind == EventKind::DispatchCompleted)
        .count();
    assert_eq!(
        k, completed,
        "one reading per dispatch through the child (ADR-0163 decision 2); rows: {kinds:?}"
    );
    for reading in &owned {
        for window in ["five_hour", "seven_day"] {
            assert!(
                reading.payload["resets_at"][window].is_i64(),
                "resets_at.{window} is not integer seconds: {}",
                reading.payload
            );
        }
        assert_eq!(
            reading.payload["resets_at"]["five_hour"].as_i64(),
            Some(RUN_FIVE_HOUR_RESETS_AT),
            "the five_hour key is the window's own resetsAt, not the top-level one: {}",
            reading.payload
        );
        assert_eq!(
            reading.payload["windows"],
            serde_json::json!({"five_hour": 4, "seven_day": 37}),
            "the ticks are the transcript's 0.04 / 0.37, verbatim: {}",
            reading.payload
        );
    }

    // ---- (3) `cost` over the store the run wrote.
    let out = spawn_cost(root, "bug-1");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "cost exits 0 on an item with dispatches.\nstderr:\n{}\nstdout:\n{stdout}",
        String::from_utf8_lossy(&out.stderr)
    );
    let key = format!("five_hour: n={k} resets_at={RUN_FIVE_HOUR_RESETS_AT}");
    assert!(stdout.contains(&key), "no `{key}` line in:\n{stdout}");
    assert!(
        stdout.contains(" low_sample"),
        "n={k} is below ten and not flagged low_sample:\n{stdout}"
    );
    assert!(
        !stdout.contains("delta_ticks="),
        "a delta was quoted at n={k} < 10 (AC-372):\n{stdout}"
    );

    // ---- (4) An item that did not run: the empty case, positively.
    let other = spawn_cost(root, "story-1");
    assert_eq!(
        other.status.code(),
        Some(0),
        "cost exits 0 on the empty case too.\nstderr:\n{}",
        String::from_utf8_lossy(&other.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&other.stdout),
        nothing_line("story-1", 0),
        "story-1 never ran, so it owns no reading and has no dispatch row"
    );
}

/// **The seeded corpus, through the binary** — every rendering ADR-0163 decision 4 names, byte
/// for byte (ADR-0163 decision 6).
///
/// The case set is asserted equal to [`CASES`] in both directions before any case runs, on
/// AC-438's precedent: a walker over the directory alone passes over an empty one. Each case seeds
/// a fresh repository's store from `rows.jsonl` and compares `redkiln cost bug-1`'s exit code with
/// `expected.exit` (`0` when the case carries none) and its stdout BYTES with `expected.stdout`,
/// which was computed by hand. A case expecting a non-zero exit must also carry an EMPTY
/// `expected.stdout`: a refusal prints nothing on stdout.
///
/// Red by: the interleaving counter ignoring `dispatch` rows (`interleaved` reads `=1`); a `null`
/// `resets_at` grouped under a value (`null-resets-at`); a flat reading written into both windows
/// (`flat-reading`); the reset split lost (`reset-crossing`); a delta at `n < 10`; an owned reading
/// missing a member rendered as "no sample" (`malformed-owned-missing-member` exits `0`); the
/// owned-row validation moved after the interleaving check (`malformed-owned-in-interleaved-span`
/// exits `0` with `not attributable`); a foreign dispatch that produced no reading left uncounted
/// (`foreign-dispatch-no-reading` renders draws); a `null` `resets_at` that does not close its
/// segment (`null-splits-a-batch` renders one five-hour draw of `n=11`); the span widened by one
/// row at either edge (`foreign-adjacent-to-span` reads `not attributable`); the empty case
/// counting rows of other kinds that carry the item (`empty-counts-dispatch-rows-only` reads `2`);
/// a flat reading naming no window dropped silently or extrapolated (`flat-unwindowed`), or
/// counted only when there are two of it or only when its type is `null`
/// (`flat-unwindowed-other-type`); a foreign
/// reading validated (`malformed-foreign-in-interleaved-span` exits `3`).
#[test]
fn cost_fixture_corpus_binary_arm() {
    let dir = cost_dir();
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .expect("tests/fixtures/cost/ exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    on_disk.sort();
    assert_eq!(
        on_disk, CASES,
        "the `cost/` case set and this test's list disagree. A table that walked the directory \
         alone would pass over an empty one"
    );

    for name in CASES {
        let case_dir = dir.join(name);
        let rows = cost_rows(&case_dir.join("rows.jsonl"));
        let expected = std::fs::read(case_dir.join("expected.stdout"))
            .unwrap_or_else(|e| panic!("{name}/expected.stdout is readable: {e}"));
        assert!(
            !expected.contains(&b'\r'),
            "{name}/expected.stdout carries a CR — the corpus is smudged, and the comparison below \
             would fail for a reason unrelated to `cost`"
        );

        let want = expected_exit(&case_dir);
        if want != 0 {
            assert!(
                expected.is_empty(),
                "{name}: a case expecting exit {want} must carry an empty expected.stdout"
            );
        }

        let repo = temp_repo();
        seed_store(repo.path(), &rows);
        let out = spawn_cost(repo.path(), "bug-1");
        assert_eq!(
            out.status.code(),
            Some(want),
            "{name}: cost exited {:?}, expected {want}.\nstderr:\n{}\nstdout:\n{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
        assert_eq!(
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&expected),
            "{name}: the binary's stdout is not expected.stdout"
        );
        assert_eq!(
            out.stdout, expected,
            "{name}: the strings agree and the bytes do not"
        );
    }
}

/// A `Harness` whose `dispatch` fails the test — AC-395's instrument.
struct PanickingHarness;

impl Harness for PanickingHarness {
    fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        panic!("AC-395: `cost` dispatched a model call — it renders from the event store alone")
    }
}

/// **AC-395** — *"from the event store alone … it dispatches no model call"*, and ADR-0163
/// decision 1's *"appends nothing to the store"*.
///
/// `run_with` is handed a harness whose `dispatch` panics, over a store holding one owned reading
/// — a store with something to render, so the verb is not trivially idle. It returns `Success`,
/// writes a non-empty rendering, and the store's row count is unchanged.
///
/// Red by: any call through the harness (the panic); an append from the verb (the count).
#[test]
fn ac395_cost_dispatches_nothing_under_a_panicking_harness() {
    let rows = cost_rows(&cost_dir().join("single-owned").join("rows.jsonl"));
    let mut seeded = VecStore::default();
    for row in &rows {
        seeded.append(row.clone()).expect("a VecStore append");
    }
    let before = seeded.events().len();
    assert!(
        seeded
            .events()
            .iter()
            .any(|e| e.kind == EventKind::RateLimitReading),
        "the seeded store owns no reading, so the verb would render the empty case and this test \
         would not reach the path that renders"
    );
    let concrete = Arc::new(Mutex::new(seeded));
    let store: Arc<Mutex<dyn EventStore>> = concrete.clone();

    let mut out: Vec<u8> = Vec::new();
    let exit = cost::run_with(&PanickingHarness, &store, "bug-1", &mut out);
    assert_eq!(exit, Ok(Exit::Success));
    let text = String::from_utf8(out).expect("the rendering is UTF-8");
    assert!(
        text.contains("five_hour: n=1 "),
        "the rendering does not show the owned reading:\n{text}"
    );

    let after = concrete
        .lock()
        .expect("the store lock is not poisoned")
        .events()
        .len();
    assert_eq!(before, after, "cost appended to the store");
}

/// **An owned reading is the whole six-member shape, or it is malformed** — ADR-0163 decision 3's
/// fixed key set, read by `cost` as a contract, not a best effort.
///
/// Over `single-owned` (which renders): each of the six members removed in turn, both `windows`
/// and `utilization` non-`null`, both `null`, a float `utilization` beside a `null` `windows`
/// under a `rate_limit_type` naming no window, a non-integer `resets_at` member (either window), a
/// numeric `rate_limit_type`, a string or out-of-`u16` `windows.five_hour`, and a `cli_version`
/// that is not a serialised `CliVersion` (`null`, a number, a bare string, a non-string arm, an
/// unknown arm, two arms, `{}`) — every one is [`cost::Malformed`], never a rendering, and every
/// one is still refused when replayed as the last owned reading of an interleaved journal. Both
/// `CliVersion` arms as serde writes them are accepted. The same mutations on a FOREIGN reading
/// are not validated: the item that owns nothing still renders its empty line.
///
/// Red by: a missing key read as "no sample" (`utilization`, `rate_limit_type` or `cli_version`
/// removed renders); `utilization` checked only when `rate_limit_type` names a window; the
/// exactly-one rule dropped. Removing `item` is the one mutation that is NOT malformed: the
/// reading is then owned by nothing, and an unowned reading is never validated.
#[test]
fn render_refuses_an_owned_reading_that_is_not_the_six_member_shape() {
    let rows = cost_rows(&cost_dir().join("single-owned").join("rows.jsonl"));
    let at = rows
        .iter()
        .position(|e| e.kind == EventKind::RateLimitReading)
        .expect("single-owned carries a reading");
    assert!(
        cost::render(&rows, "bug-1").is_ok(),
        "PREMISE: the unmutated journal renders"
    );
    let mutate = |f: &dyn Fn(&mut serde_json::Map<String, serde_json::Value>)| {
        let mut out = rows.clone();
        let payload = out[at]
            .payload
            .as_object_mut()
            .expect("the reading's payload is an object");
        f(payload);
        out
    };
    let mut cases: Vec<(String, Vec<Event>)> = Vec::new();
    for key in [
        "windows",
        "utilization",
        "resets_at",
        "rate_limit_type",
        "cli_version",
    ] {
        cases.push((
            format!("`{key}` removed"),
            mutate(&|p| {
                p.remove(key);
            }),
        ));
    }
    cases.push((
        "both windows and utilization".to_owned(),
        mutate(&|p| {
            p.insert("utilization".to_owned(), serde_json::json!(4));
        }),
    ));
    cases.push((
        "neither windows nor utilization".to_owned(),
        mutate(&|p| {
            p.insert("windows".to_owned(), serde_json::Value::Null);
        }),
    ));
    cases.push((
        "a float utilization under an unnamed window".to_owned(),
        mutate(&|p| {
            p.insert("windows".to_owned(), serde_json::Value::Null);
            p.insert("utilization".to_owned(), serde_json::json!(0.5));
            p.insert("rate_limit_type".to_owned(), serde_json::Value::Null);
        }),
    ));
    cases.push((
        "a string resets_at member".to_owned(),
        mutate(&|p| {
            p.insert(
                "resets_at".to_owned(),
                serde_json::json!({ "five_hour": "soon", "seven_day": null }),
            );
        }),
    ));
    cases.push((
        "a string seven_day resets_at member".to_owned(),
        mutate(&|p| {
            p.insert(
                "resets_at".to_owned(),
                serde_json::json!({ "five_hour": 1_788_370_800_i64, "seven_day": "later" }),
            );
        }),
    ));
    cases.push((
        "a numeric rate_limit_type".to_owned(),
        mutate(&|p| {
            p.insert("rate_limit_type".to_owned(), serde_json::json!(5));
        }),
    ));
    cases.push((
        "a string windows.five_hour".to_owned(),
        mutate(&|p| {
            p.insert(
                "windows".to_owned(),
                serde_json::json!({ "five_hour": "4", "seven_day": 37 }),
            );
        }),
    ));
    cases.push((
        "windows.five_hour beyond u16".to_owned(),
        mutate(&|p| {
            p.insert(
                "windows".to_owned(),
                serde_json::json!({ "five_hour": 70_000, "seven_day": 37 }),
            );
        }),
    ));
    for (label, version) in [
        ("null", serde_json::Value::Null),
        ("a number", serde_json::json!(42)),
        ("a bare string", serde_json::json!("2.1.278")),
        ("an observed number", serde_json::json!({ "observed": 1 })),
        ("an unknown arm", serde_json::json!({ "x": "y" })),
        (
            "two arms",
            serde_json::json!({ "observed": "2.1.278", "pinned": "2.1.278" }),
        ),
        ("an empty object", serde_json::json!({})),
    ] {
        cases.push((
            format!("cli_version {label}"),
            mutate(&|p| {
                p.insert("cli_version".to_owned(), version.clone());
            }),
        ));
    }
    for (label, journal) in &cases {
        assert!(
            cost::render(journal, "bug-1").is_err(),
            "{label}: an owned reading off the six-member shape rendered"
        );
    }
    // Each malformation replayed as the LAST owned reading of an interleaved journal — a valid
    // owned reading, a foreign one, then the malformed one — is still refused: validation runs
    // before interleaving is counted, so `not attributable` never masks a malformed owned row.
    let mut foreign_reading = rows[at].clone();
    foreign_reading
        .payload
        .as_object_mut()
        .expect("the reading's payload is an object")
        .insert("item".to_owned(), serde_json::json!("story-1"));
    for (label, journal) in &cases {
        let mut interleaved = rows.clone();
        interleaved.push(foreign_reading.clone());
        interleaved.push(journal[at].clone());
        assert!(
            cost::render(&interleaved, "bug-1").is_err(),
            "{label}: a malformed owned reading inside an interleaved span rendered"
        );
    }
    // PREMISE of the interleaved half: with a VALID last owned reading the same journal renders,
    // and renders `not attributable` — so each refusal above is the malformation's.
    let mut valid = rows.clone();
    valid.push(foreign_reading);
    valid.push(rows[at].clone());
    let text = cost::render(&valid, "bug-1").expect("the valid interleaved journal renders");
    assert!(
        text.contains("not attributable interleaving=1\n"),
        "premise: the interleaved journal interleaves:\n{text}"
    );
    // Both arms of `rk_types::usage::CliVersion`, serialised by serde itself, are accepted — the
    // structural check matches the enum and not a hand-written guess at it.
    for version in [
        rk_types::usage::CliVersion::Observed("2.1.278".to_owned()),
        rk_types::usage::CliVersion::Pinned("2.1.277".to_owned()),
    ] {
        let value = serde_json::to_value(&version).expect("a CliVersion serialises");
        let journal = mutate(&|p| {
            p.insert("cli_version".to_owned(), value.clone());
        });
        assert!(
            cost::render(&journal, "bug-1").is_ok(),
            "{version:?} serialised as {value} was refused"
        );
    }
    // `item` removed: the reading is no longer owned by anything, so it is not validated and
    // bug-1 renders the empty case — a missing owner is never a malformed OWNED row.
    let unowned = mutate(&|p| {
        p.remove("item");
    });
    assert!(
        cost::render(&unowned, "bug-1").is_ok(),
        "an unowned reading is traffic, not a sample, and is never validated"
    );
    // A foreign item's view of the mutated journals: never validated, always renders.
    for (label, journal) in &cases {
        assert!(
            cost::render(journal, "story-1").is_ok(),
            "{label}: a reading foreign to story-1 was validated"
        );
    }
}

/// **The empty case renders nothing sampled** — `s7i` §7i.2's *"A vacuous rendering is not a
/// pass"*, and ADR-0163's forbids bullet on an item that owns no reading.
///
/// Both an empty journal and one holding only another item's traffic render the one
/// nothing-to-attribute line, and neither contains any of ADR-0110's three renderings.
#[test]
fn render_empty_is_nothing_to_attribute_never_low_sample() {
    let foreign = cost_rows(&cost_dir().join("foreign-only").join("rows.jsonl"));
    assert!(
        !foreign.is_empty(),
        "the foreign-only fixture is empty, so its half would repeat the empty-journal half"
    );
    for (label, rows) in [("empty journal", Vec::new()), ("foreign-only", foreign)] {
        let text = cost::render(&rows, "x").expect("the empty case renders");
        assert_eq!(text, nothing_line("x", 0), "{label}");
        for forbidden in [
            "low_sample",
            "n=",
            "delta",
            "not attributable",
            "resolution_limited",
        ] {
            assert!(
                !text.contains(forbidden),
                "{label}: the empty case names `{forbidden}`:\n{text}"
            );
        }
    }
}

/// Is `s` one or more ASCII digits?
fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

/// Is `s` a positive integer with no leading zero — `[1-9][0-9]*`?
fn positive(s: &str) -> bool {
    digits(s) && !s.starts_with('0')
}

/// One line against the closed grammar of ADR-0163 decision 4, hand-matched because `rk-cli`
/// carries no `regex` dependency. The forms, as `cost.rs`'s module doc states them:
///
/// ```text
/// ^cost \S+: nothing to attribute — … \([0-9]+ dispatch rows\)$
/// ^cost \S+: rate_limit_reading rows in span: [0-9]+, this store's traffic only$
/// ^not attributable interleaving=[1-9][0-9]*$
/// ^(five_hour|seven_day): n=[1-9] resets_at=-?[0-9]+ low_sample( non_monotonic| resolution_limited)?$
/// ^(five_hour|seven_day): n=[1-9][0-9]+ resets_at=-?[0-9]+( non_monotonic| delta_ticks=[01] resolution_limited| delta_ticks=([2-9]|[1-9][0-9]+))$
/// ^(five_hour|seven_day): unkeyed=[1-9][0-9]*$
/// ^(five_hour|seven_day): no sample$
/// ^unwindowed=[1-9][0-9]*$
/// ```
///
/// The two draw forms encode what `throttle::batch_draw` makes true: below ten a draw is
/// `low_sample` with no delta, and `resolution_limited` (a raw delta of `0` or `1`) excludes
/// `non_monotonic` (no raw delta); at ten or more a draw is never `low_sample`, carries no delta
/// exactly when it is `non_monotonic` (and then is not `resolution_limited`), and is
/// `resolution_limited` exactly when its delta is `0` or `1`.
fn in_grammar(line: &str) -> bool {
    if let Some(rest) = line.strip_prefix("cost ") {
        let Some((item, tail)) = rest.split_once(": ") else {
            return false;
        };
        if item.is_empty() || item.chars().any(char::is_whitespace) {
            return false;
        }
        if let Some(n) = tail
            .strip_prefix(
                "nothing to attribute \u{2014} no rate_limit_reading row in this store names this \
                 item (",
            )
            .and_then(|t| t.strip_suffix(" dispatch rows)"))
        {
            return digits(n);
        }
        return tail
            .strip_prefix("rate_limit_reading rows in span: ")
            .and_then(|t| t.strip_suffix(", this store's traffic only"))
            .is_some_and(digits);
    }
    if let Some(n) = line.strip_prefix("not attributable interleaving=") {
        return positive(n);
    }
    if let Some(n) = line.strip_prefix("unwindowed=") {
        return positive(n);
    }
    let Some(body) = line
        .strip_prefix("five_hour: ")
        .or_else(|| line.strip_prefix("seven_day: "))
    else {
        return false;
    };
    if body == "no sample" {
        return true;
    }
    if let Some(n) = body.strip_prefix("unkeyed=") {
        return positive(n);
    }
    let Some(draw) = body.strip_prefix("n=") else {
        return false;
    };
    let Some((n, rest)) = draw.split_once(" resets_at=") else {
        return false;
    };
    if !positive(n) {
        return false;
    }
    let (secs, flags) = rest.find(' ').map_or((rest, ""), |i| rest.split_at(i));
    if !digits(secs.strip_prefix('-').unwrap_or(secs)) {
        return false;
    }
    if n.len() == 1 {
        // n < 10: `low_sample`, no delta, then at most one of the two exclusive flags.
        return matches!(
            flags,
            " low_sample" | " low_sample non_monotonic" | " low_sample resolution_limited"
        );
    }
    // n >= 10: never `low_sample`; `non_monotonic` alone, or a delta that is
    // `resolution_limited` exactly when it is `0` or `1`.
    if flags == " non_monotonic" {
        return true;
    }
    let Some(after) = flags.strip_prefix(" delta_ticks=") else {
        return false;
    };
    if let Some(delta) = after.strip_suffix(" resolution_limited") {
        return delta == "0" || delta == "1";
    }
    positive(after) && after != "1"
}

/// **The output grammar is closed** — AC-373: no per-run or per-dispatch figure, and no line
/// outside the eight forms of ADR-0163 decision 4's closed grammar (the `cost.rs` module doc).
///
/// Every line of every corpus `expected.stdout`, and every line `render` produces in-process over
/// every corpus journal for the target item, the foreign one and an absent one, must match. The
/// in-process render of each case is also asserted equal to its `expected.stdout`, so the binary
/// arm and the pure function cannot drift apart.
///
/// **Self-controlled first**: the matcher must accept one line of each form and reject a line
/// quoting `first`/`last`, a zero `n`, a fractional delta, a flag out of order, a delta at
/// `n < 10`, `low_sample` at `n >= 10`, a flag combination `throttle::batch_draw` cannot produce,
/// and a per-run figure — or every acceptance below would be the matcher's, not the renderer's.
#[test]
fn cost_output_grammar_is_closed() {
    for good in [
        "cost bug-1: nothing to attribute \u{2014} no rate_limit_reading row in this store names \
         this item (0 dispatch rows)",
        "cost bug-1: rate_limit_reading rows in span: 3, this store's traffic only",
        "not attributable interleaving=2",
        "five_hour: n=10 resets_at=1788370800 delta_ticks=3",
        "seven_day: n=1 resets_at=-5 low_sample resolution_limited",
        "five_hour: n=10 resets_at=1 non_monotonic",
        "seven_day: unkeyed=4",
        "five_hour: no sample",
        "unwindowed=2",
        "five_hour: n=12 resets_at=1 delta_ticks=0 resolution_limited",
        "seven_day: n=3 resets_at=1 low_sample non_monotonic",
    ] {
        assert!(
            in_grammar(good),
            "the matcher rejects a grammatical line: {good}"
        );
    }
    for bad in [
        "five_hour: n=3 resets_at=1 first=10 last=12",
        "five_hour: n=0 resets_at=1 low_sample",
        "five_hour: n=10 resets_at=1 delta_ticks=2.5",
        "five_hour: n=2 resets_at=1 resolution_limited low_sample",
        "not attributable interleaving=0",
        "five_hour: unkeyed=0",
        "per_run: 0.3",
        "cost bug-1: headroom_for_run=0.03",
        "cost bug-1: 3 readings in span, this store's traffic only",
        "five_hour: n=3 resets_at=1 delta_ticks=4 low_sample",
        "five_hour: n=3 resets_at=1 delta_ticks=4",
        "five_hour: n=12 resets_at=1 low_sample",
        "per-run headroom: 0.01",
        "cost x: headroom_for_run=0.03",
        "unwindowed=0",
        "five_hour: n=3 resets_at=1 low_sample non_monotonic resolution_limited",
        "five_hour: n=12 resets_at=1 delta_ticks=3 non_monotonic",
        "five_hour: n=12 resets_at=1 delta_ticks=5 resolution_limited",
        "five_hour: n=12 resets_at=1 delta_ticks=1",
    ] {
        assert!(
            !in_grammar(bad),
            "the matcher accepts an ungrammatical line: {bad}"
        );
    }

    let mut texts: Vec<(String, String)> = Vec::new();
    for name in CASES {
        let case_dir = cost_dir().join(name);
        let expected = std::fs::read_to_string(case_dir.join("expected.stdout"))
            .unwrap_or_else(|e| panic!("{name}/expected.stdout is readable: {e}"));
        let rows = cost_rows(&case_dir.join("rows.jsonl"));
        if expected_exit(&case_dir) != 0 {
            // A malformed-owned case: the pure function refuses it for the target item, which is
            // what the binary's exit 3 is made of; the other items still render.
            assert!(
                cost::render(&rows, "bug-1").is_err(),
                "{name}: the in-process render accepted a journal the binary must refuse"
            );
            for item in ["story-1", "absent-9"] {
                let text = cost::render(&rows, item)
                    .unwrap_or_else(|e| panic!("{name} as {item}: the journal renders: {e}"));
                texts.push((format!("{name} rendered for {item}"), text));
            }
            continue;
        }
        let rendered = cost::render(&rows, "bug-1")
            .unwrap_or_else(|e| panic!("{name}: the journal renders: {e}"));
        assert_eq!(
            rendered, expected,
            "{name}: the in-process render is not expected.stdout"
        );
        texts.push((format!("{name}/expected.stdout"), expected));
        for item in ["bug-1", "story-1", "absent-9"] {
            if name == "malformed-foreign-in-interleaved-span" && item == "story-1" {
                // The malformed reading is foreign to bug-1 and OWNED by story-1: for story-1 it
                // is validated, and refused.
                assert!(
                    cost::render(&rows, item).is_err(),
                    "{name}: story-1's own malformed reading rendered"
                );
                continue;
            }
            let text = cost::render(&rows, item)
                .unwrap_or_else(|e| panic!("{name} as {item}: the journal renders: {e}"));
            texts.push((format!("{name} rendered for {item}"), text));
        }
    }
    let lines: usize = texts.iter().map(|(_, t)| t.lines().count()).sum();
    assert!(lines > CASES.len(), "only {lines} lines were checked");

    for (label, text) in &texts {
        assert!(
            text.ends_with('\n'),
            "{label}: the rendering is not newline-terminated"
        );
        for line in text.lines() {
            assert!(
                in_grammar(line),
                "{label}: a line outside the grammar: {line:?}"
            );
            for banned in ["headroom_for_run", "run_headroom_cost"] {
                assert!(
                    !line.contains(banned),
                    "{label}: a per-run headroom figure (AC-373): {line:?}"
                );
            }
        }
    }
}
