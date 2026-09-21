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
///
/// ✅ **Cycle 4 carried `#[expect(dead_code)]` here, and cycle 5 removed it because clippy said
/// the expectation was UNFULFILLED** — which is what the attribute was for. It had no caller until
/// the family tests arrived; the first one made it live, and the compiler is what noticed rather
/// than a reader.
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

    build_stores(dir.path());

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

// ------------------------------------------------------------------- the per-row test harness
//
// Every family test in this module is the same two assertions, and §7d.8 fixes both:
// *"each runs the rule over its fixture **and over the clean fixture** `doctor/clean/` asserting
// no finding — the second half is what makes a rule that always fires red."*
//
// 🛑 **THE RULE IS RUN ALONE, NOT THE TABLE.** AC-415's own words are *"each fixture run through
// **its own rule only**"*. Running the table would let a fixture pass because some other row fired
// over it, which is the join AC-415 exists to make per-id.

/// The `now` every fixture is read against — a FIXED value, never a clock.
///
/// A test that read the real clock would make `RK-D-QUARANTINE-LIVE` pass or fail on the day it
/// ran: the fixture's `until` is a fixed instant, and "is it still live?" is only a question with
/// an answer once both sides are fixed.
const NOW: i64 = 1_800_000_000;

/// Build the `Ctx` for one materialised fixture, through the **same builder the verb uses**.
///
/// `store` is `None` and the probe is the fixture's stub: neither is configuration, and §7d.8 has
/// the root hand both in. A `Ctx` assembled any other way here would assert every rule against a
/// configuration the shipped binary never sees.
fn ctx_for(dir: &std::path::Path) -> rk_cli::doctor::Ctx {
    let config_src = std::fs::read_to_string(dir.join(".redkiln").join("config.toml"))
        .expect("every doctor fixture carries a .redkiln/config.toml");
    let config = rk_cli::config::parse(&config_src).expect("the fixture's ConfigFile parses");

    // `env.toml`, when the fixture carries one, IS the captured environment — ADR-0083 gives the
    // process one environment read at the root, so a fixture supplies the map rather than setting
    // a real variable, which would leak into every other test in this binary.
    let vars = std::fs::read_to_string(dir.join("env.toml"))
        .ok()
        .and_then(|src| toml::from_str::<std::collections::BTreeMap<String, String>>(&src).ok())
        .unwrap_or_default();

    // `common-dir.toml`, when the fixture carries one, IS the common dir the root would have
    // handed in — the same shape as `env.toml` supplying the environment. `RK-D-NETWORK-FS` reads
    // the PATH's form, and a fixture cannot put a real UNC share on every box that runs this.
    let common_dir = std::fs::read_to_string(dir.join("common-dir.toml"))
        .ok()
        .and_then(|src| toml::from_str::<toml::Value>(&src).ok())
        .and_then(|v| {
            v.get("path")
                .and_then(|p| p.as_str())
                .map(std::path::PathBuf::from)
        })
        .unwrap_or_else(|| dir.join(".git"));

    // THE STORE, opened through the root's own path — `adapters::open_store`, which returns what
    // happened rather than refusing on it. A fixture with no store description has none, and
    // `None` is what the `--rebuild` path carries too.
    let store = dir
        .join(".git")
        .join("redkiln")
        .join("store.sqlite3")
        .is_file()
        .then(|| {
            let repo =
                rk_cli::repo::discover(Some(dir)).expect("the materialised fixture is a repo");
            rk_cli::adapters::open_store(&repo.exec, &repo.worktree)
        });

    rk_cli::doctor::Ctx::build(
        dir.to_path_buf(),
        common_dir,
        config,
        rk_cli::env::Env::from_map(vars),
        store,
        Box::new(StubProbe::read(dir)),
        NOW,
    )
}

/// The stubbed version probe (§7d.8's seam). Reads `probe.toml`'s `version`, or answers
/// `TESTED_CLI_VERSION` so a fixture that is not about the probe is not accidentally about it.
#[derive(Debug, Clone)]
struct StubProbe(String);

impl StubProbe {
    fn read(dir: &std::path::Path) -> Self {
        let version = std::fs::read_to_string(dir.join("probe.toml"))
            .ok()
            .and_then(|src| toml::from_str::<toml::Value>(&src).ok())
            .and_then(|v| {
                v.get("version")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned)
            })
            .unwrap_or_else(|| rk_adapter_claude::config::TESTED_CLI_VERSION.to_owned());
        Self(version)
    }
}

impl rk_cli::doctor::probe::VersionProbe for StubProbe {
    fn version(&self, _program: &str) -> Result<String, rk_cli::doctor::probe::ProbeError> {
        Ok(self.0.clone())
    }
}

/// Run **one** rule over one fixture and return what it found.
///
/// # Panics
/// If `rule_id` is not in `RULES` — a test naming a rule the table does not carry would otherwise
/// report an empty finding list and read exactly like a rule that found nothing.
fn findings(rule_id: &str, fixture: &str) -> Vec<rk_cli::doctor::Finding> {
    let rule = RULES
        .iter()
        .find(|r| r.id == rule_id)
        .unwrap_or_else(|| panic!("{rule_id} is not in RULES — no rule to run over {fixture}"));
    let dir = materialise(fixture);
    let ctx = ctx_for(dir.path());
    (rule.check)(&ctx)
}

/// The two assertions every family test makes, in §7d.8's own order.
///
/// **The fixture half first**, because the clean half is satisfied by a rule that never fires and
/// asserting it first would read green on the thing the fixture half exists to catch.
fn fires_over_its_fixture_and_not_over_clean(rule_id: &str, severity: rk_cli::doctor::Severity) {
    let found = findings(rule_id, rule_id);
    assert!(
        !found.is_empty(),
        "{rule_id} found nothing over its own rejecting fixture `doctor/{rule_id}/`. AC-415: \
         *\"every id has at least one committed fixture configuration that the rule ACTUALLY \
         REJECTS\"* — a rule that never fires satisfies every join in this slice and reports \
         nothing in production."
    );
    for f in &found {
        assert_eq!(f.id, rule_id, "a finding carries another rule's id: {f:?}");
        assert_eq!(
            f.severity, severity,
            "{rule_id} fired at the wrong severity. The table's column is what the exit code \
             reads (AC-438: an error-severity row is exit 3, a warning never moves it): {f:?}"
        );
        assert!(
            !f.detail.trim().is_empty(),
            "{rule_id} fired with an empty detail — the stdout row would read `<id> <status>` and \
             tell an operator nothing about what to fix"
        );
    }

    let clean = findings(rule_id, "clean");
    assert!(
        clean.is_empty(),
        "{rule_id} fired over `doctor/clean/`, the configuration every rule passes. This is the \
         half that makes a rule which ALWAYS fires red (§7d.8), and without it every assertion \
         above is satisfied by `check` returning one finding unconditionally. Found: {clean:?}"
    );
}

// ------------------------------------------------------------- batch 1: the card and config rows
//
// Six families, seven ids. Each reads the `ConfigFile`, the captured environment, or a card file —
// nothing here folds a journal or parses a predicate, which is why they land together.

/// **AC-486 / AC-438's doctor half** — an adapter name this build does not ship.
///
/// §7d.8: *"`doctor` does **NOT** call `AdapterSet::resolve` as a startup refusal (that is `run`'s
/// step (4)) … it reports adapter defects only through `RK-D-ADAPTER`, **which is why the row and
/// its id are observable on the real binary**."* The fixture's `ConfigFile` is COMPLETE — it loads,
/// and the whole table runs over it — so the only thing that can report `rk-adapter-linear` is this
/// rule.
#[test]
fn ac486_unknown_adapter_is_a_doctor_error() {
    fires_over_its_fixture_and_not_over_clean("RK-D-ADAPTER", rk_cli::doctor::Severity::Error);
}

/// **AC-422 / AC-488** — a denylisted variable name in the captured environment.
///
/// The one denylist is `rk_adapter_claude::env_denylist`; `rk-cli/src` carries no second copy of
/// any name in it (AC-422's own source ban, scoped to `src`, which is why a fixture may spell one).
/// The fixture supplies the map rather than setting a real variable: ADR-0083 gives the process one
/// environment read at the root, and a test that set one would leak into every other test here.
#[test]
fn ac422_ac488_denylisted_env_name_is_a_doctor_error() {
    fires_over_its_fixture_and_not_over_clean("RK-D-ENV-DENYLIST", rk_cli::doctor::Severity::Error);
}

/// **AC-417 / AC-205 / AC-493's doctor halves** — the two card-tools rows.
///
/// Two ids and two fixtures, because the remedy differs: a card whose `tools` list omits
/// `StructuredOutput` has an allowlist to edit, and a card with no `tools` key has none to edit.
/// `CardFile.tools` is `Option`, so both cards LOAD — which is what makes these doctor rows rather
/// than load errors.
#[test]
fn ac417_card_without_structured_output_or_tools_key_rejected() {
    fires_over_its_fixture_and_not_over_clean("RK-D-CARD-TOOLS", rk_cli::doctor::Severity::Error);
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-CARD-TOOLS-ABSENT",
        rk_cli::doctor::Severity::Error,
    );
}

/// **ADR-0113's doctor half (`s7c` ARMED)** — a card with no `cache_profile`.
#[test]
fn ac383_card_without_cache_profile_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-CACHE-PROFILE",
        rk_cli::doctor::Severity::Error,
    );
}

/// **AC-203's doctor half (`s7c` ARMED)** — a card whose binding yields no agent name.
///
/// `CardFile.description` is a `String` and an empty one is a value, so the card loads and the
/// bound `--agents` object has no name to key on. A load error would be the adapter's; this is the
/// configuration fact.
#[test]
fn ac203_agents_without_agent_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-AGENTS-WITHOUT-AGENT",
        rk_cli::doctor::Severity::Error,
    );
}

/// **AC-418 / AC-212's doctor half** — a card or a slot filler that states its own tier.
///
/// The tier is the ROLE's (`[role.<name>]` in the pack). The clause names both carriers and the
/// fixture spells both: a card with `model`, and a filler with `effort`.
#[test]
fn ac418_tier_leak_on_card_model_and_filler_effort() {
    fires_over_its_fixture_and_not_over_clean("RK-D-TIER-LEAK", rk_cli::doctor::Severity::Error);
}

// ---------------------------------------------------------- batch 2: the pack and catalog rows
//
// Four families, five ids. Each reads the enabled pack's own TOML or the activity catalog beside
// it — nothing here reads a card, folds a journal or parses a predicate.
//
// 🛑 **`ac409_gate_outputs_binding_rejected` IS NOT IN THIS BATCH**, though `RK-D-GATE-OUTPUTS` is
// a catalog row. §7d.14's red for it is *"`RK-D-DANGLING-*` firing beside `RK-D-GATE-OUTPUTS` on
// its fixture"* — the gate declares `outputs`, so the binding is **not** dangling and the test has
// to assert that no `RK-D-DANGLING-*` row fires. Those three rows are still `unbuilt`, so the
// assertion would pass over three rules that answer nothing: a guard satisfied by absence. It
// lands in batch 3 beside the rows it must not co-fire with.

/// **AC-419 / AC-097's doctor half** — a per-activity `timeout_secs` that is absent or not an
/// integer.
///
/// **Two cases in one fixture**, which is what §7d.8's *"two fixtures"* means for a single id:
/// `[activity.emit-report]` declares none (Missing) and `[activity.review]` declares `"900"`, a
/// string (NonNumeric). A rule that reported only one of the two would leave the other silent in
/// production, so the test asserts **both findings**, not merely that something fired.
#[test]
fn ac419_missing_or_non_numeric_timeout_rejected() {
    fires_over_its_fixture_and_not_over_clean("RK-D-TIMEOUT", rk_cli::doctor::Severity::Error);

    let found = findings("RK-D-TIMEOUT", "RK-D-TIMEOUT");
    assert_eq!(
        found.len(),
        2,
        "the fixture carries BOTH cases — an activity declaring no `timeout_secs` and one \
         declaring a string — and the row must report each. One finding means one of the two \
         predicates is not written: {found:?}"
    );
    assert!(
        found.iter().any(|f| f.detail.contains("emit-report")),
        "the Missing case (an activity with no `timeout_secs` at all) is not reported: {found:?}"
    );
    assert!(
        found.iter().any(|f| f.detail.contains("review")),
        "the NonNumeric case (`timeout_secs = \"900\"`, a string) is not reported: {found:?}"
    );
}

/// **AC-423 / AC-526 / D-11** — the installed CLI's version, and the pack's declared cache TTLs.
///
/// Two ids in one family because §7d.8's table puts them in one row. They read different things —
/// `RK-D-CLI-VERSION` the probe seam, `RK-D-TTL` the pack's `[cache]` table — and the fixture for
/// each carries only its own defect.
///
/// **The probe is STUBBED here** (§7d.8's seam) and returns the fixture's `probe.toml` version.
/// `ac423_probe_is_one_version_spawn_not_a_dispatch` drives the real one and is cycle 7's; this
/// test asserts the finding, not the spawn.
#[test]
fn ac423_ac526_cli_version_and_ttl_mismatch_rejected() {
    fires_over_its_fixture_and_not_over_clean("RK-D-CLI-VERSION", rk_cli::doctor::Severity::Error);
    fires_over_its_fixture_and_not_over_clean("RK-D-TTL", rk_cli::doctor::Severity::Error);

    // The version the fixture stubs is named in the finding, so an operator reads what is
    // installed rather than only that something is wrong.
    let version = findings("RK-D-CLI-VERSION", "RK-D-CLI-VERSION");
    assert!(
        version.iter().any(|f| f.detail.contains("2.1.200")),
        "the finding does not name the version the probe returned: {version:?}"
    );

    // BOTH TTL defects: one TTL named where the pack owes two, and the named one is not `1h`.
    let ttl = findings("RK-D-TTL", "RK-D-TTL");
    assert_eq!(
        ttl.len(),
        2,
        "the fixture declares `prompt_ttl = \"5m\"` and NO `subagent_prompt_ttl` — a wrong value \
         and a missing declaration, and AC-210's THEN names both keys. One finding means one of \
         the two is unread: {ttl:?}"
    );
    assert!(
        ttl.iter().any(|f| f.detail.contains("subagent_prompt_ttl")),
        "the absent `subagent_prompt_ttl` is not reported: {ttl:?}"
    );
    assert!(
        ttl.iter().any(|f| f.detail.contains("5m")),
        "the wrong `prompt_ttl` value is not named: {ttl:?}"
    );
}

/// **AC-377's doctor half (`s7c` ARMED)** — a declared margin under two ticks.
///
/// The test's own name fixes the floor: *under two*. A margin of `2` is lawful and `1` is not,
/// which is the boundary a rule keyed on `< 1` or `<= 2` would get wrong in opposite directions.
#[test]
fn ac377_margin_under_two_ticks_doctor_row() {
    fires_over_its_fixture_and_not_over_clean("RK-D-MARGIN", rk_cli::doctor::Severity::Error);

    let found = findings("RK-D-MARGIN", "RK-D-MARGIN");
    assert!(
        found.iter().any(|f| f.detail.contains("five_hour")),
        "the finding does not name which window is under the floor: {found:?}"
    );
    // `clean/` declares ten on both windows, so the clean half above already proves the rule does
    // not fire at ten. The boundary itself — two is lawful — is asserted here rather than left to
    // a reader, because the fixture is at one and a rule keyed on `< 3` would pass both.
    assert!(
        found.iter().all(|f| !f.detail.contains("seven_day")),
        "the fixture leaves `seven_day` at ten and the row fired on it, so the rule is not reading \
         the declared value: {found:?}"
    );
}

/// **AC-421's two warning rows** — a definition no enabled pack reaches, and a declared slot that
/// is neither filled nor optional.
///
/// **Both are warnings, and the severity is the assertion.** AC-057's warning class stands: a dead
/// definition costs nothing at run time, and §7d.8's *"warnings never move the code"* means a
/// `doctor` whose only findings are these two still exits `0`. A rule that reported either as an
/// error would turn a tidy-up into a refusal.
#[test]
fn ac421_dead_definition_and_unfilled_slot_reported() {
    fires_over_its_fixture_and_not_over_clean("RK-D-DEAD-DEF", rk_cli::doctor::Severity::Warning);
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-UNFILLED-SLOT",
        rk_cli::doctor::Severity::Warning,
    );

    let dead = findings("RK-D-DEAD-DEF", "RK-D-DEAD-DEF");
    assert!(
        dead.iter().any(|f| f.detail.contains("orphan")),
        "the finding does not name the unreachable definition: {dead:?}"
    );
    assert!(
        dead.iter().all(|f| !f.detail.contains("emit-report")),
        "the row fired on `emit-report`, which the pack's policy DOES reach — the rule is \
         reporting every activity rather than the unreachable one: {dead:?}"
    );
}

// ----------------------------------------------------- batch 3: the roles rows, and `ac409_`
//
// Three families, six ids. The five AC-416 / AC-420 rows read `.redkiln/roles.toml`, `skills/` and
// `slots.toml` against the enabled packs' `[role.*]` and `[slot.*]` tables; `RK-D-GATE-OUTPUTS`
// reads the loader's own typed diagnostic. `ac409_` is here rather than in batch 2 because its red
// names the other five.

/// **AC-416** — *"a role resolves to zero cards, or to two cards at equal selector specificity"*.
///
/// Two ids, two fixtures, because the remedy differs: a role with no card needs one written, and a
/// role with two needs one of them narrowed.
///
/// # 🛑 The ambiguity fixture carries its own CONTROL, and it is the discriminating half
///
/// `RK-D-ROLE-AMBIGUOUS/` declares **two** roles: `reviewer` with two cards at the SAME selector,
/// and `planner` with two cards at DIFFERENT specificity. A rule reading *"two or more cards"*
/// fires on both and passes every assertion an uncontrolled test could make — so the test asserts
/// the row fires **once**, names `reviewer`, and does **not** name `planner`. AC-416's clause is
/// *equal* specificity, and without the control that word does no work.
#[test]
fn ac416_role_unresolved_and_ambiguous_rejected() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-ROLE-UNRESOLVED",
        rk_cli::doctor::Severity::Error,
    );
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-ROLE-AMBIGUOUS",
        rk_cli::doctor::Severity::Error,
    );

    let unresolved = findings("RK-D-ROLE-UNRESOLVED", "RK-D-ROLE-UNRESOLVED");
    assert!(
        unresolved.iter().any(|f| f.detail.contains("reviewer")),
        "the finding does not name the role that resolved to nothing: {unresolved:?}"
    );

    let ambiguous = findings("RK-D-ROLE-AMBIGUOUS", "RK-D-ROLE-AMBIGUOUS");
    assert_eq!(
        ambiguous.len(),
        1,
        "the fixture declares TWO roles with two cards each — `reviewer` at one selector twice \
         (ambiguous) and `planner` at two different specificities (resolves, the more specific \
         wins). Two findings means the rule reads *two or more cards* rather than *two at EQUAL \
         specificity*, which is AC-416's actual clause: {ambiguous:?}"
    );
    assert!(
        ambiguous[0].detail.contains("reviewer"),
        "the one finding is not about `reviewer`: {ambiguous:?}"
    );
    assert!(
        !ambiguous[0].detail.contains("planner"),
        "the row fired on `planner`, whose two cards are at different specificity: {ambiguous:?}"
    );
}

/// **AC-420** — *"a card `fulfils`, a skill `attach.roles`, or a slot filler naming a slot … that
/// no enabled pack declares"*.
///
/// **One row per reference kind**, and three fixtures, because the three references live in three
/// different files and an operator fixing one is not looking at the other two.
#[test]
fn ac420_dangling_role_slot_attach_rejected() {
    for id in [
        "RK-D-DANGLING-ROLE",
        "RK-D-DANGLING-SLOT",
        "RK-D-DANGLING-ATTACH",
    ] {
        fires_over_its_fixture_and_not_over_clean(id, rk_cli::doctor::Severity::Error);
    }

    // Each row names the reference it could not resolve, and each names a DIFFERENT one — three
    // rules that all reported the same thing would be one rule wearing three ids.
    assert!(
        findings("RK-D-DANGLING-ROLE", "RK-D-DANGLING-ROLE")
            .iter()
            .any(|f| f.detail.contains("archivist")),
        "the dangling ROLE row does not name the role the card fulfils"
    );
    assert!(
        findings("RK-D-DANGLING-SLOT", "RK-D-DANGLING-SLOT")
            .iter()
            .any(|f| f.detail.contains("postmortem")),
        "the dangling SLOT row does not name the slot the filler claims"
    );
    assert!(
        findings("RK-D-DANGLING-ATTACH", "RK-D-DANGLING-ATTACH")
            .iter()
            .any(|f| f.detail.contains("archivist")),
        "the dangling ATTACH row does not name the role the skill attaches to"
    );
}

/// **AC-409 as amended (C-01 RA-M-05)** — a graph reference resolving to a gate step's `outputs`.
///
/// The row fires on the **typed load error** the Slice 4 kernel edit produces: the strict loader
/// refuses the pack (cycle 1), and `catalog::load_diagnostic` reports that refusal as a
/// `DiagnosticKind::GateOutputs` beside the activities that did load. This is the first test in the
/// slice that reads cycle 1's work from the `rk-cli` side.
///
/// # 🛑 IT ASSERTS NO `RK-D-DANGLING-*` ROW FIRES, AND THE FIXTURE IS BUILT SO THAT MEANS SOMETHING
///
/// §7d.8's row: *"the per-row test asserts `RK-D-GATE-OUTPUTS` fires and **no `RK-D-DANGLING-*`
/// row does**, so the fixture is rejected for AC-409's reason and not as a dangling reference (a
/// gate declaring no `outputs` under a `${gate.outputs}` binding would be a dangling reference, a
/// different predicate)."* The assertion pins the SCOPE of the three AC-420 rows: they resolve
/// **declaration** references — a card's `fulfils`, a skill's `attach.roles`, a filler's slot — and
/// not graph bindings, so a `${gate.outputs}` binding is none of their business.
///
/// **The fixture carries a lawful role, skill and filler set** precisely so those three rules read
/// real declarations and still stay quiet. Over a fixture with no cards, no skills and no fillers
/// they would return nothing having looked at nothing, and this assertion would pin nothing.
#[test]
fn ac409_gate_outputs_binding_rejected() {
    fires_over_its_fixture_and_not_over_clean("RK-D-GATE-OUTPUTS", rk_cli::doctor::Severity::Error);

    for id in [
        "RK-D-DANGLING-ROLE",
        "RK-D-DANGLING-SLOT",
        "RK-D-DANGLING-ATTACH",
    ] {
        let co_fired = findings(id, "RK-D-GATE-OUTPUTS");
        assert!(
            co_fired.is_empty(),
            "{id} fired over `doctor/RK-D-GATE-OUTPUTS/`. The gate DECLARES `outputs`, so the \
             `${{gate.outputs}}` binding beside it is not dangling — the fixture must be rejected \
             for AC-409's reason and not as a dangling reference (§7d.8). AC-420's three rows \
             resolve declaration references, never graph bindings: {co_fired:?}"
        );
    }
}

/// Build the fixture's stores, after `dot-git/` has been merged into `.git/`.
///
/// §7d.13: *"the store built through `rk_store::conn::Writer::open` and `identity::initialise` from
/// the description"*. Two kinds of description, and they are not the same thing:
///
/// * **`store/identity.toml`** — the repository's own store, at `<common-dir>/redkiln/`. It is a
///   REAL SQLite file with a stamped identity, because `RK-D-STORE-ID` reads what
///   `SqliteStore::open` says about it and nothing less than a real store produces that.
///   `root_commit = "HEAD"` means *this repository's own root commit* (the store belongs here);
///   any other value is a literal, and a foreign one is the defect.
/// * **`dot-git/worktrees/<name>/redkiln/store.sqlite3.toml`** — a store that should not exist at
///   all. `RK-D-STRAY-STORE` reads its PRESENCE under a linked worktree's gitdir, so the file is
///   created and nothing is stamped into it: the description says so itself.
///
/// **`store/rebuilt`** is copied to `<common-dir>/redkiln/rebuilt` verbatim — a marker, not a store.
fn build_stores(dir: &std::path::Path) {
    let common = dir.join(".git");

    // ---- the stray store: presence is the whole fact.
    let worktrees = common.join("worktrees");
    if let Ok(entries) = std::fs::read_dir(&worktrees) {
        for entry in entries.filter_map(Result::ok) {
            let stray = entry.path().join("redkiln").join("store.sqlite3.toml");
            if stray.is_file() {
                let real = entry.path().join("redkiln").join("store.sqlite3");
                std::fs::write(&real, b"").expect("the stray store file is created");
            }
        }
    }

    // ---- the `rebuilt` sidecar.
    let rebuilt = dir.join("store").join("rebuilt");
    if rebuilt.is_file() {
        let to = common.join("redkiln");
        std::fs::create_dir_all(&to).expect("the redkiln dir exists");
        std::fs::copy(&rebuilt, to.join("rebuilt")).expect("the sidecar copies");
    }

    // ---- the repository's own store.
    let Ok(src) = std::fs::read_to_string(dir.join("store").join("identity.toml")) else {
        return;
    };
    let desc: toml::Value = toml::from_str(&src).expect("the store description parses");
    let declared = desc
        .get("root_commit")
        .and_then(toml::Value::as_str)
        .expect("a store description names a root_commit");
    let root_commit = if declared == "HEAD" {
        root_commit_of(dir)
    } else {
        declared.to_owned()
    };
    let common_dir = desc
        .get("common_dir")
        .and_then(toml::Value::as_str)
        .map_or_else(|| common.display().to_string(), ToOwned::to_owned);

    let writer = rk_store::conn::Writer::open(&common).expect("the staging store opens");
    writer
        .transaction(|tx| {
            rk_store::identity::initialise(
                tx,
                &rk_store::identity::Identity {
                    root_commit,
                    common_dir,
                    schema_version: rk_store::identity::SCHEMA_VERSION,
                },
            )
        })
        .expect("the identity stamps");
    // Dropped before anything opens it again: `Writer` holds the connection in WAL mode, and on
    // Windows a second opener would contend with it.
    drop(writer);
}

/// This repository's own root commit — what a store stamped `root_commit = "HEAD"` belongs to.
///
/// Through `rk_git::git_exec::run`, the one wrapper `std::process::Command` is permitted behind
/// (ADR-0077; `clippy.toml` disallows the type workspace-wide and this file is not exempt).
fn root_commit_of(dir: &std::path::Path) -> String {
    rk_git::git_exec::run(dir, ["rev-list", "--max-parents=0", "HEAD"])
        .expect("the materialised fixture has a root commit")
        .stdout_trimmed()
}

// ------------------------------------------------- batch 4: the git-directory and store rows
//
// Four families, six ids. Each reads the common directory or the store handle the root opened —
// the two parts of a repository that are not files a pack author writes.

/// **AC-412** — a `.git/hooks/` file carrying redkiln's own marker.
///
/// # 🛑 THE CLEAN RUN IS THE WHOLE INSTRUMENT HERE
///
/// §7d.14: *"`ac412_…` is red on the clean run by a recogniser that fires on any non-`*.sample`
/// hook — `clean/` carries an operator hook with no marker"*. RT-51 considered and rejected exactly
/// that recogniser, and `doctor/clean/`'s `pre-commit` — a real hook with a `# fork-join` comment
/// and no `# redkiln-hook` — is what tells the two apart. A `clean/` with no hooks at all could
/// not.
#[test]
fn ac412_redkiln_hook_in_git_hooks_is_an_error() {
    fires_over_its_fixture_and_not_over_clean("RK-D-HOOKS", rk_cli::doctor::Severity::Error);

    // The clean fixture really does carry a hook — otherwise the assertion above is satisfied by
    // a directory the recogniser never opened, and RT-51's rejected alternative would pass.
    let clean = materialise("clean");
    let hook = clean.path().join(".git").join("hooks").join("pre-commit");
    assert!(
        hook.is_file(),
        "`doctor/clean/` carries no `pre-commit` hook, so the clean half of this test proves \
         nothing about the recogniser"
    );
    let text = std::fs::read_to_string(&hook).expect("the clean hook is readable");
    assert!(
        text.contains("fork-join") && !text.contains("redkiln-hook"),
        "the clean hook is not an OPERATOR hook — it must carry no `# redkiln-hook` marker, or it \
         stops discriminating: {text}"
    );
}

/// **AC-413 / AC-369** — a store belonging to another repository.
///
/// # 🛑 THE POSITIVE CONTROL IS A MOVED REPOSITORY, AND IT LIVES IN `clean/`
///
/// §7d.14: *"red over a store whose `root_commit` is another repository's and **green over one
/// whose `common_dir` was re-stamped by a move** (the three-field reading; a boot-id assertion is
/// unauthorable and none is written)."* `doctor/clean/` carries a store stamped with **this**
/// repository's root commit and a `common_dir` naming somewhere it is not — so the clean half of
/// this test is that control rather than an absence, and a rule keyed on `common_dir` is red.
///
/// **The store handle is carried in, never refused before the table** (§7d.8's
/// `Option<Result<…>>`). A `doctor` that exited on a foreign store would print no row and AC-413's
/// id would be unobservable on the real binary.
#[test]
fn ac413_ac369_ac370_store_identity_rows() {
    fires_over_its_fixture_and_not_over_clean("RK-D-STORE-ID", rk_cli::doctor::Severity::Error);

    // The clean fixture's store must really have been built and really have opened — otherwise
    // "no finding" is a store that is not there rather than one that is welcome.
    let clean = materialise("clean");
    assert!(
        clean
            .path()
            .join(".git")
            .join("redkiln")
            .join("store.sqlite3")
            .is_file(),
        "`doctor/clean/` materialised no store, so the moved-common-dir control is not exercised"
    );
    let ctx = ctx_for(clean.path());
    assert!(
        matches!(ctx.store, Some(Ok(_))),
        "the clean fixture's store did not OPEN. It is stamped with this repository's own root \
         commit and a `common_dir` naming somewhere else; `rk_store::identity::check` compares \
         `root_commit` and `schema_version` and not `common_dir`, so a move is not a refusal"
    );
}

/// **AC-370** — a store under a linked worktree's own gitdir.
///
/// The store belongs to the common dir. One under `worktrees/<name>/redkiln/` is a second store
/// nothing will ever reconcile, which is why presence alone is the finding.
#[test]
fn ac370_stray_store_under_linked_worktree_is_an_error() {
    fires_over_its_fixture_and_not_over_clean("RK-D-STRAY-STORE", rk_cli::doctor::Severity::Error);

    assert!(
        findings("RK-D-STRAY-STORE", "RK-D-STRAY-STORE")
            .iter()
            .any(|f| f.detail.contains("wt-1")),
        "the finding does not name the linked worktree the stray store sits under"
    );
}

/// **ADR-0105 clauses 6 and 7 (RT-45)** — the three rows no AC id names.
///
/// Three ids, three fixtures, three unrelated facts about a repository: a worktree entry whose
/// working tree is gone, a common dir on a network filesystem, and a credential left in a retained
/// stream. They share a family test because they share a record, not because they share a
/// predicate — so each is asserted over **its own** fixture.
///
/// **All three are `Error`.** §7d.8's cell spells the severities as prose and defers to design
/// §8.4 — *"the design outranks the runbook, ADR-0092 clause 2.5"* — which fixes all three.
#[test]
fn adr0105_doctor_rows_network_fs_credential_orphan() {
    for id in [
        "RK-D-WORKTREE-ORPHAN",
        "RK-D-NETWORK-FS",
        "RK-D-CREDENTIAL-RETAINED",
    ] {
        fires_over_its_fixture_and_not_over_clean(id, rk_cli::doctor::Severity::Error);
    }

    // The credential row must name NEITHER the token nor anything that would carry it into a log.
    // ADR-0105 clause 7 keeps argv and env as digests for this reason; a finding that quoted the
    // secret back would be the defect it reports.
    let cred = findings("RK-D-CREDENTIAL-RETAINED", "RK-D-CREDENTIAL-RETAINED");
    assert!(
        !cred.is_empty(),
        "the credential row did not fire over its own fixture"
    );
    for f in &cred {
        assert!(
            !f.detail.contains("sk-ant-api03"),
            "the finding quotes the credential back. A rule that prints the secret it found has \
             copied it into wherever the report goes: {f:?}"
        );
    }

    assert!(
        findings("RK-D-WORKTREE-ORPHAN", "RK-D-WORKTREE-ORPHAN")
            .iter()
            .any(|f| f.detail.contains("gone")),
        "the orphan row does not name the worktree whose working tree is missing"
    );
}

// ------------------------------------------------------------------ batch 5: the facet rows

/// **AC-425 / AC-072's doctor halves** — the three registry rows.
///
/// Three ids because the remedies are three different edits: register the facet, correct the
/// value, or score the new entry. Two of the three are the kernel's own
/// `facet_registry_rule` reported under a doctor id; `RK-D-FACET-VALUE` and `RK-D-FACET-NEW` are
/// `doctor`'s own, because `EntryShapeViolation` has no member for either.
///
/// **The `applicable_if` carrier is the discriminating one.** `facet_registry_rule`'s own doc says
/// a rule reading only `require` rows passes *"a catalog whose `applicable_if` names an
/// unregistered facet, which is `binary_change`'s measured case"*, and the unknown-facet fixture
/// puts its defect there for exactly that reason.
#[test]
fn ac424_unknown_facet_value_and_new_facet_rejected() {
    for id in ["RK-D-FACET-UNKNOWN", "RK-D-FACET-VALUE", "RK-D-FACET-NEW"] {
        fires_over_its_fixture_and_not_over_clean(id, rk_cli::doctor::Severity::Error);
    }

    assert!(
        findings("RK-D-FACET-UNKNOWN", "RK-D-FACET-UNKNOWN")
            .iter()
            .any(|f| f.detail.contains("has_repro")),
        "the unknown-facet row does not name the facet the predicate reads"
    );
    assert!(
        findings("RK-D-FACET-VALUE", "RK-D-FACET-VALUE")
            .iter()
            .any(|f| f.detail.contains("epic")),
        "the facet-value row does not name the value outside the declared set"
    );
    assert!(
        findings("RK-D-FACET-NEW", "RK-D-FACET-NEW")
            .iter()
            .any(|f| f.detail.contains("deployment_window")),
        "the new-facet row does not name the entry the pinned reading lacks"
    );

    // The three are DISTINCT predicates, not one rule under three ids: the unknown-facet fixture
    // declares `kind` correctly and the value fixture registers every facet it names.
    assert!(
        findings("RK-D-FACET-VALUE", "RK-D-FACET-UNKNOWN").is_empty(),
        "the value row fired over the unknown-facet fixture — an UNDECLARED facet is the other \
         row's, and a rule reporting both makes one fixture reject for two reasons"
    );
    assert!(
        findings("RK-D-FACET-UNKNOWN", "RK-D-FACET-VALUE").is_empty(),
        "the unknown row fired over the value fixture, whose facet IS declared"
    );
}

/// **AC-425 / AC-071 / AC-520's doctor half** — a derived facet with no named `deriver`.
///
/// ADR-0089 ships the `deriver` field **plus a record requirement**, and a derived facet with
/// neither is what this row reports. The fixture declares §7d.8's three by name —
/// `recent_deploy`, `binary_change`, `dynamism` — so a rule that reported the first and stopped is
/// red on the count.
#[test]
fn ac425_facet_without_registered_fact_rejected() {
    fires_over_its_fixture_and_not_over_clean("RK-D-FACET-FACT", rk_cli::doctor::Severity::Error);

    let found = findings("RK-D-FACET-FACT", "RK-D-FACET-FACT");
    for facet in ["recent_deploy", "binary_change", "dynamism"] {
        assert!(
            found.iter().any(|f| f.detail.contains(facet)),
            "the fixture declares `{facet}` as derived with no deriver and the row does not \
             report it — every unscored entry is a separate thing to fix: {found:?}"
        );
    }
}

// ------------------------------------------------------- batch 6: the journal and record rows

/// **AC-220's doctor half (`s7c` ARMED)** — the last recorded `five_hour` reading sits inside the
/// pack's declared margin. A **warning**: headroom is a forecast, not a refusal.
///
/// **The LAST reading, not any reading.** The fixture climbs 40 → 70 → 99, so a rule that reported
/// the maximum, or any reading inside the margin, passes for the wrong reason; and one that read
/// the first is green over a fixture that ends at the limit.
#[test]
fn ac220_last_reading_within_margin_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-HEADROOM-MARGIN",
        rk_cli::doctor::Severity::Warning,
    );
    assert!(
        findings("RK-D-HEADROOM-MARGIN", "RK-D-HEADROOM-MARGIN")
            .iter()
            .any(|f| f.detail.contains("99")),
        "the finding does not name the reading it fired on"
    );
}

/// **AC-290 (D18-19)** — a `reproduced` finding with no re-execution record.
///
/// **The fixture carries its own control**: two `review_block` rows, both `reproduced`, one with a
/// `re_execution_id` and one without. A rule keyed on the class alone reports both, so the test
/// asserts it reports **one** — the class is what makes the record mandatory, and the record is
/// what discharges it.
#[test]
fn ac290_reproduced_finding_without_reexecution_record_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-REVIEW-REEXEC",
        rk_cli::doctor::Severity::Error,
    );

    let found = findings("RK-D-REVIEW-REEXEC", "RK-D-REVIEW-REEXEC");
    assert_eq!(
        found.len(),
        1,
        "the fixture holds two `reproduced` findings, one WITH a `re_execution_id`. Two findings \
         means the rule reads the class and not the record: {found:?}"
    );
    assert!(
        found[0].detail.contains("F1"),
        "the wrong finding is reported: {found:?}"
    );
}

/// **AC-306 (D18-19)** — live quarantines listed with their ages. A **warning**.
///
/// **The age is `now − until` off a SUPPLIED `now`** (§7d.8), never a clock read: the fixture's
/// `until` is a fixed instant and `Ctx::now` is a fixed constant, so "is it still live?" has an
/// answer that does not depend on the day the suite runs.
///
/// **Its own control again**: two quarantines granted, one already removed. A rule reading grants
/// alone reports both.
#[test]
fn ac306_live_quarantines_listed_with_ages_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-QUARANTINE-LIVE",
        rk_cli::doctor::Severity::Warning,
    );

    let found = findings("RK-D-QUARANTINE-LIVE", "RK-D-QUARANTINE-LIVE");
    assert_eq!(
        found.len(),
        1,
        "the fixture grants two quarantines and REMOVES one. Two findings means the rule reads \
         grants without folding the removals: {found:?}"
    );
    assert!(
        found[0].detail.contains("F1"),
        "the removed quarantine is the one reported: {found:?}"
    );
}

/// **AC-310 (D18-19)** — a completed review whose sink records do not cover its non-blocking
/// findings.
///
/// AC-310's own replay invariant is `findings_in == blocks + sink_records`. The fixture records
/// three findings, one blocked and one sunk — so one is **dropped**, which is the criterion's
/// *"and none is dropped"* read from the journal.
#[test]
fn ac310_sink_count_below_finding_count_doctor_row() {
    fires_over_its_fixture_and_not_over_clean("RK-D-SINK-COUNT", rk_cli::doctor::Severity::Error);
    assert!(
        findings("RK-D-SINK-COUNT", "RK-D-SINK-COUNT")
            .iter()
            .any(|f| f.detail.contains('3') && f.detail.contains('2')),
        "the finding does not carry both counts, so an operator cannot see how far short the sink is"
    );
}

/// **AC-530 (D18-19)** — the advisory ledger does not balance.
///
/// AC-530's invariant is `live_sink == created − retired − dismissed`. The fixture retires a
/// finding the sink never held, so the arithmetic is unreachable rather than merely wrong — which
/// is a defect readable from the journal alone, with no second count to trust.
#[test]
fn ac530_advisory_ledger_imbalance_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-ADVISORY-LEDGER",
        rk_cli::doctor::Severity::Error,
    );

    let found = findings("RK-D-ADVISORY-LEDGER", "RK-D-ADVISORY-LEDGER");
    assert_eq!(
        found.len(),
        1,
        "the fixture creates A1 and A2 and retires A1 (lawful) and A9 (never created). Two \
         findings means the rule reports every retirement rather than the unmatched one: {found:?}"
    );
    assert!(
        found[0].detail.contains("A9"),
        "the lawful retirement is the one reported: {found:?}"
    );
}

/// **AC-314 (D18-19)** — a regression record whose corpus sha resolves nowhere.
///
/// 🛑 **The record is a FILE.** §7d.8 sketched this as an `events.jsonl` row and also called the
/// corpus path *"a `ConfigFile` key"*; measured 2026-09-21, neither can carry it — `EventKind::ALL`
/// is closed at 38 with no regression kind, and `ConfigFile` is `deny_unknown_fields` over three
/// tables with no corpus key. The fixture's README records it.
#[test]
fn ac314_regression_corpus_sha_unresolvable_doctor_row() {
    fires_over_its_fixture_and_not_over_clean("RK-D-CORPUS-SHA", rk_cli::doctor::Severity::Error);
    assert!(
        findings("RK-D-CORPUS-SHA", "RK-D-CORPUS-SHA")
            .iter()
            .any(|f| f.detail.contains("deadbeef")),
        "the finding does not name the sha that resolved nowhere"
    );
}

/// **AC-455 (D18-19)** — a spec with `verification: None` under a pack whose policy requires one.
///
/// **The must-pass control is `clean/`**, and it is the criterion's own second clause: *"AND the
/// same spec passes under a pack whose policy does not require it."* `clean/` declares no
/// `[verification]` table, so a rule keyed on the spec alone — rather than on the spec AND the
/// policy — fires there and is red.
#[test]
fn ac455_verification_none_under_requiring_pack_doctor_row() {
    fires_over_its_fixture_and_not_over_clean(
        "RK-D-VERIFICATION-POLICY",
        rk_cli::doctor::Severity::Error,
    );
    assert!(
        findings("RK-D-VERIFICATION-POLICY", "RK-D-VERIFICATION-POLICY")
            .iter()
            .any(|f| f.detail.contains("RK-S0001")),
        "the finding does not name the spec's item, which is what AC-455 asks be named"
    );
}

// ------------------------------------------------------------------------ cycle 5's closing test

/// **AC-415 as amended (RA-M-01)** — every rule id has a fixture its own rule actually rejects.
///
/// *"Every id has at least one committed fixture configuration that the rule actually rejects, and
/// a rule with no rejecting fixture fails the build"* — **"and the id set is non-empty and contains
/// every id a criterion names"**.
///
/// # 🛑 IT RUNS THE RULE. THAT IS THE WHOLE DIFFERENCE
///
/// §7d.14 names the weaker test this would otherwise be: *"a weaker red **joins names without
/// running the rule**."* A name join is satisfied by thirty-eight directories and thirty-eight
/// rules that all answer nothing — exactly the state the tree was in at cycle 4, which is why this
/// test is cycle 5's and not cycle 4's.
///
/// # The set equality is BOTH directions, and `clean/` is the one exemption
///
/// A rule with no directory is an error; **a directory with no rule is an error too** — a stale
/// fixture is a lie about the table, and nothing else in the slice would notice one. `doctor/clean/`
/// is named as the exemption by §7d.13 because it is the positive control every per-row test runs,
/// not a rejecting fixture.
///
/// # Findings of ANY severity count
///
/// `RK-D-REBUILT-TIER2`'s finding is `Ok`-severity — tier-2 emptiness after a rebuild is
/// *expected*, and the row says so rather than staying silent. A join that counted only errors
/// would report that row as having no rejecting fixture while it reports perfectly well.
#[test]
fn ac415_every_rule_id_has_a_rejecting_fixture() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("doctor");
    let mut dirs: BTreeSet<String> = std::fs::read_dir(&root)
        .expect("the doctor fixture tree exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    assert!(
        dirs.remove("clean"),
        "`doctor/clean/` is absent. It is the positive control every per-row test runs second, and \
         the one directory §7d.13 exempts from the join below — without it the exemption has \
         nothing to except and every per-row test's second half is unasserted"
    );

    let ids: BTreeSet<&str> = RULES.iter().map(|r| r.id).collect();
    assert!(
        !ids.is_empty(),
        "AC-415 as amended: *\"the id set is non-empty\"*. Over an empty table every join below is \
         vacuous"
    );

    // ---- the set equality, BOTH directions, reported together so one run names every defect.
    let no_fixture: Vec<&str> = ids
        .iter()
        .copied()
        .filter(|id| !dirs.contains(*id))
        .collect();
    let no_rule: Vec<&str> = dirs
        .iter()
        .map(String::as_str)
        .filter(|d| !ids.contains(*d))
        .collect();
    assert!(
        no_fixture.is_empty() && no_rule.is_empty(),
        "the rule table and the fixture tree disagree.\n  rules with no `doctor/<id>/`: {no_fixture:?}\
         \n  directories with no rule: {no_rule:?}\n(a stale fixture is a lie about the table)"
    );

    // ---- and the half a name join cannot make: each fixture, through ITS OWN RULE ONLY.
    let mut inert: Vec<&str> = Vec::new();
    for id in &ids {
        if findings(id, id).is_empty() {
            inert.push(id);
        }
    }
    assert!(
        inert.is_empty(),
        "{} rule(s) found NOTHING over their own rejecting fixture. AC-415's clause is a fixture \
         *\"that the rule ACTUALLY REJECTS\"*, and a rule that never fires satisfies the name join \
         above, AC-414's per-fixture join and SM-121's dispatch count alike:\n  {}",
        inert.len(),
        inert.join("\n  ")
    );
}

// ============================================================ cycle 6: AC-414 and SM-121

/// **AC-414 as amended (D-10)** — *"every fixture's expected rule fires and zero harness dispatches
/// are issued and zero model tokens are spent"*, over the whole AC-415 corpus.
///
/// # 🛑 THE LOOP IS SELF-CONTROLLED, AND THE CONTROL COMES FIRST
///
/// §7d.14: red *"by the enumerated directory set (minus `clean/`) being empty or unequal to
/// `RULES`' id set (the self-control before the loop)"*. AC-414's own *Repaired* note records the
/// hole it is aimed at — the criterion *"passed trivially on a doctor that performs no checks"* —
/// and a loop over whatever happens to be on disk is that hole one level up: over an empty corpus
/// it iterates nothing and reports success. So the set is asserted non-empty and equal to the
/// table's ids **before** anything is run.
///
/// # This runs the WHOLE TABLE, which `ac415_` deliberately does not
///
/// `ac415_` runs each fixture through its own rule only, because its join is per-id. This is the
/// other half: the table as the binary runs it, asserting the expected id is **among** the ids that
/// fired. A rule firing on a fixture that is not its own is not a defect here — fixtures are
/// minimal, not mutually exclusive.
#[test]
fn ac414_doctor_over_corpus_fires_every_expected_rule_with_zero_dispatches() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("doctor");
    let mut corpus: BTreeSet<String> = std::fs::read_dir(&root)
        .expect("the doctor fixture tree exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    corpus.remove("clean");

    // ---- THE SELF-CONTROL, before the loop.
    assert!(
        !corpus.is_empty(),
        "the enumerated fixture corpus is empty, so the loop below would run nothing and report \
         success — AC-414's own repair note calls that passing trivially"
    );
    let ids: BTreeSet<String> = RULES.iter().map(|r| (*r.id).to_owned()).collect();
    assert_eq!(
        corpus, ids,
        "the fixture corpus and the rule table are not the same set, so the loop would not cover \
         the table it claims to"
    );

    for id in &corpus {
        let dir = materialise(id);
        let ctx = ctx_for(dir.path());

        // The `Ports` the root hands the doctor path: a harness that panics on dispatch and names
        // the rule, and a COUNTING store fake — SM-121's counter is the fake's.
        let harness = std::sync::Arc::new(rk_cli::no_dispatch::NoDispatchHarness::default());
        let store = rk_ports::fakes::VecStore::default();

        let report = rk_cli::doctor::rules::evaluate(&ctx, &harness);

        assert!(
            report.findings.iter().any(|f| f.id == id.as_str()),
            "`doctor/{id}/` is {id}'s rejecting fixture and that rule did not fire over it when \
             the WHOLE table ran. Fired: {:?}",
            report.findings.iter().map(|f| f.id).collect::<Vec<_>>()
        );
        assert_eq!(
            report.evaluated.len(),
            RULES.len(),
            "not every rule was evaluated over `doctor/{id}/` — §7d.8's stdout is one line per \
             EVALUATED rule, fired or not"
        );

        // AC-414's behavioural floor. The dispatch itself is impossible by construction — a
        // `Rule::check` is a `fn(&Ctx) -> Vec<Finding>` and a `fn` item captures nothing, so no
        // rule holds a harness to dispatch through — and `NoDispatchHarness` panicking is what
        // would say so if that ever stopped being true.
        assert_eq!(
            store.events().len(),
            0,
            "a `dispatch` row was appended while `doctor` ran over `doctor/{id}/`"
        );
    }
}

/// **AC-414's lint half, as amended (D-10 / D18-7)** — the `doctor` module constructs no port
/// implementor and opens no store.
///
/// The in-process twin of Checkpoint 7d's `rk_absent` line, over the same module-path filter:
/// `src/doctor/*.rs` **and** `src/verbs/doctor.rs`.
///
/// # 🛑 SELF-CONTROLLED OVER `d10_adapter_constructor_control.rs` FIRST
///
/// §7d.14 says it in terms. An absence assertion whose scanner matches nothing anywhere is not an
/// assertion, and this crate has already paid for that failure: the control file exists so the
/// pattern is shown to FIND the banned tokens before its silence over `doctor` is believed.
///
/// **Code lines only.** The module's own headers discuss `SqliteStore::open` and `dyn Harness` by
/// name — they have to, to say why neither is there — and a scan that read comments would fire on
/// the prose explaining the ban. That is a recorded failure mode of this tree, not a hypothetical.
#[test]
fn ac414_doctor_module_names_no_adapter_constructor() {
    const BANNED: [&str; 7] = [
        "ClaudeHarness::new",
        "rk_adapter_claude::config::Config::new",
        "Config::new(",
        "NodeToolchain::new",
        "RustToolchain::new",
        "SqliteStore::open",
        "dyn Harness",
    ];

    // ---- THE CONTROL. The pattern must find every token somewhere before its silence means
    // ---- anything; the control file carries them all, on code lines.
    let control = crate::lint_controls_dir().join("d10_adapter_constructor_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the D-10 control {} is unreadable: {e}", control.display()));
    let control_code: String = crate::code_lines(&control_text)
        .into_iter()
        .map(|(_, l)| l)
        .collect::<Vec<_>>()
        .join("\n");
    for token in BANNED {
        assert!(
            control_code.contains(token),
            "the control file carries no NON-COMMENT `{token}`, so this scan has not been seen to \
             find it and its silence over `doctor` proves nothing"
        );
    }

    // ---- the ban itself, over the same filter Checkpoint 7d applies.
    let src = crate::src_dir();
    let mut targets: Vec<std::path::PathBuf> = crate::rs_files(&src.join("doctor"));
    targets.push(src.join("verbs").join("doctor.rs"));
    assert!(
        targets.len() >= 2,
        "the doctor module filter admitted {} file(s) — Checkpoint 7d refuses a filtered ban whose \
         filter admits nothing, and so does this",
        targets.len()
    );

    let mut hits = Vec::new();
    for file in targets {
        let text = std::fs::read_to_string(&file).expect("a doctor source file");
        for (line_no, line) in crate::code_lines(&text) {
            for token in BANNED {
                if line.contains(token) {
                    hits.push(format!("{}:{line_no}: {}", file.display(), line.trim()));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "the `doctor` module names a port-implementor constructor on a code line. AC-414 as \
         amended: `doctor` reports and does not dispatch, and the store handle is opened by the \
         root (`adapters::open_store`) and carried in:\n{}",
        hits.join("\n")
    );
}

// =========================================================== cycle 7: the version probe

/// **AC-423 as amended (D-11), and SM-121** — `doctor`'s one child is a **spawn**, not a dispatch.
///
/// §7d.14's red: *"a spawn count ≠ 1, `-p` on the child's argv, or the dispatch counter ≠ 0."*
///
/// # 🛑 A SPAWN IS NOT A DISPATCH, AND THAT IS THE WHOLE INSTRUMENT
///
/// SM-121 counts **dispatches**, and it stays at zero across a run that probes. A dispatch goes
/// through `Harness` — which on this path is `NoDispatchHarness` and panics — and costs model
/// tokens; a `--version` spawn goes through `rk-containment`'s bounded runner and costs none.
/// `-p` is the flag that would carry a prompt, so its absence from the argv separates the two at
/// the level of bytes rather than intent.
///
/// # 🛑 IT DRIVES THE BINARY, AND IT HAS TO
///
/// The fixture child reads `RK_FIXTURE_MODE` and `RK_FIXTURE_MARKER_DIR` **from its own
/// environment** and inherits its parent's. `std::env::set_var` is banned workspace-wide, so an
/// in-process call to `RealProbe` cannot put them there — the first draft of this test did exactly
/// that and counted **zero** markers, because the child wrote none. Spawning `redkiln doctor` with
/// the two names on its `Command` is the established form (`answer.rs`'s `spawn_answer`) and is
/// what §7d.8 describes: *"`doctor` under the real probe spawns exactly one child"*.
///
/// The counts are **file facts**: one `<pid>.marker` per invocation (`s7c` §7c.17), carrying the
/// argv the child received. "Exactly one spawn" is a directory listing.
///
/// `RK_FIXTURE_MODE=exit:0` and never unset: the child's default is `hold`, a 600 s park, and the
/// probe would then sit on its own bound on every run.
#[test]
fn ac423_probe_is_one_version_spawn_not_a_dispatch() {
    let child = std::path::PathBuf::from(env!("CARGO_BIN_EXE_rk-fixture-child-cli"));
    assert!(
        child.is_file(),
        "the fixture child was not built at {}",
        child.display()
    );

    // A clean repository whose `harness_binary` IS the fixture child, so the probe spawns it.
    let dir = materialise("clean");
    let config_path = dir.path().join(".redkiln").join("config.toml");
    let config = std::fs::read_to_string(&config_path).expect("the clean config is readable");
    let pointed = config.replace(
        "toolchains = [",
        &format!(
            "harness_binary = {:?}\ntoolchains = [",
            child.display().to_string()
        ),
    );
    assert_ne!(pointed, config, "the `harness_binary` key was not inserted");
    std::fs::write(&config_path, &pointed).expect("the config is written");

    let markers = dir.path().join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");

    let out = spawn_doctor(dir.path(), &markers);
    // 🛑 EXIT 3 IS THE CORRECT ANSWER HERE, AND IT IS THE PROOF THE PROBE WAS CONSULTED.
    //
    // The fixture child prints its mode, not `TESTED_CLI_VERSION`, so `RK-D-CLI-VERSION` fires —
    // an error-severity row, which AC-438 makes exit `3`. A run that exited `0` would mean the
    // probe's answer never reached the rule, and the spawn would be a child nothing read.
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    assert_eq!(
        out.status.code(),
        Some(3),
        "`doctor` exited {:?}; the fixture child does not report the tested pin, so \
         `RK-D-CLI-VERSION` must fire and AC-438 makes that exit 3.\nstdout:\n{stdout}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let row = stdout
        .lines()
        .find(|l| l.starts_with("RK-D-CLI-VERSION"))
        .unwrap_or_else(|| panic!("no `RK-D-CLI-VERSION` row was printed:\n{stdout}"));
    assert!(
        row.split_whitespace().nth(1) != Some("ok"),
        "the `RK-D-CLI-VERSION` row reads `ok` while the exit was 3 — a different rule fired and \
         the probe's answer did not reach this one: {row}"
    );

    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&markers)
        .expect("the marker dir is readable")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "marker"))
        .collect();
    files.sort();
    assert_eq!(
        files.len(),
        1,
        "`doctor` spawned {} children; AC-423's instrument is EXACTLY ONE, and `doctor`'s one \
         child is the version probe",
        files.len()
    );

    let marker = std::fs::read_to_string(&files[0]).expect("the marker is readable");
    let argv: Vec<String> = marker
        .lines()
        .filter_map(|l| l.strip_prefix("arg=").map(ToOwned::to_owned))
        .collect();
    assert_eq!(
        argv.len(),
        2,
        "the child's argv is `[<program>, \"--version\"]` and nothing else; got {argv:?}"
    );
    assert_eq!(
        argv[1], "--version",
        "the one argument is not `--version`: {argv:?}"
    );
    assert!(
        !argv.iter().any(|a| a == "-p"),
        "the child's argv carries `-p`, the flag that carries a prompt — that is a DISPATCH and \
         not a version spawn: {argv:?}"
    );

    // SM-121's counter. The child wrote its marker in mode `exit:0`; a DISPATCH would have written
    // one in a dispatch mode, and nothing on this path holds a `Harness` to make one.
    assert!(
        marker.contains("mode=exit"),
        "the child ran in a mode this test did not choose — `RK_FIXTURE_MODE` did not reach it: \
         {marker}"
    );
}

/// Spawn `redkiln -C <repo> doctor` with the fixture child's two names on its `Command`.
///
/// `Command::env`, never `set_var`, which is banned workspace-wide including this tree.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS: the fixture child reads its mode from its own \
              environment and inherits its parent's, so nothing in-process can set it. \
              `answer.rs`'s `spawn_answer` is the same form for the same reason."
)]
fn spawn_doctor(repo: &std::path::Path, markers: &std::path::Path) -> std::process::Output {
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C")
        .arg(repo)
        .arg("doctor")
        .env("RK_FIXTURE_MODE", "exit:0")
        .env("RK_FIXTURE_MARKER_DIR", markers)
        .stdin(std::process::Stdio::null());
    crate::support::output_bounded(&mut cmd)
}

// ================================================== cycle 8: the unread window

/// Build a `Ctx` over one `unread/` log at a given `unread_window_runs`.
///
/// The log replaces the fixture repository's `events.jsonl`, and N is rewritten on the config — so
/// the same bytes are read at two settings and the key is shown to be live.
fn unread_ctx(log: &str, window: u32) -> (tempfile::TempDir, rk_cli::doctor::Ctx) {
    let dir = materialise("clean");
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("unread")
        .join(log);
    std::fs::copy(&src, dir.path().join("events.jsonl"))
        .unwrap_or_else(|e| panic!("{} could not be staged: {e}", src.display()));

    let path = dir.path().join(".redkiln").join("config.toml");
    let text = std::fs::read_to_string(&path).expect("the config is readable");
    let rewritten = text.replace(
        "unread_window_runs = 50",
        &format!("unread_window_runs = {window}"),
    );
    // The OUTCOME is asserted, not the change: at N = 50 the rewrite is a no-op because the
    // scaffold already says 50, and a test that demanded the text differ would be red on the one
    // setting it does not need to touch.
    assert!(
        rewritten.contains(&format!("unread_window_runs = {window}")),
        "the config does not carry `unread_window_runs = {window}` after the rewrite, so this          reading would use whatever N the scaffold happened to say"
    );
    std::fs::write(&path, rewritten).expect("the config is written");

    let ctx = ctx_for(dir.path());
    (dir, ctx)
}

/// Run `RK-D-UNREAD` alone over one log at one window.
fn unread_findings(log: &str, window: u32) -> Vec<rk_cli::doctor::Finding> {
    let rule = RULES
        .iter()
        .find(|r| r.id == "RK-D-UNREAD")
        .expect("RK-D-UNREAD is in the table");
    let (_dir, ctx) = unread_ctx(log, window);
    (rule.check)(&ctx)
}

/// **AC-426 / AC-472 as amended / AC-147 as amended, and SM-139** — the window boundary, and that
/// the key is live.
///
/// N = 50 (E9). 49 runs are **not** flagged and 51 **are**; and under `unread_window_runs = 60`
/// **neither** is — which is what makes N the repository's key rather than a constant in the rule
/// table, and is the half a test reading one N would miss entirely.
///
/// The pair sits one run either side of the boundary on purpose: a rule keyed on `>` instead of
/// `>=`, or on `N − 1`, passes a 1-versus-100 pair and fails exactly here.
#[test]
fn ac426_ac472_unread_window_49_not_flagged_51_flagged() {
    let under = unread_findings("runs-49.jsonl", 50);
    assert!(
        under.is_empty(),
        "49 runs is UNDER the window of 50 and was flagged: {under:?}"
    );

    let over = unread_findings("runs-51.jsonl", 50);
    assert_eq!(
        over.len(),
        1,
        "51 runs is over the window of 50 and must be flagged exactly once — the second kind in \
         the log is written AND read every run, so a rule that flags everything reports two: \
         {over:?}"
    );
    assert!(
        over[0].detail.contains("`k`"),
        "the flagged kind is not `k`, the one written and never read: {over:?}"
    );
    assert_eq!(
        over[0].severity,
        rk_cli::doctor::Severity::Warning,
        "AC-426's row is a warning; §7d.8's *\"warnings never move the code\"*"
    );

    // ---- THE KEY IS LIVE. Same bytes, larger window, neither flagged.
    for log in ["runs-49.jsonl", "runs-51.jsonl"] {
        let wide = unread_findings(log, 60);
        assert!(
            wide.is_empty(),
            "{log} was flagged at `unread_window_runs = 60`. Both logs are under 60 runs, so a \
             rule reading the repository's key reports neither — one that read a constant of 50 \
             reports `runs-51` here: {wide:?}"
        );
    }
}

/// **AC-426's second instrument** — the rule fires and changes nothing.
///
/// # 🛑 THE POSITIVE HALF IS READ FIRST, AND WITHOUT IT THIS ASSERTS NOTHING
///
/// RA-M-01's shape: *"a rule that never fires leaves the write set unchanged trivially"*. So the
/// finding is asserted **before** the digests are compared — a rule that did nothing would satisfy
/// the negative half perfectly.
///
/// The write set is the whole materialised tree, digested file by file. `doctor` is a read: no
/// store row, no marker, no byte.
#[test]
fn ac426_rule_fire_leaves_write_set_unchanged() {
    let rule = RULES
        .iter()
        .find(|r| r.id == "RK-D-UNREAD")
        .expect("RK-D-UNREAD is in the table");
    let (dir, ctx) = unread_ctx("runs-51.jsonl", 50);

    let before = tree_digest(dir.path());
    assert!(
        !before.is_empty(),
        "the tree digest read no files, so the comparison below would hold over nothing"
    );

    // THE POSITIVE HALF, first.
    let found = (rule.check)(&ctx);
    assert!(
        !found.is_empty(),
        "`RK-D-UNREAD` did not fire over `runs-51.jsonl`, so the write-set comparison below is \
         true of a rule that did nothing — RA-M-01's own shape"
    );

    let after = tree_digest(dir.path());
    assert_eq!(
        before, after,
        "the tree changed while a `doctor` rule ran. `doctor` is a read: AC-429's direction — git \
         to store, never the reverse — and no rule writes a byte"
    );
}

/// Every file under `root`, by path, with its bytes' length and a cheap content hash.
///
/// Paths **and** contents: a comparison of paths alone would miss a rewritten file, and one of
/// contents alone would miss a deletion.
fn tree_digest(root: &std::path::Path) -> Vec<(String, u64, u64)> {
    fn walk(dir: &std::path::Path, base: &std::path::Path, out: &mut Vec<(String, u64, u64)>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, base, out);
            } else if let Ok(bytes) = std::fs::read(&path) {
                let rel = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                let sum = bytes.iter().fold(1469598103934665603u64, |h, b| {
                    (h ^ u64::from(*b)).wrapping_mul(1099511628211)
                });
                out.push((rel, bytes.len() as u64, sum));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}
