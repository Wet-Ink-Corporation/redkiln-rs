//! The source parse AC-481's amended GIVEN requires, shared by every source-derived assertion in
//! this suite.
//!
//! **Clause 1 of the amendment**: the trait set and the fake set are derived from *"a parse of
//! `rk-ports`' own source text"* and **never** from a hand-maintained inventory list. That is why
//! this walks `CARGO_MANIFEST_DIR/src` recursively at test time rather than `include_str!`-ing a
//! file list: an `include_str!` list is itself an inventory a fixture would have to edit, which is
//! precisely what the clause forbids and what PIA-059 caught. A `build.rs` is third-best — its
//! failure is a build error, not a red test.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// Everything the parse can see, from one walk.
pub struct Source {
    /// Every `pub trait <Name>` declared at column 0.
    pub traits: BTreeSet<String>,
    /// Every `impl <Name> for <Type>` at column 0 — **not** yet intersected with `traits`.
    pub impls: BTreeSet<String>,
    /// Every trait-method DECLARATION (an indented `fn …;` with no body).
    pub methods: BTreeSet<String>,
    /// Every non-comment source line, with `path:line` for a message that can be chased.
    pub lines: Vec<(String, usize, String)>,
}

fn src_files() -> Vec<PathBuf> {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));
    let mut found = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).expect("rk-ports/src is readable");
        for entry in entries {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                found.push(path);
            }
        }
    }
    // THE FILE-SET FLOOR, and it guards ONE level: that the walk read something. It does NOT
    // close the vacuity hole it used to claim -- a walk that reads every file and parses no trait
    // out of them leaves every set below empty, `traits.len() == fakes.len()` reads `0 == 0`, and
    // the denylist finds nothing to refuse [measured: respelling the trait prefix `pub  trait`
    // left `port_fake_parity` GREEN with a non-empty file set]. The floor that closes that is on
    // the parsed SETS, in `Source::parse` below, and again in `port_fake_parity` itself.
    assert!(
        !found.is_empty(),
        "the source parse read no .rs files under rk-ports/src"
    );
    found
}

impl Source {
    /// Parse the crate's own `src/`.
    pub fn parse() -> Self {
        let mut traits = BTreeSet::new();
        let mut impls = BTreeSet::new();
        let mut methods = BTreeSet::new();
        let mut lines = Vec::new();

        for path in src_files() {
            let text = std::fs::read_to_string(&path).expect("a readable source file");
            let name = path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("?")
                .to_owned();
            for (i, line) in text.lines().enumerate() {
                // TRAP 3: strip comment lines FIRST. A line-wise match on `impl <Trait> for`
                // counts a doc-comment line as a fake -- `spikes/s-ports/src/lib.rs`:16 is
                // `//! impl Harness for NotSendHarness {` inside a compile_fail block -- and a
                // parse that counts fakes that are comments passes while the crate is wrong.
                if line.trim_start().starts_with("//") {
                    continue;
                }
                lines.push((name.clone(), i + 1, line.to_owned()));

                // TRAIT DECLARATIONS ARE MATCHED ON THE TRIMMED LINE, NOT AT COLUMN 0, and the
                // reason is measured: a real, publicly reachable sixth port declared inside a
                // nested `pub mod` in an existing port file -- `pub mod extra { pub trait Sixth:
                // Send + Sync { … } }` -- is invisible to a column-0 anchor, and with it every
                // trait-set instrument went green over a crate carrying six ports and five fakes
                // (parity, conformance, and `traits.len() == 5` all passed). Trimmed, the same
                // declaration is counted, and because the `impl` match below stays anchored at
                // column 0 an indented port with no column-0 fake fails `port_fake_parity` NAMING
                // it. The `impl` anchor is deliberate and is a different rule (Trap 3): it is what
                // keeps a `#[cfg(test)]` stub, which is always indented, out of the fake set.
                let declaration = line.trim_start();
                if let Some(rest) = declaration.strip_prefix("pub trait ")
                    && let Some(head) = rest.split(':').next()
                {
                    traits.insert(head.trim().to_owned());
                }
                if let Some(rest) = line.strip_prefix("impl ")
                    && let Some((head, _)) = rest.split_once(" for ")
                {
                    impls.insert(head.trim().to_owned());
                }
                let trimmed = line.trim();
                if let Some(rest) = trimmed.strip_prefix("fn ")
                    && trimmed.ends_with(';')
                    && let Some((head, _)) = rest.split_once('(')
                {
                    methods.insert(head.trim().to_owned());
                }
            }
        }

        // THE PARSED-SET FLOOR, which is the one the file-set floor above does not reach. Every
        // assertion built on this parse -- the parity count, the git denylist, the async denylist,
        // the conformance matrix -- has a pass condition that is satisfied by an EMPTY set, so a
        // parse that silently stops matching turns four tests green at once. It is asserted here,
        // once, rather than in each consumer, because a consumer that forgets it is exactly the
        // failure [measured: `pub  trait` with a double space left `port_fake_parity` green].
        assert!(
            !traits.is_empty() && !impls.is_empty(),
            "the source parse read {} non-comment lines and found {} trait declarations and {} \
             impls -- a parse that matches nothing is not a clean crate (Halt 8 shape (g))",
            lines.len(),
            traits.len(),
            impls.len()
        );

        Self {
            traits,
            impls,
            methods,
            lines,
        }
    }

    /// The fake set: the parsed impls **intersected with the parsed trait set**.
    ///
    /// TRAP 4, and it fails RED on a correct crate without this line. A line-wise
    /// `impl <Name> for` parse inserts EVERY trait impl into the fake set, not only the five
    /// ports -- `impl Debug for FakeHarness`, `impl Default for VecStore`, any `From`. Then
    /// `fakes.len()` exceeds `traits.len()`, the test goes red, and the message names nothing:
    /// AC-481's AND clause broken in the FAILING direction. With the intersection the count
    /// equality and the by-name set say the same thing and cannot disagree.
    pub fn fakes(&self) -> BTreeSet<String> {
        self.impls.intersection(&self.traits).cloned().collect()
    }
}

/// **The `Toolchain` port declares exactly seven methods** (record V = ADR-0170 decision 2,
/// proposed: *"The port has **seven** methods"*). Checkpoint 7a counts none -- it floors two NAMES
/// (`s7a` §7a.3: *"The floor pins **names**, never a signature"*) -- so this is the exact count. It
/// parses `pub trait Toolchain` from CODE lines of `src/toolchain.rs` (comment lines dropped first,
/// so a commented-out method is not counted) and names each method.
///
/// *(Added 2026-09-26, s8c §8c.9.)*
#[test]
fn toolchain_port_declares_exactly_seven_methods() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/toolchain.rs"))
        .expect("rk-ports/src/toolchain.rs is readable");
    let code: Vec<&str> = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect();
    let start = code
        .iter()
        .position(|l| l.starts_with("pub trait Toolchain"))
        .expect("`pub trait Toolchain` is declared at column 0 on a code line");
    let end = code[start..]
        .iter()
        .position(|l| *l == "}")
        .map(|i| start + i)
        .expect("the trait body closes at column 0");
    let names: Vec<String> = code[start + 1..end]
        .iter()
        .filter_map(|l| l.trim_start().strip_prefix("fn "))
        .filter_map(|rest| rest.split(['(', '<']).next())
        .map(|n| n.trim().to_owned())
        .collect();
    assert_eq!(
        names,
        [
            "detect",
            "build",
            "test",
            "lint",
            "classify",
            "change_summary",
            "run_verb"
        ],
        "pub trait Toolchain declares exactly these seven methods, in this order (record V)"
    );
}
