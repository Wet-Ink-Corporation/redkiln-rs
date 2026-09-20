// POSITIVE CONTROL for AC-426's "a required key, no default" (`s7d` §7d.14's self-control rule).
//
// Never compiled, never linked. `ac426_unread_window_runs_required_no_default` scans
// `crates/rk-cli/src` for the three spellings that would re-introduce a default behind the type's
// back, and §7d.14 binds it to run that scan over THIS file first: "Every in-process source-scan
// test is self-controlled: it runs its scan over its `lint-controls/` file and asserts >= 1 hit
// BEFORE asserting zero hits over `src/`, so a scanner that visits nothing is red."
//
// The three spellings the fixtures cannot see. A missing key caught by serde is one defect; a
// missing key silently filled in is the same defect one indirection later, and no fixture can
// distinguish them — which is why the scan exists beside the fixtures rather than instead of them.

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            unread_window_runs: 50,
        }
    }
}

fn also_a_default(raw: Option<u32>) -> u32 {
    raw.unwrap_or(50)
}

#[serde(default)]
struct AndAThird {
    unread_window_runs: u32,
}
