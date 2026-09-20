//! **The human gate** — Step 7b-ii, `s7b-ii` §7b-ii.2. Slice A: the record and the one derivation.
//! Slice B: the answer path and the re-reach (`src/workflow/gate.rs`). Slice C: the log-only
//! reconstruction, one gate mechanism, expiry, and the kernel halves of the cross-listed ids.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac244_gate_record_requires_all_three`] | **AC-244** | the **deserialisation** half: a `gate_opened` read back from the store lacking `kind`, `tree_hash` or `deadline` folds to a typed refusal, opens no gate and moves no state | the **type-level** half, which is `tests/ui/ac244_gate_record_missing_a_field.rs` in the crate's one `trybuild` harness; and the `doctor` rule, which is `rk-cli`'s (7d) and ARMED |
//! | [`ac524_fresh_record_on_rereach`] | **AC-524** — **PROVISIONAL on P8-B; built, NOT claimed** | a re-reach mints a fresh `(token, tree_hash, deadline)` with the reach counter in the digest, two `gate_opened` rows, the counter at 2, the first token refused, the second mark's tree taken at the re-reach | drive a loop: no `on_false { to, max }` executor exists (7b parsed and linted the form), so the test performs the re-reach the executor will |
//! | [`ac246_token_is_single_use`] | **AC-246** | a second answer on a spent token is refused, records ONE `gate_answer_spent` row (ADR-0123), moves no state; a raw duplicate `gate_answered` row cannot rewrite the verdict | AC-271's cross-process arrival order (`rk-store`'s single writer; PROVISIONAL) |
//! | [`ac249_stale_tree_hash_is_void`] | **AC-249** | a drifted answer is void with `gate_answered` + `gate_answer_void`; the fold's OWN comparison voids a raw bypassing row; a void token is terminal; no transition follows a void | the git fixture, which is [`ac245_write_tree_one_algorithm`]'s |
//! | [`ac252_void_reopens_fresh_in_all_three_fields`] | **AC-252** (answers `DECISIONS-2026-09-14 D-5` from the register) | after a void the run does not hold on the dead token and `next_action` re-opens with a fresh token, the current tree, a fresh deadline, reach 2 | an EXPIRED gate's disposition — the expiry policy's terminal path (AC-257/AC-259), slice C |
//! | [`ac250_no_reconciliation_path`] | **AC-250** | 64 seeded interleavings of (commit, answer, resume, expire): no mismatched answer is ever applied, no stage moves; the source rule over every `gate_answer_void` consumer, with the committed reconciling fixture as its positive control | prove *"there exists no sequence"* by generation — the register assigns that half to the architecture rule, and this file does too |
//! | [`ac247_answer_against_the_wrong_gate_is_refused`] | **AC-247** | A's token against B is `GateError::WrongGate` naming both, nothing written, B open; 5×5 pairings apply iff i == j | — |
//! | [`ac270_gate_state_from_the_log_alone`] | **AC-270**, and **AC-258** by name | every open / answer / spent / void / re-open / expiry / escalation is present in a fold of the read-back log, byte-identical to the live derivation, under two roots; the gate path's purity rule | cross-machine reproduction (the register's own scope note) |
//! | [`ac272_escalation_is_one_gate_mechanism`] | **AC-272**, **AC-259** | the escalation's `gate_opened` carries the stage gate's members exactly plus `question`; class `escalation`; folds into the same record set; voids the same way; may not itself escalate; resolves to fail or auto-decide; not-due writes nothing | the *activity-internal* producer — no activity graph raises an escalation yet; the record shape is what is pinned |
//! | [`ac260_expiry_policy_lint`] | **AC-260** | the missing-action row and the `escalation → escalate` row fail, the three terminal paths pass, an unknown action fails | the `doctor` rule-table row (7d) |
//! | [`ac243_no_dispatch_across_open_gate`] | **AC-243** (kernel half; 7c's adapter half cross-listed) | five ticks over an open gate: `AwaitGate` every time, 0 harness dispatches, no `dispatch` row; the replay checker with a forged positive control | the ~1 h TTL itself — no adapter, no clock |
//! | [`ac279_one_prefix_rewrite`] | **AC-279** (kernel half) | the resume writes ONE `gate_resumed { prefix_rewrites: 1 }` before its dispatch, metric fields absent; a later dispatch is not a second resume; the ≤ 1 checker with a forged control | AC-267's threshold (7c, against a recorded baseline) |
//! | [`ac253_drift_is_a_tree_hash_comparison`] | **AC-253** | an amended commit over the same tree and a touch-back are not drift; a changed tree is; the predicate's module reads no clock | — |
//! | [`ac242_gate_path_blocks_nothing`] | **AC-242** (syntactic half) | the gate path carries no sleep / recv / spawn / tokio / clock / fs / env; the committed control fails the rule on every construct | the build-failing half, which is `workflow.rs`'s `#![deny]` and its `workflow-fsm-clock-reintroduce` control |
//! | [`ac261_state_is_rederived_at_every_entry`] | **AC-261** | a second `Runtime::state` sees an append the first did not; `Runtime` carries no state; red via the deleted-derivation mutant | trailers and the tree as inputs — the kernel's world is the journal; `reconcile()` is `rk-git`'s |
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

// ================================================================== Slice B — the gate module

use rk_kernel::module_boundary::{SourceFile, void_consumer_rule};
use rk_kernel::predicate::{OnFalse, classify_on_false};
use rk_kernel::select::GateReason;
use rk_kernel::workflow::fsm::{FsmContext, NextAction, RunState, next_action};
use rk_kernel::workflow::gate::AnswerMeta;
use rk_kernel::workflow::gate::{self, Disposition, GateError};
use rk_kernel::workflow::question::QuestionRegistry;
use rk_types::digest::Digest;

pub(crate) const TREE_A: &str = "aaaa111122223333444455556666777788889999";
const TREE_B: &str = "bbbb111122223333444455556666777788889999";
const TREE_C: &str = "cccc111122223333444455556666777788889999";

const LOOP_GRAPH: &str = include_str!("../fixtures/gates/ac524_two_iteration_loop.toml");
const RECONCILE_BRANCH: &str = include_str!("../fixtures/gates/ac250_reconcile_branch.rs");

pub(crate) fn ctx(tree: &str, deadline: i64) -> FsmContext<'_> {
    FsmContext::supplied(ITEM, tree, Timestamp::from_unix_seconds(deadline), None)
}

pub(crate) fn fold_all(events: &[Event]) -> RunState {
    fold(ITEM, STAGE, events).expect("the journal folds")
}

fn gate_expired(token: &Digest) -> Event {
    Event::new(
        EventKind::GateExpired,
        serde_json::json!({ "token": token.as_str() }),
    )
}

fn selection_gate_kind(selection: &rk_kernel::select::Selection) -> String {
    assert_eq!(
        selection.gates().len(),
        1,
        "PREMISE: exactly one selection gate"
    );
    format!(
        "{}:{}",
        GateReason::UnknownFacet.as_str(),
        selection.gates()[0].axis
    )
}

/// **AC-524 — a loop-back re-reach mints a FRESH record.** PROVISIONAL on P8-B: built and tested,
/// **not claimed as evidenced** (`s6`:351-352, *"build them; do not claim them"*).
///
/// > WHEN the kernel reaches the gate step again / THEN it mints a **fresh** `(token, tree_hash,
/// > deadline)` record and a new `gate_opened` event — the earlier answer is not replayed, the
/// > earlier token stays spent (AC-246), and the new mark's `tree_hash` is derived by AC-245's
/// > algorithm at the re-reach / AND a `gate_opened` counter per `(item, gate step)` is kept …
/// > *Verified by:* a kernel test over a fixture graph whose gate sits inside a two-iteration
/// > loop, asserting two distinct tokens, two `gate_opened` events, the first token refused on
/// > the second reach, and the per-(item, step) counter at 2; a replay assertion that the second
/// > mark's tree hash was taken at the re-reach.
///
/// 🛑 **Who performs the re-reach here, and why that is stated.** Step 7b parsed and linted the
/// `{ to, max }` loop-back form and built **no executor** for it — nothing in `src/` re-enters an
/// earlier step. So the re-reach below is the TEST calling [`gate::reach`], which is the entry a
/// loop executor will call, over the fixture graph the register names (proved to classify as
/// `LoopBack { to: "gate", max: 2 }` first). The freshness rule is the kernel's; the loop that
/// invokes it is not yet anyone's, and this test does not pretend otherwise.
#[test]
fn ac524_fresh_record_on_rereach() {
    // ---- the fixture graph IS the register's shape: a gate inside a { to, max = 2 } loop -----
    let table = rk_kernel::toml_subset::parse(LOOP_GRAPH).expect("the loop graph parses");
    let steps = table["step"].as_array().expect("[[step]]");
    let check = steps[1].as_table().expect("a step table");
    let on_false = check["on_false"].as_table().expect("on_false is a table");
    let max = match &on_false["max"] {
        rk_kernel::toml_subset::Value::Int(n) => *n,
        other => panic!("max is an int, not {other:?}"),
    };
    let as_json = serde_json::json!({ "to": on_false["to"].as_str().expect("to"), "max": max });
    assert_eq!(
        classify_on_false(&as_json).expect("a closed-set form"),
        OnFalse::LoopBack {
            to: "gate".to_owned(),
            max: 2
        },
        "PREMISE: the fixture's on_false is ADR-0103 clause 6's loop-back form re-reaching the gate step"
    );
    assert_eq!(
        steps[0].as_table().expect("a step")["id"].as_str(),
        Some("gate")
    );

    // ---- reach 1: the gate is opened and answered -------------------------------------------
    let kind = "human:loop-gate";
    let mut journal: Vec<Event> = Vec::new();
    let r1 = gate::reach(&fold_all(&journal), kind, &ctx(TREE_A, 9_000)).expect("mints");
    assert_eq!(r1.reach(), 1, "the first reach is reach 1");
    journal.push(gate_opened_event(&r1));
    let first = gate::answer(
        &fold_all(&journal),
        r1.token(),
        r1.token(),
        TREE_A,
        "proceed",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(first.disposition, Disposition::Applied);
    journal.extend(first.events);
    let answered = fold_all(&journal);
    assert_eq!(answered.answered_gates().count(), 1);
    assert_eq!(
        answered.reaches(kind),
        1,
        "the per-(item, gate step) counter after one reach"
    );

    // ---- reach 2: the loop-back re-reaches the gate step ------------------------------------
    //
    // RED (observed 2026-09-17, against a `reach()` that derived the prior record — the same
    // `(kind, tree_hash, deadline)` gives the same token, so the re-reach REUSED the answered
    // record). The tree and the deadline are deliberately IDENTICAL to reach 1's — a loop-back
    // with no commit in between, inside one deadline window — so the freshness cannot come from
    // the tree or the clock; only the reach counter can supply it.
    let r2 = gate::reach(&answered, kind, &ctx(TREE_A, 9_000)).expect("mints");
    assert_ne!(
        r2.token(),
        r1.token(),
        "the re-reach minted the SAME token: the prior record was reused, not a fresh mark"
    );
    assert_eq!(r2.reach(), 2, "the second reach is reach 2");
    journal.push(gate_opened_event(&r2));
    let rereached = fold_all(&journal);

    // two gate_opened events for this gate step, and the counter at 2
    let opened_rows = journal
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened && e.payload["kind"] == kind)
        .count();
    assert_eq!(opened_rows, 2, "two gate_opened events");
    assert_eq!(
        rereached.reaches(kind),
        2,
        "the per-(item, gate step) counter at 2"
    );

    // the earlier answer is NOT replayed: the fresh gate is what the run awaits
    assert_eq!(
        rereached.awaiting_gate(),
        Some(r2.token()),
        "the re-reached gate awaits its own answer; the first answer is not carried over"
    );

    // the first token is refused on the second reach — presented against the new gate, and
    // presented against its own (now spent) gate
    assert!(
        matches!(
            gate::answer(
                &rereached,
                r2.token(),
                r1.token(),
                TREE_B,
                "proceed",
                &QuestionRegistry::default(),
                &[],
                &AnswerMeta::default(),
            ),
            Err(GateError::WrongGate { .. })
        ),
        "the first token presented against the second gate is refused"
    );
    let spent = gate::answer(
        &rereached,
        r1.token(),
        r1.token(),
        TREE_A,
        "proceed",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("a refusal is an outcome, not an error");
    assert_eq!(
        spent.disposition,
        Disposition::RefusedSpent,
        "the first token stays spent (AC-246)"
    );

    // ---- replay assertion: the second mark's tree hash was taken at the re-reach --------------
    // A second journal, where the tree DID move between the reaches: the re-reached record must
    // carry the tree the re-reach saw, and two folds of that journal must agree byte for byte.
    let mut moved: Vec<Event> = Vec::new();
    let m1 = gate::reach(&fold_all(&moved), kind, &ctx(TREE_A, 9_000)).expect("mints");
    moved.push(gate_opened_event(&m1));
    let ans = gate::answer(
        &fold_all(&moved),
        m1.token(),
        m1.token(),
        TREE_A,
        "proceed",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    moved.extend(ans.events);
    let m2 = gate::reach(&fold_all(&moved), kind, &ctx(TREE_B, 9_000)).expect("mints");
    moved.push(gate_opened_event(&m2));
    let once = fold_all(&moved);
    let again = fold_all(&moved);
    let second: Vec<&GateRecord> = again.open_gates().filter(|g| g.reach() == 2).collect();
    assert_eq!(second.len(), 1);
    assert_eq!(
        second[0].tree_hash(),
        TREE_B,
        "the second mark's tree hash is the one derived AT THE RE-REACH, not the first reach's"
    );
    assert_eq!(
        once.canonical_bytes().expect("canonical"),
        again.canonical_bytes().expect("canonical"),
        "two folds of one journal agree byte for byte"
    );
    let _ = rereached;
}

/// **AC-246 — ADR-0017's single-use token.**
///
/// > GIVEN a gate whose token has already been consumed by an applied answer / WHEN a second answer
/// > presents the same token / THEN the second answer is refused as spent, records a distinct event,
/// > and changes no item state. *Verified by:* a unit test on the answer path; a replay assertion
/// > that no item's state advances twice on one token.
///
/// The three THEN clauses are three assertions, red separately: a refusal that still advanced the
/// item would satisfy the first two.
#[test]
fn ac246_token_is_single_use() {
    let kind = "human:review";
    let mut journal: Vec<Event> = Vec::new();
    let r = gate::reach(&fold_all(&journal), kind, &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&r));
    let first = gate::answer(
        &fold_all(&journal),
        r.token(),
        r.token(),
        TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(first.disposition, Disposition::Applied);
    journal.extend(first.events);
    let before = fold_all(&journal);
    assert_eq!(
        before.answered_gates().next().map(GateAnswer::verdict),
        Some("approve")
    );

    // ---- (1) refused as spent ---------------------------------------------------------------
    //
    // RED (observed 2026-09-17, on the fold at 11b347e via the probe cited in ADR-0123, and again
    // here against an `answer()` that mirrored it): the second answer was APPLIED and the recorded
    // verdict moved `approve -> changes-requested`.
    let second = gate::answer(
        &before,
        r.token(),
        r.token(),
        TREE_A,
        "changes-requested",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("a spent token is a refusal, not an error");
    assert_eq!(
        second.disposition,
        Disposition::RefusedSpent,
        "the second answer is refused as spent"
    );

    // ---- (2) records a distinct event — ADR-0123's `gate_answer_spent`, and nothing else -------
    assert_eq!(
        second.events.len(),
        1,
        "exactly one row records the refusal"
    );
    assert_eq!(second.events[0].kind, EventKind::GateAnswerSpent);
    assert_eq!(second.events[0].payload["token"], r.token().as_str());
    assert_eq!(second.events[0].payload["tree_hash"], TREE_A);

    // ---- (3) changes no item state ------------------------------------------------------------
    journal.extend(second.events);
    let after = fold_all(&journal);
    assert_eq!(
        after.canonical_bytes().expect("canonical"),
        before.canonical_bytes().expect("canonical"),
        "the refusal moved a member of the item's state"
    );
    assert_eq!(
        after.answered_gates().next().map(GateAnswer::verdict),
        Some("approve")
    );

    // ---- the fold's own half: a RAW duplicate gate_answered row (a forged or raced duplicate
    // that bypassed the answer path) must not rewrite the verdict either — this is the probe.
    let mut raced = journal.clone();
    raced.push(Event::new(
        EventKind::GateAnswered,
        serde_json::json!({ "token": r.token().as_str(), "tree_hash": TREE_A, "verdict": "changes-requested" }),
    ));
    let folded = fold_all(&raced);
    assert_eq!(
        folded.answered_gates().next().map(GateAnswer::verdict),
        Some("approve"),
        "a duplicate gate_answered row in the journal rewrote the recorded verdict"
    );
    assert_eq!(
        folded.answered_gates().count(),
        1,
        "no item's state advances twice on one token"
    );
    assert_eq!(
        folded.survey().spent_refusals(),
        1,
        "the fold COUNTED the refused duplicate rather than dropping it silently"
    );
    assert_eq!(
        folded.survey().spent_recorded(),
        1,
        "and it saw the one gate_answer_spent row the answer path wrote"
    );
}

/// **AC-249 — void, not reconciled.** The fold-level half; AC-245's test carries the git fixture.
///
/// > GIVEN an open gate recorded with `tree_hash` T / WHEN an answer arrives and the item's current
/// > tree hash is not T / THEN the answer resolves **void**: a `gate_answer_void` event is written
/// > and no approve/changes-requested decision is applied to the item. *Verified by:* … a replay
/// > assertion that void events never precede a state transition for that gate.
///
/// RED (observed 2026-09-17) via a temporary mutant of `fsm.rs` that dropped the tree-hash
/// comparison: the drifted answer was APPLIED. Reverted before commit.
#[test]
fn ac249_stale_tree_hash_is_void() {
    let kind = "human:review";
    let mut journal: Vec<Event> = Vec::new();
    let r = gate::reach(&fold_all(&journal), kind, &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&r));
    let out = gate::answer(
        &fold_all(&journal),
        r.token(),
        r.token(),
        TREE_B,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(
        out.disposition,
        Disposition::Void,
        "a drifted answer is VOID"
    );
    let kinds: Vec<EventKind> = out.events.iter().map(|e| e.kind).collect();
    assert_eq!(
        kinds,
        [EventKind::GateAnswered, EventKind::GateAnswerVoid],
        "the answer is recorded (tier 1) AND its voiding is recorded (AC-249's gate_answer_void)"
    );
    assert_eq!(out.events[1].payload["token"], r.token().as_str());
    assert_eq!(out.events[1].payload["bound_tree_hash"], TREE_A);
    assert_eq!(out.events[1].payload["observed_tree_hash"], TREE_B);
    journal.extend(out.events);
    let state = fold_all(&journal);
    assert_eq!(
        state.answered_gates().count(),
        0,
        "no approve/changes-requested decision was applied"
    );
    assert!(state.void_gates().any(|t| t == r.token()));

    // THE FOLD'S OWN COMPARISON, pinned separately: a raw mismatched `gate_answered` row that
    // BYPASSED the answer path (forged, or raced past it) must fold to void with no
    // `gate_answer_void` row to help it. The answer path above decides void itself and writes
    // that row, so without this case a fold that applied every answer would still pass — which
    // is exactly what the first mutant run showed [measured 2026-09-17].
    let raw_only = vec![
        gate_opened_event(&r),
        gate_answered(r.token().as_str(), TREE_B),
    ];
    let bypassed = fold_all(&raw_only);
    assert_eq!(
        bypassed.answered_gates().count(),
        0,
        "a raw gate_answered row whose tree hash mismatches was APPLIED by the fold"
    );
    assert!(
        bypassed.void_gates().any(|t| t == r.token()),
        "…and it is void"
    );

    // A void token is TERMINAL: a later answer carrying it, even with the right tree hash, is
    // refused — otherwise "void" would be a retry with extra steps (ADR-0017: void, not reconciled).
    let again = gate::answer(
        &state,
        r.token(),
        r.token(),
        TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("a refusal");
    assert_eq!(
        again.disposition,
        Disposition::RefusedSpent,
        "a voided token cannot be answered later"
    );
    let mut raced = journal.clone();
    raced.push(gate_answered(r.token().as_str(), TREE_A));
    assert_eq!(
        fold_all(&raced).answered_gates().count(),
        0,
        "nor can a raw row revive it"
    );

    // ---- replay: void events never precede a state transition for that gate ----------------
    let mut void_seen = false;
    for e in &journal {
        if e.kind == EventKind::GateAnswerVoid {
            void_seen = true;
        }
        if void_seen {
            assert_ne!(
                e.kind,
                EventKind::StageTransition,
                "a stage transition followed the void"
            );
        }
    }
    assert!(void_seen);
    assert!(state.stages_entered().is_empty() && state.last_transition().is_none());
}

/// **AC-252 — a void is followed by a re-open, fresh in all three fields.** This is where
/// `DECISIONS-2026-09-14 D-5`'s *"does a voided gate re-open?"* is answered — by the register, not
/// by this file.
///
/// > GIVEN an answer that resolved void / WHEN the kernel next acts on the item / THEN a new gate
/// > is opened whose record differs from the voided gate's in all three fields — a fresh token, the
/// > current tree hash, and a fresh deadline. *Verified by:* an integration test asserting the
/// > re-opened gate's record differs in all three fields; a replay assertion that a void is always
/// > followed by either a re-open or a terminal state, never by nothing.
///
/// RED (observed 2026-09-17) on the untouched `next_action`: it declined to re-open a voided token,
/// and `awaiting_gate` held the void token as still awaited.
#[test]
fn ac252_void_reopens_fresh_in_all_three_fields() {
    let gated = crate::fsm::gated();
    let kind = selection_gate_kind(&gated);

    let mut journal: Vec<Event> = Vec::new();
    let state0 = fold(ITEM, "analysis", &journal).expect("folds");
    let voided = match next_action(&gated, &state0, &ctx(TREE_A, 9_000)).expect("computes") {
        NextAction::OpenGate(record) => record,
        other => panic!("the first reach opens the gate: {other:?}"),
    };
    assert_eq!(voided.kind(), kind);
    journal.push(gate_opened_event(&voided));
    let open = fold(ITEM, "analysis", &journal).expect("folds");
    let out = gate::answer(
        &open,
        voided.token(),
        voided.token(),
        TREE_B,
        "proceed",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(out.disposition, Disposition::Void);
    journal.extend(out.events);
    let after_void = fold(ITEM, "analysis", &journal).expect("folds");

    // "never by nothing": the void token is not what the run awaits — it is terminal
    assert_ne!(
        after_void.awaiting_gate(),
        Some(voided.token()),
        "a voided gate must not hold the run at AwaitingGate on its own dead token"
    );

    // the kernel's next act, over the CURRENT tree and a FRESH deadline, is a re-open
    let reopened = match next_action(&gated, &after_void, &ctx(TREE_B, 12_600)).expect("computes") {
        NextAction::OpenGate(record) => record,
        other => panic!("after a void the kernel re-opens; it returned {other:?}"),
    };
    assert_ne!(reopened.token(), voided.token(), "a fresh token");
    assert_eq!(
        reopened.tree_hash(),
        TREE_B,
        "the current tree hash, not the voided one"
    );
    assert_ne!(reopened.tree_hash(), voided.tree_hash());
    assert_eq!(
        reopened.deadline(),
        Timestamp::from_unix_seconds(12_600),
        "a fresh deadline"
    );
    assert_ne!(reopened.deadline(), voided.deadline());
    assert_eq!(reopened.kind(), kind, "the same gate step");
    assert_eq!(reopened.reach(), 2);

    // replay: a void is followed by a re-open, and the re-opened gate is then awaited
    journal.push(gate_opened_event(&reopened));
    let final_state = fold(ITEM, "analysis", &journal).expect("folds");
    assert_eq!(final_state.awaiting_gate(), Some(reopened.token()));
    assert_eq!(
        next_action(&gated, &final_state, &ctx(TREE_B, 12_600)).expect("computes"),
        NextAction::AwaitGate {
            token: reopened.token().clone()
        }
    );
}

/// **AC-250 — no reconciliation path.** The strongest negative in the area.
///
/// > GIVEN generated interleavings of (commit, answer, resume, expire) over a gated item / WHEN the
/// > event log is replayed / THEN in no generated interleaving does an answer whose tree hash
/// > mismatched at arrival cause the item to advance / AND the codebase contains no merge,
/// > re-application or deferred-reconciliation path that consumes a `gate_answer_void` event as an
/// > input to a state transition. *Verified by:* a property/fuzz test over generated interleavings
/// > asserting the invariant on the terminal state (not a single example test); plus an
/// > architecture test that no consumer of `gate_answer_void` writes a state transition.
///
/// The register's own scope limit is honoured: the generator carries the existential half (no
/// generated sequence advances on a mismatch); the **universal** half is the architecture rule over
/// the source, with a committed lint-fail fixture as its positive control.
///
/// RED (observed 2026-09-17): the rule returned `Ok` over the fixture that carries the reconciling
/// arm — a lint with no fixture is green over a gate path nobody has written.
#[test]
fn ac250_no_reconciliation_path() {
    // ---- (a) the interleaving property, over a seeded generator ------------------------------
    // A deterministic LCG, not `rand`: the sequence set is reproducible from the seed printed on
    // failure, and the kernel's own clippy denylist bans `rand::random`/`thread_rng` anyway.
    let gated = crate::fsm::gated();
    let kind = selection_gate_kind(&gated);
    let trees = [TREE_A, TREE_B, TREE_C];
    let mut sequences = 0usize;
    let mut mismatched_answers = 0usize;
    let mut voids_seen = 0usize;
    for seed in 1u64..=64 {
        let mut x = seed;
        let mut next = move || {
            x = x
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            usize::try_from(x >> 33).expect("fits")
        };
        let mut journal: Vec<Event> = Vec::new();
        let mut tree = 0usize; // the item's current tree; a commit moves it
        let mut deadline = 9_000i64;
        for _step in 0..12 {
            let state = fold(ITEM, "analysis", &journal).expect("folds");
            match next() % 4 {
                0 => tree = (tree + 1) % trees.len(), // commit
                1 => {
                    // answer the latest gate at the CURRENT tree — which may not be the bound one
                    if let Some(token) = state.latest_gate(&kind).cloned()
                        && let Ok(out) = gate::answer(
                            &state,
                            &token,
                            &token,
                            trees[tree],
                            "proceed",
                            &QuestionRegistry::default(),
                            &[],
                            &AnswerMeta::default(),
                        )
                    {
                        if out.disposition == Disposition::Void {
                            mismatched_answers += 1;
                        }
                        journal.extend(out.events);
                    }
                }
                2 => {
                    // resume: the kernel's next action
                    deadline += 600;
                    if let NextAction::OpenGate(record) =
                        next_action(&gated, &state, &ctx(trees[tree], deadline)).expect("computes")
                    {
                        journal.push(gate_opened_event(&record));
                    }
                }
                _ => {
                    if let Some(token) = state.awaiting_gate() {
                        journal.push(gate_expired(token));
                    }
                }
            }
        }
        // ---- the invariant on the terminal state ----
        let terminal = fold(ITEM, "analysis", &journal).expect("folds");
        for answer in terminal.answered_gates() {
            let record = terminal
                .open_gates()
                .find(|g| g.token() == answer.token())
                .unwrap_or_else(|| panic!("seed {seed}: an answer with no record"));
            assert_eq!(
                answer.tree_hash(),
                record.tree_hash(),
                "seed {seed}: an answer whose tree hash mismatched at arrival was APPLIED"
            );
        }
        assert!(
            terminal.stages_entered().is_empty() && terminal.last_transition().is_none(),
            "seed {seed}: the item advanced a stage inside a gate-only interleaving"
        );
        voids_seen += terminal.void_gates().count();
        sequences += 1;
    }
    assert_eq!(sequences, 64);
    assert!(
        mismatched_answers > 0 && voids_seen > 0,
        "PREMISE: the generator must actually produce mismatched answers ({mismatched_answers}) and voids ({voids_seen}), or the property is vacuous"
    );

    // ---- (b) the architecture rule: no consumer of gate_answer_void writes a transition ------
    let files = rk_kernel::module_boundary::kernel_sources().expect("src/ is readable");
    let report = void_consumer_rule(&files).unwrap_or_else(|violations| {
        panic!("a consumer of gate_answer_void writes a state transition:\n{violations:#?}")
    });
    assert!(
        report.consumers_scanned >= 1,
        "PREMISE: the rule found no gate_answer_void consumer at all — the fold's arm exists, so the walk is wrong"
    );
    // POSITIVE CONTROL: the committed reconciling arm MUST be a violation.
    let fixture = vec![SourceFile {
        path: "tests/fixtures/gates/ac250_reconcile_branch.rs".to_owned(),
        text: RECONCILE_BRANCH.to_owned(),
    }];
    let violations = void_consumer_rule(&fixture)
        .expect_err("the reconciling fixture is the shape the rule forbids and must be reported");
    assert!(
        violations
            .iter()
            .any(|v| v.text().contains("state.stage = ")),
        "the violation names the transition write: {violations:#?}"
    );
}

/// **AC-247 — a token is scoped to its gate.**
///
/// > GIVEN two open gates A and B on the same or different items / WHEN an answer carrying A's token
/// > is submitted against B / THEN it is refused and B remains open. *Verified by:* a unit test over
/// > a two-gate fixture; a fuzz test over token/gate pairings.
///
/// RED (observed 2026-09-17) against an `answer()` that ignored which gate the answer was
/// submitted against: A's token against B was applied.
#[test]
fn ac247_answer_against_the_wrong_gate_is_refused() {
    let mut journal: Vec<Event> = Vec::new();
    let a = gate::reach(&fold_all(&journal), "human:a", &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&a));
    let b = gate::reach(&fold_all(&journal), "human:b", &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&b));
    let state = fold_all(&journal);
    assert_eq!(state.open_gates().count(), 2);

    let refused = gate::answer(
        &state,
        b.token(),
        a.token(),
        TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    );
    assert!(
        matches!(&refused, Err(GateError::WrongGate { gate, presented }) if gate == b.token() && presented == a.token()),
        "A's token against B is refused, naming both: {refused:?}"
    );
    // B remains open, and nothing was written: the refusal is an error, not an event.
    assert!(state.open_gates().any(|g| g.token() == b.token()));
    assert!(state.awaiting_gate().is_some());

    // ---- the pairing fuzz: over N gates, (gate_j, token_i) applies iff i == j ----------------
    let mut journal: Vec<Event> = Vec::new();
    let mut gates: Vec<GateRecord> = Vec::new();
    for i in 0..5 {
        let g = gate::reach(
            &fold_all(&journal),
            &format!("human:g{i}"),
            &ctx(TREE_A, 9_000 + i),
        )
        .expect("mints");
        journal.push(gate_opened_event(&g));
        gates.push(g);
    }
    let state = fold_all(&journal);
    for (j, target) in gates.iter().enumerate() {
        for (i, presented) in gates.iter().enumerate() {
            let out = gate::answer(
                &state,
                target.token(),
                presented.token(),
                TREE_A,
                "approve",
                &QuestionRegistry::default(),
                &[],
                &AnswerMeta::default(),
            );
            if i == j {
                assert!(
                    matches!(&out, Ok(o) if o.disposition == Disposition::Applied),
                    "({i},{j}) applies"
                );
            } else {
                assert!(
                    matches!(out, Err(GateError::WrongGate { .. })),
                    "({i},{j}) is refused"
                );
            }
        }
    }
}

// ================================================================== Slice C — log-only state, one gate mechanism

use rk_kernel::module_boundary::gate_path_purity_rule;
use rk_kernel::runtime::StepOutcome;
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::gate::{ExpiryAction, Resolution};

const GATE_PATH_CONTROL: &str = include_str!("../fixtures/lint-controls/ac242_gate_path_blocks.rs");
const EXPIRY_POLICY_ROWS: &str = include_str!("../fixtures/gates/ac260_expiry_policy_rows.toml");

fn at(secs: i64) -> Timestamp {
    Timestamp::from_unix_seconds(secs)
}

/// A journal that exercises every gate transition AC-270 names — open, answer, void, expiry,
/// escalation and re-open — built through the answer path and the runtime, never by hand.
fn every_transition_journal() -> (Vec<Event>, Vec<GateRecord>) {
    let mut journal: Vec<Event> = Vec::new();
    let mut records: Vec<GateRecord> = Vec::new();
    let kind = "human:review";

    // open, answer, spent
    let g1 = gate::reach(&fold_all(&journal), kind, &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&g1));
    let a = gate::answer(
        &fold_all(&journal),
        g1.token(),
        g1.token(),
        TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    journal.extend(a.events);
    let spent = gate::answer(
        &fold_all(&journal),
        g1.token(),
        g1.token(),
        TREE_A,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("refuses");
    journal.extend(spent.events);
    records.push(g1);

    // a second gate step: open, void, re-open (AC-252's shape via reach)
    let kind2 = "human:release";
    let g2 = gate::reach(&fold_all(&journal), kind2, &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&g2));
    let v = gate::answer(
        &fold_all(&journal),
        g2.token(),
        g2.token(),
        TREE_B,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(v.disposition, Disposition::Void);
    journal.extend(v.events);
    let g2b = gate::reach(&fold_all(&journal), kind2, &ctx(TREE_B, 12_600)).expect("mints");
    journal.push(gate_opened_event(&g2b));
    records.push(g2);
    records.push(g2b.clone());

    // expiry with escalation: the re-opened gate passes its deadline unanswered
    let esc = gate::expire(
        &fold_all(&journal),
        g2b.token(),
        at(12_601),
        &ExpiryAction::Escalate {
            deadline: at(16_200),
        },
        TREE_B,
    )
    .expect("resolves");
    let Resolution::Escalated { record: e } = &esc.resolution else {
        panic!(
            "an escalate policy past its deadline escalates: {:?}",
            esc.resolution
        );
    };
    let e = e.clone();
    journal.extend(esc.events);
    records.push(e);
    (journal, records)
}

/// **AC-270 — every gate transition is reconstructible from the log alone.** Doubles as
/// **AC-258**'s clock-independence instrument, by the register's own instruction.
///
/// > GIVEN only the event log for an item / WHEN it is replayed / THEN every gate open, answer,
/// > void, expiry, escalation and re-open is present and the item's terminal gate state is
/// > reconstructible without reading the working tree. *Verified by:* a replay assertion that
/// > reconstructs state from the log alone and compares it to the live derivation; the same test
/// > doubles as AC-258's clock-independence instrument.
///
/// RED (observed 2026-09-17) against an `expire()` that never resolved: the journal carried no
/// expiry and no escalation, so the reconstruction could not contain them.
#[test]
fn ac270_gate_state_from_the_log_alone() {
    let (journal, records) = every_transition_journal();
    let kinds: Vec<EventKind> = journal.iter().map(|e| e.kind).collect();
    for k in [
        EventKind::GateOpened,
        EventKind::GateAnswered,
        EventKind::GateAnswerSpent,
        EventKind::GateAnswerVoid,
        EventKind::GateExpired,
    ] {
        assert!(kinds.contains(&k), "the journal carries a {k:?} row");
    }
    assert_eq!(
        kinds
            .iter()
            .filter(|k| **k == EventKind::GateOpened)
            .count(),
        4,
        "open, open, re-open, escalation — four gate_opened rows"
    );

    // the LIVE derivation: what the runtime computes from the store it appended to
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = runtime_over(&store);
    for e in &journal {
        rt.append(e.clone()).expect("append");
    }
    let live = rt.state(ITEM).expect("the live derivation folds");

    // the reconstruction: the log ALONE, read back, with no worktree in reach — the fold takes
    // events and nothing else, and the purity rule below is what makes "nothing else" a fact
    let read_back = rt.journal(ITEM).expect("query");
    let reconstructed = fold_all(&read_back);
    assert_eq!(
        reconstructed.canonical_bytes().expect("canonical"),
        live.canonical_bytes().expect("canonical"),
        "the log alone reconstructs the live derivation byte for byte"
    );
    // every transition is PRESENT in the reconstruction
    assert!(reconstructed.is_answered(records[0].token()), "the answer");
    assert!(reconstructed.is_void(records[1].token()), "the void");
    assert!(
        reconstructed.is_expired(records[2].token()),
        "the expiry of the re-opened gate"
    );
    assert_eq!(records[3].kind(), "escalation", "the escalation");
    assert_eq!(
        reconstructed.awaiting_gate(),
        Some(records[3].token()),
        "…which is what the run awaits"
    );
    assert_eq!(reconstructed.reaches("human:release"), 2, "the re-open");
    // The two spent counters read differently on purpose: the answer path refused the duplicate
    // BEFORE writing a `gate_answered` row, so the fold refused nothing (0) and recorded the one
    // `gate_answer_spent` row the path wrote (1). A raw duplicate row would move the first.
    assert_eq!(
        reconstructed.survey().spent_refusals(),
        0,
        "no raw duplicate reached the fold"
    );
    assert_eq!(
        reconstructed.survey().spent_recorded(),
        1,
        "the refusal's own row is present"
    );

    // ---- AC-258, by name: two injected clocks and two roots give one terminal state -----------
    //
    // > WHEN the log is replayed twice under two different injected clocks and two different
    // > working directories / THEN the gate's terminal state and the item's next action are
    // > identical.
    // The fold takes no clock — `Timestamp` has no `now()` — so the "injected clock" is the
    // journaled instant the expiry was resolved at, already in the log; what can vary between
    // two replays is the ambient host, so the root is moved between them.
    let here = std::env::current_dir().expect("cwd");
    let elsewhere = tempfile::tempdir().expect("tempdir");
    let first = fold_all(&read_back);
    std::env::set_current_dir(elsewhere.path()).expect("chdir");
    let second = fold_all(&read_back);
    std::env::set_current_dir(&here).expect("chdir back");
    assert_eq!(
        first.terminal(),
        second.terminal(),
        "AC-258: one terminal state under two roots"
    );
    assert_eq!(
        first.canonical_bytes().expect("canonical"),
        second.canonical_bytes().expect("canonical"),
        "AC-258: byte-identical state under two roots"
    );
    let purity = gate_path_purity_rule(
        &rk_kernel::module_boundary::kernel_sources().expect("src/ is readable"),
    )
    .unwrap_or_else(|v| {
        panic!("AC-258/AC-242: the gate path reads ambient host state or blocks:\n{v:#?}")
    });
    assert!(
        purity.files_scanned >= 2,
        "PREMISE: the gate path is fsm.rs and gate.rs; the rule scanned {}",
        purity.files_scanned
    );
}

/// **AC-272 — escalation is the same gate mechanism.**
///
/// > GIVEN an escalation raised from inside an activity's internal graph / WHEN it is observed in
/// > the event log and the trailer / THEN it is indistinguishable in mechanism from a stage-level
/// > gate: the same record shape, the same reconcile path, the same terminal-expiry rule.
/// > *Verified by:* a test asserting the two producers emit structurally identical `gate_opened`
/// > records; a schema test that one gate type exists, not two.
///
/// And **AC-259**'s clauses in the same test: class `escalation`, a payload carrying only a
/// question id beyond the record, and an escalation that may not itself escalate.
///
/// RED (observed 2026-09-17) against an `expire()` that never escalated.
#[test]
fn ac272_escalation_is_one_gate_mechanism() {
    let mut journal: Vec<Event> = Vec::new();
    let stage_gate =
        gate::reach(&fold_all(&journal), "human:review", &ctx(TREE_A, 9_000)).expect("mints");
    journal.push(gate_opened_event(&stage_gate));
    let out = gate::expire(
        &fold_all(&journal),
        stage_gate.token(),
        at(9_001),
        &ExpiryAction::Escalate {
            deadline: at(12_600),
        },
        TREE_A,
    )
    .expect("resolves");
    let Resolution::Escalated { record: escalation } = &out.resolution else {
        panic!("escalate past the deadline escalates: {:?}", out.resolution);
    };
    let kinds: Vec<EventKind> = out.events.iter().map(|e| e.kind).collect();
    assert_eq!(kinds, [EventKind::GateExpired, EventKind::GateOpened]);
    assert_eq!(out.events[0].payload["action"], "escalate");

    // (a) structurally identical `gate_opened` records: the SAME payload keys as the stage gate's
    // — plus exactly one, the question id (AC-259: "carries only a question id", ADR-0038).
    let stage_row = gate_opened_event(&stage_gate);
    let stage_keys: Vec<&str> = stage_row
        .payload
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    let esc_payload = out.events[1].payload.as_object().expect("object");
    let mut esc_keys: Vec<&str> = esc_payload.keys().map(String::as_str).collect();
    assert!(
        esc_keys.contains(&"question"),
        "the escalation names its question: {esc_keys:?}"
    );
    esc_keys.retain(|k| *k != "question");
    assert_eq!(
        esc_keys, stage_keys,
        "the record members are the stage gate's, exactly"
    );
    assert_eq!(
        esc_payload["question"],
        stage_gate.kind(),
        "the question id is the gate step it escalates"
    );
    assert_eq!(escalation.kind(), "escalation", "the class is fixed");
    assert_eq!(escalation.tree_hash(), TREE_A);
    assert_eq!(escalation.deadline(), at(12_600));
    assert_eq!(escalation.reach(), 1, "the first escalation of this step");

    // (b) one gate type, not two: the escalation folds into the SAME record set as a stage gate,
    // through the same arm, and no second kind exists for it.
    journal.extend(out.events.clone());
    let state = fold_all(&journal);
    assert!(state.is_expired(stage_gate.token()));
    assert_eq!(state.awaiting_gate(), Some(escalation.token()));
    let folded: Vec<&GateRecord> = state
        .open_gates()
        .filter(|g| g.token() == escalation.token())
        .collect();
    assert_eq!(
        folded,
        vec![escalation],
        "the escalation is a GateRecord like any other"
    );
    assert_eq!(
        EventKind::from_wire("gate_escalated"),
        None,
        "there is no second gate kind — escalation rides gate_opened"
    );
    // …and the same reconcile path: an answer over a drifted tree voids it exactly as a stage gate
    let drifted = gate::answer(
        &state,
        escalation.token(),
        escalation.token(),
        TREE_B,
        "approve",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    assert_eq!(drifted.disposition, Disposition::Void);

    // (c) the same terminal-expiry rule, bounded: an escalation may not itself escalate
    // (ADR-0105 clause 10; AC-259/AC-260) — it resolves to fail or auto-decide, never a third mark.
    let again = gate::expire(
        &state,
        escalation.token(),
        at(12_601),
        &ExpiryAction::Escalate {
            deadline: at(20_000),
        },
        TREE_A,
    );
    assert!(
        matches!(again, Err(GateError::EscalateOnEscalation { .. })),
        "escalate-on-escalate is refused: {again:?}"
    );
    let failed = gate::expire(
        &state,
        escalation.token(),
        at(12_601),
        &ExpiryAction::Fail,
        TREE_A,
    )
    .expect("fail resolves");
    assert_eq!(failed.resolution, Resolution::Failed);
    let auto = gate::expire(
        &state,
        escalation.token(),
        at(12_601),
        &ExpiryAction::AutoDecide {
            verdict: "approve".to_owned(),
        },
        TREE_A,
    )
    .expect("auto-decide resolves");
    assert_eq!(auto.resolution, Resolution::AutoDecided);
    // a gate not yet due is not resolved at all: nothing written
    let early = gate::expire(
        &state,
        escalation.token(),
        at(12_599),
        &ExpiryAction::Fail,
        TREE_A,
    )
    .expect("not due is an outcome");
    assert_eq!(early.resolution, Resolution::NotDue);
    assert!(early.events.is_empty());
}

/// **AC-260 — the expiry policy has no implicit default and no escalate-on-escalate.**
///
/// > GIVEN a gate class in the policy table with no expiry action declared, or the `escalation`
/// > class with expiry action `escalate` / WHEN the table is linted / THEN the lint fails.
/// > *Verified by:* a lint rule over the policy table with a fixture table that must fail on the
/// > missing action and a second must-fail fixture row `escalation → escalate`; a `doctor`
/// > rule-table row.
///
/// The `doctor` row is `rk-cli`'s (7d) and ARMED. RED (observed 2026-09-17) against a lint that
/// accepted every row.
#[test]
fn ac260_expiry_policy_lint() {
    let table = rk_kernel::toml_subset::parse(EXPIRY_POLICY_ROWS).expect("the fixture parses");
    let rows = table["row"].as_array().expect("[[row]]");
    let mut outcomes: Vec<(String, bool)> = Vec::new();
    for row in rows {
        let row = row.as_table().expect("a row table");
        let class = row["class"].as_str().expect("class");
        let action = row
            .get("action")
            .and_then(rk_kernel::toml_subset::Value::as_str);
        let expect_ok = matches!(row["expect"].as_str(), Some("ok"));
        let verdict = gate::lint_expiry_policy_row(class, action);
        outcomes.push((class.to_owned(), verdict.is_ok()));
        assert_eq!(
            verdict.is_ok(),
            expect_ok,
            "row `{class}` / {action:?}: expected {} and got {verdict:?}",
            if expect_ok { "ok" } else { "a lint failure" }
        );
    }
    // the two must-fail rows the register names are both present and both failed
    assert!(
        outcomes.iter().any(|(c, ok)| c == "review" && !ok),
        "the missing-action row failed"
    );
    assert!(
        outcomes.iter().any(|(c, ok)| c == "escalation" && !ok),
        "the escalation→escalate row failed"
    );
    assert!(
        outcomes.iter().any(|(_, ok)| *ok),
        "PREMISE: at least one valid row passes, or the lint refuses everything"
    );
}

/// **AC-243 — zero dispatches while a gate is open.** The kernel half; cross-listed with 7c.
///
/// > THEN zero harness dispatches are issued for that item until the gate reaches a terminal
/// > state. *Verified by:* a replay assertion over the event log — no `dispatch` event with a
/// > timestamp between `gate_opened` and the gate's terminal event for the same item; a regression
/// > test on the P8 fixture asserting the dispatch count across an open gate is 0.
///
/// RED (observed 2026-09-17) against a replay checker that never reported: its positive control —
/// a hand-built journal with a dispatch between open and answer — was read as clean.
#[test]
fn ac243_no_dispatch_across_open_gate() {
    // ---- the runtime: a gate opens, and the next action is AwaitGate, never Dispatch ----------
    let gated = crate::fsm::gated();
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(Vec::new()));
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = Runtime::new(
        crate::runtime::ports(
            Arc::clone(&harness) as Arc<dyn rk_ports::harness::Harness>,
            &store,
        ),
        "run-ac243",
        "analysis",
    );
    let state0 = rt.state(ITEM).expect("folds");
    let open = match next_action(&gated, &state0, &ctx(TREE_A, 9_000)).expect("computes") {
        NextAction::OpenGate(record) => record,
        other => panic!("{other:?}"),
    };
    let outcome = rt
        .step(
            &NextAction::OpenGate(open.clone()),
            None,
            AttemptCap::declared(4),
            JournaledNow::from_journal(at(9_000)),
        )
        .expect("steps");
    assert!(matches!(outcome, StepOutcome::GateOpened { .. }));
    for tick in 1..=5 {
        let state = rt.state(ITEM).expect("folds");
        let action = next_action(&gated, &state, &ctx(TREE_A, 9_000 + tick)).expect("computes");
        assert!(
            matches!(action, NextAction::AwaitGate { .. }),
            "tick {tick}: with a gate open the next action is AwaitGate, not {action:?}"
        );
        rt.step(
            &action,
            None,
            AttemptCap::declared(4),
            JournaledNow::from_journal(at(9_000 + tick)),
        )
        .expect("awaiting is a step");
    }
    assert_eq!(
        harness.dispatches(),
        0,
        "the P8 regression: 0 dispatches across an open gate"
    );
    let journal = rt.journal(ITEM).expect("query");
    assert!(
        journal.iter().all(|e| e.kind != EventKind::Dispatch),
        "no dispatch row was written while the gate was open"
    );

    // ---- the replay assertion, as a checker over ANY journal ---------------------------------
    assert_eq!(
        gate::dispatches_across_open_gates(&journal),
        Vec::<gate::DispatchAcrossGate>::new(),
        "the runtime's own journal is clean"
    );
    // POSITIVE CONTROL: a dispatch between gate_opened and the gate's terminal event is reported.
    let mut forged = vec![gate_opened_event(&open)];
    forged.push(Event::new(
        EventKind::Dispatch,
        serde_json::json!({ "item": ITEM, "activity_id_at_version": "impl-report@1", "attempt": 1 }),
    ));
    forged.push(gate::gate_answered_event(open.token(), TREE_A, "proceed"));
    let found = gate::dispatches_across_open_gates(&forged);
    assert_eq!(
        found.len(),
        1,
        "the dispatch inside the open window is reported: {found:?}"
    );
    assert_eq!(found[0].gate, *open.token());
    // …and a dispatch AFTER the terminal event is not.
    let mut after = vec![
        gate_opened_event(&open),
        gate::gate_answered_event(open.token(), TREE_A, "proceed"),
    ];
    after.push(forged[1].clone());
    assert!(
        gate::dispatches_across_open_gates(&after).is_empty(),
        "a dispatch after the answer is the resume, not a violation"
    );
}

/// **AC-279 — at most one prompt-prefix rewrite per resume.** The kernel half; cross-listed with
/// 7c, whose adapter fills `gate_resumed`'s metric fields (AC-267, ARMED there).
///
/// > WHEN the item resumes / THEN the resume performs at most one prompt-prefix rewrite.
/// > *Verified by:* a count assertion on prefix-rewrite events per `gate_resumed`, asserted ≤ 1.
///
/// The kernel's count is the number of resume dispatches it issues for the gate — one — and the
/// `gate_resumed` row (ADR-0123) carries it. RED (observed 2026-09-17) against a runtime that
/// wrote no `gate_resumed` at all: the resume had no row to count against.
#[test]
fn ac279_one_prefix_rewrite() {
    use rk_kernel::workflow::idem::checked_key;
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![
        crate::runtime::ok_typed(serde_json::json!({ "n": 1 })),
        crate::runtime::ok_typed(serde_json::json!({ "n": 2 })),
    ]));
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = Runtime::new(
        crate::runtime::ports(
            Arc::clone(&harness) as Arc<dyn rk_ports::harness::Harness>,
            &store,
        ),
        "run-ac279",
        STAGE,
    );
    // a gate opens and is answered, then the item resumes into its first dispatch
    let g = gate::reach(
        &rt.state(ITEM).expect("folds"),
        "human:review",
        &ctx(TREE_A, 9_000),
    )
    .expect("mints");
    rt.append(gate_opened_event(&g)).expect("append");
    let ans = gate::answer(
        &rt.state(ITEM).expect("folds"),
        g.token(),
        g.token(),
        TREE_A,
        "proceed",
        &QuestionRegistry::default(),
        &[],
        &AnswerMeta::default(),
    )
    .expect("answers");
    for e in ans.events {
        rt.append(e).expect("append");
    }
    let resume = NextAction::Dispatch {
        key: checked_key(ITEM, crate::runtime::AID, 1).expect("well-formed"),
        step_id: crate::runtime::STEP.to_owned(),
    };
    rt.step(
        &resume,
        Some(&crate::runtime::spec()),
        AttemptCap::declared(crate::runtime::CAP),
        JournaledNow::from_journal(at(9_600)),
    )
    .expect("the resume dispatches");
    assert_eq!(harness.dispatches(), 1);

    let journal = rt.journal(ITEM).expect("query");
    let resumed: Vec<&Event> = journal
        .iter()
        .filter(|e| e.kind == EventKind::GateResumed)
        .collect();
    assert_eq!(
        resumed.len(),
        1,
        "the resume recorded exactly one gate_resumed row"
    );
    assert_eq!(resumed[0].payload["token"], g.token().as_str());
    assert_eq!(
        resumed[0].payload["prefix_rewrites"], 1,
        "one prefix rewrite for one resume"
    );
    assert!(
        resumed[0]
            .payload
            .get("cache_creation_input_tokens")
            .is_none()
            && resumed[0].payload.get("total_cost_usd").is_none(),
        "the adapter's metric fields are ABSENT, not zero, until an adapter fills them (ADR-0123)"
    );
    // the row precedes the dispatch it describes
    let i_resumed = journal
        .iter()
        .position(|e| e.kind == EventKind::GateResumed)
        .expect("present");
    let i_dispatch = journal
        .iter()
        .position(|e| e.kind == EventKind::Dispatch)
        .expect("present");
    assert!(
        i_resumed < i_dispatch,
        "gate_resumed is written before the resumed dispatch's row"
    );

    // the ≤ 1 assertion as a checker over any journal, and its positive control
    assert!(gate::prefix_rewrites_exceeding_one(&journal).is_empty());
    let mut forged = journal.clone();
    forged.push(Event::new(
        EventKind::GateResumed,
        serde_json::json!({ "token": g.token().as_str(), "prefix_rewrites": 2 }),
    ));
    assert_eq!(
        gate::prefix_rewrites_exceeding_one(&forged).len(),
        1,
        "a resume that rewrote twice is reported"
    );

    // a second dispatch in the same resumed run is NOT a second resume: no second gate_resumed
    rt.step(
        &NextAction::Dispatch {
            key: checked_key(ITEM, crate::runtime::AID, 2).expect("well-formed"),
            step_id: crate::runtime::STEP.to_owned(),
        },
        Some(&crate::runtime::spec()),
        AttemptCap::declared(crate::runtime::CAP),
        JournaledNow::from_journal(at(9_700)),
    )
    .expect("dispatches");
    let n = rt
        .journal(ITEM)
        .expect("query")
        .iter()
        .filter(|e| e.kind == EventKind::GateResumed)
        .count();
    assert_eq!(
        n, 1,
        "one resume, one row — a later dispatch in the same run is not a resume"
    );
}

/// **AC-253 — drift is a comparison of tree hashes and nothing else.**
///
/// > THEN the decision is a comparison of tree hashes only — never file mtimes, wall-clock elapsed
/// > time, commit counts, or branch tip identity. *Verified by:* a unit test over a fixture that
/// > rewrites history to an identical tree (drift must be false) and one that touches files back
/// > to identical content (drift must be false); an architecture test that the drift predicate
/// > reads no clock.
///
/// RED (observed 2026-09-17) against a predicate that answered `true` for every pair.
#[test]
fn ac253_drift_is_a_tree_hash_comparison() {
    let repo = Repo::with_one_commit();
    let exec = repo.exec();
    let h0 = working_tree(&repo, &exec);
    let record = GateRecord::derive("human:review", h0.as_str(), at(9_000)).expect("derive");

    // (a) history rewritten to an IDENTICAL tree: a new commit id, the same tree → no drift
    repo.git(&["commit", "--quiet", "--amend", "-m", "one, reworded"]);
    let h_amended = working_tree(&repo, &exec);
    assert_eq!(
        h_amended, h0,
        "PREMISE: the amend changed the commit and not the tree"
    );
    assert!(
        !gate::drifted(&record, h_amended.as_str()),
        "a rewritten history over the same tree is not drift"
    );

    // (b) files touched back to identical content: mtime moved, content did not → no drift
    repo.write("a.txt", b"one, modified\n");
    repo.write("a.txt", b"one\n");
    let h_touched = working_tree(&repo, &exec);
    assert_eq!(
        h_touched, h0,
        "PREMISE: touching back to identical content leaves the tree alone"
    );
    assert!(
        !gate::drifted(&record, h_touched.as_str()),
        "an mtime change is not drift"
    );

    // (c) the control: a real content change IS drift
    repo.write("a.txt", b"two\n");
    let h_changed = working_tree(&repo, &exec);
    assert_ne!(h_changed, h0);
    assert!(
        gate::drifted(&record, h_changed.as_str()),
        "a changed tree is drift"
    );

    // (d) the predicate reads no clock and no filesystem: the gate path's purity rule
    gate_path_purity_rule(&rk_kernel::module_boundary::kernel_sources().expect("src/ is readable"))
        .unwrap_or_else(|v| {
            panic!("the drift predicate's module reads a clock or the host:\n{v:#?}")
        });
}

/// **AC-242 — nothing on the gate path sleeps, blocks or spawns a timer.** The syntactic half.
///
/// > THEN the build fails if that code contains a call to `thread::sleep`, a blocking channel
/// > receive, or a timer/scheduler spawn on the gate-lifecycle path. *Verified by:* a
/// > `cargo`-level deny-list / clippy lint over the named modules, failing the build rather than
/// > a test run; a fixture module containing each forbidden construct that must fail the lint.
///
/// The build-failing half is `workflow.rs`'s `#![deny(clippy::disallowed_methods)]` over
/// `thread::sleep`, which reaches `gate.rs` as a child (and `workflow-clock-reintroduce` is its
/// committed control). This test is the source rule for the constructs clippy's denylist does not
/// name — a blocking `recv`, a `thread::spawn`, a `tokio::` timer — with the fixture module the
/// register asks for as its positive control. RED (observed 2026-09-17) against a rule that
/// reported nothing over the fixture.
#[test]
fn ac242_gate_path_blocks_nothing() {
    let report = gate_path_purity_rule(
        &rk_kernel::module_boundary::kernel_sources().expect("src/ is readable"),
    )
    .unwrap_or_else(|v| panic!("the gate path blocks, sleeps or spawns:\n{v:#?}"));
    assert!(
        report.files_scanned >= 2,
        "fsm.rs and gate.rs are the gate path; scanned {}",
        report.files_scanned
    );
    assert!(
        report.code_lines_scanned > 100,
        "PREMISE: the rule read code, not an empty walk"
    );

    // POSITIVE CONTROL: the fixture module carries each forbidden construct and must fail.
    let fixture = vec![SourceFile {
        path: "crates/rk-kernel/src/workflow/gate.rs".to_owned(), // the path the rule scopes on
        text: GATE_PATH_CONTROL.to_owned(),
    }];
    let violations =
        gate_path_purity_rule(&fixture).expect_err("the control carries every banned construct");
    for needle in [
        "thread::sleep",
        ".recv(",
        "thread::spawn",
        "SystemTime",
        "std::fs",
        "std::env",
    ] {
        assert!(
            violations.iter().any(|v| v.text().contains(needle)),
            "the rule reports `{needle}`: {violations:#?}"
        );
    }
}

/// **AC-261 — the state is re-derived from the world at every entry, with no cached fast path.**
///
/// > THEN it re-derives the item's state from the world (tree, trailers, event log) before acting,
/// > with no cached-state fast path that skips that derivation. *Verified by:* an architecture
/// > test asserting the single entry point calls the derivation; a mutation-style test — deleting
/// > the re-derivation call must fail at least one test.
///
/// The kernel's entry point is `Runtime::state`, and its derivation is the fold over the journal
/// read at that call. RED (observed 2026-09-17) via a temporary mutant of `runtime.rs` that
/// returned the fold of an EMPTY journal — the deleted derivation the register names — and the
/// assertion below failed on the append the second call could not see. Reverted before commit.
#[test]
fn ac261_state_is_rederived_at_every_entry() {
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let rt = runtime_over(&store);
    let g = gate::reach(
        &rt.state(ITEM).expect("folds"),
        "human:review",
        &ctx(TREE_A, 9_000),
    )
    .expect("mints");
    let before = rt.state(ITEM).expect("folds");
    assert_eq!(before.open_gates().count(), 0);
    rt.append(gate_opened_event(&g)).expect("append");
    let after = rt.state(ITEM).expect("folds");
    assert_eq!(
        after.open_gates().count(),
        1,
        "the second entry re-derived the state and saw the append"
    );
    assert_ne!(
        before.canonical_bytes().expect("c"),
        after.canonical_bytes().expect("c")
    );

    // the architecture half: the entry point holds no state to serve from — `Runtime` carries
    // ports, a run id and a stage, and nothing of `RunState`'s
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime.rs"),
    )
    .expect("runtime.rs is readable");
    let start = src.find("pub struct Runtime {").expect("the struct");
    let end = src[start..].find('}').expect("closes") + start;
    let body = &src[start..end];
    assert!(
        !body.contains("RunState") && !body.contains("cache") && !body.contains("Cell"),
        "Runtime carries a cached state: {body}"
    );
    assert!(
        src.contains("crate::workflow::fsm::fold(item, &self.stage, &events)"),
        "the single entry point calls the derivation"
    );
}
