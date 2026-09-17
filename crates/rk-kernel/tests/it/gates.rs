//! **The human gate** — Step 7b-ii, `s7b-ii` §7b-ii.2, slice A. The record and the one derivation.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac244_gate_record_requires_all_three`] | **AC-244** | the **deserialisation** half: a `gate_opened` read back from the store lacking `kind`, `tree_hash` or `deadline` folds to a typed refusal, opens no gate and moves no state | the **type-level** half, which is `tests/ui/ac244_gate_record_missing_a_field.rs` in the crate's one `trybuild` harness; and the `doctor` rule, which is `rk-cli`'s (7d) and ARMED |
//! | [`ac245_write_tree_one_algorithm`] | **AC-245**, and AC-249's *commits-between* fixture | over a REAL repository: the recorded hash resolves to one tree object; an untracked file does **not** move it and a tracked modification **does**; the same derivation at open and at answer, so an answer over a drifted tree is **void** | a second derivation in this crate — there is none to test, and the architecture half is `rk-git`'s own `there_is_exactly_one_write_tree_call_site`, cited and not re-credited |
//!
//! **Where red was observed, and against what.** Golden constraint 12 asks for a failure *"for the
//! right reason"*, and two of the behaviours below were built at Step 7b (`src/workflow/fsm.rs`'s
//! `gate_opened` arm refuses a half-shaped payload; its `gate_answered` arm voids on a `tree_hash`
//! mismatch), so their red could not be the absent behaviour. Each is labelled at the assertion:
//! **red via a temporary mutant of `fsm.rs`**, run and reverted before the slice was committed, with
//! the failing output quoted in the commit message. AC-245's tracked-modification case was red
//! against a **second derivation** — the answer-side hash taken from `HEAD^{tree}` instead of the
//! wrapper's `write-tree` — which is the precise failure F19 measured on 12/174 gates.
//!
//! **`token` is derived, never deserialised.** The fold ignores a journaled `token` member and
//! re-derives it from the other three (`fsm.rs`'s header: *"a `token` member recorded in the journal
//! is not read"*), so *"lacks a token"* is not a shape a read-back record can have — the type-level
//! case carries that clause and this file's fixtures are the three the fold can meet.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_git::git_exec::{self, GitExec};
use rk_git::path::WorktreePath;
use rk_git::tree::{self, TreeBasis, TreeHash};
use rk_git::version;
use rk_kernel::runtime::{Runtime, RuntimeError, gate_opened_event};
use rk_kernel::workflow::fsm::{FsmError, GateAnswer, GateRecord, fold};
use rk_ports::event_store::EventStore;
use rk_ports::fakes::VecStore;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

const ITEM: &str = "INC-1";
const STAGE: &str = "implement";
const KIND: &str = "human:review";
const DEADLINE: i64 = 1_700_003_600;

const MISSING_KIND: &str = include_str!("../fixtures/gates/ac244/missing_kind.json");
const MISSING_TREE_HASH: &str = include_str!("../fixtures/gates/ac244/missing_tree_hash.json");
const MISSING_DEADLINE: &str = include_str!("../fixtures/gates/ac244/missing_deadline.json");
const COMPLETE: &str = include_str!("../fixtures/gates/ac244/complete.json");

fn runtime_over(store: &Arc<Mutex<dyn EventStore>>) -> Runtime {
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(Vec::new()));
    Runtime::new(crate::runtime::ports(harness, store), "run-gates", STAGE)
}

fn gate_opened_from(fixture: &str) -> Event {
    let payload: serde_json::Value = serde_json::from_str(fixture).expect("fixture is JSON");
    Event::new(EventKind::GateOpened, payload)
}

fn gate_answered(token: &str, tree_hash: &str) -> Event {
    Event::new(
        EventKind::GateAnswered,
        serde_json::json!({ "token": token, "tree_hash": tree_hash, "verdict": "approve" }),
    )
}

/// **AC-244 — the deserialisation half.**
///
/// > GIVEN a request to open a gate whose record lacks a token, a tree hash, or a deadline / WHEN
/// > the kernel processes it / THEN the gate is not opened and the run fails with a configuration
/// > error, leaving no trailer and no event. *Verified by:* a type-level test … plus a runtime test
/// > on the deserialisation path for records read back from the store.
#[test]
fn ac244_gate_record_requires_all_three() {
    // ---- positive control FIRST: the complete record, read back from the store, opens ONE gate.
    // Without this the three refusals below could be a fixture path that never reached the fold.
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = runtime_over(&store);
    rt.append(gate_opened_from(COMPLETE)).expect("append");
    let state = rt.state(ITEM).expect("a complete record folds");
    assert_eq!(
        state.open_gates().count(),
        1,
        "PREMISE: the complete fixture opens exactly one gate; the refusals below are measured against this"
    );
    let expected = GateRecord::derive(
        KIND,
        "aaaa111122223333444455556666777788889999",
        Timestamp::from_unix_seconds(DEADLINE),
    )
    .expect("derive");
    assert_eq!(
        state.open_gates().next().map(GateRecord::token),
        Some(expected.token()),
        "the token is DERIVED from the three members the fixture carries, not read from it"
    );

    // ---- the three partial records, each read back from the store and folded.
    for (missing, fixture) in [
        ("kind", MISSING_KIND),
        ("tree_hash", MISSING_TREE_HASH),
        ("deadline", MISSING_DEADLINE),
    ] {
        let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
        let rt = runtime_over(&store);
        rt.append(gate_opened_from(fixture)).expect("append");

        // The run FAILS with a configuration error — a typed one that names the kind.
        //
        // RED (observed 2026-09-17, via a temporary mutant of `fsm.rs` that defaulted the absent
        // `deadline` to 0): the `missing_deadline` arm folded to Ok with one open gate.
        let err = rt.state(ITEM).err().unwrap_or_else(|| {
            panic!("a gate_opened lacking `{missing}` was read back and OPENED a gate")
        });
        match err {
            RuntimeError::Fsm(FsmError::MalformedPayload { kind, detail }) => {
                assert_eq!(kind, "gate_opened", "the refusal names the kind it refused");
                assert!(
                    detail.contains(missing) || detail.contains("neither producer's shape"),
                    "the refusal for a record lacking `{missing}` names what is missing: {detail}"
                );
            }
            other => panic!("a partial record must be MalformedPayload, not {other:?}"),
        }

        // "leaving no trailer and no event": the store holds exactly the one row appended and the
        // fold appended nothing of its own — the journal is what was read, not what was written.
        let journal = rt.journal(ITEM).expect("query");
        assert_eq!(
            journal.len(),
            1,
            "the refused open wrote nothing: one row in, one row out (missing `{missing}`)"
        );

        // And the fold over the same journal, called directly, refuses identically — the runtime
        // added no second reading of its own.
        assert!(
            matches!(
                fold(ITEM, STAGE, &journal),
                Err(FsmError::MalformedPayload {
                    kind: "gate_opened",
                    ..
                })
            ),
            "fold() over the read-back journal refuses the record lacking `{missing}`"
        );
    }
}

// ------------------------------------------------------------------ AC-245: a real repository

/// A repository built at test time under a `tempfile` directory, isolated from this box's git
/// configuration the same way `crates/rk-git/tests/it/support` isolates its fixtures.
struct Repo {
    dir: tempfile::TempDir,
    path: PathBuf,
}

impl Repo {
    fn env(&self) -> Vec<(String, String)> {
        vec![
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            (
                "GIT_CONFIG_GLOBAL".into(),
                self.dir
                    .path()
                    .join("empty.gitconfig")
                    .display()
                    .to_string(),
            ),
            ("GIT_AUTHOR_NAME".into(), "rk".into()),
            ("GIT_COMMITTER_NAME".into(), "rk".into()),
            ("GIT_AUTHOR_EMAIL".into(), "rk@example.invalid".into()),
            ("GIT_COMMITTER_EMAIL".into(), "rk@example.invalid".into()),
            ("GIT_AUTHOR_DATE".into(), "1700000000 +0000".into()),
            ("GIT_COMMITTER_DATE".into(), "1700000000 +0000".into()),
        ]
    }

    fn git(&self, args: &[&str]) -> String {
        let mut all: Vec<String> = vec![
            "-c".into(),
            "core.autocrlf=false".into(),
            "-c".into(),
            format!(
                "core.hooksPath={}",
                self.dir.path().join("nohooks").display()
            ),
            "-c".into(),
            "commit.gpgsign=false".into(),
            "-c".into(),
            "init.defaultBranch=main".into(),
        ];
        all.extend(args.iter().map(|s| (*s).to_owned()));
        match git_exec::run_with_env(&self.path, &all, self.env()) {
            Ok(out) => out.stdout_trimmed(),
            Err(e) => panic!("git {args:?} in {}: {e:?}", self.path.display()),
        }
    }

    fn with_one_commit() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("empty.gitconfig"), b"").expect("empty gitconfig");
        std::fs::create_dir_all(dir.path().join("nohooks")).expect("nohooks");
        let path = dir.path().join("repo");
        std::fs::create_dir_all(&path).expect("mkdir repo");
        let repo = Self { dir, path };
        repo.git(&["init", "--quiet"]);
        std::fs::write(repo.path.join("a.txt"), b"one\n").expect("write a.txt");
        repo.git(&["add", "a.txt"]);
        repo.git(&["commit", "--quiet", "-m", "one"]);
        repo
    }

    fn exec(&self) -> GitExec {
        let tc = version::assert_at_startup(&self.path).expect("git at or above the floor");
        GitExec::new(self.dir.path().join("hooks-empty"), tc).expect("wrapper")
    }

    fn common_dir(&self) -> PathBuf {
        self.path.join(".git")
    }

    fn write(&self, rel: &str, bytes: &[u8]) {
        std::fs::write(self.path.join(rel), bytes).expect("write");
    }
}

/// **The one derivation, called from both sides of the gate.** ADR-0105 clause 4: *"`tree_hash` is
/// one algorithm, written once, used at gate open and at answer"* — and it is `rk-git`'s. This
/// crate computes no tree hash of its own; both the open and the answer below go through this
/// function, and this function goes through the wrapper.
fn working_tree(repo: &Repo, exec: &GitExec) -> TreeHash {
    let cwd = WorktreePath::new(&repo.path).expect("normalises");
    tree::tree_hash(exec, &cwd, &repo.common_dir(), &TreeBasis::WorkingState)
        .expect("write-tree on a temporary index")
}

/// Open a gate at `hash`, then answer it at `answer_hash`, and return the folded state.
fn open_then_answer(hash: &TreeHash, answer_hash: &TreeHash) -> rk_kernel::workflow::fsm::RunState {
    let record = GateRecord::derive(KIND, hash.as_str(), Timestamp::from_unix_seconds(DEADLINE))
        .expect("derive");
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = runtime_over(&store);
    rt.append(gate_opened_event(&record)).expect("append open");
    rt.append(gate_answered(record.token().as_str(), answer_hash.as_str()))
        .expect("append answer");
    rt.state(ITEM).expect("folds")
}

fn assert_applied(state: &rk_kernel::workflow::fsm::RunState, why: &str) {
    assert_eq!(
        state.answered_gates().count(),
        1,
        "the answer APPLIED: {why}"
    );
    assert_eq!(
        state.void_gates().count(),
        0,
        "…and nothing was voided: {why}"
    );
}

fn assert_void(state: &rk_kernel::workflow::fsm::RunState, why: &str) {
    assert_eq!(state.void_gates().count(), 1, "the answer is VOID: {why}");
    assert_eq!(
        state.answered_gates().count(),
        0,
        "…and no approve/changes-requested decision was applied (AC-249): {why}"
    );
}

/// **AC-245 — one algorithm, used at gate open AND at answer.**
///
/// > THEN the recorded `tree_hash` is the hash of that same tree object, not of HEAD's commit, not
/// > of the index at a later instant — derived by one algorithm, written once, used at gate open
/// > and at answer (ADR-0105 clause 4): … `git write-tree` on a temporary index built from the
/// > current index plus tracked modifications, untracked files excluded, through the wrapper.
/// > *Verified by:* a unit test asserting the rendered tree and the recorded hash resolve to one
/// > object id; a property test over dirty-worktree fixtures, including one with an untracked file
/// > (whose presence must not change the hash) and one with a tracked modification (whose presence
/// > must).
///
/// The property is quantified over the two dirty-worktree shapes the register names, plus the
/// **commits-between** shape AC-249 names, each built in its own repository so no arm's state leaks
/// into the next. A generator here would generate what these three enumerate.
#[test]
fn ac245_write_tree_one_algorithm() {
    // ---- (a) the recorded hash and the rendered tree are ONE object id ----------------------
    let repo = Repo::with_one_commit();
    let exec = repo.exec();
    let h0 = working_tree(&repo, &exec);
    let kind = repo.git(&["cat-file", "-t", h0.as_str()]);
    assert_eq!(
        kind, "tree",
        "the recorded hash names a TREE object in the repository, not a commit"
    );
    let resolved = repo.git(&[
        "rev-parse",
        "--verify",
        &format!("{}^{{tree}}", h0.as_str()),
    ]);
    assert_eq!(
        resolved,
        h0.as_str(),
        "the rendered tree and the recorded hash resolve to one id"
    );
    // …and it is not HEAD's COMMIT id, which is the register's named wrong object.
    let head = repo.git(&["rev-parse", "HEAD"]);
    assert_ne!(
        head,
        h0.as_str(),
        "the recorded hash is the tree's, not HEAD's commit id"
    );

    // ---- (b) an UNTRACKED file must not move the hash: the answer still applies ---------------
    let record = GateRecord::derive(KIND, h0.as_str(), Timestamp::from_unix_seconds(DEADLINE))
        .expect("derive");
    repo.write("scratch.tmp", b"not tracked\n");
    let h_untracked = working_tree(&repo, &exec);
    assert_eq!(
        h_untracked, h0,
        "an untracked file left the working-state tree unchanged (ADR-0105 clause 4: excluded)"
    );
    let state = open_then_answer(&h0, &h_untracked);
    assert_applied(
        &state,
        "an untracked file dropped in the worktree is not drift",
    );
    assert_eq!(
        state.answered_gates().next().map(GateAnswer::token),
        Some(record.token()),
        "the applied answer is bound to the token the open derived"
    );

    // ---- (c) a TRACKED modification must move the hash: the answer is VOID --------------------
    //
    // RED (observed 2026-09-17) against a SECOND DERIVATION: with the answer-side hash taken
    // from `rev-parse HEAD^{tree}` instead of the wrapper's write-tree, this arm's hash did not
    // move and the answer APPLIED over a drifted tree -- `assert_void` failed on
    // "the answer is VOID: a tracked modification is drift". That is F19's 12/174.
    let repo = Repo::with_one_commit();
    let exec = repo.exec();
    let h0 = working_tree(&repo, &exec);
    repo.write("a.txt", b"one, modified\n");
    let h_modified = working_tree(&repo, &exec);
    assert_ne!(
        h_modified, h0,
        "a tracked modification moved the working-state tree (ADR-0105 clause 4: included)"
    );
    let state = open_then_answer(&h0, &h_modified);
    assert_void(&state, "a tracked modification is drift");

    // ---- (d) AC-249's fixture: a COMMIT between open and answer -------------------------------
    //
    // > GIVEN an open gate recorded with `tree_hash` T / WHEN an answer arrives and the item's
    // > current tree hash is not T / THEN the answer resolves void … *Verified by:* a unit test
    // > over a fixture that commits between open and answer.
    let repo = Repo::with_one_commit();
    let exec = repo.exec();
    let h0 = working_tree(&repo, &exec);
    repo.write("b.txt", b"two\n");
    repo.git(&["add", "b.txt"]);
    repo.git(&["commit", "--quiet", "-m", "two"]);
    let h_committed = working_tree(&repo, &exec);
    assert_ne!(
        h_committed, h0,
        "a commit between open and answer moved the tree"
    );
    let state = open_then_answer(&h0, &h_committed);
    assert_void(
        &state,
        "the human answered a tree that is no longer the item's tree",
    );

    // ---- (e) the control the property needs: the SAME tree, re-derived, still applies ---------
    // Without it, (c) and (d) are satisfied by an answer path that voids everything.
    let repo = Repo::with_one_commit();
    let exec = repo.exec();
    let h0 = working_tree(&repo, &exec);
    let again = working_tree(&repo, &exec);
    assert_eq!(
        again, h0,
        "the derivation is stable across two calls on an unchanged tree"
    );
    let state = open_then_answer(&h0, &again);
    assert_applied(
        &state,
        "the same tree re-derived at answer time is not drift",
    );

    let _keep: &Path = repo.path.as_path();
}
