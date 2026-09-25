//! **AC-497 (classify half) — PROVISIONAL (S-BIN unrun).** *"both Phase-1 Toolchain adapters …
//! implement `classify` and leave `change_summary`'s body to S-BIN"* (s7c §7c.14). `classify` is
//! per path (design:269; ADR-0029 (i): *"computed per item from affected paths via
//! `Toolchain::classify`, never per repo"*) and answers honestly for non-Rust paths — a
//! `spikes/*.py` is `Text`, not a claim about Rust.
//!
//! *(Added 2026-09-25, s8a, D21-51; ADR-0165 decision 9.)* `classify` is a function of the path and
//! the bytes at that path and spawns nothing (ADR-0029 (i); `s8c`'s *"it spawns nothing"*). Two
//! tests pin that: [`classify_is_invariant_to_a_cargo_manifest`] (a `Cargo.toml` beside the paths
//! moves no answer, so no `cargo metadata` read can be deciding) and
//! [`classify_source_spawns_nothing`] (a code-only lint over `src/classify.rs`).

use std::path::PathBuf;

use rk_adapter_toolchain_rust::{DEFAULT_PROGRAM, RustToolchain};
use rk_ports::toolchain::{Classification, Toolchain};

use crate::support::{GENEROUS, adapter, config_with, minimal_env};

/// The binary paths S-BIN's design fixture names (`.uasset` / `.umap`), a text source, a non-Rust
/// text source, the generated set, and an unknown extension decided by its bytes (both arms of
/// the NUL heuristic, over probes in the test's scratch — never a write into the fixture tree).
#[test]
fn ac497_classify_binary_paths() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let c = |p: &str| tc.classify(p).expect("classify never fails on a path");

    assert_eq!(c("Content/Maps/Lobby.umap"), Classification::Binary);
    assert_eq!(
        c("Content/Blueprints/BP_Door.uasset"),
        Classification::Binary
    );
    assert_eq!(c("assets/logo.png"), Classification::Binary);
    assert_eq!(c("src/lib.rs"), Classification::Text);
    assert_eq!(c("Cargo.toml"), Classification::Text);
    assert_eq!(
        c("spikes/p1/select-items.py"),
        Classification::Text,
        "a non-Rust text path is Text — honest, not a Rust claim (ADR-0029 i)"
    );
    assert_eq!(c("Cargo.lock"), Classification::Generated);
    assert_eq!(c("target/debug/deps/it-abc.d"), Classification::Generated);

    // An extension in NO table (`.unknownext` — `.bin` is in the binary table and would answer
    // from it before any byte is read) is decided by the bytes: NUL within the first block is
    // git's own binary heuristic; no NUL is Text. The probes live under the test's own scratch
    // and are classified by ABSOLUTE path (`classify` accepts one), never written into the
    // committed fixture tree.
    let nul_probe = scratch.path().join("classify-probe.unknownext");
    std::fs::write(&nul_probe, b"\x00\x01\x02binary").expect("write probe");
    assert_eq!(
        c(&nul_probe.to_string_lossy()),
        Classification::Binary,
        "a NUL-carrying file of unknown extension is Binary (the byte heuristic, not a table)"
    );
    let text_probe = scratch.path().join("classify-notes.unknownext");
    std::fs::write(&text_probe, b"plain prose, no NUL\n").expect("write probe");
    assert_eq!(
        c(&text_probe.to_string_lossy()),
        Classification::Text,
        "a NUL-free file of unknown extension is Text (the byte heuristic's other arm)"
    );
    assert_eq!(
        c("no-such-file.unknownext"),
        Classification::Text,
        "an absent path with an unknown extension is Text (the affected path may be a deletion)"
    );
}

/// The paths every `classify` branch answers, in the order [`classify_is_invariant_to_a_cargo_manifest`]
/// asks them, with the answer each branch gives.
const BRANCHES: &[(&str, Classification)] = &[
    ("src/lib.rs", Classification::Text),        // Text by extension
    ("Cargo.toml", Classification::Text),        // Text by extension, present in one root only
    ("blob.unknownext", Classification::Binary), // NUL-Binary, by the bytes on disk
    ("notes.unknownext", Classification::Text),  // Text, by the bytes on disk (no NUL)
    ("absent.unknownext", Classification::Text), // Text, no file (a deletion)
    ("Content/Hero.uasset", Classification::Binary), // Binary by extension
    ("target/debug/app", Classification::Generated), // Generated, under target/
    ("Cargo.lock", Classification::Generated),   // Generated, a lockfile
];

/// Write the same tree under `root`: every `BRANCHES` path but the absent one, `Cargo.toml` only
/// when `with_manifest`.
fn branch_tree(root: &std::path::Path, with_manifest: bool) {
    let write = |path: &str, bytes: &[u8]| {
        let full = root.join(path);
        std::fs::create_dir_all(full.parent().expect("a parent")).expect("mkdir");
        std::fs::write(full, bytes).expect("write");
    };
    write("src/lib.rs", b"pub fn one() -> u32 {\n    1\n}\n");
    write("blob.unknownext", b"\x00\x01\x02binary");
    write("notes.unknownext", b"plain prose, no NUL\n");
    write("Content/Hero.uasset", b"\x00uasset");
    write("target/debug/app", b"\x7fELF\x00");
    write("Cargo.lock", b"version = 4\n");
    if with_manifest {
        write(
            "Cargo.toml",
            b"[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        );
    }
}

/// `classify` over a tree carrying a `Cargo.toml` and over an identical tree without one answers
/// the same vector, one entry per branch (added 2026-09-25, s8a, D21-51; ADR-0165 decision 9): the
/// answer is a function of the path and the bytes at it, so no `cargo metadata` read, and no
/// manifest at all, can be deciding it. The expected vector is asserted first, so equal vectors
/// cannot be two equal wrong answers.
#[test]
fn classify_is_invariant_to_a_cargo_manifest() {
    let scratch = tempfile::tempdir().expect("scratch");
    let with = tempfile::tempdir().expect("root with Cargo.toml");
    let without = tempfile::tempdir().expect("root without Cargo.toml");
    branch_tree(with.path(), true);
    branch_tree(without.path(), false);
    assert!(
        with.path().join("Cargo.toml").is_file() && !without.path().join("Cargo.toml").exists(),
        "PREMISE: the two roots differ by the manifest alone"
    );
    let over = |root: &std::path::Path| -> Vec<(String, Classification)> {
        let tc = RustToolchain::new(config_with(
            PathBuf::from(DEFAULT_PROGRAM),
            GENEROUS,
            root,
            minimal_env(),
            scratch.path(),
        ))
        .expect("the shipped registry loads");
        BRANCHES
            .iter()
            .map(|(path, _)| {
                (
                    (*path).to_owned(),
                    tc.classify(path).expect("classify never fails on a path"),
                )
            })
            .collect()
    };
    let expected: Vec<(String, Classification)> = BRANCHES
        .iter()
        .map(|(path, c)| ((*path).to_owned(), *c))
        .collect();
    let a = over(with.path());
    assert_eq!(a, expected, "every branch answers its own classification");
    assert_eq!(
        over(without.path()),
        a,
        "a Cargo.toml beside the paths moves no classification"
    );
}

/// The code of `src` with every comment, string literal and char literal removed, and every
/// whitespace character dropped, so a lint over it sees code only and cannot be satisfied or
/// broken by prose (added 2026-09-25, s8a, D21-51).
fn code_only(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let ident_before = |out: &String| {
        let mut rev = out.chars().rev();
        match rev.next() {
            Some('b') => rev.next().is_some_and(|c| c.is_alphanumeric() || c == '_'),
            Some(c) => c.is_alphanumeric() || c == '_',
            None => false,
        }
    };
    while i < b.len() {
        let c = b[i];
        let next = b.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && next == Some('*') {
            let mut depth = 1;
            i += 2;
            while i < b.len() && depth > 0 {
                if b[i] == '/' && b.get(i + 1) == Some(&'*') {
                    depth += 1;
                    i += 2;
                } else if b[i] == '*' && b.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if c == 'r' && matches!(next, Some('"' | '#')) && !ident_before(&out) {
            let mut j = i + 1;
            let mut hashes = 0;
            while b.get(j) == Some(&'#') {
                hashes += 1;
                j += 1;
            }
            if b.get(j) == Some(&'"') {
                j += 1;
                while j < b.len() {
                    if b[j] == '"' && (0..hashes).all(|k| b.get(j + 1 + k) == Some(&'#')) {
                        j += 1 + hashes;
                        break;
                    }
                    j += 1;
                }
                out.push_str("\"\"");
                i = j;
                continue;
            }
        }
        if c == '"' {
            i += 1;
            while i < b.len() {
                if b[i] == '\\' {
                    i += 2;
                    continue;
                }
                if b[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str("\"\"");
            continue;
        }
        if c == '\'' {
            if next == Some('\\') {
                let mut j = i + 3;
                while j < b.len() && b[j] != '\'' {
                    j += 1;
                }
                i = j + 1;
                out.push_str("' '");
                continue;
            }
            if b.get(i + 2) == Some(&'\'') {
                i += 3;
                out.push_str("' '");
                continue;
            }
        }
        if !c.is_whitespace() {
            out.push(c);
        }
        i += 1;
    }
    out
}

/// What a `classify` body that spawns, or hands a child to the bounded runner, must write.
const SPAWN_ROUTES: &[&str] = &[
    "Command::new",
    ".spawn(",
    ".output(",
    ".status(",
    "std::process",
    "process::",
    "run_program",
    "run_bounded",
];

/// The spawn routes present in `src`'s code.
fn spawn_routes(src: &str) -> Vec<&'static str> {
    let code = code_only(src);
    SPAWN_ROUTES
        .iter()
        .copied()
        .filter(|route| code.contains(route))
        .collect()
}

/// `src/classify.rs` names no spawn route on a code line (added 2026-09-25, s8a, D21-51; ADR-0165
/// decision 9 and its forbids bullet): `classify` reads the path and the bytes it is handed and
/// runs nothing, `cargo metadata` included. Comments and string literals are stripped first, so a
/// mention in prose neither satisfies nor breaks the lint; the controls prove the stripper both
/// ways before the real file is read.
#[test]
fn classify_source_spawns_nothing() {
    // CONTROLS: real code lines fire, whatever their spacing, including after a `'"'` char
    // literal that a naive stripper would read as opening a string.
    for (planted, route) in [
        (
            "fn f() { let _ = std::process::Command::new(\"cargo\"); }",
            "Command::new",
        ),
        ("fn f(c: C) { c . spawn ( ) ; }", ".spawn("),
        (
            "fn f() { let d = '\"'; let _ = Command::new(\"x\"); }",
            "Command::new",
        ),
        ("fn f() { let d = '\\\\'; x.output(); }", ".output("),
        (
            "fn f() { rk_containment::run_bounded(c, t); }",
            "run_bounded",
        ),
    ] {
        assert!(
            spawn_routes(planted).contains(&route),
            "CONTROL: {route} on a code line must fire: {planted}"
        );
    }
    // CONTROLS: prose and literals do not fire.
    for planted in [
        "// Command::new(\"cargo\") .spawn( std::process",
        "/// run_bounded and run_program are the runner's",
        "/* .output( /* nested .status( */ std::process */ fn f() {}",
        "fn f() { let s = \"Command::new(\\\"cargo\\\") .spawn(\"; }",
        "fn f() { let r = r#\"std::process::Command::new(\"cargo\")\"#; }",
        "fn f<'a>(x: &'a str) -> char { let _ = x; '\\'' }",
    ] {
        assert!(
            spawn_routes(planted).is_empty(),
            "CONTROL: a comment or literal must not fire: {planted} -> {:?}",
            spawn_routes(planted)
        );
    }

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/classify.rs");
    let src = std::fs::read_to_string(path).expect("read src/classify.rs");
    let code = code_only(&src);
    assert!(
        code.contains("pubfnclassify(") && code.contains("pubfnclassify_by_path("),
        "PREMISE: the lint reads the real classify module's code, not an empty or wrong file"
    );
    assert_eq!(
        spawn_routes(&src),
        Vec::<&str>::new(),
        "src/classify.rs names a spawn route on a code line; classify is a function of the path \
         and the bytes it is handed and spawns nothing (ADR-0165 decision 9, D21-51)"
    );
}
