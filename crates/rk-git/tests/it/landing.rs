//! **The landing primitive** — `s8f` §8f.4, the landing record ADR-0188 (proposed).
//!
//! *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)* Ten tests, each over a real
//! repository whose main worktree sits on `rk-run/<item>`, through the journalled seam. Every git
//! child the primitive spawns runs with `GIT_CONFIG_NOSYSTEM=1` and an empty `GIT_CONFIG_GLOBAL`
//! (this box carries `core.autocrlf=true` globally), so no reading here is a property of the
//! operator's configuration.

use std::collections::BTreeSet;
use std::path::PathBuf;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::gitops::{GitOps, Mode, VecJournal};
use rk_git::landing::{
    self, Baseline, DENYLIST, Landed, MAX_LANDED_BLOB_BYTES, Message, Request, Rule, Staged,
};
use rk_git::path::WorktreePath;
use rk_git::trailer::grammar::{GateState, ParsedTrailer};

const ITEM: &str = "land-1";
const BRANCH: &str = "rk-run/land-1";

struct Lx {
    fx: crate::support::Fixture,
    repo: PathBuf,
    exec: GitExec,
}

impl Lx {
    fn cwd(&self) -> WorktreePath {
        WorktreePath::new(&self.repo).expect("the repo path normalises")
    }

    fn git(&self, args: &[&str]) -> String {
        self.fx.git(&self.repo, args).stdout_trimmed()
    }

    fn tip(&self) -> String {
        self.git(&["rev-parse", &format!("refs/heads/{BRANCH}")])
    }

    fn write(&self, rel: &str, bytes: &[u8]) {
        let p = self.repo.join(rel);
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d).expect("mkdir");
        }
        std::fs::write(p, bytes).expect("write");
    }

    /// Every path the run ref's tip tracks.
    fn landed_paths(&self) -> BTreeSet<String> {
        self.git(&["ls-tree", "-r", "--name-only", BRANCH])
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

/// A repository on `rk-run/land-1` with `a.txt` and `b.txt` tracked, `.gitattributes`'
/// `* text=auto eol=lf` (ADR-0164) and a `.gitignore` of `*.log`.
fn fixture() -> Lx {
    let fx = crate::support::Fixture::new();
    let repo = fx.sub("repo");
    std::fs::create_dir_all(&repo).expect("mkdir repo");
    fx.git(&repo, &["init", "--quiet"]);
    for (k, v) in [
        ("user.name", "rk"),
        ("user.email", "rk@example.invalid"),
        ("core.autocrlf", "false"),
        ("commit.gpgsign", "false"),
    ] {
        fx.git(&repo, &["config", k, v]);
    }
    std::fs::write(repo.join(".gitattributes"), b"* text=auto eol=lf\n").expect("attrs");
    std::fs::write(repo.join(".gitignore"), b"*.log\n").expect("ignore");
    std::fs::write(repo.join("a.txt"), b"one\n").expect("a");
    std::fs::write(repo.join("b.txt"), b"two\n").expect("b");
    fx.git(&repo, &["add", "."]);
    fx.git(&repo, &["commit", "--quiet", "-m", "base"]);
    fx.git(&repo, &["checkout", "--quiet", "-b", BRANCH]);
    let tc = rk_git::version::assert_at_startup(&repo).expect("git at or above the floor");
    let exec = GitExec::new(fx.sub("hooks-empty"), tc)
        .expect("the wrapper")
        .with_env("GIT_CONFIG_NOSYSTEM", "1")
        .with_env("GIT_CONFIG_GLOBAL", fx.sub("empty.gitconfig"));
    Lx { fx, repo, exec }
}

fn empty_baseline() -> Baseline {
    Baseline::Known(BTreeSet::new())
}

fn message(subject: &str) -> Message {
    Message {
        subject: subject.to_owned(),
        body: None,
        trailer: None,
    }
}

/// Stage over a fresh live seam.
fn stage(lx: &Lx, baseline: &Baseline) -> Staged {
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(lx.exec.clone(), &mut journal, Mode::Live);
    let cwd = lx.cwd();
    let none = BTreeSet::new();
    landing::stage(
        &mut git,
        &Request {
            cwd: &cwd,
            item: ITEM,
            baseline,
            excluded: &none,
        },
    )
    .expect("the landing stages")
}

/// Stage and commit over one live seam.
fn land(lx: &Lx, baseline: &Baseline, msg: &Message, allow_empty: bool) -> Landed {
    let mut journal = VecJournal::default();
    land_journalled(lx, baseline, msg, allow_empty, &mut journal)
}

fn land_journalled(
    lx: &Lx,
    baseline: &Baseline,
    msg: &Message,
    allow_empty: bool,
    journal: &mut VecJournal,
) -> Landed {
    let mut git = GitOps::new(lx.exec.clone(), journal, Mode::Live);
    let cwd = lx.cwd();
    let none = BTreeSet::new();
    let staged = landing::stage(
        &mut git,
        &Request {
            cwd: &cwd,
            item: ITEM,
            baseline,
            excluded: &none,
        },
    )
    .expect("the landing stages");
    let Staged::Ready(prepared) = staged else {
        panic!("the landing was refused: {staged:?}");
    };
    landing::commit(&mut git, &prepared, msg, allow_empty).expect("the landing commits")
}

// ------------------------------------------------------------------------------ what is staged

/// **Run-created files are staged, and nothing `.gitignore` excludes is.** A file in a new
/// sub-directory lands too (`--untracked-files=all`). Red under M1 (the run-created `add`
/// dropped) and M3 (untracked listed with ignores disabled).
#[test]
fn run_created_file_is_staged_and_an_ignored_file_is_not() {
    let lx = fixture();
    lx.write("new.txt", b"created by the run\n");
    lx.write("sub/deep.txt", b"a nested creation\n");
    lx.write("debug.log", b"ignored\n");
    let landed = land(&lx, &empty_baseline(), &message("rk step land-1"), false);
    assert!(
        matches!(landed, Landed::Commit { content: true, .. }),
        "a run-created file is a content landing: {landed:?}"
    );
    let paths = lx.landed_paths();
    for p in ["new.txt", "sub/deep.txt"] {
        assert!(
            paths.contains(p),
            "the run-created `{p}` did not land: {paths:?}"
        );
    }
    assert!(
        !paths.contains("debug.log"),
        "an ignored file was staged: {paths:?}"
    );
    assert!(
        lx.repo.join("debug.log").exists(),
        "the ignored file must be left alone in the worktree"
    );
}

/// **A baseline untracked file is never staged**: it was there when the tree was created, so the
/// run did not create it. Red under M2 (the baseline not subtracted).
#[test]
fn baseline_untracked_file_is_never_staged() {
    let lx = fixture();
    lx.write("pre-existing.txt", b"there at creation\n");
    let baseline = Baseline::Known(BTreeSet::from(["pre-existing.txt".to_owned()]));
    lx.write("made.txt", b"made by the run\n");
    let Staged::Ready(prepared) = stage(&lx, &baseline) else {
        panic!("PREMISE: nothing here is refused");
    };
    assert_eq!(
        prepared.set().created,
        vec!["made.txt".to_owned()],
        "the staging set's run-created files"
    );
    assert_eq!(
        prepared.set().untracked,
        vec!["made.txt".to_owned(), "pre-existing.txt".to_owned()],
        "PREMISE: the read saw both untracked files"
    );
    land(&lx, &baseline, &message("rk step land-1"), false);
    let paths = lx.landed_paths();
    assert!(paths.contains("made.txt"), "the run's file did not land");
    assert!(
        !paths.contains("pre-existing.txt"),
        "a baseline file was staged: {paths:?}"
    );
}

/// **Tracked modifications and deletions are staged.**
#[test]
fn tracked_modification_and_deletion_are_staged() {
    let lx = fixture();
    lx.write("a.txt", b"one, modified\n");
    std::fs::remove_file(lx.repo.join("b.txt")).expect("delete b.txt");
    let landed = land(&lx, &empty_baseline(), &message("rk step land-1"), false);
    let Landed::Commit { sha, content, .. } = landed else {
        panic!("a modification and a deletion are a landing: {landed:?}");
    };
    assert!(content, "the landing carries content");
    assert_eq!(lx.tip(), sha, "the run ref moved to the landing");
    assert_eq!(
        lx.git(&["show", &format!("{BRANCH}:a.txt")]),
        "one, modified",
        "the modification did not land"
    );
    assert!(
        !lx.landed_paths().contains("b.txt"),
        "the deletion did not land"
    );
    assert_eq!(
        lx.git(&["status", "--porcelain"]),
        "",
        "after a landing the worktree's own index agrees with the tip (step 7)"
    );
}

// ------------------------------------------------------------------------------- due, not due

/// **Not due → an empty commit (a boundary) or none (a step); due → exactly one content commit.**
/// Red under M4 (always the empty commit, never a content commit).
#[test]
fn not_due_makes_an_empty_commit_and_due_makes_one_content_commit() {
    let lx = fixture();
    let base = lx.tip();

    // A step over a clean tree: not due, no commit, no ref move.
    let step = land(&lx, &empty_baseline(), &message("rk step land-1"), false);
    assert!(
        matches!(step, Landed::Nothing { .. }),
        "a step that changed nothing makes no commit: {step:?}"
    );
    assert_eq!(
        lx.tip(),
        base,
        "the run ref moved for a step that changed nothing"
    );

    // A boundary over a clean tree: not due, the trailer rides an EMPTY commit.
    let empty = land(
        &lx,
        &empty_baseline(),
        &message("land-1 enters review"),
        true,
    );
    let Landed::Commit { sha, content, .. } = empty else {
        panic!("a boundary always commits: {empty:?}");
    };
    assert!(!content, "a boundary over a clean tree is not due");
    assert_eq!(
        lx.git(&["rev-parse", &format!("{sha}^{{tree}}")]),
        lx.git(&["rev-parse", &format!("{sha}^^{{tree}}")]),
        "the not-due boundary commit is empty"
    );
    assert_eq!(
        lx.git(&["rev-list", "--count", &format!("{base}..{BRANCH}")]),
        "1",
        "exactly one commit"
    );

    // Due: exactly one content commit, whose tree differs from its parent's.
    lx.write("work.txt", b"the step's work\n");
    let due = land(
        &lx,
        &empty_baseline(),
        &message("land-1 enters verify"),
        true,
    );
    let Landed::Commit { sha, content, .. } = due else {
        panic!("a due landing commits: {due:?}");
    };
    assert!(content, "a due landing is a content commit");
    assert_ne!(
        lx.git(&["rev-parse", &format!("{sha}^{{tree}}")]),
        lx.git(&["rev-parse", &format!("{sha}^^{{tree}}")]),
        "the due landing's commit is empty"
    );
    assert_eq!(
        lx.git(&["rev-list", "--count", &format!("{base}..{BRANCH}")]),
        "2",
        "exactly one more commit"
    );
    assert!(lx.landed_paths().contains("work.txt"));
}

// ---------------------------------------------------------------------------------- refusals

/// One file name per denylist glob, matched on the final component.
const DENIED: [(&str, &str); 17] = [
    ("nested/.env", ".env"),
    ("nested/.env.local", ".env.*"),
    ("nested/server.pem", "*.pem"),
    ("nested/TLS.KEY", "*.key"),
    ("nested/id_rsa", "id_*"),
    ("nested/credentials.json", "credentials*"),
    ("nested/cert.p12", "*.p12"),
    ("nested/cert.pfx", "*.pfx"),
    ("nested/.netrc", ".netrc"),
    ("nested/.pgpass", ".pgpass"),
    ("nested/.git-credentials", ".git-credentials"),
    ("nested/.npmrc", ".npmrc"),
    ("nested/.pypirc", ".pypirc"),
    ("nested/putty.ppk", "*.ppk"),
    ("nested/store.jks", "*.jks"),
    ("nested/app.keystore", "*.keystore"),
    ("nested/vault.kdbx", "*.kdbx"),
];

/// **Every denylisted path is refused by name, nothing lands, and no ref moves** — table-driven
/// over every glob, each row beside an innocent file that must not land either (no partial
/// landing). Red under M5 (the denylist check dropped).
#[test]
fn denylisted_path_is_refused_by_name_and_no_ref_moves() {
    let lx = fixture();
    let globs: BTreeSet<&str> = DENIED.iter().map(|(_, g)| *g).collect();
    assert_eq!(
        globs,
        DENYLIST.iter().copied().collect::<BTreeSet<&str>>(),
        "PREMISE: the table covers every denylist glob, one row each"
    );
    let base = lx.tip();
    for (path, glob) in DENIED {
        lx.write(path, b"secret\n");
        lx.write("innocent.txt", b"fine on its own\n");
        let staged = stage(&lx, &empty_baseline());
        let Staged::Refused { refusals, .. } = &staged else {
            panic!("`{path}` ({glob}) was not refused: {staged:?}");
        };
        assert_eq!(
            refusals.len(),
            1,
            "exactly the denylisted path is refused: {refusals:?}"
        );
        assert_eq!(refusals[0].path, path, "the refusal names the path");
        assert_eq!(
            refusals[0].rule,
            Rule::Denylist(glob),
            "the refusal names the rule"
        );
        assert_eq!(
            refusals[0].to_string(),
            format!("{path} (denylist:{glob})"),
            "the refusal's spelling"
        );
        assert_eq!(lx.tip(), base, "a ref moved under a refusal of `{path}`");
        assert!(
            !lx.landed_paths().contains("innocent.txt"),
            "a refused landing landed its innocent path"
        );
        std::fs::remove_file(lx.repo.join(path)).expect("remove the denied file");
    }
}

/// **A blob of exactly the limit lands; one byte over is refused by name, and nothing lands.**
/// Red under M6 (`>` → `>=`).
#[test]
fn blob_at_the_limit_lands_and_one_byte_over_is_refused() {
    let lx = fixture();
    let limit = usize::try_from(MAX_LANDED_BLOB_BYTES).expect("fits");
    lx.write("at-limit.bin", &vec![b'a'; limit]);
    let landed = land(&lx, &empty_baseline(), &message("rk step land-1"), false);
    assert!(
        matches!(landed, Landed::Commit { content: true, .. }),
        "a blob of exactly {limit} bytes must land: {landed:?}"
    );
    assert!(lx.landed_paths().contains("at-limit.bin"));

    let before = lx.tip();
    lx.write("over-limit.bin", &vec![b'a'; limit + 1]);
    let staged = stage(&lx, &empty_baseline());
    let Staged::Refused { refusals, .. } = &staged else {
        panic!("a blob one byte over the limit was not refused: {staged:?}");
    };
    assert_eq!(
        refusals.iter().map(ToString::to_string).collect::<Vec<_>>(),
        vec![format!("over-limit.bin (size:{})", limit + 1)],
        "the refusal names the path and its size"
    );
    assert_eq!(lx.tip(), before, "a ref moved under a size refusal");
}

/// **A name refusal and a size refusal in one staging set are named at once** (ADR-0188 decisions
/// 5 and 6: *"Every path in the staging set is checked"*; the gate *"names every refused path with
/// its rule"*). *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 minor.)* Red while
/// a name refusal returns before the size check: the gate names `.env` alone, and the oversize
/// blob waits for a second gate. The name-refused file is never staged, so its bytes never enter
/// the object store.
#[test]
fn a_name_refusal_and_a_size_refusal_are_named_at_once() {
    let lx = fixture();
    let limit = usize::try_from(MAX_LANDED_BLOB_BYTES).expect("fits");
    let before = lx.tip();
    lx.write(".env", b"API_TOKEN=not-a-real-secret-r1fix\n");
    lx.write("over-limit.bin", &vec![b'a'; limit + 1]);
    let staged = stage(&lx, &empty_baseline());
    let Staged::Refused { refusals, .. } = &staged else {
        panic!("a denylisted path and an oversize blob were not refused: {staged:?}");
    };
    assert_eq!(
        refusals.iter().map(ToString::to_string).collect::<Vec<_>>(),
        vec![
            ".env (denylist:.env)".to_owned(),
            format!("over-limit.bin (size:{})", limit + 1)
        ],
        "every refused path is named with its rule, in one refusal"
    );
    assert_eq!(lx.tip(), before, "a ref moved under a refusal");
    let hashed = lx.git(&["hash-object", ".env"]);
    assert!(
        lx.fx
            .try_git(&lx.repo, &["cat-file", "-e", hashed.as_str()])
            .is_err(),
        "the denylisted file's bytes entered the object store (it was staged)"
    );
}

/// **A staged path one over ADR-0180's worst-case bound is refused by name; one under lands**
/// (D21-47, review m12), with `core.longpaths` unset. Red under M49 (the bound dropped).
#[test]
fn a_staged_path_over_the_worst_case_length_is_refused_by_name() {
    let lx = fixture();
    assert!(
        lx.fx
            .try_git(&lx.repo, &["config", "--get", "core.longpaths"])
            .is_err(),
        "PREMISE: core.longpaths is unset for this repository"
    );
    let listed = lx.git(&["worktree", "list", "--porcelain"]);
    let root = listed
        .lines()
        .find_map(|l| l.strip_prefix("worktree "))
        .expect("the main worktree")
        .to_owned();
    let fixed = root.encode_utf16().count() + 20 + ITEM.len() + 11 + 1;
    assert!(
        fixed < rk_git::worktree::PATH_LIMIT,
        "PREMISE: the fixture root leaves room"
    );
    let at = rk_git::worktree::PATH_LIMIT - fixed;
    let fits = "f".repeat(at);
    assert_eq!(fixed + fits.len(), 259, "PREMISE: the bound is exactly 259");
    lx.write(&fits, b"fits\n");
    let landed = land(&lx, &empty_baseline(), &message("rk step land-1"), false);
    assert!(
        matches!(landed, Landed::Commit { content: true, .. }),
        "a path whose bound is exactly 259 must land: {landed:?}"
    );

    let before = lx.tip();
    let over = "g".repeat(at + 1);
    lx.write(&over, b"one over\n");
    let staged = stage(&lx, &empty_baseline());
    let Staged::Refused { refusals, .. } = &staged else {
        panic!("a path whose bound is 260 was not refused: {staged:?}");
    };
    assert_eq!(refusals.len(), 1, "exactly the long path is refused");
    assert_eq!(refusals[0].path, over, "the refusal names the path");
    assert_eq!(
        refusals[0].rule,
        Rule::PathLength(260),
        "the rule and bound"
    );
    assert_eq!(lx.tip(), before, "a ref moved under a length refusal");
}

// ---------------------------------------------------------------------------- one derivation

/// **The tree a `Gate:` trailer carries is the commit's own tree**: T is derived before the commit
/// exists, written into the trailer, and the committed object re-reads as T.
#[test]
fn the_trailer_tree_is_the_commits_own_tree() {
    let lx = fixture();
    lx.write("reviewed.txt", b"the tree the human sees\n");
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(lx.exec.clone(), &mut journal, Mode::Live);
    let cwd = lx.cwd();
    let none = BTreeSet::new();
    let baseline = empty_baseline();
    let Staged::Ready(prepared) = landing::stage(
        &mut git,
        &Request {
            cwd: &cwd,
            item: ITEM,
            baseline: &baseline,
            excluded: &none,
        },
    )
    .expect("stages") else {
        panic!("PREMISE: nothing is refused");
    };
    let t = prepared.tree().to_owned();
    assert!(prepared.due(), "PREMISE: a new file makes the landing due");
    let trailer = ParsedTrailer::Gate {
        token: "gate-tok".to_owned(),
        tree_hash: t.clone(),
        deadline: 1_900_000_000,
        kind: "unknown_facet:x".to_owned(),
        reach: 1,
        state: GateState::Opened,
    }
    .to_trailer()
    .expect("renders");
    let landed = landing::commit(
        &mut git,
        &prepared,
        &Message {
            subject: "land-1 gate opened".to_owned(),
            body: None,
            trailer: Some(trailer),
        },
        true,
    )
    .expect("commits");
    let Landed::Commit { sha, tree, .. } = landed else {
        panic!("a gate always commits");
    };
    assert_eq!(tree, t, "the landing reports T");
    assert_eq!(
        lx.git(&["rev-parse", &format!("{sha}^{{tree}}")]),
        t,
        "the commit's own tree is not the trailer's"
    );
    let trailers = lx.git(&["log", "-1", "--format=%(trailers:only,unfold)", &sha]);
    assert!(
        trailers.contains(&format!("gate-tok {t} ")),
        "the trailer does not carry T: {trailers}"
    );
    assert!(
        lx.landed_paths().contains("reviewed.txt"),
        "the reviewed file is in the gate's commit"
    );
}

/// **Replay serves the journal and performs zero writes** (ADR-0105 clause 1): T and the commit
/// are served from the recorded invocations, the wrapper is never reached, and no ref moves —
/// even after the worktree has changed underneath.
#[test]
fn replay_serves_the_journal_and_performs_zero_writes() {
    let lx = fixture();
    lx.write("recorded.txt", b"the recorded landing\n");
    let mut journal = VecJournal::default();
    let live = land_journalled(
        &lx,
        &empty_baseline(),
        &message("rk step land-1"),
        false,
        &mut journal,
    );
    let Landed::Commit { sha, .. } = &live else {
        panic!("PREMISE: the live landing commits: {live:?}");
    };
    assert!(
        journal
            .entries
            .iter()
            .any(|e| e.op == GitOp::CommitTree.as_str()),
        "PREMISE: the commit-tree invocation was journalled"
    );
    let tip = lx.tip();
    assert_eq!(&tip, sha, "PREMISE: the live landing moved the ref");

    // The worktree moves on: a replay that read git would see it.
    lx.write("later.txt", b"after the recording\n");
    let tmp = lx.repo.join(".git").join("redkiln").join("tmp");
    let _ = std::fs::remove_dir_all(&tmp);

    let mut replaying = VecJournal::from_entries(journal.entries.clone());
    let mut git = GitOps::new(lx.exec.clone(), &mut replaying, Mode::Replay);
    let cwd = lx.cwd();
    let none = BTreeSet::new();
    let baseline = empty_baseline();
    let staged = landing::stage(
        &mut git,
        &Request {
            cwd: &cwd,
            item: ITEM,
            baseline: &baseline,
            excluded: &none,
        },
    )
    .expect("the replay stages");
    let Staged::Ready(prepared) = staged else {
        panic!("the replay was refused: {staged:?}");
    };
    let served = landing::commit(&mut git, &prepared, &message("rk step land-1"), false)
        .expect("the replay commits");
    assert_eq!(
        served, live,
        "the replay was not served the recorded landing"
    );
    assert_eq!(
        git.writes_performed(),
        0,
        "the replay reached the git wrapper"
    );
    assert_eq!(lx.tip(), tip, "the replay moved the run ref");
    assert!(
        !tmp.exists(),
        "the replay wrote the landing's temporary files"
    );
}

/// **A repository hook does not run on a landing commit** (ADR-0105 clause 2's fixture). The
/// repository's `core.hooksPath` holds `pre-commit`, `commit-msg`, `post-commit` and
/// `reference-transaction`, each appending to a marker. The CONTROL moves a ref with plain git and
/// the marker appears; the landing leaves it absent.
#[test]
fn a_repository_hook_does_not_run_on_a_landing_commit() {
    let lx = fixture();
    let hooks = lx.fx.sub("repo-hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    let marker = lx.fx.sub("hook-ran.txt");
    let shown = marker.display().to_string().replace('\\', "/");
    for name in [
        "pre-commit",
        "commit-msg",
        "post-commit",
        "reference-transaction",
    ] {
        std::fs::write(
            hooks.join(name),
            format!("#!/bin/sh\necho {name} >> \"{shown}\"\nexit 0\n"),
        )
        .expect("hook");
        // POSIX git runs a hook only if it is executable (it warns "ignored because it's not set
        // as executable" and skips it); Windows git has no exec bit to read.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(hooks.join(name), std::fs::Permissions::from_mode(0o755))
                .expect("chmod hook");
        }
    }
    lx.git(&["config", "core.hooksPath", &hooks.display().to_string()]);

    // CONTROL: the hooks bite a plain ref move on this box.
    let tip = lx.tip();
    lx.fx.git(
        &lx.repo,
        &[
            "-c",
            &format!("core.hooksPath={}", hooks.display()),
            "update-ref",
            "refs/heads/control",
            &tip,
        ],
    );
    assert!(
        marker.exists(),
        "CONTROL: a plain `update-ref` did not run the reference-transaction hook, so the \
         fixture cannot see a hook at all"
    );
    std::fs::remove_file(&marker).expect("reset the marker");

    lx.write("hooked.txt", b"a landing\n");
    let msg = Message {
        subject: "land-1 enters review".to_owned(),
        body: None,
        trailer: ParsedTrailer::Gate {
            token: "gate-tok".to_owned(),
            tree_hash: "a".repeat(40),
            deadline: 1_900_000_000,
            kind: "k".to_owned(),
            reach: 1,
            state: GateState::Opened,
        }
        .to_trailer(),
    };
    let landed = land(&lx, &empty_baseline(), &msg, true);
    assert!(
        matches!(landed, Landed::Commit { content: true, .. }),
        "PREMISE: the landing committed: {landed:?}"
    );
    let ran = std::fs::read_to_string(&marker).unwrap_or_default();
    assert!(
        ran.is_empty(),
        "a repository hook ran during the landing: {ran}"
    );
}
