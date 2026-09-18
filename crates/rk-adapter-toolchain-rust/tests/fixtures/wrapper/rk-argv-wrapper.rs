//! AC-325's wrapper fixture — the spawnable program each toolchain crate's `ac325_…` puts in the
//! adapter's `program` value (`s7c` §7c.14: *"a fixture wrapper that drops its trailing arguments,
//! which must be visible in the record"*; *"an adapter test asserting the recorded argv differs
//! from the template when a wrapper rewrites it"*; *"a re-execution test on the recorded vector
//! asserting the same test count"*).
//!
//! **Why a compiled bin and not the `.cmd` script §7c.17 sketched:** `cmd.exe` parses a batch
//! file's `%1`…`%9` / `shift` arguments on `=` as well as on whitespace, so
//! `--reporter=json` reaches a batch wrapper as TWO elements (`--reporter`, `json`) and
//! `--outputFile=<f>` as two more [measured 2026-09-18 on this box: the logged argv read
//! `--reporter` / `json` / `--outputFile` / `<f>`]. A wrapper that cannot see the vector it was
//! handed cannot log it or drop its tail faithfully; a `std` program sees `args_os()` exactly.
//! Declared `[[bin]]` in **both** toolchain crates from this ONE source file under two names
//! (`rk-argv-wrapper-rust`, `rk-argv-wrapper-node`) for the reason `rk-adapter-claude`'s second
//! `[[bin]]` states (§7c.17: one bin name in two packages is one output path and a link race).
//! Tests reach their own copy through `env!("CARGO_BIN_EXE_<name>")`, never `std::env::var`.
//!
//! **What it does, in order:** append to the file named by `RK_ARGV_LOG` in its environment — the
//! adapter's constructed child map, into which the test put the name — its own path
//! (`current_exe`), then every argument it received, one per line, then a `---` line; then run the
//! program named by `RK_WRAPPED_PROGRAM` (`cargo` / `npx.cmd`, resolved through the map's `PATH`,
//! which the test's map carries) with the arguments per `RK_WRAPPER_MODE`: `forward` (every
//! argument, in order) or `drop-tail:<n>` (the first `n` only — the Node side's `-t <pattern>` is
//! what falls off a `drop-tail:5`, so a filtered run through the wrapper executes the whole file
//! and the count says so); exit with the wrapped program's status. **Its environment reads are its
//! own edge**, outside every `src/` ban (as `rk-fixture-child`'s are); it uses `std` only and no
//! `unsafe`. Its one `Command` site carries the per-site `#[expect]` the workspace lint posture
//! requires (ADR-0102 decision 3); the spawn is outside `src/` and is AC-325's instrument, not a
//! violation of ADR-0127's forbids (the record's own wording, `s7c` §7c.17).

use std::ffi::OsString;
use std::io::Write as _;

fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let log = std::env::var_os("RK_ARGV_LOG").expect("RK_ARGV_LOG names the argv log file");
    {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
            .expect("the argv log opens for append");
        let me = std::env::current_exe().expect("current_exe");
        writeln!(f, "{}", me.display()).expect("write");
        for a in &args {
            writeln!(f, "{}", a.to_string_lossy()).expect("write");
        }
        writeln!(f, "---").expect("write");
    }
    let program =
        std::env::var_os("RK_WRAPPED_PROGRAM").expect("RK_WRAPPED_PROGRAM names the real program");
    let mode = std::env::var("RK_WRAPPER_MODE").unwrap_or_else(|_| "forward".to_owned());
    let forwarded: Vec<OsString> = match mode.strip_prefix("drop-tail:") {
        Some(n) => {
            let keep: usize = n.parse().expect("drop-tail:<n> with a decimal n");
            args.into_iter().take(keep).collect()
        }
        None => args,
    };
    let status = spawn_wrapped(&program, &forwarded);
    std::process::exit(status.code().unwrap_or(1));
}

/// The one spawn: the wrapped program, its environment inherited from this wrapper's (the
/// adapter's constructed map), stdio inherited (the adapter's files).
#[expect(
    clippy::disallowed_types,
    reason = "AC-325's wrapper fixture runs the real toolchain program it stands in front of; a \
              fixture bin outside src/ is the criterion's instrument (s7c §7c.17), and clippy.toml's \
              type ban has no path scoping, so the exemption is stated at the one site."
)]
fn spawn_wrapped(program: &std::ffi::OsStr, args: &[OsString]) -> std::process::ExitStatus {
    std::process::Command::new(program)
        .args(args)
        .status()
        .expect("the wrapped program spawns")
}
