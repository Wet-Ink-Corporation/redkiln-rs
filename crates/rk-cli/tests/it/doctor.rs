//! **`doctor`** — Step 7d Slice 4. What is here is **cycle 3's** anti-vacuity floor and **cycle
//! 4's** fixture materialiser.
//!
//! # 🚧 Cycle 4 claims NO test, and that is §7d.14's own reading rather than a shortcut
//!
//! Cycle 4 lands the thirty-eight rejecting fixture directories and `clean/`. It does **not** land
//! `ac415_every_rule_id_has_a_rejecting_fixture`, because §7d.14's red for that test names the
//! failure mode landing it here would be: *"a weaker red **joins names without running the
//! rule**."* Every `Rule::check` is still the `unbuilt` placeholder until cycle 5, so a test
//! landed now could assert the set equality and nothing else — which is that weaker red exactly,
//! under a name that claims a *rejecting* fixture. It lands at cycle 5, after the last predicate,
//! where its middle clause — *"each fixture run through its own rule only yields ≥ 1 finding"* —
//! can actually run.
//!
//! The set equality was **measured** in cycle 4's body instead: 38 ids, 38 rejecting directories,
//! no rule without a directory, no directory without a rule, `clean/` the one named exemption.
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
/// partition closes — re-measured 2026-09-21 off the table's first column, and the thirty-eight
/// rejecting directories under `fixtures/doctor/` are one per id.
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
/// the fixture half is `ac415_every_rule_id_has_a_rejecting_fixture`'s, at cycle 5 (see the module
/// header for why it is not cycle 4's).
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

// ---------------------------------------------------------------------------- the materialiser
//
// `s7d` §7d.13: *"each per-row test (and `ac415_…`) **materialises** the directory into a
// `tempfile` repository (`git init`, `dot-git/` → `.git/`, the store built through
// `rk_store::conn::Writer::open` and `identity::initialise` from the description) before running
// the rule, so a fixture that materialises to nothing is red on the rule's own finding count."*
//
// 🛑 **IT ASSERTS ITS OWN OUTPUT IS LIVE, AND THAT IS WHY THERE IS NO SEPARATE TEST FOR IT.**
// A materialiser that silently produced an empty directory would make every rule in the slice
// report nothing and every per-row test red for a reason that has nothing to do with the rule —
// or worse, would make an absence-assertion pass having read nothing. `rk-kernel`'s `rs_files`
// sets the precedent one level down: *"Panics on an absent root rather than returning nothing: a
// scan over a missing directory reports a pass having read nothing."* The panics below are that
// rule applied to a tree rather than to a file list.
//
// It is deliberately NOT a `#[test]`. §7d.16 item 4 fixes this slice's test count at thirty-eight
// named tests, and a self-control on test infrastructure is not one of them — `tests/it/main.rs`'s
// own header records the same division for `support.rs`.

/// Materialise one `fixtures/doctor/<name>/` into a real git repository.
///
/// Returns the temp directory; the caller keeps it alive for as long as it reads the tree.
///
/// **What it does, in order**: `git init` a fresh repository with one commit (so the tree has a
/// root commit an identity can be stamped against), copy every tracked file across, rename
/// `dot-git/` over `.git/`, and leave the `store/` descriptions where the rule's own builder will
/// read them.
///
/// # Panics
/// If the fixture directory does not exist, if it carries no `.redkiln/config.toml`, or if the
/// materialised tree is missing either of those afterwards. Each is a fixture that would otherwise
/// be read as a configuration with nothing wrong with it.
#[expect(
    dead_code,
    reason = "cycle 4 lands the materialiser beside the fixtures it materialises; its callers are \
              `ac415_every_rule_id_has_a_rejecting_fixture` and the twenty-six family tests, all \
              of which are cycle 5's. The expectation goes UNFULFILLED on the first caller, which \
              is what says this attribute can go."
)]
pub fn materialise(name: &str) -> tempfile::TempDir {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("doctor")
        .join(name);
    assert!(
        src.is_dir(),
        "the doctor fixture {name} does not exist at {} — a rule run over a missing directory \
         reports nothing and reads as a configuration with no defect in it",
        src.display()
    );
    assert!(
        src.join(".redkiln").join("config.toml").is_file(),
        "the doctor fixture {name} carries no `.redkiln/config.toml` — every rule reads it, so \
         there would be no table to run"
    );

    let dir = crate::temp_repo();
    copy_tree(&src, dir.path());

    // `dot-git/` → `.git/`. Git refuses to TRACK a nested `.git/`, which is why the fixture spells
    // it this way and why the rename is the materialiser's job rather than the author's.
    let staged = dir.path().join("dot-git");
    if staged.is_dir() {
        let git_dir = dir.path().join(".git");
        merge_into(&staged, &git_dir);
        std::fs::remove_dir_all(&staged).expect("the staged dot-git is removed");
    }

    assert!(
        dir.path().join(".redkiln").join("config.toml").is_file(),
        "{name} materialised without its `.redkiln/config.toml` — the copy read nothing"
    );
    assert!(
        dir.path().join(".git").is_dir(),
        "{name} materialised without a `.git/` — `temp_repo` did not initialise, and every rule \
         that reads the common dir would report an absence that is the harness's, not the \
         fixture's"
    );
    dir
}

/// Copy `src`'s contents into `dst`, recursively. `dst` exists already.
fn copy_tree(src: &std::path::Path, dst: &std::path::Path) {
    for entry in std::fs::read_dir(src).expect("the fixture directory is readable") {
        let entry = entry.expect("a directory entry");
        let to = dst.join(entry.file_name());
        if entry.path().is_dir() {
            std::fs::create_dir_all(&to).expect("a directory is created");
            copy_tree(&entry.path(), &to);
        } else {
            std::fs::copy(entry.path(), &to).expect("a file copies");
        }
    }
}

/// Merge `src`'s contents into an EXISTING `dst`, overwriting file by file.
///
/// **A merge and not a rename**, because `.git/` already exists — `temp_repo` initialised it — and
/// renaming over it would throw away the repository the fixture is being materialised into. A
/// fixture's `dot-git/hooks/pre-commit` has to land *beside* git's own `HEAD` and `objects/`, not
/// instead of them.
fn merge_into(src: &std::path::Path, dst: &std::path::Path) {
    std::fs::create_dir_all(dst).expect("the destination exists");
    for entry in std::fs::read_dir(src).expect("the staged directory is readable") {
        let entry = entry.expect("a directory entry");
        let to = dst.join(entry.file_name());
        if entry.path().is_dir() {
            merge_into(&entry.path(), &to);
        } else {
            std::fs::copy(entry.path(), &to).expect("a file copies");
        }
    }
}
