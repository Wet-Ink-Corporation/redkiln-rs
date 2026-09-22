//! **The conformance gate — four assertions over the design's own closed sets.**
//!
//! `s7f` §7f.1. Five stages of checkpoints passed over a runtime that does not implement its own
//! spec, and none of them was weak — each tested what was built. What no gate did was compare the
//! **built surface** against the **design's closed sets**. This module is that comparison, authored
//! before the three stages that discharge it so they are measured by something written ahead of
//! them rather than by their own account of themselves.
//!
//! # The four, and what each pins
//!
//! | lint | closed set it reads | measured 2026-09-21 | state at `s7g`'s close |
//! | --- | --- | --- | --- |
//! | [`conformance_every_step_kind_has_an_executor`] | design §4.5's five step kinds | **0** `StepKind` references in `runtime/` | **DISCHARGED by `s7g`** — `runtime/step.rs` routes all five |
//! | [`conformance_every_port_has_a_call_site`] | ADR-0003's port table | harness 1, store 1, **toolchain 0, tracker 0, docs 0** | **DISCHARGED by `s7g`**, narrowed by **ADR-0134** — see below |
//! | [`conformance_every_trailer_key_has_a_producer`] | design §5.4's trailer grammar | `Gate` 2, **`Story` 0, `Slice-Verdict` 0** | still ARMED, `s7h` |
//! | [`shipped_pack_loads`] | the product's own `.redkiln/` | GREEN from `s7f` §7f.2 | GREEN, 102 pinned |
//!
//! 🛑 **The `#[ignore]` reason spelling is load-bearing.** Checkpoint 7f greps each armed lint's
//! reason for the literal `discharged by s7g` / `discharged by s7h`. An armed lint is coverage
//! theatre unless something counts it, so the checkpoint counts all three **by name** and asserts
//! each reason names the stage that discharges it. The ignore is removed by the landing that makes
//! the lint pass, **in the same commit**.
//!
//! 🛑 **AND THAT CONVENTION NECESSARILY INVALIDATES CHECKPOINT 7F, WHICH IS NOT A DEFECT OF `s7g`.**
//! Checkpoint 7f requires the literal `test result: ok. 0 passed; 0 failed; 3 ignored` and asserts
//! three ARMED lints. Removing **any** `#[ignore]` breaks it, so *any* discharging stage invalidates
//! it — the two-in-one-commit rule guarantees that. Checkpoint 7f is `s7g`'s **entry** gate only and
//! nothing downstream requires it green; `00-preamble.md`'s maintenance rule already provides for a
//! pass attesting a specific text at a specific time. Two of the three are discharged here, so that
//! block is now historical; the one that remains still carries `discharged by s7h` verbatim.
//!
//! # ADR-0134: the port lint asserts an EQUALITY, because the emptiness was unsatisfiable
//!
//! [`conformance_every_port_has_a_call_site`] asserted `uncalled.is_empty()` over all five ports.
//! `tracker` and `docs` have **no Phase-1 real adapter by design** — design:265 *"none in Phase 1
//! (GitHub is Phase 3)"*, design:266 *"none in Phase 1 (mdBook is Phase 3)"*, AC-499 instantiated
//! only for ports that have one, AC-507/AC-508/AC-510 Phase 3, and `s7a` §7a:198-199 authoring both
//! argument types as contentless newtypes so no Phase-3 schema is invented early. So `s7f`'s *"later
//! (tracker/docs)"* meant **Phase 3**, no stage in this runbook claims either port, and the
//! emptiness form could only ever stay `#[ignore]`d. **An assertion that cannot become true is an
//! assertion that never runs.** The equality form runs on every commit from `s7g` and fails in
//! **both** directions.
//!
//! 🛑 **The first three lints scan CODE LINES ONLY — comments stripped AND string literals
//! blanked.** A comment naming `StepKind`, a doc-comment naming a port, a grammar file naming a
//! trailer key, *or a string literal spelling any of them*, must not satisfy them — that is how a
//! guard gets satisfied by the prose beside it, which this programme has been bitten by repeatedly.
//! The kernel already carries such prose: `src/workflow/gate.rs` states *"Step 7b parsed and linted
//! that form … and built no executor"*, and `src/catalog.rs` carries *"`outputs` was parsed for
//! every `StepKind`"*. Both would satisfy a naive grep.
//!
//! **The first draft of this module was defeated three ways and each is now a committed case.** An
//! adversarial pass turned all three armed lints GREEN without adding one line of behaviour: a
//! **trailing** comment on a real code line (`const _X: () = (); // ports.toolchain`), which the
//! line-based `retry.rs` filter keeps whole; a dead `const` of **string literals** naming the five
//! step kinds; and two literals placed in `rk-cli`'s `doctor/rebuild.rs`, which is a trailer
//! *reader*. So [`code_lines`] reuses the tree's real stripper —
//! `rk_kernel::kernel_denylist::strip_comments`, block-aware and trailing-aware — and pairs it with
//! [`blank_string_literals`], the shape `module_boundary.rs` already uses for the same reason.
//! [`the_comment_stripper_ignores_prose`] is the **control**, and it carries all three regressions.
//!
//! # Why the source half walks rather than `include_str!`s
//!
//! The three named source-lint precedents — `retry.rs`'s `ac028_policy_match_has_no_wildcard`,
//! `recorded_log.rs`'s `the_replay_still_calls_the_divergence_detection` and `projection.rs`'s
//! `stage_action_still_filters_to_tier_one` — all use `include_str!`, and all three read **one
//! named file**. These lints read a **directory**, and `rk-ports`' own `source_parse.rs` refuses
//! `include_str!` by name for exactly this job: *"an `include_str!` list is itself an inventory a
//! fixture would have to edit, which is precisely what the clause forbids and what PIA-059
//! caught"*. So the walk is the precedent for locating files and the three named lints remain the
//! precedent for stripping comments and for asserting **anchors first**.
//!
//! 🛑 **Scope every walk to `crates/`.** `redkiln run` creates a run-owned worktree at
//! `.git/redkiln/worktrees/<item>/` which is a **full second checkout of the workspace**, and it
//! can sit many commits behind HEAD. Measured 2026-09-21: with one present, `grep -rn 'TrailerKey::'`
//! from the repo root returned **38** where `crates/` returned **19** — an exact doubling against a
//! stale tree. Every walk here starts at `workspace_root().join("crates")`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_kernel::registry::Registry;
use rk_kernel::select::{
    ArtifactRegistry, Catalog, dangling_requires_lint, excludes_lint, load_lints, rank_lint,
    requires_cycle_lint,
};

/// The workspace root, resolved from this crate's manifest directory.
///
/// Redefined here rather than imported: `workspace_root` is not shared — it is defined
/// identically in nine sibling modules and `support` does not export it.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// Every `.rs` file under `dir`, recursively, sorted for a deterministic failure message.
fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let entries = match std::fs::read_dir(&d) {
            Ok(e) => e,
            Err(e) => panic!("the source tree at {} is readable: {e}", d.display()),
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Blank the CONTENTS of every double-quoted string literal on one code line, keeping the quotes.
///
/// This mirrors `crate::module_boundary`'s private `blank_string_literals`, whose warrant applies
/// here unchanged: *"A call, a read or an extraction is code; the same spelling inside a string is
/// a message or a key, and a rule that matched it would fire on its own violation text."*
///
/// 🛑 **Without this, every lint below is defeated by a dead `const` of string literals.** Measured
/// 2026-09-21 against an earlier draft that stripped comments only: four lines naming the five
/// kinds as `&str` turned `conformance_every_step_kind_has_an_executor` fully GREEN with nothing
/// routed and no `StepKind` match arm anywhere; two literals in `rk-cli`'s `doctor/rebuild.rs` —
/// a pure trailer *reader* — turned the trailer lint GREEN; and one literal turned `toolchain`
/// into a "called" port. An armed lint that a comment cannot satisfy but a string literal can is
/// not a source lint, and the shape is live rather than hypothetical: `module_boundary.rs` carries
/// `"Tracker",` as a string literal on a code line in this very crate today.
fn blank_string_literals(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let mut chars = code.chars();
    let mut in_str = false;
    while let Some(c) = chars.next() {
        if in_str {
            match c {
                '\\' => {
                    chars.next();
                }
                '"' => {
                    in_str = false;
                    out.push('"');
                }
                _ => {}
            }
        } else {
            out.push(c);
            if c == '"' {
                in_str = true;
            }
        }
    }
    out
}

/// `src`'s lines as CODE: comments removed and string-literal contents blanked.
///
/// The comment half is `rk_kernel::kernel_denylist::strip_comments`, the tree's own stripper,
/// reused rather than re-implemented. It carries block-comment state across lines and removes a
/// **trailing** comment on a code line, which the `retry.rs` line-based filter this module first
/// copied does not.
///
/// 🛑 **The line-based filter was not enough and the measurement is recorded so it is not
/// reintroduced.** `retry.rs`'s `retry_source_without_comments` drops a line whose first
/// non-whitespace is `//`, `/*` or `*` — correct for its own single-file job, and defeated here by
/// one trailing comment: `const _X: () = (); // ports.toolchain` moved `toolchain` out of
/// `conformance_every_port_has_a_call_site`'s uncalled set [measured 2026-09-21]. Three lints read
/// whole directories of other people's code, so the stripper has to be the real one.
fn code_lines(src: &str) -> Vec<String> {
    let mut in_block = false;
    let mut out = Vec::new();
    for raw in src.lines() {
        let code = blank_string_literals(&rk_kernel::kernel_denylist::strip_comments(
            raw,
            &mut in_block,
        ));
        if code.trim().is_empty() {
            continue;
        }
        out.push(code);
    }
    out
}

/// Every code line under `dir`, paired with the file it came from.
fn code_lines_under(dir: &Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for path in rs_files(dir) {
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        for line in code_lines(&src) {
            out.push((path.clone(), line));
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// The stripper's own control.
// ---------------------------------------------------------------------------------------------

/// **[`code_lines`] drops prose and string contents, and keeps code.** Without this the stripper
/// ships untested: it changes no answer on today's tree (measured 2026-09-21 — no `ports.<field>`
/// pattern sits on a comment-only line in `rk-kernel/src/`), so a broken stripper would be
/// invisible until prose was added, which is precisely when it would matter.
///
/// 🛑 **Three of these cases are regressions, not hypotheticals.** An adversarial pass over the
/// first draft of this module defeated all three armed lints by exactly these shapes: a trailing
/// comment on a real code line, a dead `const` of string literals, and a string literal inside a
/// file that is a *reader* rather than a producer. Each is now a case here.
#[test]
fn the_comment_stripper_ignores_prose() {
    let src = concat!(
        "// StepKind::Agent in a line comment\n",
        "/// StepKind::Tool in a doc comment\n",
        "    //! StepKind::Check in an inner doc comment\n",
        "/* StepKind::Gate in a block comment\n",
        " * StepKind::Emit continued\n",
        " */\n",
        "const _T: () = (); // ports.toolchain in a TRAILING comment\n",
        "const _S: [&str; 2] = [\"StepKind::Emit\", \"TrailerKey::Story\"];\n",
        "let k = StepKind::Agent;\n",
    );
    let kept = code_lines(src);

    // THE CODE SURVIVES.
    assert!(
        kept.iter().any(|l| l.contains("StepKind::Agent")),
        "the code line must survive the stripper: {kept:?}"
    );

    // EVERY COMMENT SHAPE IS GONE, the trailing one included.
    for prose in [
        "in a line comment",
        "in a doc comment",
        "in an inner doc comment",
        "in a block comment",
        "continued",
        "in a TRAILING comment",
    ] {
        assert!(
            !kept.iter().any(|l| l.contains(prose)),
            "the stripper kept a comment containing {prose:?}: {kept:?}"
        );
    }

    // THE TRAILING COMMENT'S PAYLOAD IS GONE, not merely its prose. This is the case that defeated
    // the first draft: the line is real code, so a line-based filter keeps it whole.
    assert!(
        !kept.iter().any(|l| l.contains("ports.toolchain")),
        "a trailing comment's `ports.toolchain` survived and would satisfy the port lint: {kept:?}"
    );

    // STRING-LITERAL CONTENTS ARE BLANKED. A dead const of literals must satisfy no lint.
    for payload in ["StepKind::Emit", "TrailerKey::Story"] {
        assert!(
            !kept.iter().any(|l| l.contains(payload)),
            "a string literal containing {payload:?} survived and would satisfy a lint: {kept:?}"
        );
    }
    // …and the line itself is still seen as code, quotes kept, so blanking is not deletion.
    assert!(
        kept.iter().any(|l| l.contains("const _S")),
        "blanking must keep the line, not drop it: {kept:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// 1. Every step kind has an executor.  ARMED — discharged by s7g.
// ---------------------------------------------------------------------------------------------

/// **design §4.5 fixes five closed step kinds and the runtime routes on none of them.**
///
/// The design's table row reads *"Step kinds | **Five, closed**: `agent` (the only place a model
/// runs), `tool` (a toolchain verb through the port), `check` (a deterministic predicate in Rust;
/// cannot dispatch), `gate` (the kernel writes the mark and **exits 0**, resuming into the same
/// step), `emit` (binds outputs to `provides`; exactly one per kind provided; terminal)"*. The row
/// sits under `### 4.5 The catalog v0`; the design's own provenance table bundles it as *"§4.4,
/// §4.5"*, which is where the runbook's "§4.4" came from. `crates/rk-kernel/src/catalog.rs`'s
/// `StepKind` carries exactly those five and the loader parses all five.
///
/// Measured 2026-09-21: `grep -rn 'StepKind' crates/rk-kernel/src/runtime/` returns **0**, and
/// `crates/rk-cli/src/` returns 0 as well — the CLI cannot route a step kind either. Across the
/// shipped catalog that leaves **389 of 574** steps with no executor: **113 `tool`** (28 of them
/// `toolchain:test`), **102 `emit`**, **94 `check`** and **80 `gate`**; **102 of 102** activities
/// carry at least one non-agent step.
///
/// 🛑 **The absence is structural, not an oversight at the call site.** `ActivitySpec` — the only
/// thing `run_activity` is told about the work — carries `card_digest`, `tools`, `schema`,
/// `timeout_secs` and `inputs`, and **neither a step kind nor a step list**. `StepKind` could not
/// be routed on at this seam even if someone wanted to. s7g's scope is a new seam, not a new match
/// arm, and this lint stays red until one exists.
///
/// 🛑 **"No executor" is a claim about the port, not about cost.** `src/workflow/budget.rs` already
/// charges an attempt to `StepKind::Agent | StepKind::Tool` and nothing to `Check | Gate | Emit`,
/// so by the kernel's own accounting 298 steps are attempt-bearing. What this lint asserts is
/// narrower and is the thing that is false today: **only the agent kind reaches a port**.
#[test]
fn conformance_every_step_kind_has_an_executor() {
    let root = workspace_root();
    let runtime_dir = root.join("crates/rk-kernel/src/runtime");

    // ANCHORS FIRST. An absent or emptied directory passes every assertion below.
    assert!(
        runtime_dir.is_dir(),
        "{} is missing; an absence assertion over a moved runtime is a vacuous pass",
        runtime_dir.display()
    );
    let files = rs_files(&runtime_dir);
    assert!(
        !files.is_empty(),
        "{} holds no .rs files; the scan would be vacuous",
        runtime_dir.display()
    );
    let lines = code_lines_under(&runtime_dir);
    assert!(
        lines.iter().any(|(_, l)| l.contains("fn run_activity")),
        "the runtime's dispatch entry point `run_activity` is not in {}; this lint is scanning the \
         wrong tree",
        runtime_dir.display()
    );

    // THE CLOSED SET, PARSED FROM THE ENUM ITSELF — not a literal array.
    //
    // 🛑 **This was a hardcoded `["Agent", "Tool", "Check", "Gate", "Emit"]` until `s7g`, and its
    // sibling lint below has parsed `Ports` from source since it was written.** The asymmetry was
    // the weakness: a literal list is an inventory a fixture would have to edit, which is exactly
    // what `rk-ports`' `source_parse.rs` refuses `include_str!` for, and it meant a SIXTH step kind
    // could be added to `catalog.rs` and this lint would keep passing by never asking about it —
    // the same vacuity shape as a `> 0` anchor on a pairwise lint (`shipped_pack_loads`'s 102 pin).
    // The set is now read from `StepKind`'s own declaration, so a kind enters the question by
    // existing rather than by being remembered.
    let catalog_src = root.join("crates/rk-kernel/src/catalog.rs");
    let catalog_text = std::fs::read_to_string(&catalog_src)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", catalog_src.display()));
    let catalog_code = code_lines(&catalog_text);
    assert!(
        catalog_code.iter().any(|l| l.contains("pub enum StepKind")),
        "`pub enum StepKind` is not in {}; this lint is reading the wrong file",
        catalog_src.display()
    );
    let five: Vec<String> = catalog_code
        .iter()
        .skip_while(|l| !l.contains("pub enum StepKind"))
        .skip(1)
        .take_while(|l| !l.starts_with('}'))
        .filter_map(|l| {
            let t = l.trim().strip_suffix(',')?.trim();
            (!t.is_empty()
                && t.starts_with(|c: char| c.is_ascii_uppercase())
                && t.chars().all(|c| c.is_ascii_alphanumeric()))
            .then(|| t.to_owned())
        })
        .collect();

    // ANCHORS ON THE PARSE, before it is used. A parse that dropped a variant would report that
    // kind as executed by never asking about it — the port lint's `fields.len() == 5` guard, for
    // the same reason.
    assert_eq!(
        five.len(),
        5,
        "design §4.5 closes the step kinds at five and `StepKind` parsed as {five:?} — a parse \
         that drops a variant would report that kind as having an executor by never asking"
    );
    for expected in ["Agent", "Check", "Emit", "Gate", "Tool"] {
        assert!(
            five.iter().any(|k| k == expected),
            "`StepKind` parsed as {five:?}, which is missing the {expected:?} variant"
        );
    }

    let missing: Vec<&str> = five
        .iter()
        .map(String::as_str)
        .filter(|k| {
            let pat = format!("StepKind::{k}");
            !lines.iter().any(|(_, l)| l.contains(&pat))
        })
        .collect();

    assert!(
        missing.is_empty(),
        "design §4.5 closes the step kinds at five and the runtime routes on {} of them — no \
         executor for: {missing:?}. `Runtime::run_activity` dispatches an ACTIVATION, not an \
         activity's STEPS, and `ActivitySpec` carries no step list to route on.",
        5 - missing.len()
    );
}

// ---------------------------------------------------------------------------------------------
// 2. Every port has a call site.  ARMED — discharged by s7g (toolchain) and later (tracker/docs).
// ---------------------------------------------------------------------------------------------

/// **ADR-0003's five ports, and three of them are unreachable from the kernel.**
///
/// ADR-0003 fixes the closed set of five. It carries `superseded_by: ["0094"]`, and a reader who
/// stops at the frontmatter would conclude the table is dead: it is not. ADR-0094 supersedes it
/// **in part** and says so in terms — *"The five ports, the no-Postgres decision, git-is-not-a-port
/// and the fake-per-port rule stand; ADR-0003 stays `accepted`"* — withdrawing only the Toolchain
/// row's Phase-1 adapter column. The closed set of five names is intact and is still ADR-0003's.
///
/// The set is read from `crates/rk-ports/src/root.rs`'s `Ports` struct rather than from the
/// record's prose, because the struct is what a call site can actually reach, and the two port
/// names that do **not** match their field names (`EventStore` → `store`, `DocsSite` → `docs`)
/// would make a name-derived lint look for `ports.eventstore` and find nothing — a filter that
/// matches nothing, reported as a clean zero.
///
/// Measured 2026-09-21 over code lines in `crates/rk-kernel/src/`, excluding `src/replay.rs`:
/// **harness 1, store 1, toolchain 0, tracker 0, docs 0**. `Tracker`, `DocsSite` and `Toolchain`
/// have no `dyn` binding, no import and no method call anywhere in the kernel.
///
/// 🛑 **`src/replay.rs` is excluded and the exclusion is asserted.** `rk_kernel::replay::ReplayPorts`
/// is a *second* struct with fields `harness` and `store`, and `pub mod replay;` is unconditional
/// (only the *test* module `tests/it/replay.rs` is `#![cfg(feature = "corpus")]`). Counting its
/// hits would let a port read as called through a struct that is not the composition root's.
///
/// 🛑 **Method-name matching is not viable and is not attempted.** `DocsSite::build` and
/// `Toolchain::build` share a name; `Toolchain::classify` collides with three unrelated `fn
/// classify`; `Toolchain::lint` collides with the four `*_lint` functions in `select/lints.rs`;
/// `Toolchain::test` is hopeless. The receiver is mandatory.
#[test]
fn conformance_every_port_has_a_call_site() {
    let root = workspace_root();
    let ports_src = root.join("crates/rk-ports/src/root.rs");
    let kernel_src = root.join("crates/rk-kernel/src");

    // ANCHORS FIRST, over the PARSED SET and not merely over the file set. `rk-ports`'
    // `source_parse.rs` records the measured vacuity this closes: "respelling the trait prefix
    // `pub  trait` with a double space left `port_fake_parity` GREEN with a non-empty file set".
    let ports_text = std::fs::read_to_string(&ports_src)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", ports_src.display()));
    let ports_code = code_lines(&ports_text);
    assert!(
        ports_code.iter().any(|l| l.contains("pub struct Ports")),
        "`pub struct Ports` is not in {}; this lint is reading the wrong file",
        ports_src.display()
    );

    // THE CLOSED SET, parsed from the struct itself.
    let fields: BTreeSet<String> = ports_code
        .iter()
        .skip_while(|l| !l.contains("pub struct Ports"))
        .take_while(|l| !l.starts_with('}'))
        .filter_map(|l| {
            let t = l.trim();
            let rest = t.strip_prefix("pub ")?;
            let (name, _) = rest.split_once(':')?;
            let name = name.trim();
            name.chars()
                .all(|c| c.is_ascii_lowercase() || c == '_')
                .then(|| name.to_owned())
        })
        .collect();

    assert_eq!(
        fields.len(),
        5,
        "ADR-0003 closes the port set at five and `Ports` parsed as {fields:?} — a parse that \
         drops a field would report that port as callable by never asking about it"
    );
    for expected in ["docs", "harness", "store", "toolchain", "tracker"] {
        assert!(
            fields.contains(expected),
            "`Ports` parsed as {fields:?}, which is missing the {expected:?} field"
        );
    }

    assert!(kernel_src.is_dir(), "{} is missing", kernel_src.display());
    let replay = kernel_src.join("replay.rs");
    assert!(
        replay.is_file(),
        "{} is missing — the `ReplayPorts` exclusion below is stale and the counts it guards \
         against would silently re-enter",
        replay.display()
    );

    let lines: Vec<(PathBuf, String)> = code_lines_under(&kernel_src)
        .into_iter()
        .filter(|(p, _)| p != &replay)
        .collect();
    assert!(
        !lines.is_empty(),
        "no code lines under {}; the scan would be vacuous",
        kernel_src.display()
    );

    // BOTH RECEIVER SPELLINGS. The composition root is reached two ways in this crate — the field
    // directly (`self.ports.store.lock()`, runtime.rs) and through the accessor
    // (`self.ports().harness.dispatch(...)`, runtime/activity.rs). Measured 2026-09-21: a lint
    // matching only `ports.<field>` reports `harness` as UNCALLED while `dispatch` is the one
    // harness call site on the tree — an armed lint red for a reason that is not true.
    let uncalled: Vec<String> = fields
        .iter()
        .filter(|f| {
            let field = format!("ports.{f}");
            let accessor = format!("ports().{f}");
            !lines
                .iter()
                .any(|(_, l)| l.contains(&field) || l.contains(&accessor))
        })
        .cloned()
        .collect();

    // **ADR-0134: AN EQUALITY, NOT AN EMPTINESS — AND THE WARRANT IS PER ENTRY.**
    //
    // `uncalled.is_empty()` asserted a property Phase 1 is DESIGNED never to satisfy, so its only
    // reachable state inside this phase was `#[ignore]`d — an assertion that cannot become true is
    // an assertion that never runs, which is the coverage theatre this module's header exists to
    // prevent. The two entries below have no Phase-1 real adapter BY DECISION, and their argument
    // types are contentless newtypes so that no Phase-3 schema is invented early: there is no
    // meaningful call to write, and one written to satisfy a substring scan is not a call site.
    //
    // The instrument is `runbook/tools/check-checkpoints.py`'s, whose allowlist is
    // `flagged == allowed`, "never 'flagged is small'", enumerated with a reason per entry.
    // Equality means this FAILS IN BOTH DIRECTIONS: if `toolchain` regresses to 0 after s7g, and
    // if `tracker` or `docs` gains a call site while a record still says none exists. Moving an
    // entry out belongs with the change that earns it, on `shipped_pack_loads`'s pinned-count
    // precedent.
    let expected_uncalled: [(&str, &str); 2] = [
        (
            "docs",
            "no Phase-1 real adapter — design:266, `DocsSite` | … | \"none in Phase 1 (mdBook is \
             Phase 3)\"; AC-510 is Phase 3; `s7a` §7a:198-199 authors `DocSet`'s companions as \
             opaque newtypes so no Phase-3 schema is invented early",
        ),
        (
            "tracker",
            "no Phase-1 real adapter — design:265, `Tracker` | … | \"none in Phase 1 (GitHub is \
             Phase 3)\"; AC-508's idempotency key is Phase 3 and `s7a` §7a:198-199 declines to \
             design it; `ExternalRef` / `TypedInput` are `pub(crate) ()` newtypes with no field set",
        ),
    ];
    let expected: BTreeSet<String> = expected_uncalled
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
    // The expected set is itself checked against the parsed domain, so a renamed or removed port
    // cannot sit in this list unnoticed and quietly excuse itself.
    for (name, _) in &expected_uncalled {
        assert!(
            fields.contains(*name),
            "the expected-uncalled entry {name:?} is not a port in `Ports` ({fields:?}) — the \
             warrant list has drifted from ADR-0003's set"
        );
    }

    let actual: BTreeSet<String> = uncalled.iter().cloned().collect();
    assert_eq!(
        actual,
        expected,
        "the kernel's unreached ports are {actual:?} and the warranted set is {expected:?}.\n  \
         A port that LEFT the expected set has gained a call site while a record still says it has \
         none — move the entry and its warrant in the same change (ADR-0134 clause 4).\n  \
         A port that JOINED it has lost its call site — that is a regression, not a scope change.\n\
         The warrants are:\n  {}",
        expected_uncalled
            .iter()
            .map(|(n, why)| format!("{n}: {why}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

// ---------------------------------------------------------------------------------------------
// 3. Every trailer key has a producer.  ARMED — discharged by s7h.
// ---------------------------------------------------------------------------------------------

/// **design §5.4's trailer grammar names three keys and two of them are written by nothing.**
///
/// The grammar table closes the set at `Story:`, `Slice-Verdict:` and `Gate:` plus an
/// *"any other key"* catch-all whose rule is *"Unknown keys are ignored and counted"*.
/// `rk_git::trailer::grammar::TrailerKey` carries exactly that shape — `Story`, `SliceVerdict`,
/// `Gate`, `Other(String)` — and `rk_types::EventKind::trailer_axis()` already encodes the same
/// map from the event side (`StageTransition` → `Story`, `GateOpened | GateAnswered` → `Gate`,
/// `SliceSealed` → `Slice-Verdict`). This lint cross-checks the two so the closed set is pinned by
/// a machine-readable map and not by a transcription of design prose.
///
/// Measured 2026-09-21: **`Gate` 2** (`rk-cli/src/verbs/run.rs` and `verbs/answer.rs`),
/// **`Story` 0**, **`Slice-Verdict` 0**.
///
/// 🛑 **The literal spelling finds nothing and is not used.** No producer ever writes `"Gate: "`,
/// `"Story: "` or `"Slice-Verdict: "`; the key becomes text exactly once, in `Trailer::to_line()`
/// via `format!("{}: {}", self.key.as_str(), self.value)`. Grepping the literals reports the
/// producer set as **empty for all three keys**, `Gate` included — a filter that matches nothing,
/// read as a clean zero. The only `/src/` hits for those literals are three diagnostic message
/// strings in `rk-cli/src/doctor/rebuild.rs`.
///
/// 🛑 **`rk-git/src/trailer/` is the READER direction and is excluded.** That module defines and
/// parses the keys — eleven `TrailerKey::` occurrences, including all four `SliceVerdict` ones.
/// Counting them would report `Slice-Verdict` as produced by the code that reads it. The producer
/// scope is the action side: `rk-cli/src/` and `rk-kernel/src/`.
///
/// 🛑 **The `Story:` zero is a live tier-1 durability gap, not a missing convenience.**
/// `runtime.rs` appends the `stage_transition` event to the **store** and no `Story:` trailer
/// commit accompanies it, while design §5.4's tier-1 rule is *"git first"* and AC-162 requires the
/// trailer commit to exist before any store row for that transition is readable.
#[test]
#[ignore = "ARMED: Story and Slice-Verdict have 0 producers; only Gate is minted; discharged by s7h"]
fn conformance_every_trailer_key_has_a_producer() {
    let root = workspace_root();

    // THE CLOSED SET, cross-checked against the event side's own map. If the two ever disagree,
    // the set this lint guards is not the set the kernel believes in.
    let axes: BTreeSet<&str> = rk_types::event_kind::EventKind::ALL
        .iter()
        .filter_map(|k| k.trailer_axis())
        .collect();
    let declared: BTreeSet<&str> = ["Gate", "Slice-Verdict", "Story"].into_iter().collect();
    assert_eq!(
        axes, declared,
        "design §5.4's trailer grammar and `EventKind::trailer_axis()` name different key sets"
    );

    // ANCHORS FIRST.
    let producer_dirs = [
        root.join("crates/rk-cli/src"),
        root.join("crates/rk-kernel/src"),
    ];
    let mut lines: Vec<(PathBuf, String)> = Vec::new();
    for d in &producer_dirs {
        assert!(d.is_dir(), "{} is missing", d.display());
        lines.extend(code_lines_under(d));
    }
    assert!(
        lines.iter().any(|(_, l)| l.contains("TrailerKey::")),
        "no `TrailerKey::` construction anywhere in the producer scope — the scope is wrong, or \
         the trailer API was renamed and this lint is now asserting nothing"
    );

    // `Story` / `Slice-Verdict` are the design's spellings; the variants are `Story` /
    // `SliceVerdict`. The map is written out so a rename of either side fails loudly here.
    let variant_of = |axis: &str| match axis {
        "Story" => "Story",
        "Slice-Verdict" => "SliceVerdict",
        "Gate" => "Gate",
        other => panic!("no `TrailerKey` variant is mapped for the {other:?} axis"),
    };

    let unproduced: Vec<&str> = declared
        .iter()
        .copied()
        .filter(|axis| {
            let pat = format!("TrailerKey::{}", variant_of(axis));
            !lines.iter().any(|(_, l)| l.contains(&pat))
        })
        .collect();

    assert!(
        unproduced.is_empty(),
        "design §5.4 closes the trailer grammar at three keys and {} of them are minted — no \
         producer for: {unproduced:?}. Only `Gate:` is ever written (run.rs, answer.rs); a stage \
         advance appends `stage_transition` to the store with no `Story:` commit beside it.",
        3 - unproduced.len()
    );
}

// ---------------------------------------------------------------------------------------------
// 4. The shipped pack loads.  GREEN from s7f §7f.2.
// ---------------------------------------------------------------------------------------------
/// **design §4.5's `check` arm CANNOT DISPATCH — asserted structurally, `s7g` §7g.4.**
///
/// §7g.4 states this as a bullet and its checkpoint block asserted nothing for it. The prohibition is
/// carried by a **signature**: `runtime/step.rs`'s `check_verdict` is a FREE FUNCTION taking no
/// `&self`, so it has no `Ports`, no store and no journal in scope and there is no path from the arm
/// to a port. This test pins that shape, because "we wrote it as a free function" is a fact about
/// today's source and nothing else was checking it.
///
/// 🛑 **A docstring saying `check` cannot dispatch is exactly the shape this module exists to
/// distrust.** So the assertion is over CODE LINES — comments stripped, string literals blanked by
/// [`code_lines`] — and it is two-sided: the signature must take no receiver, AND the function body
/// must name no port.
#[test]
fn the_check_arm_has_no_receiver_and_so_cannot_reach_a_port() {
    let root = workspace_root();
    let src = root.join("crates/rk-kernel/src/runtime/step.rs");
    let text = std::fs::read_to_string(&src)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", src.display()));
    let code = code_lines(&text);

    // ANCHOR FIRST. A renamed or deleted function must not read as a satisfied prohibition.
    let sig = code
        .iter()
        .find(|l| l.contains("fn check_verdict"))
        .unwrap_or_else(|| {
            panic!(
                "`fn check_verdict` is not in {} — the `check` arm was renamed or removed and this                  lint is now asserting nothing about it",
                src.display()
            )
        });

    // (1) NO RECEIVER. `&self`, `&mut self`, `self` in the parameter list would all give the arm a
    // `Runtime`, and a `Runtime` is one field access from every port.
    assert!(
        !sig.contains("self"),
        "design §4.5 makes `check` unable to dispatch and `check_verdict` now takes a receiver,          which puts `Ports` one field access away: {sig}"
    );

    // (2) AND THE BODY NAMES NO PORT. A free function could still be handed a `&Ports`.
    let start = code
        .iter()
        .position(|l| l.contains("fn check_verdict"))
        .unwrap_or_else(|| unreachable!("the anchor above found it"));
    let body: Vec<&String> = code
        .iter()
        .skip(start)
        .take_while(|l| !l.starts_with('}'))
        .collect();
    assert!(
        body.len() > 1,
        "`check_verdict`'s body did not parse as more than its signature — the scan is vacuous"
    );
    for forbidden in [
        "Ports",
        "ports.",
        "ports()",
        "dispatch",
        "harness",
        "toolchain",
    ] {
        assert!(
            !body.iter().any(|l| l.contains(forbidden)),
            "the `check` arm names {forbidden:?}, so it can dispatch: design §4.5's              \"cannot dispatch\" is a prohibition, not a convention.
  {body:?}"
        );
    }
}

/// **The product's own `.redkiln/` passes every load-time lint.**
///
/// This is the one that would have caught all of the above, and its absence is the whole lesson:
/// **every catalog test in this tree runs over a FIXTURE catalog.** Every `SHIPPED_CATALOG` /
/// `SHIPPED_FACETS` constant in `tests/it/` points at `spikes/p9/`, and nothing ever loaded the
/// 102 activities the product ships. Step 8 ran the first `redkiln run` ever attempted on this
/// tree and the shipped pack did not load.
///
/// 🛑 **`spikes/p9/` is NOT the shipped pack and reusing `dag.rs`'s helpers would test the wrong
/// bytes.** Both directories hold 102 `.toml` files and produce identical step-kind totals, but
/// `diff -rq` reports **10 differing files** (data-migration-plan, feature-flag-kill,
/// flag-progressive-rollout, migration-dry-run, migration-expand-execute, schema-migration-script,
/// security-requirements, security-review, threat-model, ux-flow-design) and the two
/// `facets.toml` differ as well. This test joins `.redkiln/` for all three inputs.
///
/// It asserts every load-time lint — `requires_cycle_lint`, `excludes_lint`,
/// `dangling_requires_lint`, `rank_lint` — and not merely that the catalog parses. The four are
/// run **individually and all four verdicts collected** before anything is asserted, because
/// `load_lints` short-circuits on the first error in a fixed order with `rank_lint` **last**: a
/// cycle defect would mask the rank tie, and at Step 8 the tree carried both. `load_lints` is then
/// called as well, so the shipped entry point is pinned rather than merely the four rules.
///
/// `dead_content_report` is deliberately absent: AC-057 is *"reported as a warning … and does not
/// fail the build"*, and `load_lints` excludes it for that reason. No pack is needed here.
#[test]
fn shipped_pack_loads() {
    // The shipped pack's activity count, pinned rather than floored — see the anchor below.
    const SHIPPED_ACTIVITIES: usize = 102;

    let root = workspace_root();
    let pack_dir = root.join(".redkiln");
    let catalog_dir = pack_dir.join("catalog");

    // ANCHORS FIRST. A missing directory must not read as a clean load.
    assert!(
        catalog_dir.is_dir(),
        "{} is missing; the shipped catalog is what this test exists to load",
        catalog_dir.display()
    );
    let activity_files = std::fs::read_dir(&catalog_dir)
        .expect("the shipped catalog directory is readable")
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "toml"))
        .count();
    // 🛑 A FLOOR OF ONE IS NOT AN ANCHOR HERE, and the counter-example is measured. `rank_lint`
    // detects ties PAIRWISE, so a catalog holding a single activity can never tie: with
    // `keyed_by` removed from `[artifact.ImplReport]` AND the catalog reduced to one file, an
    // earlier draft of this test reported a clean load while carrying the exact defect it exists
    // to catch [measured 2026-09-21, adversarial pass]. The shipped count is therefore PINNED, so
    // that deleting the pack's contents fails loudly instead of passing quietly. Moving this
    // number is a deliberate act that belongs with the pack change that moves it.
    assert_eq!(
        activity_files,
        SHIPPED_ACTIVITIES,
        "{} holds {activity_files} .toml activities and the shipped pack has \
         {SHIPPED_ACTIVITIES}; a pairwise lint over a shrunken catalog is vacuously green, so this \
         count is pinned rather than floored",
        catalog_dir.display()
    );

    let artifacts_path = pack_dir.join("artifacts.toml");
    let facets_path = pack_dir.join("facets.toml");
    let artifacts_src = std::fs::read_to_string(&artifacts_path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", artifacts_path.display()));
    let facets_src = std::fs::read_to_string(&facets_path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", facets_path.display()));

    let catalog = Catalog::from_dir(&catalog_dir).unwrap_or_else(|e| {
        panic!(
            "the shipped catalog at {} loads: {e:?}",
            catalog_dir.display()
        )
    });
    let artifacts = ArtifactRegistry::from_toml("artifacts.toml", &artifacts_src)
        .unwrap_or_else(|e| panic!("the shipped artifact registry loads: {e:?}"));
    let facets = Registry::from_toml(&facets_src)
        .unwrap_or_else(|e| panic!("the shipped facet registry loads: {e:?}"));

    assert_eq!(
        catalog.activities().len(),
        activity_files,
        "the loader dropped an activity: {activity_files} .toml files on disk, \
         {} in the catalog",
        catalog.activities().len()
    );

    // ALL FOUR VERDICTS, then the assertion. `load_lints` would stop at the first.
    let verdicts: [(&str, Result<(), rk_kernel::select::LoadError>); 4] = [
        ("requires_cycle_lint", requires_cycle_lint(&catalog)),
        ("excludes_lint", excludes_lint(&catalog)),
        (
            "dangling_requires_lint",
            dangling_requires_lint(&catalog, &artifacts),
        ),
        ("rank_lint", rank_lint(&catalog, &artifacts, &facets)),
    ];

    let failures: Vec<String> = verdicts
        .iter()
        .filter_map(|(name, r)| r.as_ref().err().map(|e| format!("{name}: {e:?}")))
        .collect();

    assert!(
        failures.is_empty(),
        "the product's own pack at {} fails {} of the four load-time lints:\n  {}",
        pack_dir.display(),
        failures.len(),
        failures.join("\n  ")
    );

    // And the shipped entry point itself, so the test pins `load_lints` and not only its parts.
    load_lints(&catalog, &artifacts, &facets)
        .unwrap_or_else(|e| panic!("`load_lints` refuses the shipped pack: {e:?}"));
}
