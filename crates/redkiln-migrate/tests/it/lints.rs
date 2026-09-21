//! AC-442's provenance lint and AC-474's clean break — the in-process twins of `ci.yml`'s two
//! Slice-5 steps.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::{code_lines, lint_controls_dir, migrate_bin, rs_files, src_dir};

/// The literals `ci.yml`'s AC-442 step and both tests below search for.
///
/// The same set, spelled once: `template_sig`, `rendered_sig`, `yaml_serde` and a
/// `processes/*.yaml` path. A scanner and a shell step that disagree about their pattern are two
/// instruments, not one.
const LEGACY_TOKENS: [&str; 3] = ["template_sig", "rendered_sig", "yaml_serde"];

/// Whether a line names a legacy-format site.
fn names_a_legacy_format(line: &str) -> bool {
    LEGACY_TOKENS.iter().any(|t| line.contains(t))
        || (line.contains("processes/") && (line.contains(".yaml") || line.contains(".yml")))
}

/// The ONE allowlisted directory: the migrator's legacy module.
///
/// Layout-neutral — `legacy.rs` beside `legacy/` would satisfy it too, because §7d.1 leaves module
/// file names to the implementer.
fn is_allowlisted(rel: &str) -> bool {
    rel.starts_with("crates/redkiln-migrate/src/legacy/")
        || rel == "crates/redkiln-migrate/src/legacy.rs"
}

/// Scan a tree of `crates/*/src/**/*.rs` for legacy-format sites outside the allowlist.
///
/// Returns `"<rel path>:<line>: <content>"` per hit. **`src/` only, never `tests/`** — that is
/// `ci.yml`'s own scope, and it matters: `rk-cli`'s `lint-controls/ac442_template_sig_control.rs`
/// carries `template_sig` by design and this crate's own `tests/it/` names both literals in order
/// to assert their absence, so a scan over all of `crates/` is red on a correct tree.
fn scan(root: &Path) -> Vec<String> {
    let crates = root.join("crates");
    assert!(
        crates.is_dir(),
        "no crates/ under {} — the scan would read nothing and report a pass (Halt 8 shape (a))",
        root.display()
    );

    let mut sources: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(&crates).expect("crates/") {
        let krate = entry.expect("an entry").path();
        let src = krate.join("src");
        if src.is_dir() {
            sources.extend(rs_files(&src));
        }
    }
    assert!(
        !sources.is_empty(),
        "no crates/*/src/**/*.rs under {} — the scan read nothing",
        root.display()
    );

    let mut hits = Vec::new();
    for file in sources {
        let rel = file
            .strip_prefix(root)
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        if is_allowlisted(&rel) {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (n, line) in code_lines(&text) {
            if names_a_legacy_format(&line) {
                hits.push(format!("{rel}:{n}: {}", line.trim()));
            }
        }
    }
    hits.sort();
    hits
}

/// AC-442's TWO must-fail instruments.
///
/// *"a fixture PR that adds a legacy reader to `rk-store` and must fail; a fixture that widens the
/// detector to parse frontmatter and must fail."*
///
/// Both are built as a temp tree: the scan function is run over a synthetic
/// `crates/rk-store/src/` holding (a) `ac442_template_sig_control.rs` verbatim and (b) a second
/// copy that parses a `---` frontmatter fence. Each must be flagged.
///
/// **Instrument limit stands as written: the lint is SYNTACTIC.** It cannot tell a legacy reader
/// from a line that merely names the literal, which is why the allowlist is a path list and not a
/// judgement.
#[test]
fn ac442_lint_flags_legacy_reader_outside_legacy_module() {
    let control = lint_controls_dir().join("ac442_template_sig_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the control {} must exist: {e}", control.display()));

    // The control must carry the token on a NON-COMMENT line, or both instruments below are
    // satisfied by prose.
    let control_hits = code_lines(&control_text)
        .into_iter()
        .filter(|(_, l)| names_a_legacy_format(l))
        .count();
    assert!(
        control_hits >= 1,
        "the control {} carries no NON-COMMENT line naming a legacy format — the scan would be \
         asserted by a scanner that has not been seen to find anything",
        control.display()
    );

    let tree = tempfile::tempdir().expect("a temp tree");
    let synthetic = tree.path().join("crates").join("rk-store").join("src");
    std::fs::create_dir_all(&synthetic).expect("the synthetic rk-store src");

    // (a) the legacy reader, at a path that is NOT the migrator's legacy module.
    std::fs::write(
        synthetic.join("ac442_template_sig_control.rs"),
        control_text.as_bytes(),
    )
    .expect("instrument (a) writes");

    // (b) the detector WIDENED to parse frontmatter — AC-442's second fixture.
    std::fs::write(
        synthetic.join("old_layout_detector.rs"),
        b"// A detector that reads a path is allowed. This one parses the FORMAT, which is not.\n\
          fn detect(doc: &str) -> bool {\n\
          \x20   let mut lines = doc.lines();\n\
          \x20   if lines.next() != Some(\"---\") {\n\
          \x20       return false;\n\
          \x20   }\n\
          \x20   lines.any(|l| l.starts_with(\"template_sig:\"))\n\
          }\n",
    )
    .expect("instrument (b) writes");

    let hits = scan(tree.path());

    for (label, needle) in [
        (
            "(a) a legacy reader added to rk-store",
            "ac442_template_sig_control.rs",
        ),
        (
            "(b) a detector widened to parse frontmatter",
            "old_layout_detector.rs",
        ),
    ] {
        assert!(
            hits.iter().any(|h| h.contains(needle)),
            "the scan did NOT flag {label} at crates/rk-store/src/{needle} — a lint that misses \
             its own must-fail fixture forbids nothing. Hits:\n{}",
            hits.join("\n")
        );
    }
}

/// AC-442, and AC-471's write half.
///
/// Every hit for a legacy format across `crates/*/src/**/*.rs` lies under
/// `crates/redkiln-migrate/src/legacy/` — **self-controlled first**, as every source-scan test in
/// this file is.
#[test]
fn ac442_legacy_readers_only_under_legacy_module() {
    // (a) THE CONTROL: the scanner fires over the lint-control file.
    let control = lint_controls_dir().join("ac442_template_sig_control.rs");
    let control_text = std::fs::read_to_string(&control).expect("the control exists");
    assert!(
        code_lines(&control_text)
            .into_iter()
            .any(|(_, l)| names_a_legacy_format(&l)),
        "the control carries no non-comment legacy-format line — the ban below is vacuous"
    );

    // (b) THE FILTER IS ITSELF CONTROLLED. Three guards on this tree have passed on their own
    // prose; a scan whose comment filter is asserted only by the absence it reports is one of them
    // waiting to happen.
    for token in LEGACY_TOKENS {
        let prose = format!("//! a header naming {token}\n// and a line comment: {token}\n");
        assert_eq!(
            code_lines(&prose)
                .into_iter()
                .filter(|(_, l)| names_a_legacy_format(l))
                .count(),
            0,
            "`code_lines` returned a COMMENT line for {token}, so this scan counts prose"
        );
        let code = format!("{prose}let key = \"{token}\";\n");
        assert_eq!(
            code_lines(&code)
                .into_iter()
                .filter(|(_, l)| names_a_legacy_format(l))
                .count(),
            1,
            "`code_lines` dropped the CODE line for {token}, so this scan sees nothing"
        );
    }

    // (c) THE ALLOWLIST IS NOT VACUOUS: the module it names exists and actually reads the formats.
    let legacy = src_dir().join("legacy");
    assert!(
        legacy.is_dir(),
        "there is no legacy module under the migrator's src — the allowlist excludes nothing and \
         the ban below would be trivially satisfied"
    );
    let legacy_hits: usize = rs_files(&legacy)
        .iter()
        .map(|f| {
            let text = std::fs::read_to_string(f).expect("a legacy source file");
            code_lines(&text)
                .into_iter()
                .filter(|(_, l)| names_a_legacy_format(l))
                .count()
        })
        .sum();
    assert!(
        legacy_hits >= 1,
        "the allowlisted legacy module names NO legacy format — the exclusion excludes nothing"
    );

    // (d) THE BAN, over the real workspace.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root")
        .to_path_buf();
    let hits = scan(&root);
    assert!(
        hits.is_empty(),
        "legacy-format sites exist outside crates/redkiln-migrate/src/legacy/ (AC-442):\n{}",
        hits.join("\n")
    );

    // (e) AC-471's write half: `lift.rs` READS the literals through the legacy module and WRITES
    // neither. The two constants are declared under legacy/ and nothing outside it names them.
    let lift = std::fs::read_to_string(src_dir().join("lift.rs")).expect("lift.rs");
    for token in ["template_sig", "rendered_sig"] {
        let written: Vec<String> = code_lines(&lift)
            .into_iter()
            .filter(|(_, l)| l.contains(token))
            .map(|(n, l)| format!("lift.rs:{n}: {}", l.trim()))
            .collect();
        assert!(
            written.is_empty(),
            "lift.rs names the literal `{token}` — it must reach it only through \
             legacy::frontmatter's constant, and write it nowhere (AC-471):\n{}",
            written.join("\n")
        );
    }
}

/// AC-474's migrator half.
///
/// *"no configuration key, flag or environment variable enables a dual-write or
/// compatibility-write mode."* The flag set IS the snapshot, read off the real binary's `--help`,
/// and the environment half is a scan over `src/`.
#[test]
fn ac474_no_dual_write_key_flag_or_env() {
    // (a) THE FLAG SURFACE, read off the binary rather than off the source that declares it.
    #[expect(
        clippy::disallowed_types,
        reason = "a test reading the migrator's own --help to snapshot its flag surface; the \
                  PRODUCT spawns nothing but git, through rk-git's one GitExec wrapper"
    )]
    let help = std::process::Command::new(migrate_bin())
        .arg("--help")
        .stdin(Stdio::null())
        .output()
        .expect("--help runs");
    assert!(help.status.success(), "--help failed");
    let text = String::from_utf8_lossy(&help.stdout);

    let mut flags: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        let word = word.trim_end_matches([',', '.', ']', '>']);
        if word.starts_with("--") && word.len() > 2 {
            flags.push(word.to_owned());
        }
    }
    flags.sort();
    flags.dedup();
    assert!(
        !flags.is_empty(),
        "--help listed no flags at all — the snapshot below would assert nothing"
    );

    // `--help` is clap's own and is not a declared flag; the three declared ones are §7d.11's.
    let declared: Vec<&String> = flags.iter().filter(|f| f.as_str() != "--help").collect();
    assert_eq!(
        declared,
        vec!["--dry-run", "--target", "--version"],
        "the migrator's flag surface is not exactly {{--target, --dry-run, --version}}. A fourth \
         flag is how a second write path would arrive (AC-474), and this set is the snapshot"
    );

    // (b) THE ENVIRONMENT HALF: no read anywhere under src/, self-controlled first.
    let control = lint_controls_dir().join("adr0083_env_var_control.rs");
    let control_text = std::fs::read_to_string(&control)
        .unwrap_or_else(|e| panic!("the shared control {} must exist: {e}", control.display()));
    let spellings = ["std::env::var", "env::var(", "env::vars(", "env::var_os("];
    assert!(
        code_lines(&control_text)
            .into_iter()
            .any(|(_, l)| spellings.iter().any(|s| l.contains(s))),
        "the control {} carries no non-comment environment read — the ban below is vacuous. It is \
         SHARED with rk-cli's own ban, which is why §7d.13 counts eighteen controls for nineteen \
         rk_absent lines",
        control.display()
    );

    let mut env_hits = Vec::new();
    for file in rs_files(&src_dir()) {
        let source = std::fs::read_to_string(&file).expect("a source file");
        for (n, line) in code_lines(&source) {
            if spellings.iter().any(|s| line.contains(s)) {
                env_hits.push(format!("{}:{n}: {}", file.display(), line.trim()));
            }
        }
    }
    assert!(
        env_hits.is_empty(),
        "redkiln-migrate reads the environment — no env toggle may change its write path \
         (AC-474):\n{}",
        env_hits.join("\n")
    );

    // (c) `env!` is a COMPILE-TIME macro and is deliberately NOT banned: `report.rs` reads
    // CORPUS_PIN and CARGO_PKG_VERSION through it. A ban that caught `env!` would forbid the
    // compiled-in corpus pin, and a ban that caught nothing would forbid nothing — so the
    // distinction is asserted rather than left to the pattern's luck.
    let report = std::fs::read_to_string(src_dir().join("report.rs")).expect("report.rs");
    assert!(
        code_lines(&report).iter().any(|(_, l)| l.contains("env!(")),
        "report.rs no longer reads its compiled-in values through env!, so this distinction is \
         untested"
    );
    // `code_lines`, NOT the raw text. `report.rs`'s own header names `std::env::var` in order to
    // explain why `env!` is not one, so a raw `contains` fails on the prose that documents the
    // distinction — which is the recorded failure mode of a guard satisfied (or here, broken) by
    // its own comment. This assertion was written the wrong way first and caught by itself.
    let runtime_reads: Vec<String> = code_lines(&report)
        .into_iter()
        .filter(|(_, l)| spellings.iter().any(|s| l.contains(s)))
        .map(|(n, l)| format!("report.rs:{n}: {}", l.trim()))
        .collect();
    assert!(
        runtime_reads.is_empty(),
        "report.rs performs a RUNTIME environment read:\n{}",
        runtime_reads.join("\n")
    );

    // (d) the phrase sweep, over src/ and the manifest, in the block's own form.
    for (label, body) in [
        (
            "src/",
            rs_files(&src_dir())
                .iter()
                .map(|f| std::fs::read_to_string(f).expect("a source file"))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        (
            "Cargo.toml",
            std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
                .expect("the manifest"),
        ),
    ] {
        for line in body.lines() {
            let t = line.trim_start();
            if t.starts_with("//") || t.starts_with('#') {
                continue;
            }
            let lower = line.to_lowercase();
            for phrase in [
                "dual_write",
                "dual-write",
                "dual write",
                "compat_write",
                "compat-write",
            ] {
                assert!(
                    !lower.contains(phrase),
                    "{label} names `{phrase}` on a non-comment line (AC-474): {}",
                    line.trim()
                );
            }
        }
    }
}
