//! `answer` — AC-405, AC-248, AC-406, AC-407, AC-410, AC-263, AC-435 (machine half).
//!
//! **This module opens with a source scan and no product code, deliberately.** `s7d` §7d.16 item
//! 3's critic sequences `ac405_gate_answer_has_exactly_one_call_site` FIRST in this cycle because
//! it is *red at zero* on the tree that precedes `verbs/answer.rs` and it constrains that file's
//! **import spelling before a character of it is written**. Every other test here needs the verb to
//! exist; this one needs it not to, first.

use rk_cli::verbs::answer::Channel;

use crate::{assert_no_block_comment, code_lines, lint_controls_dir, rs_files, src_dir};
use std::path::PathBuf;

/// Does `line` match Checkpoint 7d's AC-405 count pattern?
///
/// The block's line, verbatim from §7d.17:
///
/// ```sh
/// grep -rnE 'gate::answer\(|use [^;]*gate::(\{[^}]*\banswer\b|answer\b)' crates/rk-cli/src --include=*.rs
/// ```
///
/// Two branches, and **an import counts as a call site** — that is the whole point of the second
/// one. `rk-cli` has no `regex` dev-dependency, so the alternation is spelled out here rather than
/// approximated, and [`ac405_gate_answer_has_exactly_one_call_site`] controls **each branch
/// separately** against the committed control file. A matcher whose import branch silently never
/// fired would count one hit on a file carrying an import plus a bare call — the exact two-site
/// shape the criterion refuses.
fn is_call_site(line: &str) -> bool {
    // Branch one: a qualified call. `gate::answer\(` — a substring, so `workflow::gate::answer(`
    // matches too.
    if line.contains("gate::answer(") {
        return true;
    }
    // Branch two: `use [^;]*gate::(\{[^}]*\banswer\b|answer\b)`. `[^;]*` cannot cross a `;`, so the
    // search region is the text from the first `use ` up to the first `;` after it.
    let Some(u) = line.find("use ") else {
        return false;
    };
    let after_use = &line[u + "use ".len()..];
    let segment = after_use.split(';').next().unwrap_or(after_use);

    let mut from = 0usize;
    while let Some(g) = segment[from..].find("gate::") {
        let tail = &segment[from + g + "gate::".len()..];
        if let Some(braced) = tail.strip_prefix('{') {
            // `\{[^}]*\banswer\b` — a brace group naming `answer` among its items.
            let inner = braced.split('}').next().unwrap_or(braced);
            if contains_word(inner, "answer") {
                return true;
            }
        } else if starts_with_word(tail, "answer") {
            // `answer\b` — the leading boundary is the `::` we just consumed.
            return true;
        }
        from += g + "gate::".len();
    }
    false
}

/// `\w` in the grep sense: the character class a word boundary is defined against.
const fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Does `hay` begin with `needle` followed by a word boundary?
fn starts_with_word(hay: &str, needle: &str) -> bool {
    hay.strip_prefix(needle)
        .is_some_and(|rest| rest.as_bytes().first().is_none_or(|b| !is_word_byte(*b)))
}

/// Does `needle` occur in `hay` with a word boundary on **both** sides?
///
/// `answers` and `gate_answer` must not count; `answer` and `answer,` must.
fn contains_word(hay: &str, needle: &str) -> bool {
    let bytes = hay.as_bytes();
    let mut from = 0usize;
    while let Some(at) = hay[from..].find(needle) {
        let start = from + at;
        let end = start + needle.len();
        let left_ok = start == 0 || !is_word_byte(bytes[start - 1]);
        let right_ok = end == bytes.len() || !is_word_byte(bytes[end]);
        if left_ok && right_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

/// **AC-405 as amended (D-13) — the gate-answer wrapper has EXACTLY ONE non-test call site, and it
/// is `rk-cli`'s `answer` verb.**
///
/// > AC-405: *"the gate-answer store method has exactly one call site, in `rk-cli`'s `answer`
/// > command."* Amended 2026-09-18: *"the answer path is `rk_kernel::workflow::gate::answer` behind
/// > AC-262's single caller-facing wrapper … so the call-site lint counts non-test callers of that
/// > wrapper and requires exactly one, in `rk-cli`'s `answer` verb (the kernel's own tests and the
/// > FSM fold are exempt)."*
///
/// # The instrument, and which half of it is load-bearing
///
/// The scan is the whole test — there is no runtime observable for "exactly one call site". What
/// makes it evidence rather than decoration is the **self-control**: the committed
/// `lint-controls/ac405_gate_answer_control.rs` is never compiled and never linked, and it carries
/// the token on non-comment lines **twice, once per branch of the alternation**. Its own header
/// says why: *"the control carries a call AND a use line"*. So this test asserts each branch finds
/// its line **before** it asserts anything about `src/`, and a scanner that visits nothing — or
/// whose import branch is broken — is red here rather than silently green over there.
///
/// # 🛑 AN IMPORT COUNTS AS A SITE, AND THAT FIXES `answer.rs`'s IMPORT LINE
///
/// `use rk_kernel::workflow::gate::answer;` **plus** one bare `answer(…)` call is **two hits** and
/// fails this test. The house form is therefore one *module* import — `use
/// rk_kernel::workflow::gate;`, which matches neither branch because the text is `gate;` and not
/// `gate::` — followed by exactly one `gate::answer(` under `crates/rk-cli/src/verbs/answer.rs`.
///
/// The scan root is `crates/rk-cli/src`, so the definition in `rk-kernel/src/workflow/gate.rs`, the
/// kernel's own tests and every `tests/` tree are out of scope by construction rather than by an
/// exemption list this test would have to keep true.
///
/// # Red
///
/// Zero or two non-test call sites, or the one site not being under `verbs/answer.rs`. At the tree
/// this test was authored against the count is **zero** — `verbs/answer.rs` does not exist — and
/// that absence is this test's first red.
#[test]
fn ac405_gate_answer_has_exactly_one_call_site() {
    // (1) THE CONTROL, BOTH BRANCHES, BEFORE ANY CLAIM ABOUT `src/`.
    let control = lint_controls_dir().join("ac405_gate_answer_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));
    assert_no_block_comment(&control_text, "the AC-405 control");

    let control_lines = code_lines(&control_text);
    let call_branch = control_lines
        .iter()
        .any(|(_, l)| l.contains("gate::answer("));
    assert!(
        call_branch,
        "the control {} carries no NON-COMMENT qualified call — the count's first branch is \
         unexercised and an absence over `src/` would prove nothing",
        control.display()
    );
    let import_branch = control_lines
        .iter()
        .any(|(_, l)| !l.contains("gate::answer(") && is_call_site(l));
    assert!(
        import_branch,
        "the control {} carries no NON-COMMENT line that matches ONLY through the import \
         alternation. That branch is what makes `use …gate::answer;` count as a site; unexercised, \
         a file importing the symbol and calling it bare would read as ONE hit instead of two. \
         Control code lines:\n{}",
        control.display(),
        control_lines
            .iter()
            .map(|(n, l)| format!("  {n}: {}", l.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // (2) THE COUNT, over crates/rk-cli/src only.
    let mut hits: Vec<(PathBuf, usize, String)> = Vec::new();
    for file in rs_files(&src_dir()) {
        let text = std::fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        for (line_no, line) in code_lines(&text) {
            if is_call_site(&line) {
                hits.push((file.clone(), line_no, line.trim().to_owned()));
            }
        }
    }
    let rendered = hits
        .iter()
        .map(|(f, n, l)| format!("  {}:{n}: {l}", f.display()))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        hits.len(),
        1,
        "AC-405 requires EXACTLY ONE non-test call site of the gate-answer wrapper under \
         crates/rk-cli/src; found {}. Remember that an IMPORT counts as a site: the house form is \
         `use rk_kernel::workflow::gate;` plus one `gate::answer(` call, never `use \
         …gate::answer;` plus a bare call. Sites:\n{rendered}",
        hits.len()
    );

    // (3) AND IT IS THE `answer` VERB'S. AC-405 names the command, not merely the count.
    let expected = src_dir().join("verbs").join("answer.rs");
    assert_eq!(
        hits[0].0, expected,
        "the one gate-answer call site is not `rk-cli`'s `answer` verb. AC-405 names the command: \
         *\"exactly one call site, in `rk-cli`'s `answer` command\"*. Found at:\n{rendered}"
    );
}

// ---------------------------------------------------------------- spawning the verb under test

/// Spawn `redkiln answer <item> <option> --explain-sufficed .. --toml-opened ..`.
///
/// `RK_FIXTURE_MODE` is set even though `answer` dispatches nothing: leaving it unset would make an
/// UNEXPECTED dispatch park for 600 s (the fixture child's default is `hold`) instead of failing
/// fast. `Command::env`, never `set_var`, which is banned workspace-wide including this tree.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS — golden constraint 4's 'a stub never replaced' is \
              caught only by driving the real binary (s7d §7d.13's temp-repository rule); \
              run::spawn_run_in_mode's form, and its reason"
)]
pub(crate) fn spawn_answer(
    repo: &std::path::Path,
    item: &str,
    option: &str,
    flags: &[&str],
) -> std::process::Output {
    let markers = repo.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(repo).arg("answer").arg(item).arg(option);
    for flag in flags {
        cmd.arg(flag);
    }
    cmd.env("RK_FIXTURE_MODE", "exit:0")
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    crate::support::output_bounded(&mut cmd)
}

/// Is this row a gate a human could answer -- i.e. the RUNTIME record, not `select()`'s
/// announcement?
///
/// A gate writes TWO `gate_opened` rows and that is correct: `select()` announces the gate it
/// carries (no token), and `Runtime::step` writes `(kind, token, tree_hash, deadline)`. Only the
/// second names something a token can be presented against, so every read of "the gate" in this
/// module goes through here. Asserting over the unfiltered kind is red on a correct binary.
pub(crate) fn is_answerable_gate(e: &rk_types::event::Event) -> bool {
    e.kind == rk_types::event_kind::EventKind::GateOpened
        && e.payload.get("token").is_some_and(|t| !t.is_null())
}

/// The repository's real store, opened as `answer`'s outer path opens it.
fn open_real_store(
    resolved: &rk_cli::verbs::run::Resolved,
) -> std::sync::Arc<std::sync::Mutex<dyn rk_ports::event_store::EventStore>> {
    let opened = rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree)
        .expect("the store the run wrote opens");
    std::sync::Arc::new(std::sync::Mutex::new(opened))
}

/// The two flags AC-435 makes required, in their passing form.
pub(crate) const BOTH_FLAGS: [&str; 4] = ["--explain-sufficed", "yes", "--toml-opened", "no"];

/// **AC-248 — an `answer` with no live token is a NAMED OUTCOME, not a crash and not a default.**
///
/// Register AC-248's THEN, as §7d.6 reproduces it: an answer presented when the item has no live
/// gate token is refused with a named outcome, and **no answer row is written**.
///
/// # 🛑 THE BASELINE IS `+1`, NOT ZERO — AND WRITING "ZERO ROWS" HERE IS RED ON A CORRECT BINARY
///
/// §7d.6: *"the invocation's one `reconcile_started` row (AC-172; RT-46) is the baseline every
/// refusal below is counted against: `ac248_…`, `ac410_…` and `ac435_…` assert the store's row
/// count equals **exactly one more** than before the invocation, that the added row is
/// `reconcile_started`, and that no `gate_*` / `artifact.*` row was appended; only `ac263_…` …
/// assert a literally unchanged count"*. An implementer who reads "no answer row is written" as
/// "the store is untouched" writes a test that fails against a correct binary.
///
/// # The two arms, and why the second is not `ac257_…`
///
/// **No record at all**: `bug-1` carries `has_repro = true`, so the pack's one `require` row
/// DECIDES and no gate is ever opened. **Expired**: the 1-second `gate_deadline_secs` variant, a
/// test-side wait, and then an answer — whose expiry pass resolves the deadline *at that read*
/// (AC-257's *"observed and resolved at that read"*). `ac257_…` (cycle 8) asserts the same
/// resolution happens in a **fresh process**; this one asserts that the answer refuses rather than
/// applying to a gate whose deadline merely passed — §7d.6: *"without it `gate::answer` would
/// `Applied` a gate whose deadline merely passed, because it refuses `Expired` only on a
/// `gate_expired` row already in the journal"*.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side wait past a 1 s deadline, not a decision (AC-001) — s7d §7d.15's form. \
              The PRODUCT never sleeps; the expiry is observed at a read."
)]
fn ac248_answer_without_live_token_exits_named_outcome() {
    // ---- ARM 1: an item that never opened a gate. --------------------------------------------
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();

    let _ran = crate::run::spawn_run(root, "bug-1", None);

    // THE PREMISE IS "NO GATE", NOT "THE RUN SUCCEEDED".
    //
    // `bug-1` carries `has_repro = true`, so the pack's one `require` row DECIDES and no gate is
    // ever opened -- which is the state AC-248 is about. Whether the run then went on to complete
    // is a different fact and not this test's: under `exit:0` the fixture child exits before
    // emitting a frame, the dispatch does not commit, and the run exits 3. Asserting `Some(0)`
    // here would make this test depend on a scripted transcript it does not need, and its red
    // would be about the transcript rather than about the answer.
    //
    // What IS asserted is the precondition the refusal must be about, plus enough store content
    // for the +1 below to be a count rather than a measurement of emptiness.
    let before = crate::support::store_rows(root);
    assert!(
        !before.is_empty(),
        "PREMISE: the run wrote nothing, so the +1 assertion below would measure an empty store"
    );
    // A GATE WRITES TWO `gate_opened` ROWS AND ONLY ONE OF THEM IS A GATE.
    //
    // `select()` announces every gate the selection carries -- for `bug-1` that is an
    // `unsatisfiable` announcement with NO token -- and `Runtime::step` writes the runtime record
    // carrying `(kind, token, tree_hash, deadline)`. Anything reading "the gate" filters on the row
    // that carries a TOKEN; a premise that counted `gate_opened` rows would read this repository as
    // gated when nothing answerable was ever opened.
    assert!(
        !before.iter().any(is_answerable_gate),
        "PREMISE: `bug-1` opened an ANSWERABLE gate, so the refusal below would not be about an \
         absent one. Token-bearing gate rows: {:?}",
        before
            .iter()
            .filter(|e| is_answerable_gate(e))
            .map(|e| e.payload.clone())
            .collect::<Vec<_>>()
    );

    let out = spawn_answer(root, "bug-1", "approve", &BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an answer with no live token is AC-248's named outcome, exit 3. stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("no open gate on bug-1: no live token to answer"),
        "the refusal does not carry AC-248's named outcome. Exit 3 alone is what the unbuilt stub \
         also produced, so the WORDING is what makes this the absent behaviour. stderr:\n{stderr}"
    );

    let after = crate::support::store_rows(root);
    assert_eq!(
        after.len(),
        before.len() + 1,
        "the refusal must leave EXACTLY the invocation's one `reconcile_started` row (AC-172): \
         {} rows before, {} after. Added: {:?}",
        before.len(),
        after.len(),
        after[before.len()..]
            .iter()
            .map(|e| e.kind)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        after[before.len()].kind,
        rk_types::event_kind::EventKind::ReconcileStarted,
        "the one added row is not `reconcile_started`"
    );

    // ---- ARM 2: a gate whose deadline has passed. ---------------------------------------------
    let expiring = crate::support::pack_min_repo(crate::support::PackVariant::GateOneSecond);
    let eroot = expiring.path();
    let opened = crate::run::spawn_run(eroot, "gate-1", None);
    assert_eq!(
        opened.status.code(),
        Some(0),
        "PREMISE: the gate must OPEN before it can expire. stderr:\n{}",
        String::from_utf8_lossy(&opened.stderr)
    );
    assert!(
        crate::support::store_rows(eroot)
            .iter()
            .any(is_answerable_gate),
        "PREMISE: no ANSWERABLE gate was opened, so the expiry below would be vacuous"
    );

    // Past the 1 s deadline. Nothing is alive at the deadline — that is AC-257's whole point.
    std::thread::sleep(std::time::Duration::from_millis(1500));

    let before = crate::support::store_rows(eroot);
    let out = spawn_answer(eroot, "gate-1", "approve", &BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an answer to an EXPIRED gate is refused, not applied. stderr:\n{stderr}"
    );

    let after = crate::support::store_rows(eroot);
    let added: Vec<_> = after[before.len()..].iter().map(|e| e.kind).collect();
    assert!(
        added.contains(&rk_types::event_kind::EventKind::GateExpired),
        "the expiry was not OBSERVED AND RESOLVED AT THAT READ — no `gate_expired` row was \
         appended by the answer. Added rows: {added:?}"
    );
    // ZERO ANSWER ROWS. The `gate_expired` row IS written; what must not be is an answer.
    assert!(
        !added.contains(&rk_types::event_kind::EventKind::GateAnswered)
            && !added.contains(&rk_types::event_kind::EventKind::GateAnswerVoid)
            && !added.contains(&rk_types::event_kind::EventKind::GateAnswerSpent),
        "an expired gate was ANSWERED. §7d.6: *\"zero answer rows means zero `gate_answered` / \
         `gate_answer_*` rows\"* — the expiry policy's disposition is not an answer. Added: {added:?}"
    );
}

/// The run-owned worktree a `run` created for `item`, and the branch it sits on.
///
/// *(dated 2026-09-27, `s8d-ii` build, P6; ADR-0180 decision 1, proposed.)* Through rk-git's one
/// derivation: `.redkiln/worktrees/<item>` under the main worktree.
fn owned_worktree(root: &std::path::Path, item: &str) -> std::path::PathBuf {
    crate::run_tree(root, item).path
}

/// Open the abstention gate on `gate-1` and return the fixture repository.
///
/// `PackVariant::Base` is `ac403_...`'s fixture and §7d.6 reuses it for every gate case in this
/// module: `gate-item.toml` omits the `has_repro` facet the pack's one `require` row reads, so
/// `select()` abstains and exactly one answerable gate opens.
fn repo_with_open_gate() -> tempfile::TempDir {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let out = crate::run::spawn_run(repo.path(), "gate-1", None);
    assert_eq!(
        out.status.code(),
        Some(0),
        "PREMISE: opening the gate must exit 0 (AC-403). stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        crate::support::store_rows(repo.path())
            .iter()
            .filter(|e| is_answerable_gate(e))
            .count(),
        1,
        "PREMISE: exactly one ANSWERABLE gate must be open before an answer is presented"
    );
    repo
}

/// **AC-407 / ADR-0123 — a spent token is refused, and the refusal writes ONE row.**
///
/// Register AC-407: the second answer against the same token is refused. §7d.6 fixes what that
/// costs: *"its spent token IS presented, `gate::answer` returns `RefusedSpent`, one
/// `gate_answer_spent` row"* -- so this is NOT a CLI-side short circuit. The CLI presents the spent
/// token to the kernel and the kernel decides; a verb that noticed the gate was answered and
/// refused on its own would write no row and would be red here.
///
/// # 🛑 WHAT SEPARATES THIS FROM `ac248_...`
///
/// §7d.6 states the rule once *"because `ac248_...` and `ac407_...` read the same store state and
/// demand different effects"*: a no-record item and a spent-token item BOTH exit 3, but one writes
/// nothing and the other writes exactly one `gate_answer_spent`. The row counts are the whole
/// discrimination -- the exit code alone cannot tell the two apart.
///
/// # The git-side absences, and what they are evidence of
///
/// `rev-list --count` on the run-owned branch is unchanged **across the second answer**: a refusal
/// makes no `Gate:` trailer commit. And the checkout the binary was spawned in never moves --
/// AC-518, H-5: every commit an answer makes lands in the run-owned worktree.
#[test]
fn ac407_second_token_refused_one_gate_answered_event() {
    let repo = repo_with_open_gate();
    let root = repo.path();
    let owned = owned_worktree(root, "gate-1");

    let checkout_head_before = crate::support::head_message(root);
    let checkout_commits_before = crate::support::commit_count(root);
    // AC-518 is "UNCHANGED across", not "empty": the fixture repository carries an untracked
    // `.redkiln/` from the moment it is built, so an emptiness assertion would be red on a correct
    // binary and would say nothing about what the answer did.
    let checkout_porcelain_before = crate::porcelain(root);

    // ---- the FIRST answer: applied.
    let first = spawn_answer(root, "gate-1", "approve", &BOTH_FLAGS);
    assert_eq!(
        first.status.code(),
        Some(0),
        "PREMISE: the first answer must APPLY, or the second is not a SPENT token. stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let owned_commits_after_first = crate::support::commit_count(&owned);

    // ---- the SECOND answer: refused.
    let before = crate::support::store_rows(root);
    let second = spawn_answer(root, "gate-1", "approve", &BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert_eq!(
        second.status.code(),
        Some(3),
        "the second answer against a spent token must be refused (AC-407). stderr:\n{stderr}"
    );

    // ---- THE ROW COUNTS. This is the discrimination.
    let after = crate::support::store_rows(root);
    let answered = after
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
        .count();
    let spent = after
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::GateAnswerSpent)
        .count();
    assert_eq!(
        answered,
        1,
        "exactly ONE `gate_answered` row may exist after two answers -- the second must not be \
         recorded as an answer. Rows: {:?}",
        after.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert_eq!(
        spent,
        1,
        "the refused second answer must write exactly ONE `gate_answer_spent` row. A CLI that \
         refused on its own, without presenting the spent token to the kernel, writes NONE -- and \
         that is the implementation §7d.6 rules out. Rows: {:?}",
        after.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    let added: Vec<_> = after[before.len()..].iter().map(|e| e.kind).collect();
    assert_eq!(
        added,
        vec![
            rk_types::event_kind::EventKind::ReconcileStarted,
            rk_types::event_kind::EventKind::GateAnswerSpent,
        ],
        "the refused invocation must add exactly its `reconcile_started` row (AC-172) and the \
         kernel's one `gate_answer_spent`, in that order. Added: {added:?}"
    );

    // ---- THE GIT-SIDE ABSENCES.
    assert_eq!(
        crate::support::commit_count(&owned),
        owned_commits_after_first,
        "the REFUSED answer made a commit on the run-owned branch. A refusal writes no `Gate:` \
         trailer."
    );
    assert_eq!(
        crate::support::commit_count(root),
        checkout_commits_before,
        "an answer committed into the checkout the binary was spawned in (AC-518, H-5)"
    );
    assert_eq!(
        crate::support::head_message(root),
        checkout_head_before,
        "the checkout's HEAD moved across the answers (AC-518)"
    );
    assert_eq!(
        crate::porcelain(root),
        checkout_porcelain_before,
        "an answer changed the working state of the checkout it was spawned in (AC-518)"
    );
}

/// **AC-406 — an answer bound to a tree that has since MOVED is VOID, and the gate stays open.**
///
/// ADR-0017's binding: the answer is bound to the tree hash the human saw. If the run-owned tree
/// has moved since the gate opened, the answer no longer describes the thing that was reviewed --
/// so it is recorded as VOID and the gate remains open for a fresh answer. §7d.6 fixes the exit:
/// the exit table's `0` row names *"a **`Void`** answer (recorded as void, the gate stays open --
/// AC-406; H-1)"*. A void answer is an OUTCOME, not a failure.
///
/// # 🛑 THE COMMIT GOES IN THE RUN-OWNED WORKTREE, WHICH IS WHAT MAKES THE TREE MOVE
///
/// H-5: the tree `answer` hashes is the run-owned worktree the store records, never `cwd`. So the
/// move this test stages must happen THERE. A commit in the checkout the binary was spawned in
/// would not move the hash `answer` takes, and the test would be green against a binary that
/// hashed the wrong tree -- which is the defect AC-518 and H-5 exist to prevent.
///
/// # What discriminates
///
/// Three things, and the first is the one that says VOID rather than merely "not applied":
///
/// 1. a `gate_answer_void` row whose `recorded` and `observed` tree hashes DIFFER. Equal hashes
///    would mean the gate was voided against the tree it was bound to, which is not a void at all.
/// 2. the gate is STILL OPEN in the fold, and no `gate_answered` row stands alone as an applied
///    answer -- the kernel writes `gate_answered` AND `gate_answer_void` together for a void.
/// 3. **no new `Gate:` trailer commit**: `rev-list --count` on the run-owned branch is unchanged
///    ACROSS THE ANSWER (the test's own staging commit is counted before the answer, not after).
#[test]
fn ac406_answer_after_commit_is_void_gate_stays_open() {
    let repo = repo_with_open_gate();
    let root = repo.path();
    let owned = owned_worktree(root, "gate-1");

    // ---- MOVE THE RUN-OWNED TREE. This is the whole premise.
    std::fs::write(
        owned.join("moved.txt"),
        b"the tree the human saw has changed\n",
    )
    .expect("the run-owned worktree is writable");
    crate::git(&owned, &["add", "moved.txt"]);
    crate::git(
        &owned,
        &[
            "commit",
            "-m",
            "move the run-owned tree after the gate opened",
        ],
    );

    let owned_commits_before = crate::support::commit_count(&owned);
    let checkout_head_before = crate::support::head_message(root);
    let checkout_porcelain_before = crate::porcelain(root);
    let before = crate::support::store_rows(root);

    let out = spawn_answer(root, "gate-1", "approve", &BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "a VOID answer is exit 0 -- it was recorded, as void (H-1). stderr:\n{stderr}"
    );

    // ---- (1) THE VOID ROW, AND ITS TWO HASHES MUST DIFFER.
    let after = crate::support::store_rows(root);
    let void = after
        .iter()
        .find(|e| e.kind == rk_types::event_kind::EventKind::GateAnswerVoid)
        .unwrap_or_else(|| {
            panic!(
                "no `gate_answer_void` row: the answer was not voided against the moved tree. \
                 Rows: {:?}",
                after.iter().map(|e| e.kind).collect::<Vec<_>>()
            )
        });
    let recorded = void
        .payload
        .get("bound_tree_hash")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let observed = void
        .payload
        .get("observed_tree_hash")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    assert!(
        !recorded.is_empty() && !observed.is_empty(),
        "the void row carries no tree hashes: {:?}",
        void.payload
    );
    assert_ne!(
        recorded, observed,
        "the void row's recorded and observed tree hashes are EQUAL, so nothing was voided -- a \
         void is precisely the case where the tree the answer was bound to is not the tree \
         observed (ADR-0017). Payload: {:?}",
        void.payload
    );

    // ---- (2) THE GATE IS STILL OPEN, AND NO STAGE MOVED.
    assert!(
        after.iter().any(is_answerable_gate),
        "the answerable gate row is gone from the journal"
    );
    assert!(
        !after[before.len()..]
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::StageTransition),
        "a VOID answer advanced the stage. It records that the answer did not apply; it decides \
         nothing. Added: {:?}",
        after[before.len()..]
            .iter()
            .map(|e| e.kind)
            .collect::<Vec<_>>()
    );

    // ---- (3) NO TRAILER COMMIT. A void writes no `Gate:` trailer.
    assert_eq!(
        crate::support::commit_count(&owned),
        owned_commits_before,
        "a VOID answer made a commit on the run-owned branch. §7d.6: *\"`gate_answered` + \
         `gate_answer_void` appended, NO trailer commit (the gate stays open -- AC-406)\"*"
    );

    // ---- AC-518: the checkout the binary was spawned in is untouched.
    assert_eq!(
        crate::support::head_message(root),
        checkout_head_before,
        "the checkout's HEAD moved across a void answer (AC-518)"
    );
    assert_eq!(
        crate::porcelain(root),
        checkout_porcelain_before,
        "a void answer changed the working state of the checkout it was spawned in (AC-518)"
    );
}

/// The git author identity of `dir`'s `HEAD`, as `%an <%ae>`.
#[expect(
    clippy::disallowed_types,
    reason = "reading a fixture worktree's own HEAD author; the product's git goes through rk-git's \
              one GitExec wrapper (support::head_message's form, and its reason)"
)]
fn author_of(dir: &std::path::Path) -> String {
    let out = std::process::Command::new("git")
        .args(["log", "-1", "--format=%an <%ae>"])
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("git log runs");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// **AC-435 (machine half) — both flags are REQUIRED, and what they say is RECORDED.**
///
/// ADR-0129 decision 1 as accepted 2026-09-19: *"two required flags, one record field"*. The
/// register's THEN -- *"the answer (yes/no, plus whether a TOML was opened) is recorded as a
/// required field"* -- is **two data in one field**, and SM-126's kill (*"opened a TOML anyway"*)
/// is the cell *sufficed = yes AND opened = yes*, which a one-flag reading could not represent.
///
/// # 🛑 THE PAYLOAD IS AN OBJECT WITH TWO MEMBERS, VERBATIM
///
/// §7d.6's AC-435 bullet: the `gate_answered` payload carries one member `explain_sufficiency: {
/// sufficed: "yes", toml_opened: "yes" }` / `{ "no", "no" }` **verbatim**. §7d.14's red is *"by
/// `gate_answered` lacking `explain_sufficiency` **or either of its two members**"*. The values are
/// the STRINGS `"yes"` / `"no"`, not booleans.
///
/// This is the half that was red when the test was authored: `gate::AnswerMeta::explain_sufficiency`
/// is an `Option<String>` on the tree and serialises as a JSON string, so the row could carry only
/// ONE of the two data. The kernel type is corrected in this cycle.
///
/// # The answerer, and why it is read BEFORE the call
///
/// RT-42 as corrected: the identity is read through the wrapper off the run-owned branch's `HEAD`
/// **before** `gate::answer`, so the KERNEL can write it on the row it builds -- never the CLI
/// editing a returned `Event`. It is asserted equal to the answer commit's `%an <%ae>` after the
/// commit, which is what ties the recorded identity to the one git actually stamped.
#[test]
fn ac435_answer_refuses_without_explain_sufficiency_and_records_it() {
    // ---- ARM 1: neither flag may be defaulted. The parser refuses, and nothing is written. -----
    let repo = repo_with_open_gate();
    let root = repo.path();
    let before = crate::support::store_rows(root);

    for missing in [
        vec!["--toml-opened", "no"],
        vec!["--explain-sufficed", "yes"],
        vec![],
    ] {
        let out = spawn_answer(root, "gate-1", "approve", &missing);
        assert_eq!(
            out.status.code(),
            Some(2),
            "an answer missing an AC-435 flag must be a USAGE error (clap's 2), never a defaulted \
             datum. Flags given: {missing:?}. stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    assert_eq!(
        crate::support::store_rows(root).len(),
        before.len(),
        "a parser refusal wrote to the store. It happens before the verb is entered, so not even \
         the invocation's `reconcile_started` row is owed -- this is the +0 case, unlike AC-248's."
    );

    // ---- ARM 2: the FULL truth table, one gate each. -----------------------------------------
    //
    // 🛑 §7d.6 PRESCRIBES TWO CELLS AND THEY ARE BOTH ON THE DIAGONAL -- RAISED.
    //
    // The stage file's AC-435 bullet names `{ sufficed: "yes", toml_opened: "yes" }` and
    // `{ "no", "no" }`. Measured with the declared mutant (`"toml_opened": sufficiency.sufficed` --
    // the second member echoing the first, which IS the one-flag reading ADR-0129 decision 1 was
    // accepted to prevent): over those two cells the mutant is **GREEN**. On the diagonal the two
    // members are equal, so an implementation carrying one datum twice is indistinguishable from
    // one carrying two.
    //
    // The OFF-DIAGONAL cells are what discriminate, and SM-126 is why they matter: its kill is a
    // cell where the two data disagree about what the answerer did. So the table is walked whole.
    // §7d.6's two-cell prescription is reported as a stage-file defect (§7d.18), not edited.
    for (sufficed, toml_opened) in [("yes", "yes"), ("yes", "no"), ("no", "yes"), ("no", "no")] {
        let repo = repo_with_open_gate();
        let root = repo.path();
        let owned = owned_worktree(root, "gate-1");

        let out = spawn_answer(
            root,
            "gate-1",
            "approve",
            &["--explain-sufficed", sufficed, "--toml-opened", toml_opened],
        );
        assert_eq!(
            out.status.code(),
            Some(0),
            "the answer must apply so that a `gate_answered` row exists to read. stderr:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );

        let rows = crate::support::store_rows(root);
        let answered = rows
            .iter()
            .find(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
            .unwrap_or_else(|| {
                panic!(
                    "no `gate_answered` row: {:?}",
                    rows.iter().map(|e| e.kind).collect::<Vec<_>>()
                )
            });

        let sufficiency = answered
            .payload
            .get("explain_sufficiency")
            .unwrap_or_else(|| {
                panic!(
                    "the `gate_answered` payload carries no `explain_sufficiency` member. ADR-0129 \
                 decision 1 puts it there, KERNEL-SIDE. Payload: {:?}",
                    answered.payload
                )
            });
        assert_eq!(
            sufficiency.get("sufficed").and_then(|v| v.as_str()),
            Some(sufficed),
            "`explain_sufficiency.sufficed` is not the flag's value, verbatim and as a STRING. \
             §7d.14's red names `either of its two members`. Payload: {:?}",
            answered.payload
        );
        assert_eq!(
            sufficiency.get("toml_opened").and_then(|v| v.as_str()),
            Some(toml_opened),
            "`explain_sufficiency.toml_opened` is not the flag's value. This is the member a \
             one-flag reading of AC-435 would drop, and SM-126's kill is exactly the cell it \
             carries. Payload: {:?}",
            answered.payload
        );

        // ---- THE ANSWERER, tied to what git actually stamped on the answer commit.
        let stamped = author_of(&owned);
        assert!(
            !stamped.is_empty(),
            "PREMISE: the answer commit carries no author, so the comparison below is vacuous"
        );
        assert_eq!(
            answered.payload.get("answerer").and_then(|v| v.as_str()),
            Some(stamped.as_str()),
            "the `gate_answered` row does not name who answered, or names someone other than the \
             identity git stamped on the answer commit (RT-42 as corrected). Payload: {:?}",
            answered.payload
        );
    }
}

/// **AC-263 — a non-CLI answer channel is refused by a NAMED outcome, before anything is read.**
///
/// ADR-0129's *What this forbids*: *"any second answer channel in Phase 1 ... only a CLI `redkiln
/// answer` invocation reconciles"*. §7d.1's routing row fixes the seam: a test-only stub drives
/// `verbs::answer::submit(Channel::TrackerComment, item, option, attachments)`, *"the one non-`Cli`
/// arm returns `Err(ChannelNotEnabled { phase: 1 })` **before any store read**, and
/// `submit(Channel::Cli, …)` is the only path that reaches `gate::answer`"*.
///
/// # 🛑 THIS IS THE `+0` CASE, AND IT IS THE ONLY ONE BESIDES `ac486_…`
///
/// §7d.6's baseline rule: `ac248_…`, `ac410_…` and `ac435_…` assert the store's row count is
/// **exactly one more** (the invocation's `reconcile_started`); *"only `ac263_…` (which refuses
/// before any store read) and `ac486_…` (step (4)) assert a literally unchanged count"*. The
/// refusal happens before step (6), so not even that row is owed.
///
/// # 🛑 THE ONE-LIVE-VARIANT VACUITY, DISCLOSED
///
/// `Channel::Cli` is the only arm any production path constructs, so the refused variant is one no
/// shipped code reaches. What this pins is therefore not "the tracker channel is refused today" --
/// nothing offers it -- but that **adding** a channel does not silently acquire an answer path:
/// the refusal is structural, before the store, and `Cli` is the only arm that proceeds.
///
/// # Two observables, and why the second needs the verb rather than `submit`
///
/// `submit` holds no store, so a "+0" assertion against `submit` alone could not fail. The store
/// counts are therefore taken over `run_with`, which DOES hold one: a verb that admitted the
/// channel would write its `reconcile_started` row and be red here.
#[test]
fn ac263_stub_channel_refused_named_outcome() {
    // ---- (1) THE TYPED REFUSAL, from the seam §7d.1 names.
    for channel in [Channel::TrackerComment, Channel::WebWrite] {
        assert_eq!(
            rk_cli::verbs::answer::submit(channel, "gate-1", "approve", &[]),
            Err(rk_cli::verbs::answer::ChannelNotEnabled { phase: 1 }),
            "{channel:?} was admitted. ADR-0129 forbids any second answer channel in Phase 1."
        );
    }
    assert_eq!(
        rk_cli::verbs::answer::submit(Channel::Cli, "gate-1", "approve", &[]),
        Ok(()),
        "the CLI channel must be ADMITTED -- a refuse-all `submit` would satisfy the two \
         assertions above while making the verb unusable, which is the shape §7d.14 calls a \
         refuse-all validator"
    );

    // ---- (2) THE STORE IS UNTOUCHED, over the verb that holds one.
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let resolved = crate::support::resolved_over_pack_min(root, "gate-1");
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);

    let args = rk_cli::cli::AnswerArgs {
        item: "gate-1".to_owned(),
        option: "approve".to_owned(),
        attach: Vec::new(),
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    };
    let exit = rk_cli::verbs::answer::run_with(
        Channel::TrackerComment,
        &rk_kernel::workflow::question::QuestionRegistry::default(),
        &resolved,
        &args,
        &store,
        crate::support::FIXED_NOW,
    )
    .expect("a sound lock is not a halt");

    assert_eq!(
        exit,
        rk_cli::exit::Exit::Usage,
        "a refused channel is a USAGE outcome, not a configuration one"
    );
    let steps = trace.steps();
    assert!(
        steps.is_empty(),
        "the refusal touched the store. AC-263's refusal is BEFORE any store read, which is what \
         makes it the +0 case rather than AC-248's +1. Steps: {steps:?}"
    );
}

/// The gate kind `pack-min` opens: `GateReason::UnknownFacet` on the facet the one `require` row
/// reads. `fsm::next_action` spells it `format!("{}:{}", reason.as_str(), axis)` -- axis-bearing,
/// never a bare `unknown_facet`.
const GATE_KIND: &str = "unknown_facet:has_repro";

/// A registry holding the default question PLUS `decide`, with the fixture gate BOUND to it.
///
/// # 🛑 THE BINDING IS WHAT MAKES THE PAIR MEAN ANYTHING
///
/// §7d.6: the default binding stays `unknown_facet:*` -> the default question, *"so under the
/// default binding alone the fixture gate could never resolve to `decide`, `proceed` would be an
/// undeclared option, and the negative would exit `2` for the wrong reason -- `UnknownOption`, not
/// the attachment -- while the positive control could never pass"*. Both arms bind.
fn decide_registry() -> rk_kernel::workflow::question::QuestionRegistry {
    use rk_kernel::workflow::question::{Question, QuestionRegistry};
    QuestionRegistry::default()
        .with_question(
            "decide",
            Question::new(["proceed"]).attaching("proceed", ["Findings"]),
        )
        .bind(GATE_KIND, "decide")
}

/// An `AnswerArgs` over the fixture gate.
fn answer_args(option: &str, attach: Vec<rk_cli::cli::Attachment>) -> rk_cli::cli::AnswerArgs {
    rk_cli::cli::AnswerArgs {
        item: "gate-1".to_owned(),
        option: option.to_owned(),
        attach,
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    }
}

/// **AC-410's NEGATIVE — an attachment no declared option admits is refused, and NOTHING is
/// written.**
///
/// ADR-0129 decision 4: *"An attachment whose `<kind>` no declared option of the gate's question
/// admits is **refused at the CLI boundary and nothing is written**"*.
///
/// # 🛑 THE VARIANT IS ASSERTED BY NAME, AND `UnknownOption` IS RED
///
/// §7d.6: *"the refusal's variant is asserted: `OptionError::AttachmentNotAdmitted { kind }`, never
/// `UnknownOption` (an `UnknownOption` refusal is red)"*. An `UnknownOption` here would mean the
/// gate never resolved to `decide` -- the test would be exiting 2 for the wrong reason and the
/// positive control could not pass.
///
/// # 🛑 THIS TEST ALONE CANNOT DISCHARGE AC-410
///
/// Every assertion here is satisfied by a validator that refuses EVERYTHING. That is why
/// `ac410_admitted_attachment_stored_as_human_root` exists and why the two land as a pair: D-04's
/// evidentiary content is the PAIRING, not either half.
#[test]
fn ac410_unadmitted_attachment_refused_zero_events() {
    let registry = decide_registry();

    // ---- ARM 1: a kind no option admits -> AttachmentNotAdmitted, BY NAME.
    let spec = rk_cli::cli::Attachment {
        kind: "Spec".to_owned(),
        path: std::path::PathBuf::from("spec.md"),
    };
    match rk_cli::verbs::answer::admit(
        &registry,
        GATE_KIND,
        "proceed",
        &[rk_kernel::workflow::question::Attachment {
            kind: spec.kind.clone(),
            path: "spec.md".to_owned(),
        }],
    ) {
        Err(rk_kernel::workflow::question::OptionError::AttachmentNotAdmitted { kind }) => {
            assert_eq!(kind, "Spec", "the refusal names the wrong kind");
        }
        other => panic!(
            "an unadmitted attachment must be refused as `AttachmentNotAdmitted`, never as \
             `UnknownOption` (which would mean the gate never resolved to `decide`) and never \
             accepted. Got: {other:?}"
        ),
    }

    // ---- ARM 2: an option that DECLARES `attaches`, answered without it -> AttachmentMissing.
    match rk_cli::verbs::answer::admit(&registry, GATE_KIND, "proceed", &[]) {
        Err(rk_kernel::workflow::question::OptionError::AttachmentMissing { kind }) => {
            assert_eq!(kind, "Findings", "the refusal names the wrong kind");
        }
        other => panic!(
            "`proceed` declares `attaches = [\"Findings\"]`, so answering it with no attachment \
             must be refused (ADR-0063's *\"refuses to close the mark unless that attachment is \
             supplied\"*). Got: {other:?}"
        ),
    }

    // ---- ARM 3: through the VERB -- exit 2, and the store gains ONLY its `reconcile_started`.
    let repo = repo_with_open_gate();
    let root = repo.path();
    let resolved = crate::support::resolved_over_pack_min(root, "gate-1");
    let before = crate::support::store_rows(root);

    let store = open_real_store(&resolved);
    let exit = rk_cli::verbs::answer::run_with(
        Channel::Cli,
        &registry,
        &resolved,
        &answer_args("proceed", vec![spec]),
        &store,
        crate::support::FIXED_NOW,
    )
    .expect("a sound lock is not a halt");
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Usage,
        "an inadmissible attachment is a USAGE error at the CLI boundary (exit 2)"
    );

    let after = crate::support::store_rows(root);
    let added: Vec<_> = after[before.len()..].iter().map(|e| e.kind).collect();
    assert_eq!(
        added,
        vec![rk_types::event_kind::EventKind::ReconcileStarted],
        "the refusal must add the invocation's `reconcile_started` row and NOTHING else -- no \
         `gate_*`, no `artifact.*` (§7d.6's baseline rule; AC-172). Added: {added:?}"
    );
}

/// **AC-410's POSITIVE CONTROL — an admitted attachment is ACCEPTED and stored as a human root.**
///
/// ADR-0129 decision 4: *"An attachment an option admits is **accepted and stored as a
/// human-supplied root** ... and this is AC-410's **positive control**"*. ADR-0129 also forbids the
/// shape this control exists to rule out: *"an attachment path that is refuse-only: the registry
/// must be able to admit at least the default question's declared kinds, and the positive control
/// must exist as a test"*.
///
/// # 🛑 THE PAIRING IS THE EVIDENCE
///
/// A refuse-all validator makes `ac410_unadmitted_attachment_refused_zero_events` GREEN and this
/// test RED. That pairing -- not either half -- is what says the three negatives are discharged by
/// a validator that actually admits something.
///
/// # The row's shape is ADR-0116 decision 1's, and drift is asserted ABSENT
///
/// `{ kind, item, schema_version, digest }` -- four members. §7d.14's red: *"a row whose payload is
/// outside ADR-0116 decision 1's shape ... an `instance` or `derived_from` member is schema drift,
/// decision 5"*. The binding path `<gate step>.attachments.<Kind>` is the artifact's instance name
/// inside the kind's registered slot, NOT a fifth payload member.
///
/// 🛑 **`schema_version` carries the registry's `schema` id.** `ArtifactKind` has `schema:
/// Option<String>` and no version field [measured]. The stage file says *"the kind's registered
/// schema version"*; the tree registers an id. Reported as drift (§7d.18), not invented.
///
/// 🛑 **The producer routing is RT-59 and is OWED a record.** That `answer` writes an
/// `artifact.write` row at all is routed, not decided: nothing in `rk-kernel/src` writes that kind
/// [measured], so this verb is its first producer.
#[test]
fn ac410_admitted_attachment_stored_as_human_root() {
    let registry = decide_registry();
    let repo = repo_with_open_gate();
    let root = repo.path();
    let resolved = crate::support::resolved_over_pack_min(root, "gate-1");

    // The attachment's bytes live outside the run-owned worktree: `answer` reads the file, it does
    // not adopt the path.
    let findings = root.join("findings.md");
    std::fs::write(&findings, b"the findings the human supplied\n").expect("the file is written");
    let expected_digest =
        rk_types::digest::sha256(&std::fs::read(&findings).expect("the attachment reads"));

    let before = crate::support::store_rows(root);
    let store = open_real_store(&resolved);
    let exit = rk_cli::verbs::answer::run_with(
        Channel::Cli,
        &registry,
        &resolved,
        &answer_args(
            "proceed",
            vec![rk_cli::cli::Attachment {
                kind: "Findings".to_owned(),
                path: findings.clone(),
            }],
        ),
        &store,
        crate::support::FIXED_NOW,
    )
    .expect("a sound lock is not a halt");

    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "an ADMITTED attachment must be accepted. A refuse-all validator satisfies AC-410's \
         negative and fails here -- that pairing is the criterion's evidentiary content."
    );

    let after = crate::support::store_rows(root);
    let added: Vec<_> = after[before.len()..].iter().map(|e| e.kind).collect();
    assert!(
        added.contains(&rk_types::event_kind::EventKind::GateAnswered),
        "the gate was not applied. Added: {added:?}"
    );

    let writes: Vec<_> = after
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ArtifactWrite)
        .collect();
    assert_eq!(
        writes.len(),
        1,
        "an admitted attachment is stored as a human-supplied root: exactly ONE `artifact.write` \
         row. Added: {added:?}"
    );
    let payload = &writes[0].payload;

    assert_eq!(
        payload.get("kind").and_then(|v| v.as_str()),
        Some("Findings"),
        "payload: {payload:?}"
    );
    assert_eq!(
        payload.get("item").and_then(|v| v.as_str()),
        Some("gate-1"),
        "payload: {payload:?}"
    );
    assert_eq!(
        payload.get("schema_version").and_then(|v| v.as_str()),
        Some("Findings"),
        "the row does not carry the kind's registered schema (the tree registers an id, not a \
         version -- RAISED). payload: {payload:?}"
    );
    assert_eq!(
        payload.get("digest").and_then(|v| v.as_str()),
        Some(expected_digest.as_str()),
        "the row's digest is not sha256 of the attached FILE. payload: {payload:?}"
    );

    // ADR-0116 decision 5: schema drift is asserted ABSENT, by name.
    for drift in ["instance", "derived_from"] {
        assert!(
            payload.get(drift).is_none(),
            "the payload carries `{drift}` -- ADR-0116 decision 5 calls a fifth member schema \
             drift. The binding path is the artifact's instance name inside the kind's registered \
             slot, not a payload member. payload: {payload:?}"
        );
    }
    assert_eq!(
        payload.as_object().map(serde_json::Map::len),
        Some(4),
        "ADR-0116 decision 1's shape is exactly four members. payload: {payload:?}"
    );
}
