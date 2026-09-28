//! **`answer` — the token is resolved by the CLI, never typed.**
//!
//! §7d.6 fixes the order: steps (1)–(6) as `run`'s with [`Entry::Answer`]; then the **expiry pass**
//! of `run`'s step (9); then the item's most recent gate record is read from the store, its token
//! presented to the kernel's gate-answer wrapper, and the outcome mapped to an exit.
//!
//! # 🛑 THIS FILE IS A SKELETON, AND THAT IS THE METHOD
//!
//! Cycle 7 lands this verb **one test at a time**. What is here now is the module's real
//! signatures and nothing else: [`Channel`], [`ChannelNotEnabled`], [`submit`], [`run`] and
//! [`run_with`] exist with the types every later test will call them through, so that each test's
//! first run is red for the **absent behaviour** rather than for a missing `mod`, a missing
//! function or a type that does not line up. The critic's sequencing says exactly this: *"land the
//! module skeleton with its real signature FIRST, then the test, so the red is behavioural and not
//! a missing `mod`."*
//!
//! [`run_with`] therefore refuses without doing anything, and [`submit`] admits every channel. Both
//! are wrong on purpose. They are corrected by the tests that name them, in the order §7d.6's
//! critic fixes: `ac248_…`, `ac407_…`, `ac406_…`, `ac435_…`, `ac263_…`, and the AC-410 pair last
//! and as a pair.
//!
//! # 🛑 THE ONE `gate::answer` CALL SITE, AND WHY THERE IS NONE YET
//!
//! AC-405 as amended (D-13) requires **exactly one** non-test call site of the gate-answer wrapper
//! under `crates/rk-cli/src`, and Checkpoint 7d counts it with an alternation in which **an import
//! counts as a site**:
//!
//! ```sh
//! grep -rnE 'gate::answer\(|use [^;]*gate::(\{[^}]*\banswer\b|answer\b)' crates/rk-cli/src --include=*.rs
//! ```
//!
//! `ac405_gate_answer_has_exactly_one_call_site` was RED against the skeleton, at zero sites, and
//! greened when the answer path landed with `ac407_…`. The import is a MODULE import — `use
//! rk_kernel::workflow::gate;`, which the pattern does not match because the text is `gate;` and
//! not `gate::` — plus exactly one `gate::answer(`, in [`answer_the_gate`]. A `use
//! …gate::answer;` plus a bare call would count **two** and fail the block.
//!
//! # 🛑 WHAT THIS VERB WILL NOT DO, EVEN WHEN BUILT
//!
//! §7d.6, quoted: *"it does not reconcile the answer into a verdict (the FSM's fold does, on the
//! next `run`); it does not take `--token`; it does not read stdin; it does not default either
//! AC-435 datum (both flags are required …); it never commits to the checkout it was spawned in
//! (H-5)."* It also builds no `gate_answered` row of its own: the meta's members are added
//! **kernel-side**, and no `payload.as_object_mut()` may ever appear in this file — *a row the CLI
//! could amend is a row the CLI could forge*.

use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_git::reconciled::Entry;
use rk_kernel::workflow::gate;
use rk_kernel::workflow::question::QuestionRegistry;
use rk_ports::event_store::EventStore;

use crate::Halt;
use crate::cli::{AnswerArgs, Cli, RunArgs, YesNo};
use crate::env::Env;
use crate::exit::Exit;
use crate::verbs::{Startup, run};

/// Where an answer arrived from (AC-263).
///
/// # 🛑 ONE LIVE VARIANT, AND THE VACUITY IS DISCLOSED RATHER THAN HIDDEN
///
/// ADR-0129 forbids *"any second answer channel in Phase 1 … only a CLI `redkiln answer`
/// invocation reconciles"*. So [`Channel::Cli`] is the only arm that will proceed and the other two
/// exist to be refused. A reader should know that the refusal is therefore exercised against a
/// variant no production path constructs: what it pins is that **adding** a channel does not
/// silently acquire an answer path, because every arm but `Cli` is refused before any store read.
///
/// §7d.15 does not admit an `rk-ports` edit, so this is an `rk-cli` enum and the register's
/// `rk-ports` tag stays RAISED (§7d.18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// A `redkiln answer` invocation. The only arm that reaches the kernel.
    Cli,
    /// A tracker comment. Phase 3.
    TrackerComment,
    /// A web write. Phase 3.
    WebWrite,
}

/// The refusal every non-[`Channel::Cli`] arm returns, before any store read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelNotEnabled {
    /// The phase in which the channel is refused — `1`.
    pub phase: u8,
}

impl std::fmt::Display for ChannelNotEnabled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "that answer channel is not enabled in phase {}: only a `redkiln answer` invocation \
             reconciles an answer (ADR-0129)",
            self.phase
        )
    }
}

impl std::error::Error for ChannelNotEnabled {}

/// **AC-263's seam.** Admit the channel, or refuse it before anything is read.
///
/// This holds no store and opens nothing, which is the property AC-263 asserts: the refusal cannot
/// have read anything because there is nothing here to read from. [`run_with`] calls it first, so
/// the same refusal governs the verb.
///
/// # Errors
/// [`ChannelNotEnabled`] for every arm but [`Channel::Cli`].
pub const fn submit(
    channel: Channel,
    _item: &str,
    _option: &str,
    _attachments: &[crate::cli::Attachment],
) -> Result<(), ChannelNotEnabled> {
    match channel {
        Channel::Cli => Ok(()),
        Channel::TrackerComment | Channel::WebWrite => Err(ChannelNotEnabled { phase: 1 }),
    }
}

/// `yes` / `no`, as the payload spells them — strings, never booleans (§7d.6's AC-435 bullet).
const fn yes_no(v: YesNo) -> &'static str {
    match v {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}

/// **The CLI boundary's validation** (ADR-0129 decision 4) -- the OPERATOR-facing half.
///
/// §7d.6: *"`<option>` and every `--attach <kind>=<path>` are validated against it **first at the
/// CLI boundary** ... the refusal is a usage error, exit `2`, zero answer rows ... **and again
/// inside `gate::answer`** ... so an `Applied` is **not** reachable by a caller that skips the CLI
/// check"*.
///
/// # 🛑 TWO CHECKS, TWO DIFFERENT FUNCTIONS, AND NEITHER REPLACES THE OTHER
///
/// This calls `QuestionRegistry::validate`, which is the ONLY one of the pair that can return
/// [`rk_kernel::workflow::question::OptionError::UnknownOption`]. `gate::answer` calls
/// `validate_attachments`, which never does. The kernel's is ADR-0063's invariant -- an accepted
/// Decision sentence a CLI check cannot discharge on behalf of a non-CLI caller; this one is the
/// operator's usage error, raised before the answer path is entered.
///
/// The question is resolved from the GATE RECORD's `kind`, never from a flag: §7d.6, *"the gate's
/// question is resolved from the gate record's `kind` (`unknown_facet:<axis>` -- no gate carries a
/// question id on the tree) -> the default question at 7d"*.
///
/// # Errors
/// The registry's refusal -- `UnknownOption`, `AttachmentNotAdmitted` or `AttachmentMissing`.
pub fn admit(
    registry: &QuestionRegistry,
    gate_kind: &str,
    option: &str,
    attachments: &[rk_kernel::workflow::question::Attachment],
) -> Result<(), rk_kernel::workflow::question::OptionError> {
    registry.validate(registry.question_for(gate_kind), option, attachments)
}

/// The outer verb: steps (1)–(6), the store, then [`run_with`].
///
/// This much is real from the start: every test below drives either the binary (which enters here)
/// or [`run_with`] directly, and a skeleton that could not resolve its inputs would make every red
/// a resolution failure instead of the behaviour under test.
///
/// # Errors
/// [`Halt::PoisonedLock`] from the store acquisition. Every other refusal is a value.
pub fn run(cli: &Cli, args: &AnswerArgs, env: &Env, startup: Startup<'_>) -> Result<Exit, Halt> {
    // Steps (1)-(6). `answer` takes no `--lane`: the lane is derived from the item record exactly
    // as `run` derives it, because the pack is what declares the gate's expiry action.
    let run_args = RunArgs {
        item: args.item.clone(),
        lane: None,
    };
    let resolved = match run::through_step_seven(cli, &run_args, env, startup, Entry::Answer) {
        Ok(resolved) => resolved,
        Err(exit) => return Ok(exit),
    };

    // ---- (5) the store. AC-413's identity check is inside `open`.
    let opened = match rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));

    // ---- THE PACK'S QUESTIONS (R5 = ADR-0177 decision 8, proposed; `s8d` §8d.6). *(2026-09-26,
    // `s8d` build, package P7.)* This passed the built-in default registry, so every gate was
    // answerable only as `approve` / `changes-requested`. The registry is now the one `run` loads,
    // from the same worktree the catalog is read from; a file that does not load refuses the answer
    // by name, before any row is written.
    let registry = match crate::processes::load(Path::new(resolved.worktree.as_str())) {
        Ok(registry) => registry,
        Err(e) => {
            eprintln!("redkiln: the question registry was refused: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    // THE ONE WALL-CLOCK READ of this invocation.
    let now = crate::clock::now().unix_seconds();
    run_with(Channel::Cli, &registry, &resolved, args, &store, now)
}

/// The inner verb — the in-process twin the AC-410 pair and AC-263 drive.
///
/// **The registry is a parameter.** §7d.6's `ac410_…` constructs *"a `QuestionRegistry` holding the
/// default question plus `decide`"* and binds the fixture gate's kind to it; that is the only way
/// a non-default question exists at 7d, because pack-declared questions are Step 7e's. A verb that
/// built its own registry could not be handed one.
///
/// *(dated 2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* ~~"pack-declared questions are
/// Step 7e's"~~: they are loaded now, by `crate::processes::load` from `.redkiln/processes/`, and [`run`]
/// hands this function THAT registry, never `QuestionRegistry::default()` (M16). The parameter stays, for
/// the in-process tests that construct their own.
///
/// 🛑 **SKELETON: this refuses without reading, writing or answering anything.** It is replaced in
/// slices, each by the test that names the slice.
///
/// # Errors
/// [`Halt::PoisonedLock`] at a store acquisition, once there is one.
pub fn run_with(
    channel: Channel,
    registry: &QuestionRegistry,
    resolved: &run::Resolved,
    args: &AnswerArgs,
    store: &Arc<Mutex<dyn EventStore>>,
    now: i64,
) -> Result<Exit, Halt> {
    // ---- AC-263. BEFORE any store read, which is what makes this the +0 case.
    if let Err(refused) = submit(channel, &args.item, &args.option, &args.attach) {
        eprintln!("redkiln: {refused}");
        return Ok(Exit::Usage);
    }

    // ---- (6)'s ROW. `answer` creates no worktree, so its own row carries RT-41's two members and
    // never RT-47's pair: the pair it needs is the one a previous `run` recorded, read back below.
    // This is the baseline every refusal in this verb is counted against (AC-172; RT-46).
    let event = match run::reconcile_started_event(&resolved.config, None) {
        Ok(event) => event,
        Err(exit) => return Ok(exit),
    };
    {
        let mut guard = crate::store::acquire(store, "the answer")?;
        let appended = guard.append(event);
        drop(guard);
        if let Err(e) = appended {
            eprintln!("redkiln: the invocation's first row could not be written: {e:?}");
            return Ok(Exit::ConfigError);
        }
    }

    // ---- The run-owned worktree the STORE records (ADR-0105 clause 8), read FIRST.
    //
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.4; D21-42 8d; the three-kinds
    // record, proposed.)* ~~`recorded_ownership` read the pair off the last `reconcile_started` row
    // (RT-47)~~ — ownership is the latest `worktree_assigned` row for THIS item, through the one
    // resolver. **No row, no answer: fail-closed**, by name, before the heal and before any read of
    // the gate, and nothing is resolved from the `.redkiln/worktrees/<item>` convention. The
    // deleted-database reading: a `run` re-enters the registered tree and writes the row again.
    let Some(ownership) = run::assigned_ownership(store, &args.item)? else {
        eprintln!(
            "redkiln: the store holds no `worktree_assigned` row for `{}`, so no run-owned \
             worktree is recorded and nothing is answered (fail-closed; ADR-0105 clause 8 \
             resolves a tier-1 write's target from the store, never from the working directory \
             or a path convention). A `redkiln run {}` re-enters its tree and records it.",
            args.item, args.item
        );
        return Ok(Exit::ConfigError);
    };

    // ---- THE HEAL (`s8e` §8e.3), after ownership: a trailer commit on `rk-run/<item>` whose sha
    // anchors no row gets its rows. Git -> store only; it writes nothing to git.
    if let Err(e) =
        rk_kernel::runtime::tier_one::heal(&resolved.exec, &resolved.worktree, store, &args.item)
    {
        eprintln!("redkiln: the tier-1 heal of `{}` failed: {e}", args.item);
        return Ok(Exit::ConfigError);
    }

    // ---- The recorded journal.
    let events = match read_journal(store, &args.item)? {
        Ok(events) => events,
        Err(exit) => return Ok(exit),
    };

    // §7d.6: *"a recorded worktree that no longer exists is exit 3 naming its path, a refusal of
    // the item's recorded state"*. `authorise_tier1_write` is that refusal AND the guard: its
    // private field is the mechanism -- *was this write checked?* is answered by the type -- so the
    // hash below and any later trailer commit are both taken at a tree proved run-owned.
    let owned = match rk_git::worktree::authorise_tier1_write(
        &resolved.exec,
        &ownership.worktree,
        &ownership,
    ) {
        Ok(owned) => owned,
        Err(e) => {
            eprintln!(
                "redkiln: the worktree recorded for `{}` ({}) is not usable: {e:?}",
                args.item,
                ownership.worktree.display()
            );
            return Ok(Exit::ConfigError);
        }
    };

    // ---- THE TREE HASH, over the RUN-OWNED worktree (H-5 -- never `cwd`; ADR-0105 clause 4).
    //
    // RT-43 says *"reconcile, then read the gate, then hash"*, and the reconcile-before-hash half
    // is what it justifies: *"a hash before the reconcile would be a hash of a tree the reconcile
    // may report drifted against"*. That half holds -- the reconcile happened at step (6). The hash
    // is taken BEFORE the gate read rather than after, because `gate::expire` takes a tree hash and
    // the expiry pass must run before the record is read, or an expired gate would be answered as
    // live. The gate record the ANSWER presents is read after the pass, which is the property the
    // ordering exists for. §7d.6's phrasing is RAISED, not edited.
    let tree = match rk_git::tree::tree_hash(
        &resolved.exec,
        owned.path(),
        &resolved.common_dir,
        &rk_git::tree::TreeBasis::WorkingState,
    ) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("redkiln: the tree hash of the run-owned worktree failed: {e:?}");
            return Ok(Exit::ConfigError);
        }
    };
    let tree_hash = tree.as_str().to_owned();

    // ---- THE EXPIRY PASS (step (9)'s, called from here -- §7d.6). AC-257.
    let state = match fold(&events, resolved, &args.item) {
        Ok(state) => state,
        Err(exit) => return Ok(exit),
    };
    let expiry_events = match run::expiry_pass(&resolved.extras, &state, now, &tree_hash) {
        Ok(events) => events,
        Err(exit) => return Ok(exit),
    };
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3, N6.)* ~~store-only~~: each
    // expiry row is a tier-1 act through the boundary, its `Gate: … expired` mark first.
    let mut boundary = boundary_of(resolved, &ownership, &args.item);
    if !expiry_events.is_empty()
        && let Err(exit) =
            run::gate_rows_through(store, Some(&mut boundary), &state, expiry_events, None)?
    {
        return Ok(exit);
    }

    // ---- THE GATE READ, over the journal AS THE EXPIRY PASS LEFT IT.
    let events = match read_journal(store, &args.item)? {
        Ok(events) => events,
        Err(exit) => return Ok(exit),
    };
    let state = match fold(&events, resolved, &args.item) {
        Ok(state) => state,
        Err(exit) => return Ok(exit),
    };

    // §7d.6's four store-state outcomes. AC-248's is the one that lands here: no record at all, and
    // a record the expiry pass has just resolved, are the SAME named outcome and the SAME exit --
    // what differs is only what the pass already wrote.
    // 🛑 AN EXPIRED GATE IS STILL IN `open_gates()`, AND THE KERNEL WILL APPLY IT.
    //
    // Measured on the tree. `fsm::apply`'s `GateExpired` arm KEEPS the record in `gates_open`
    // deliberately (its own note, 2026-09-17: *"the record is KEPT in `gates_open` -- an escalation
    // names it and `stage_action` carries it"*), so `RunState::gate_record` still returns it. And
    // `gate::answer` tests `gate_record(presented)` -- returning `Applied`/`Void` -- BEFORE it
    // tests `is_expired`. So for a gate that was opened and then expired, the kernel's `Expired`
    // branch is UNREACHABLE: it fires only for a token in `gates_expired` and NOT in `gates_open`,
    // which a real expiry never produces.
    //
    // §7d.6 reads the other way -- *"it refuses `Expired` only on a `gate_expired` row already in
    // the journal"* -- and that premise does not hold against this tree. The sentence is RAISED as
    // a stage-file defect (§7d.18), not edited, and the CLI does what §7d.6's own outcome table
    // requires of it: an expired gate is AC-248's named outcome, exit 3, ZERO answer rows. The
    // expiry pass above is what makes the state observable; this filter is what honours it.
    //
    // Without the filter `ac248_...`'s expired arm is GREEN for the wrong reason while the verb is
    // unbuilt (both branches exit 3) and RED the moment the answer path lands -- which is how it
    // was found.
    let live: Option<&rk_kernel::workflow::fsm::GateRecord> = state
        .open_gates()
        .filter(|record| !state.is_expired(record.token()))
        .last();

    // 🛑 AN ANSWERED GATE IS ALSO STILL IN `open_gates()`, AND THAT IS WHY IT IS STILL PRESENTED.
    //
    // The same 2026-09-17 fold decision that keeps an EXPIRED record in `gates_open` keeps an
    // ANSWERED one there too. So `live` above is "opened, and not expired" -- deliberately NOT
    // "unanswered". AC-407 requires the spent token to be PRESENTED: §7d.6, *"its spent token IS
    // presented, `gate::answer` returns `RefusedSpent`, one `gate_answer_spent` row"*. A CLI that
    // noticed the gate was answered and refused on its own would write no row at all, and that is
    // the implementation the row count in `ac407_...` rules out.
    //
    // (An earlier draft of this function carried an `answered_gates()` fallback beneath the `live`
    // branch. It was DEAD: the mutant that removed it was GREEN, because an answered record never
    // leaves `gates_open` and so is always taken by `live`. It is gone rather than left to read as
    // a path that runs.)
    if live.is_none() {
        eprintln!(
            "redkiln: no open gate on {}: no live token to answer",
            args.item
        );
        return Ok(Exit::ConfigError);
    }

    // A LIVE gate is answered; a SPENT one is presented anyway so the kernel writes its one
    // `gate_answer_spent` row (AC-407). The CLI does not decide which -- it resolves the token and
    // lets `gate::answer` classify, which is what keeps the spent refusal a RECORDED event rather
    // than a silent CLI short circuit.
    // ---- THE CLI BOUNDARY'S VALIDATION, FIRST (ADR-0129 decision 4). Exit 2, zero answer rows.
    if let Some(record) = live {
        let attachments: Vec<rk_kernel::workflow::question::Attachment> = args
            .attach
            .iter()
            .map(|a| rk_kernel::workflow::question::Attachment {
                kind: a.kind.clone(),
                path: a.path.display().to_string(),
            })
            .collect();
        if let Err(refused) = admit(registry, record.kind(), &args.option, &attachments) {
            eprintln!("redkiln: {refused}");
            return Ok(Exit::Usage);
        }
    }

    let Some(record) = live else {
        // Unreachable: the branch above returned for exactly this state.
        return Ok(Exit::ConfigError);
    };
    let token = record.token().clone();

    answer_the_gate(
        registry,
        resolved,
        args,
        store,
        &owned,
        &token,
        &tree_hash,
        &state,
        &mut boundary,
    )
}

/// The tier-1 boundary an answer commits through: the run-owned worktree the store records, as
/// both the target and the ownership (`answer` enters no tree of its own). *(dated 2026-09-28,
/// `s8e` build, package S8E-P5; `s8e` §8e.3.)*
fn boundary_of(
    resolved: &run::Resolved,
    ownership: &rk_git::worktree::RunOwnership,
    item: &str,
) -> rk_kernel::runtime::tier_one::Boundary {
    rk_kernel::runtime::tier_one::Boundary::live(
        resolved.exec.clone(),
        rk_kernel::runtime::tier_one::Target {
            worktree: ownership.worktree.clone(),
            ownership: ownership.clone(),
            project: resolved.project.clone(),
            item: item.to_owned(),
        },
    )
}

/// **The one `gate::answer` call site in `crates/rk-cli/src`** (AC-405 as amended, D-13).
///
/// Everything the kernel needs is resolved by the time this is called: the token the CLI found and
/// the tree hash it took over the run-owned worktree. The registry travels in because ADR-0063's
/// attachment invariant is the KERNEL's -- `gate::answer` calls `validate_attachments` before the
/// mark closes, and a CLI check cannot discharge an accepted Decision sentence on behalf of a
/// non-CLI caller.
#[expect(
    clippy::too_many_arguments,
    reason = "the arity is the record's: every value here is one §7d.6 names as resolved BEFORE the \
              call, and bundling them into a struct would hide which of them the kernel reads"
)]
fn answer_the_gate(
    registry: &QuestionRegistry,
    resolved: &run::Resolved,
    args: &AnswerArgs,
    store: &Arc<Mutex<dyn EventStore>>,
    owned: &rk_git::worktree::OwnedWorktree,
    token: &rk_types::digest::Digest,
    tree_hash: &str,
    state: &rk_kernel::workflow::fsm::RunState,
    boundary: &mut rk_kernel::runtime::tier_one::Boundary,
) -> Result<Exit, Halt> {
    // The KERNEL's attachment type, not the CLI's: `rk-kernel` opens no files, so it carries the
    // kind and the path as a string and any digest is computed on this side.
    let attachments: Vec<rk_kernel::workflow::question::Attachment> = args
        .attach
        .iter()
        .map(|a| rk_kernel::workflow::question::Attachment {
            kind: a.kind.clone(),
            path: a.path.display().to_string(),
        })
        .collect();

    // ---- THE ANSWERER, READ BEFORE THE CALL so the KERNEL can write it (RT-42 as corrected).
    //
    // §7d.6: the identity is *"the git author identity the wrapper stamped on the run-owned
    // branch's HEAD, read through the wrapper BEFORE the `gate::answer` call ... so the kernel can
    // write it on the `gate_answered` row it builds"*. Through the wrapper, never `Command::new`:
    // `GitOp::Log` is already on the closed enum, so no `rk-git` edit is owed.
    let answerer = resolved
        .exec
        .run(
            rk_git::git_exec::GitOp::Log,
            owned.path(),
            ["log", "-1", "--format=%an <%ae>"],
        )
        .map(|out| out.stdout_trimmed())
        .ok()
        .filter(|s| !s.is_empty());

    let meta = gate::AnswerMeta {
        explain_sufficiency: Some(gate::ExplainSufficiency {
            sufficed: yes_no(args.explain_sufficed).to_owned(),
            toml_opened: yes_no(args.toml_opened).to_owned(),
        }),
        answerer,
    };

    let outcome = match gate::answer(
        state,
        token,
        token,
        tree_hash,
        &args.option,
        registry,
        &attachments,
        &meta,
    ) {
        Ok(outcome) => outcome,
        Err(gate::GateError::OptionRefused(refused)) => {
            eprintln!("redkiln: {refused}");
            return Ok(Exit::Usage);
        }
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3, N6: "the answer (`answer.rs`,
    // today append-then-commit — **the order N6 fixes**)".)* ~~Every outcome row appended, then the
    // `Gate:` commit~~: an APPLIED answer's `gate_answered` row and a VOID answer's two rows are
    // tier-1 acts through the kernel's boundary — the `Gate: … answered <option>` mark (and, for a
    // void, the `Gate: … voided` mark after it) committed FIRST, each row anchored to its commit —
    // so a commit git refuses leaves no row. A SPENT token's `gate_answer_spent` row is tier 2 and
    // records a refusal, not a transition, so it is appended as before.
    let rows_written = match outcome.disposition {
        gate::Disposition::Applied | gate::Disposition::Void => {
            run::gate_rows_through(store, Some(boundary), state, outcome.events, None)?
        }
        gate::Disposition::RefusedSpent => {
            run::gate_rows_through(store, None, state, outcome.events, None)?
        }
    };
    if let Err(exit) = rows_written {
        return Ok(exit);
    }

    match outcome.disposition {
        gate::Disposition::Applied => {
            // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3.)* The `Gate: …
            // answered <option>` mark (ADR-0184's widened value, the gate record's own token, tree
            // hash, deadline, kind and reach, carrying the chosen option) was committed ABOVE,
            // through the kernel's tier-1 boundary and before its row — the direct
            // call that stood here, after the row, is deleted.
            // ---- AC-410's POSITIVE CONTROL: every admitted attachment is stored as a
            // human-supplied root, one `artifact.write` row each, in ADR-0116 decision 1's shape.
            //
            // RT-59: that `answer` is this row's producer is ROUTED, not decided -- nothing in
            // `rk-kernel/src` writes `artifact.write` [measured], so this verb is its first
            // producer. §7d.18 holds the record OWED.
            if let Err(exit) = store_attachments(resolved, args, store)? {
                return Ok(exit);
            }
            Ok(Exit::Success)
        }
        // AC-406: the gate STAYS OPEN. The answer was recorded, as void -- an outcome, not a
        // failure, so exit 0 (H-1). `ac406_...` is its test. *(dated 2026-09-28, `s8e` build,
        // package S8E-P5; `s8e` §8e.3, N6.)* ~~and NO trailer commit is made~~: every gate
        // transition is git-first, so the void's two rows each rode their own act above — the
        // `Gate: … answered <option>` mark, then the `Gate: … voided` mark.
        gate::Disposition::Void => Ok(Exit::Success),
        // AC-407 / ADR-0123. The kernel's one `gate_answer_spent` row is already appended above.
        gate::Disposition::RefusedSpent => {
            eprintln!(
                "redkiln: the gate on {} has already been answered; the second answer is refused",
                args.item
            );
            Ok(Exit::ConfigError)
        }
    }
}

/// The item's recorded journal.
///
/// The outer `Result` is the halt a poisoned lock raises; the inner one is a refusal value.
fn read_journal(
    store: &Arc<Mutex<dyn EventStore>>,
    item: &str,
) -> Result<Result<Vec<rk_types::event::Event>, Exit>, Halt> {
    let guard = crate::store::acquire(store, "the answer")?;
    let read = guard.query(rk_kernel::runtime::journal_query(item));
    drop(guard);
    Ok(read.map_err(|e| {
        eprintln!("redkiln: the recorded journal could not be read: {e:?}");
        Exit::ConfigError
    }))
}

/// Fold the item's journal at the stage its record names.
fn fold(
    events: &[rk_types::event::Event],
    resolved: &run::Resolved,
    item: &str,
) -> Result<rk_kernel::workflow::fsm::RunState, Exit> {
    rk_kernel::workflow::fsm::fold(item, &resolved.stage, events).map_err(|e| {
        eprintln!("redkiln: the recorded journal did not fold: {e}");
        Exit::ConfigError
    })
}

// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.4.)* `recorded_ownership` — *"The
// run-owned pair a previous `run` recorded -- RT-47, read back off the journal"* — is deleted: it
// filtered by kind alone, so with two items in one store it took the last pair of EITHER item
// (R-63). Ownership is `run::assigned_ownership`'s, over `worktree_assigned` through `ByItem`.

/// Store every attachment as a human-supplied root -- one `artifact.write` row each.
///
/// # 🛑 FOUR MEMBERS, AND A FIFTH IS SCHEMA DRIFT
///
/// ADR-0116 decision 1's shape is `{ kind, item, schema_version, digest }`. Decision 5 calls an
/// `instance` or `derived_from` member drift, and §7d.14 makes one a RED. The binding path
/// `<gate step>.attachments.<Kind>` is the artifact's instance name inside the kind's registered
/// slot -- not a payload member.
///
/// 🛑 **`schema_version` carries the registry's `schema` ID.** `ArtifactKind` is
/// `{ name, source, keyed_by, schema: Option<String> }` -- there is no version field on the tree
/// [measured]. §7d.7 says *"the kind's registered schema version"*; the tree registers an id, and
/// the id is what is written rather than a version invented here. RAISED (§7d.18).
///
/// # Errors
/// [`Halt::PoisonedLock`] from the acquisition; an [`Exit`] for an unreadable attachment or a kind
/// the registry does not register.
fn store_attachments(
    resolved: &run::Resolved,
    args: &AnswerArgs,
    store: &Arc<Mutex<dyn EventStore>>,
) -> Result<Result<(), Exit>, Halt> {
    if args.attach.is_empty() {
        return Ok(Ok(()));
    }

    let root = std::path::Path::new(resolved.worktree.as_str());
    let path = root.join(".redkiln").join("artifacts.toml");
    let src = match std::fs::read_to_string(&path) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("redkiln: {}: {e}", path.display());
            return Ok(Err(Exit::ConfigError));
        }
    };
    let registry = match rk_kernel::select::ArtifactRegistry::from_toml("artifacts.toml", &src) {
        Ok(registry) => registry,
        Err(e) => {
            eprintln!("redkiln: {}: {e:?}", path.display());
            return Ok(Err(Exit::ConfigError));
        }
    };

    let mut rows = Vec::new();
    for attachment in &args.attach {
        let Some(kind) = registry.kind(&attachment.kind) else {
            eprintln!(
                "redkiln: `{}` is not a registered artifact kind, so there is no slot to store it \
                 in.",
                attachment.kind
            );
            return Ok(Err(Exit::ConfigError));
        };
        let bytes = match std::fs::read(&attachment.path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("redkiln: {}: {e}", attachment.path.display());
                return Ok(Err(Exit::ConfigError));
            }
        };
        rows.push(rk_types::event::Event::new(
            rk_types::event_kind::EventKind::ArtifactWrite,
            serde_json::json!({
                "kind": attachment.kind,
                "item": args.item,
                "schema_version": kind.schema,
                "digest": rk_types::digest::sha256(&bytes).as_str(),
            }),
        ));
    }

    let mut guard = crate::store::acquire(store, "the answer")?;
    for row in rows {
        if let Err(e) = guard.append(row) {
            drop(guard);
            eprintln!("redkiln: an `artifact.write` row could not be written: {e:?}");
            return Ok(Err(Exit::ConfigError));
        }
    }
    drop(guard);
    Ok(Ok(()))
}
