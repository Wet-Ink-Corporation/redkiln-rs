//! **`status`'s AC-372 half** — an item's draw lines, every flag included, through the ONE draw
//! renderer `cost` uses (`s8f` §8f.8; the telemetry-key record ADR-0190 decision 5, proposed).
//! *(Added 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL.)*
//!
//! `status::run_with` is driven in process over a `VecStore` seeded from `cost`'s own corpus, with a
//! real repository's wrapper and worktree handed in (the verb takes them and calls nothing; see its
//! header). What `status` prints for an item's draws is compared with what `cost::render` prints
//! for the same journal: the two surfaces must agree line for line, flags included.

use std::sync::{Arc, Mutex};

use rk_cli::verbs::{cost, status};
use rk_ports::event_store::EventStore;
use rk_ports::fakes::VecStore;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::support::{RecordingReconciler, Trace, cost_rows};

/// `status`'s stdout over `rows`.
fn status_over(rows: &[Event]) -> String {
    let repo_dir = crate::temp_repo();
    let discovered =
        rk_cli::repo::discover(Some(repo_dir.path())).expect("the temp repository discovers");
    let mut seeded = VecStore::default();
    for row in rows {
        seeded.append(row.clone()).expect("a VecStore append");
    }
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(seeded));
    let trace = Trace::new();
    let reconciler = RecordingReconciler::new(&trace);
    let mut out: Vec<u8> = Vec::new();
    let exit = status::run_with(
        &reconciler,
        &discovered.exec,
        &discovered.worktree,
        &store,
        &mut out,
    )
    .expect("status does not halt");
    assert_eq!(exit.code(), 0, "status exits 0");
    String::from_utf8(out).expect("status prints UTF-8")
}

/// The draw lines `cost` renders for `item` over `rows`: every line after the header that names a
/// window's draw (`five_hour: n=` / `seven_day: n=`).
fn cost_draws(rows: &[Event], item: &str) -> Vec<String> {
    cost::render(rows, item)
        .expect("cost renders the corpus case")
        .lines()
        .filter(|l| l.starts_with("five_hour: n=") || l.starts_with("seven_day: n="))
        .map(str::to_owned)
        .collect()
}

/// **Every flag beside its reading, on `status` as on `cost`** (AC-372: *"the `redkiln cost` and
/// `redkiln status` renderers print the flags alongside any reading that carries them"*).
///
/// Over three corpus journals that between them carry all three flags — `n9-low-sample`
/// (`low_sample`), `non-monotonic` (`non_monotonic`) and `n10-10-11` (`resolution_limited`) — every
/// draw line `cost` prints for `bug-1` is printed by `status` on `bug-1`'s own lines, byte for byte.
///
/// Red by: `status` printing no draw (the pre-`s8f` verb); `M19` (`status` renders a draw without
/// its flags); `M28` (the shared renderer drops `resolution_limited`, so neither surface carries it
/// and the premise's flag census fails).
#[test]
fn status_prints_every_flag_beside_its_reading() {
    let mut flags = [false; 3];
    for case in ["n9-low-sample", "non-monotonic", "n10-10-11"] {
        let rows = cost_rows(
            &crate::fixtures_dir()
                .join("cost")
                .join(case)
                .join("rows.jsonl"),
        );
        let draws = cost_draws(&rows, "bug-1");
        assert!(!draws.is_empty(), "PREMISE: {case} renders draws on cost");
        for draw in &draws {
            flags[0] |= draw.split(' ').any(|t| t == "low_sample");
            flags[1] |= draw.split(' ').any(|t| t == "non_monotonic");
            flags[2] |= draw.split(' ').any(|t| t == "resolution_limited");
        }
        let text = status_over(&rows);
        let mine: Vec<&str> = text
            .lines()
            .filter_map(|l| l.strip_prefix("bug-1\t"))
            .collect();
        for draw in &draws {
            assert!(
                mine.contains(&draw.as_str()),
                "{case}: `status` does not print cost's draw line {draw:?} on bug-1's lines:\n{text}"
            );
        }
    }
    assert_eq!(
        flags, [true; 3],
        "PREMISE: the three journals carry every flag between them"
    );
}

/// **No headroom line for an item with no reading** (ADR-0190 decision 5): an item whose journal
/// carries a `dispatch` row and no reading gets its roll-up line and nothing else, while an item in
/// the SAME journal that owns readings gets its draws — so the silence is the rule, not a `status`
/// that prints no draw for anyone.
///
/// Red by: `status` printing draws for no item (the premise half); a headroom or `no sample` line
/// printed for the item that owns no reading.
#[test]
fn status_prints_no_headroom_line_for_an_item_with_no_reading() {
    let mut rows = cost_rows(
        &crate::fixtures_dir()
            .join("cost")
            .join("n9-low-sample")
            .join("rows.jsonl"),
    );
    rows.push(Event::new(
        EventKind::Dispatch,
        serde_json::json!({
            "activity_id_at_version": "a@1",
            "attempt": 1,
            "item": "quiet-1",
            "step_id": "s",
        }),
    ));
    let text = status_over(&rows);
    let quiet: Vec<&str> = text
        .lines()
        .filter_map(|l| l.strip_prefix("quiet-1\t"))
        .collect();
    assert_eq!(
        quiet.len(),
        1,
        "the item with no reading prints its roll-up line alone:\n{text}"
    );
    for word in [
        "five_hour",
        "seven_day",
        "n=",
        "headroom",
        "no sample",
        "unkeyed",
    ] {
        assert!(
            !quiet[0].contains(word),
            "a headroom line for an item with no reading: {:?}",
            quiet[0]
        );
    }
    let loud = text
        .lines()
        .filter(|l| l.starts_with("bug-1\tfive_hour: n="))
        .count();
    assert!(
        loud >= 1,
        "PREMISE: the item that owns readings gets its draws in the same journal:\n{text}"
    );
}

/// A tier-1 `stage_transition` row, keyed to `item` when one is given (ADR-0182's interim key).
fn transition(item: Option<&str>, from: Option<&str>, to: &str) -> Event {
    let payload = match from {
        Some(from) => serde_json::json!({ "from": from, "to": to }),
        None => serde_json::json!({ "to": to }),
    };
    let row = Event::new(EventKind::StageTransition, payload);
    match item {
        Some(item) => row.with_item(item),
        None => row,
    }
}

/// `status`'s roll-up lines alone (the ones whose last field is `gates=`), in print order.
fn roll_up_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| l.rsplit('\t').next().is_some_and(|f| f.starts_with("gates=")))
        .collect()
}

/// **Each item folds over the rows keyed to it** (issue #6; ADR-0182 decision 1). Two items'
/// transitions, keyed by the interim key and carrying no payload `item`, give each item its own
/// transition.
///
/// Red by: the pre-fix partition, which read only the payload's `item`, found no item in this
/// journal and printed nothing; and by any partition that hands one item's keyed rows to the other.
#[test]
fn status_folds_each_item_over_the_rows_keyed_to_it() {
    let rows = [
        transition(Some("lane-a"), Some("triage"), "implement"),
        transition(Some("lane-b"), Some("triage"), "review"),
    ];
    let text = status_over(&rows);
    assert_eq!(
        roll_up_lines(&text),
        vec![
            "lane-a\ttriage->implement\tgates=0",
            "lane-b\ttriage->review\tgates=0"
        ],
        "each item's line carries its own transition:\n{text}"
    );
}

/// **An item's stage is seeded from its own keyed transitions**, never from the journal's last.
/// `lane-a`'s transition names no `from`, so the fold takes it from the seed; `lane-b`'s row is the
/// journal's last, and its stage must not reach `lane-a`'s line.
///
/// Red by: the pre-fix seed, the `to` of the LAST `stage_transition` in the whole journal.
#[test]
fn status_seeds_each_item_from_its_own_keyed_transitions() {
    let rows = [
        transition(Some("lane-a"), None, "implement"),
        transition(Some("lane-b"), Some("triage"), "review"),
    ];
    let text = status_over(&rows);
    let lane_a: Vec<&str> = roll_up_lines(&text)
        .into_iter()
        .filter(|l| l.starts_with("lane-a\t"))
        .collect();
    assert_eq!(lane_a.len(), 1, "PREMISE: lane-a has one line:\n{text}");
    assert!(
        lane_a.iter().all(|l| !l.contains("review")),
        "lane-a's line names lane-b's stage:\n{text}"
    );
}

/// **An item-less row of a `Carried` kind is read by every item** (ADR-0182 decision 4's inclusive
/// interim): neither item owns it, so both fold it.
///
/// Red by: a partition that drops item-less rows, or hands them to one item.
#[test]
fn status_reads_an_item_less_row_of_a_carried_kind_on_every_item() {
    let rows = [
        transition(Some("lane-a"), Some("triage"), "implement"),
        transition(Some("lane-b"), Some("triage"), "review"),
        transition(None, Some("unowned"), "closed"),
    ];
    let text = status_over(&rows);
    assert_eq!(
        roll_up_lines(&text),
        vec![
            "lane-a\tunowned->closed\tgates=0",
            "lane-b\tunowned->closed\tgates=0"
        ],
        "the item-less transition is folded by both items:\n{text}"
    );
}

/// **A kind that is not item-scoped names no item** (ADR-0182 decision 3: the memo is shared
/// across items by design), even when its payload carries an `item` member.
///
/// Red by: the pre-fix partition, which listed any row whose payload names an `item`.
#[test]
fn status_lists_no_item_for_a_kind_that_is_not_item_scoped() {
    let rows = [
        transition(Some("lane-a"), Some("triage"), "implement"),
        Event::new(EventKind::MemoHit, serde_json::json!({ "item": "ghost" })),
    ];
    let text = status_over(&rows);
    assert_eq!(
        roll_up_lines(&text),
        vec!["lane-a\ttriage->implement\tgates=0"],
        "only the item-scoped row's item is listed:\n{text}"
    );
}
