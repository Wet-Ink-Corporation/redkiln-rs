//! AC-483's own named fixture: *"a compile-fail fixture declaring `trait GitPort`"* (register
//! :6958). This file **must fail to compile**.
//!
//! **A file that merely declares `trait GitPort` compiles** [measured 2026-09-13 on the pin: a
//! `pub trait GitPort: Send + Sync` with four git-surface methods builds clean, exit 0]. So the
//! fixture has to reach a rule `rk-ports` actually owns, and the rule is the **port inventory**:
//! ADR-0003:34 gives exactly five ports, ADR-0003:57-58 says *"Git may not be abstracted behind a
//! port"*, and `rk_ports::root::Ports` is where the five live. A declared `GitPort` has nowhere to
//! go — there is no sixth field to hold it — and that is what fails here, naming the inventory.
//!
//! The runtime half of AC-483 is `git_surface_denylist_rejects_git_port`, which refuses the name
//! and the method names from the same source parse the parity count uses. The two are separate
//! instruments and neither substitutes for the other: this one is about the port SET, that one is
//! about what a trait in this crate may be CALLED.

use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::root::Ports;
use std::sync::{Arc, Mutex};

pub trait GitPort: Send + Sync {
    fn trailer(&self);
    fn worktree(&self);
    fn clone_repo(&self);
    fn reconcile(&self);
}

struct FakeGit;

impl GitPort for FakeGit {
    fn trailer(&self) {}
    fn worktree(&self) {}
    fn clone_repo(&self) {}
    fn reconcile(&self) {}
}

fn main() {
    let _ports = Ports {
        store: Arc::new(Mutex::new(VecStore::default())),
        harness: Arc::new(FakeHarness::scripted(Vec::new())),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::from_fixture(Vec::new())),
        git: Arc::new(FakeGit),
    };
}
