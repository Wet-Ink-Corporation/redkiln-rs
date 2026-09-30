//! **`s8e` §8e.8 — `Slice-Verdict:` is sealed on both review outcomes, and the slice is declared**
//! (D21-41 e, first half; ADR-0160's route; the slice record, proposed). *(Added 2026-09-28, `s8e`
//! build, package S8E-P7.)*
//!
//! Every test here runs the REAL `redkiln` binary over the fixture child (`rk-fixture-child-claude`,
//! D21-47's canonical fixture harness) — ADR-0162's precedent that *"a real `redkiln run` over a
//! fixture corpus reaches a producer"*, which is what un-ignores the ARMED trailer lint in the same
//! commit. Each reads `Slice-Verdict:` off `rk-run/<item>` with **git's own parser**
//! (`%(trailers:key=Slice-Verdict,valueonly)`), never with `rk-git`'s, so a renderer and a reader
//! that agreed on a wrong spelling could not pass here together.
//!
//! - `approved_is_sealed_on_the_advance_path` — `slice/approved/`, an empty finding set: exit `0`,
//!   one `Slice-Verdict: rk-fixture/review-1-fix approved`, and its `t1_slice_seal` row anchored to
//!   that commit.
//! - `changes_requested_is_sealed_on_the_findings_block_path` — `exit/findings-block/`: exit `4`,
//!   one `Slice-Verdict: rk-fixture/review-1-fix changes-requested`, sealed BEFORE the exit returns.
//! - the two refusals, at the mint and by name (ADR-0159 decision 2's siting): an item record with
//!   no `slice`, and one whose `slice` is the item id. Each is exit `3`, the refusal is named on
//!   stderr, and no `Slice-Verdict:` reaches the ref. The first names the ABSENCE, not the item-id
//!   rule, so a mint that defaulted an absent slice to the item id (S8E-M8) is red here.
//!   *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* Both refusals are
//!   now met at the lane's entry, before any dispatch, through the validator the mint shares; the
//!   mint's refusal stays as the backstop and is pinned in process by
//!   `the_mint_refuses_an_undeclared_slice_as_the_backstop`.
//!   *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's tests-mutants finding.)*
//!   ~~so a mint that defaulted an absent slice to the item id (S8E-M8) is red here~~ — since the
//!   entry refusal, no binary run here reaches the mint with an undeclared slice, so a default
//!   sited in `seal_act` alone (S8E-M8a) is GREEN on the binary tests and red only on
//!   `the_mint_refuses_an_undeclared_slice_as_the_backstop`; a default sited in the shared
//!   `validate_slice` (S8E-M8b) is red on `an_absent_slice_is_refused_by_name` too, because the
//!   entry then names the item-id rule (§8e.12's S8E-M8 rows, measured).

use std::path::{Path, PathBuf};

use crate::support::{self, PackVariant};

/// `pack-min/config.toml`'s `[project] name`.
const PROJECT: &str = "rk-fixture";
/// `pack-min/items/review-item.toml`: at `stage = "review"`, `slice = "review-1-fix"`.
const ITEM: &str = "review-1";
/// The item record's declared slice.
const SLICE: &str = "review-1-fix";

/// A corpus directory: `slice/approved/` or `exit/findings-block/`.
fn corpus(parts: &[&str]) -> PathBuf {
    let mut dir = crate::fixtures_dir();
    for p in parts {
        dir = dir.join(p);
    }
    assert!(
        dir.join("frames.jsonl").is_file(),
        "PREMISE: {} carries the transcript the child replays",
        dir.display()
    );
    dir
}

/// Run `redkiln -C <root> run review-1` with the child replaying `<corpus>/frames.jsonl`.
#[expect(
    clippy::disallowed_types,
    reason = "the seal is exercised AS A PROCESS: ADR-0162's precedent is a real `redkiln run` over a \
              fixture corpus, and that is what discharges the ARMED lint"
)]
fn run_over(root: &Path, corpus: &Path) -> std::process::Output {
    let markers = root.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C")
        .arg(root)
        .args(["run", ITEM])
        .env(
            "RK_FIXTURE_MODE",
            format!("replay:{}", corpus.join("frames.jsonl").display()),
        )
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    support::output_bounded(&mut cmd)
}

/// Every `Slice-Verdict:` value on `rk-run/<item>`, oldest first, read by git's own parser.
/// Through `rk-git`'s one process-exec site (`support::git_run`; ADR-0077; D21-128 A1); a refused
/// `log` (an absent ref) reads as no seals, as before the routing, and a git that cannot start
/// panics.
fn seals_on_run_ref(root: &Path) -> Vec<(String, String)> {
    let run_ref = format!("refs/heads/rk-run/{ITEM}");
    let out = match crate::support::git_run(
        root,
        &[
            "log",
            "--reverse",
            "--format=%H%x1f%(trailers:key=Slice-Verdict,valueonly,separator=%x1e)",
            &run_ref,
        ],
    ) {
        Ok(out) => out,
        Err(rk_git::error::GitError::NonZeroExit { .. }) => return Vec::new(),
        Err(e) => panic!("git log did not run: {e:?}"),
    };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| l.split_once('\x1f'))
        .flat_map(|(sha, values)| {
            values
                .split('\x1e')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(|v| (sha.to_owned(), v.to_owned()))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// `t1_slice_seal`'s rows `(anchor_sha, project, slice, verdict)`.
fn seal_rows(root: &Path) -> Vec<(String, String, String, String)> {
    let reader = rk_store::conn::Reader::open(&crate::common_dir(root)).expect("the store reads");
    let mut stmt = reader
        .conn()
        .prepare("SELECT anchor_sha, project, slice, verdict FROM t1_slice_seal ORDER BY id")
        .expect("t1_slice_seal is readable");
    stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))
        .expect("the rows read")
        .collect::<Result<Vec<_>, _>>()
        .expect("the rows decode")
}

/// Rewrite the installed item record's `slice` line: `None` deletes it, `Some(v)` sets it.
fn set_slice(root: &Path, slice: Option<&str>) {
    let path = root
        .join(".redkiln")
        .join("items")
        .join(format!("{ITEM}.toml"));
    let src = std::fs::read_to_string(&path).expect("the item record reads");
    let mut hit = 0_usize;
    let out: Vec<String> = src
        .lines()
        .filter_map(|l| {
            if l.trim_start().starts_with("slice") {
                hit += 1;
                slice.map(|v| format!("slice = \"{v}\""))
            } else {
                Some(l.to_owned())
            }
        })
        .collect();
    assert_eq!(
        hit, 1,
        "PREMISE: the installed record carries exactly one `slice` line to rewrite"
    );
    std::fs::write(&path, out.join("\n") + "\n").expect("the item record rewrites");
}

fn stderr(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// **`approved` on the advance path** — `blocked.is_empty()` seals `approved` (`s8e` §8e.8).
#[test]
fn approved_is_sealed_on_the_advance_path() {
    let repo = support::pack_min_repo(PackVariant::Review);
    let root = repo.path();
    let out = run_over(root, &corpus(&["slice", "approved"]));
    assert_eq!(
        out.status.code(),
        Some(0),
        "the approved corpus exits 0.\nstderr:\n{}",
        stderr(&out)
    );

    let seals = seals_on_run_ref(root);
    let want = format!("{PROJECT}/{SLICE} approved");
    assert_eq!(
        seals.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>(),
        vec![want.as_str()],
        "exactly one `Slice-Verdict: {want}` on rk-run/{ITEM}, read by git's own parser. stderr:\n{}",
        stderr(&out)
    );
    let rows = seal_rows(root);
    assert_eq!(
        rows,
        vec![(
            seals[0].0.clone(),
            PROJECT.to_owned(),
            SLICE.to_owned(),
            "approved".to_owned()
        )],
        "the seal's t1_slice_seal row is anchored to its own Slice-Verdict commit"
    );
}

/// **`changes-requested` on the findings-block path** — sealed BEFORE exit `4` returns.
#[test]
fn changes_requested_is_sealed_on_the_findings_block_path() {
    let repo = support::pack_min_repo(PackVariant::Review);
    let root = repo.path();
    let out = run_over(root, &corpus(&["exit", "findings-block"]));
    assert_eq!(
        out.status.code(),
        Some(4),
        "the findings-block corpus exits 4 (ADR-0162).\nstderr:\n{}",
        stderr(&out)
    );

    let seals = seals_on_run_ref(root);
    let want = format!("{PROJECT}/{SLICE} changes-requested");
    assert_eq!(
        seals.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>(),
        vec![want.as_str()],
        "exactly one `Slice-Verdict: {want}` on rk-run/{ITEM}: the rejecting verdict crosses no \
         boundary, so it is sealed on the review stage's exit outcome. stderr:\n{}",
        stderr(&out)
    );
    let rows = seal_rows(root);
    assert_eq!(
        rows.iter().map(|r| r.3.as_str()).collect::<Vec<_>>(),
        vec!["changes-requested"],
        "one t1_slice_seal row, the rejecting verdict: {rows:?}"
    );
    assert_eq!(rows[0].0, seals[0].0, "anchored to its own commit");
}

/// **An absent `slice` refuses the seal by name** (ADR-0160: *"any value not declared as a
/// slice"*). The refusal names the absence — a mint that defaulted it to the item id would name
/// the item-id rule instead (S8E-M8).
///
/// *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* The refusal this
/// run meets is now the lane's ENTRY check, before any dispatch, through the same validator the
/// mint calls: `review-1` sits at `review`, whose `review-min` provides `Findings`. The assertions
/// hold unchanged (exit `3`, the absence named, nothing committed). The mint's own refusal is pinned
/// in process by `the_mint_refuses_an_undeclared_slice_as_the_backstop`.
///
/// *(dated 2026-09-28, `s8e` build, package R1FIX-8e.)* ~~a mint that defaulted it to the item id
/// would name the item-id rule instead (S8E-M8)~~ — the refusal met here is the ENTRY's, so what
/// reddens this test is a default in the shared `tier_one::validate_slice` (S8E-M8b: the entry
/// names the item-id rule), not one in `seal_act` alone (S8E-M8a, killed by
/// `the_mint_refuses_an_undeclared_slice_as_the_backstop`), and not the entry check dropped
/// (S8E-M53: the mint's backstop still refuses by the same name).
#[test]
fn an_absent_slice_is_refused_by_name() {
    let repo = support::pack_min_repo(PackVariant::Review);
    let root = repo.path();
    set_slice(root, None);
    let out = run_over(root, &corpus(&["slice", "approved"]));
    let err = stderr(&out);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an absent slice refuses the seal, exit 3.\nstderr:\n{err}"
    );
    assert!(
        err.contains(&format!("`{ITEM}` declares no `slice`")),
        "the refusal names the ABSENCE of a declared slice for `{ITEM}`, not another rule:\n{err}"
    );
    assert!(
        seals_on_run_ref(root).is_empty(),
        "a refused seal commits nothing to rk-run/{ITEM}"
    );
    assert!(
        seal_rows(root).is_empty(),
        "and writes no t1_slice_seal row"
    );
}

/// **The mint keeps its refusal as the backstop** (D21-94: *"The mint keeps its refusal as the
/// backstop"*). *(Added 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; ADR-0186 as amended,
/// proposed.)*
///
/// Since D21-94 the two binary tests above meet the lane's ENTRY refusal first (`review-1` sits at
/// `review`, whose `review-min` provides `Findings`), so no fixture run reaches the mint with an
/// undeclared slice any more. This pin calls the mint itself, `tier_one::seal_act`, for every value
/// ADR-0160 names, and asserts each refusal is the one the shared validator gives the entry check
/// (`tier_one::validate_slice`), so the two sitings cannot disagree. Green at entry by design; red
/// under S8E-M55 (the validator's call dropped from `seal_act`) and, *(dated 2026-09-28, `s8e`
/// build, package R1FIX-8e)*, under S8E-M8a (an absent slice defaulted to the item id in
/// `seal_act` alone), which no binary test here can reach.
#[test]
fn the_mint_refuses_an_undeclared_slice_as_the_backstop() {
    use rk_kernel::runtime::tier_one::{SliceRefusal, TierOneError, seal_act, validate_slice};
    let stages = vec!["triage".to_owned(), "review".to_owned()];
    let cases: [(Option<&str>, SliceRefusal); 6] = [
        (None, SliceRefusal::Absent),
        (Some("  "), SliceRefusal::Absent),
        (Some("a fix"), SliceRefusal::Whitespace),
        (Some(ITEM), SliceRefusal::ItemId),
        (Some(PROJECT), SliceRefusal::Project),
        (Some("review"), SliceRefusal::Stage("review".to_owned())),
    ];
    for (declared, want) in cases {
        let minted = seal_act(
            declared,
            ITEM,
            Some(PROJECT),
            &stages,
            "review",
            rk_git::trailer::grammar::Verdict::Approved,
        );
        match minted {
            Err(TierOneError::SliceRefused { reason, .. }) => assert_eq!(
                reason, want,
                "the mint refused {declared:?} for the wrong reason"
            ),
            Err(other) => panic!("the mint refused {declared:?} as {other:?}, not by name"),
            Ok(_) => panic!("the mint accepted {declared:?}: its backstop is gone"),
        }
        match validate_slice(declared, ITEM, Some(PROJECT), &stages) {
            Err(TierOneError::SliceRefused { reason, .. }) => assert_eq!(
                reason, want,
                "the entry's validator and the mint disagree on {declared:?}"
            ),
            other => panic!("the entry's validator did not refuse {declared:?}: {other:?}"),
        }
    }
    assert!(
        seal_act(
            Some(SLICE),
            ITEM,
            Some(PROJECT),
            &stages,
            "review",
            rk_git::trailer::grammar::Verdict::Approved,
        )
        .is_ok(),
        "CONTROL: the declared slice `{SLICE}` mints"
    );
}

/// **A `slice` equal to the item id is refused** (D21-41 e's rule; ADR-0160's first named value).
#[test]
fn a_slice_equal_to_the_item_id_is_refused() {
    let repo = support::pack_min_repo(PackVariant::Review);
    let root = repo.path();
    set_slice(root, Some(ITEM));
    let out = run_over(root, &corpus(&["slice", "approved"]));
    let err = stderr(&out);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a slice equal to the item id refuses the seal, exit 3.\nstderr:\n{err}"
    );
    assert!(
        err.contains("is the item id"),
        "the refusal names the item-id rule:\n{err}"
    );
    assert!(
        seals_on_run_ref(root).is_empty(),
        "a refused seal commits nothing to rk-run/{ITEM}"
    );
}
