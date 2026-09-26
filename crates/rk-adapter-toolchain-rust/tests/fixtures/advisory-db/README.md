# The scan fixture (s8c §8c.4)

A FIXTURE advisory database and the lockfile it matches, for the `sca_scan` verb's tests. Nothing
here is a real advisory.

- `db/` is a RustSec-layout database (`crates/<name>/RUSTSEC-*.md`, TOML front matter) holding two
  advisories against `itoa`: `RUSTSEC-2099-0001` carries a CVSS v3.1 vector (base score 10.0,
  `critical`), and `RUSTSEC-2099-0002` carries none (`unrated`).
- `project/` is its own workspace root with one registry dependency, `itoa = "=1.0.18"`, and a
  committed `Cargo.lock`. Both tools match advisories against registry packages only, so the matched
  crate cannot be a path package.

Measured 2026-09-26 on this box (s8c stage open), for record V's evidence:

- `cargo audit 0.22.2` reads `db/` as a plain directory with `--db <dir> --no-fetch`, and exits 1
  with the `itoa` findings. Its JSON `database` member is present, but under `--no-fetch` its
  `last-commit` and `last-updated` are `null`, whether or not the directory is a git checkout. The
  per-advisory `cvss` member is the vector string, or `null`.
- `cargo deny 0.20.2` refuses a directory that is not a git checkout: it runs
  `git show -s --format=%cI HEAD` in the database directory. A test therefore copies `db/` into a
  temporary directory and commits it there (`git init`, `git add -A`, `git commit`) before a
  `cargo deny` run. It is offline under the top-level `--offline` flag (0.20.2 has no
  `check --disable-fetch`). Its `--audit-compatible-output` JSON carries no `database` member.
- Everything here is LF under ADR-0164.
