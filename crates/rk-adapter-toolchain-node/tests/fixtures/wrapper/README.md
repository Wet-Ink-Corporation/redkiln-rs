# AC-325's argument-dropping wrapper — where it is

`s7c` §7c.17 places "the AC-325 argument-dropping wrapper" under this crate's `project/` fixture. It
is not there. It is **one compiled, std-only program**, whose source lives in the Rust crate —
`crates/rk-adapter-toolchain-rust/tests/fixtures/wrapper/rk-argv-wrapper.rs` — declared `[[bin]]` in
**both** toolchain crates under two names (`rk-argv-wrapper-rust`, `rk-argv-wrapper-node`; this
crate's `Cargo.toml` names the Rust crate's file by relative path), and reached by each crate's
tests through `env!("CARGO_BIN_EXE_<name>")`.

Why not the `.cmd` script §7c.17 sketched: `cmd.exe` splits a batch file's arguments on `=` as well
as on whitespace, so `--reporter=json` and `--outputFile=<f>` each reach a batch wrapper as TWO
elements [measured 2026-09-18 — the wrapper source's header carries the reading], and a wrapper
that cannot see the vector it was handed cannot log it or drop its tail faithfully. Why two `[[bin]]`
names from one source: two packages declaring one bin name compile to one output path and race the
linker (§7c.17's reason for `rk-adapter-claude`'s second `[[bin]]`).

This location and the two `[[bin]]` rows are a deviation from §7c.17 and §7c.19 as written, named in
the Slice 6 commit body for the stage owner to amend.
