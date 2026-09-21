//! `legacy_mini_provenance_matches_bytes` — D18-10, AC-465, and the Phase-1 evidence for AC-443
//! and AC-445 (both PROVISIONAL, §7d.18).

use std::collections::BTreeMap;

use crate::legacy_mini_dir;

/// The note's own filename — **the one file its table does not list**, and necessarily so: a note
/// cannot carry its own sha256. Everything else under the fixture must appear.
const NOTE: &str = "PROVENANCE.md";

/// Parse the note's `| `path` | bytes | `sha256` |` rows.
fn listed(note: &str) -> BTreeMap<String, (u64, String)> {
    let mut out = BTreeMap::new();
    for line in note.lines() {
        let line = line.trim();
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 3 {
            continue;
        }
        let path = cells[0].trim_matches('`').to_owned();
        let Ok(bytes) = cells[1].parse::<u64>() else {
            continue;
        };
        out.insert(path, (bytes, cells[2].trim_matches('`').to_owned()));
    }
    out
}

/// Every fixture file's sha256 and byte count match `PROVENANCE.md`, and the note lists every file
/// under the fixture.
///
/// # What this CANNOT do, stated because the ledger depends on it
///
/// It pins the note's **self-consistency**, not the absence of predecessor bytes. It cannot fail on
/// a copied predecessor file whose sha256 the note happens to list — which is exactly why neither
/// AC-443 nor AC-445 is discharged on it (§7d.18 labels both PROVISIONAL). AC-443's real instrument
/// is a content-hash comparison against the old tree at `47faec9f`, which is local-only.
#[test]
fn legacy_mini_provenance_matches_bytes() {
    let root = legacy_mini_dir();
    let note_path = root.join(NOTE);
    let note = std::fs::read_to_string(&note_path)
        .unwrap_or_else(|e| panic!("{} must exist: {e}", note_path.display()));

    let listed = listed(&note);
    assert!(
        !listed.is_empty(),
        "PROVENANCE.md lists no files at all — the re-digest below would compare nothing and \
         report a pass (Halt 8 shape (a))"
    );

    // Walk the fixture, not the note: a file the note omits must be caught.
    let mut on_disk = BTreeMap::new();
    let mut stack = vec![root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the fixture directory") {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(&root)
                    .expect("under the fixture root")
                    .to_string_lossy()
                    .replace('\\', "/");
                on_disk.insert(rel, std::fs::read(&path).expect("a fixture file"));
            }
        }
    }

    let unlisted: Vec<&String> = on_disk
        .keys()
        .filter(|rel| rel.as_str() != NOTE && !listed.contains_key(*rel))
        .collect();
    assert!(
        unlisted.is_empty(),
        "these files are under legacy-mini/ but PROVENANCE.md does not list them — a fixture file \
         with no recorded digest is exactly what the note exists to prevent:\n{unlisted:#?}"
    );

    let absent: Vec<&String> = listed
        .keys()
        .filter(|rel| !on_disk.contains_key(*rel))
        .collect();
    assert!(
        absent.is_empty(),
        "PROVENANCE.md lists files that are not on disk:\n{absent:#?}"
    );

    assert!(
        !listed.contains_key(NOTE),
        "PROVENANCE.md lists itself — a note cannot carry its own sha256, and a row for it would \
         be red the moment the row was written"
    );

    let mut drift = Vec::new();
    for (rel, bytes) in &on_disk {
        if rel.as_str() == NOTE {
            continue;
        }
        let (want_len, want_sha) = &listed[rel];
        let got_sha = rk_types::digest::sha256(bytes).as_str().to_owned();
        if bytes.len() as u64 != *want_len || &got_sha != want_sha {
            drift.push(format!(
                "{rel}: note says {want_len} bytes / {want_sha}, disk has {} bytes / {got_sha}",
                bytes.len()
            ));
        }
    }
    assert!(
        drift.is_empty(),
        "fixture bytes have drifted from PROVENANCE.md:\n{}",
        drift.join("\n")
    );

    // The counts the contract took off §7d.13, asserted here so a fixture that silently loses a
    // file is red on the number and not only on the digest.
    assert_eq!(
        listed.len(),
        23,
        "PROVENANCE.md lists {} content files, not the twenty-three §7d.13 spells",
        listed.len()
    );

    // AC-465's residency half, over the fixture itself.
    let corpus_shaped: Vec<&String> = on_disk
        .keys()
        .filter(|rel| rel.ends_with(".jsonl") || rel.starts_with("spikes/"))
        .collect();
    assert!(
        corpus_shaped.is_empty(),
        "legacy-mini carries corpus-shaped paths (AC-465):\n{corpus_shaped:#?}"
    );
}
