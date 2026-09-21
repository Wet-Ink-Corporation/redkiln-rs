//! **`doctor`** — Step 7d Slice 4. What is here is **cycle 3**: the rule table's anti-vacuity
//! floor, and nothing else.
//!
//! # Why a floor test exists at all, and why it is this slice's FIRST test
//!
//! §7d.14, at the head of the stage file: *"over an empty rule table AC-414's per-fixture join,
//! AC-415's per-id join and SM-121's dispatch count"* are **all vacuous**. Three criteria, each
//! with real instruments, each of which a `doctor` that checks nothing passes — AC-415's own
//! repair note records that it *"passed trivially on a doctor that performs no checks"*. RA-M-01's
//! answer is a floor that is not derived from the table it polices: the ids a criterion in the
//! register's §9, §12 or §14 names **by instrument**, joined against `RULES`.
//!
//! So this test is the one thing in the slice that a table-shaped hole cannot satisfy, and it lands
//! before the fixtures (cycle 4), the per-row rules (cycle 5) and the corpus run (cycle 6).

use std::collections::BTreeSet;

use rk_cli::doctor::rules::RULES;

/// **The twenty-nine register-named rule ids** — `s7d` §7d.8's own list, transcribed.
///
/// *"the **register-named floor** RA-M-01 asks the checkpoint to join is the subset a criterion in
/// the register's §9, §12 or §14 names by instrument … — **twenty-nine**, the rows AC-409, 412,
/// 413, 416–426, 486 and 488 demand plus the six AC-290, 306, 310, 314, 455 and 530 name
/// (D18-19)."*
///
/// **This is a FLOOR and not the table.** The table is thirty-eight; the nine outside this list —
/// the five `s7c`-ARMED halves, `RK-D-REBUILT-TIER2`, and RT-45's three — are *"outside the floor
/// and are asserted by their own tests"* (§7d.8), which are cycle 5's. 29 + 9 = 38, and the
/// partition closes.
///
/// **Transcribed and not derived, deliberately.** A floor computed from `RULES` would be a
/// restatement of the table rather than a check on it — the exact shape RA-M-01 exists to break.
/// The guard against the transcription itself rotting is [`FLOOR`]'s asserted length below:
/// §7d.8's sentence says *twenty-nine* in terms, so a list that lost an entry is red before any
/// membership is tested.
///
/// # 🛑 THIS LIST EXISTS THREE TIMES, AND THE THIRD COPY IS THE ONE TO KNOW ABOUT
///
/// §7d.8's prose sentence, Checkpoint 7d's own `FLOOR="…"` shell variable (§7d.17), and this
/// const. **Diffed all three at this commit** [measured 2026-09-21]: same 29 ids, no duplicates,
/// and the same order. Each polices `RULES` from its own side — the block greps the ids out of the
/// real binary's stdout, this joins them against the table in process — so a copy that gained or
/// lost an id on its own goes red.
///
/// **What none of the three catches is the same id dropping out of two of them at once.** That
/// takes two edits and would leave a criterion with no rule and nothing to say so. It is recorded
/// here rather than guarded, because the only guard would be a fourth copy.
const FLOOR: [&str; 29] = [
    "RK-D-GATE-OUTPUTS",
    "RK-D-HOOKS",
    "RK-D-STORE-ID",
    "RK-D-ROLE-UNRESOLVED",
    "RK-D-ROLE-AMBIGUOUS",
    "RK-D-CARD-TOOLS",
    "RK-D-CARD-TOOLS-ABSENT",
    "RK-D-TIER-LEAK",
    "RK-D-TIMEOUT",
    "RK-D-DANGLING-ROLE",
    "RK-D-DANGLING-SLOT",
    "RK-D-DANGLING-ATTACH",
    "RK-D-DEAD-DEF",
    "RK-D-UNFILLED-SLOT",
    "RK-D-ENV-DENYLIST",
    "RK-D-CLI-VERSION",
    "RK-D-TTL",
    "RK-D-FACET-UNKNOWN",
    "RK-D-FACET-VALUE",
    "RK-D-FACET-NEW",
    "RK-D-FACET-FACT",
    "RK-D-ADAPTER",
    "RK-D-UNREAD",
    "RK-D-REVIEW-REEXEC",
    "RK-D-QUARANTINE-LIVE",
    "RK-D-SINK-COUNT",
    "RK-D-ADVISORY-LEDGER",
    "RK-D-CORPUS-SHA",
    "RK-D-VERIFICATION-POLICY",
];

/// **RA-M-01 — every register-named rule id is in the table.**
///
/// AC-415 as amended: *"every id has at least one committed fixture configuration that the rule
/// actually rejects, and a rule with no rejecting fixture fails the build"* — **"and the id set is
/// non-empty and contains every id a criterion names"**. That amended clause is what this asserts;
/// the fixture half is `ac415_every_rule_id_has_a_rejecting_fixture`'s, at cycle 4.
///
/// # What is the instrument, and what is not
///
/// **The instrument is the JOIN**, and it is one-directional on purpose: every id in [`FLOOR`] is
/// in `RULES`. It says nothing about the nine ids outside the floor and nothing about fixtures, and
/// it is not the set equality — that is `ac415_…`'s, in both directions, against the fixture
/// directories.
///
/// The two structural assertions beside it are **not** the floor and are not decoration. AC-415's
/// GIVEN is *"the `doctor` rule table, which is data with a **stable id per rule**"*: a table
/// carrying one id twice satisfies every membership test above and still breaks the per-id join,
/// because two rules would answer to one fixture directory. And a table that is empty satisfies
/// the *"no duplicates"* half trivially, which is why the non-empty assertion is read first.
#[test]
fn doctor_rule_table_covers_every_register_named_row() {
    // The transcription's own guard, read before anything is joined against it. §7d.8 says
    // twenty-nine in terms; a shortened list would make every assertion below weaker in a way
    // nothing else in the slice would notice.
    assert_eq!(
        FLOOR.len(),
        29,
        "the register-named floor is TWENTY-NINE ids (`s7d` §7d.8). This list has {} — it has been \
         edited, and a floor that lost an entry stops policing the rule it was transcribed for",
        FLOOR.len()
    );
    let floor: BTreeSet<&str> = FLOOR.iter().copied().collect();
    assert_eq!(
        floor.len(),
        FLOOR.len(),
        "the floor list repeats an id, so it names fewer than twenty-nine distinct rows"
    );

    // AC-415 as amended: *"the id set is non-empty"*. Read FIRST, because every join below is
    // vacuously satisfiable over an empty table and so is the duplicate check.
    assert!(
        !RULES.is_empty(),
        "`RULES` is empty. Over an empty rule table AC-414's per-fixture join, AC-415's per-id \
         join and SM-121's dispatch count are ALL vacuous (`s7d` §7d.14) — a `doctor` that checks \
         nothing passes every one of them, which is the state AC-415's own repair note records"
    );

    let ids: Vec<&str> = RULES.iter().map(|r| r.id).collect();
    let distinct: BTreeSet<&str> = ids.iter().copied().collect();
    assert_eq!(
        distinct.len(),
        ids.len(),
        "the rule table carries the same id twice. AC-415's GIVEN is a table that is *\"data with a \
         stable id per rule\"*, and two rules answering to one id break the per-id join against \
         `fixtures/doctor/<id>/`: one directory, two rules, and no way to say which one it rejects. \
         Ids in table order: {ids:?}"
    );

    // THE JOIN. Every miss is collected, never the first — a failure that named one id would be
    // re-run once per missing row.
    let missing: Vec<&str> = FLOOR
        .iter()
        .copied()
        .filter(|id| !distinct.contains(id))
        .collect();
    assert!(
        missing.is_empty(),
        "{} of the twenty-nine register-named rule ids are not in `RULES`. Each is named by a \
         criterion's own instrument in the register's §9, §12 or §14, so a table missing one is a \
         criterion with no rule rather than a rule with no criterion:\n  {}\nTable has {} row(s).",
        missing.len(),
        missing.join("\n  "),
        ids.len()
    );
}
