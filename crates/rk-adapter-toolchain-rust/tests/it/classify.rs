//! **AC-497 (classify half) — PROVISIONAL (S-BIN unrun).** *"both Phase-1 Toolchain adapters …
//! implement `classify` and leave `change_summary`'s body to S-BIN"* (s7c §7c.14). `classify` is
//! per path (design:269; ADR-0029 (i): *"computed per item from affected paths via
//! `Toolchain::classify`, never per repo"*) and answers honestly for non-Rust paths — a
//! `spikes/*.py` is `Text`, not a claim about Rust.

use rk_ports::toolchain::{Classification, Toolchain};

use crate::support::{GENEROUS, adapter};

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
