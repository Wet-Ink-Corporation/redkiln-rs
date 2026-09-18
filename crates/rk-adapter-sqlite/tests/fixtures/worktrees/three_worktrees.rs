//! AC-370's three-worktree repository builder (s7c §7c.17: `worktrees/`).
//!
//! One main worktree with one commit, two branches, and two **real linked worktrees** added
//! through `rk_git::worktree::add` over the `GitExec` the test mints — so `git rev-parse
//! --git-common-dir` from either linked tree answers the main repository's `.git`, and from the
//! main tree answers the cwd-relative `.git` the adapter must join (s7c §7c.13's join rule).
//! Step 6's AC-178 tests handed a temp dir in as `common_dir` and created no linked worktree
//! (critic gap 8); this builder is what makes the three-worktree test genuinely 7c's.

use std::path::PathBuf;

use rk_git::git_exec::GitExec;

use crate::support::{Fixture, wt};

/// The three worktrees, main first.
pub struct ThreeWorktrees {
    pub main: PathBuf,
    pub linked: [PathBuf; 2],
}

impl ThreeWorktrees {
    /// All three, main first.
    pub const fn all(&self) -> [&PathBuf; 3] {
        [&self.main, &self.linked[0], &self.linked[1]]
    }
}

/// Build the repository: `main` with one commit, then `wt1` and `wt2` as linked worktrees on
/// branches of the same name, each under the fixture's own directory (never inside `main`).
pub fn build(fx: &Fixture, exec: &GitExec) -> ThreeWorktrees {
    let main = fx.repo_with_one_commit("main");
    let mut linked: Vec<PathBuf> = Vec::new();
    for name in ["wt1", "wt2"] {
        fx.git(&main, &["branch", name]);
        let path = fx.sub(name);
        rk_git::worktree::add(exec, &wt(&main), &path, name)
            .unwrap_or_else(|e| panic!("worktree add {name}: {e:?}"));
        assert!(
            path.join(".git").is_file(),
            "a linked worktree carries a `.git` FILE pointing at the common dir; {} does not",
            path.display()
        );
        linked.push(path);
    }
    let second = linked.pop().expect("wt2");
    let first = linked.pop().expect("wt1");
    ThreeWorktrees {
        main,
        linked: [first, second],
    }
}
