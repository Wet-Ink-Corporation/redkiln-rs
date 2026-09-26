//! The strict cargo-nextest status-line parser, pinned per nextest version.
//!
//! s8c §8c.1's nextest bullet; D21-48 X15: *"parses nextest's output beside `cargo test`'s"*; record T = ADR-0169
//! decision 4, proposed; D21-62 B's `ListFailed`, provisional).
//!
//! One pure function, [`parse`], beside [`crate::libtest_lines::parse`] and returning the same
//! [`Tally`]. It reads nextest's status stream, which on `0.9.146` is STDERR (stdout is empty
//! without `--nocapture`), run with `--no-fail-fast --retries 0 --color never --status-level pass
//! --final-status-level none`, so each executed test prints exactly one status line. Cargo's build
//! output and `build.rs`'s `cargo:warning=` lines share the stream, so every line outside the
//! grammar below is skipped. It STAYS STRICT: D21-62 A's foreign-line skip is the libtest path's
//! only. The grammar, MEASURED on `0.9.146` and written into record T before this parser (leading
//! spaces right-align a 12-column status field):
//!
//! - `START` `^\s+Starting (\d+) tests? across (\d+) binar(?:y|ies)(?: \((.+) skipped\))?$`
//! - `STATUS` `^\s+(PASS|FAIL) \[\s*[0-9.]+s\] \(\s*(\d+)/(\d+)\) (\S+) (.+)$`
//! - `SLOW` `^\s+SLOW \[>\s*[0-9.]+s\] \(\S+\) (\S+) (.+)$`: skipped, and it must name a
//!   (binary, test) the run records
//! - `REFUSE` `^\s+(?:TRY \d+ \S+|RETRY|FLAKY|TIMEOUT|TERMINATING|ABORT|SIGSEGV|SIGABRT|SIGKILL|SIGTERM|LEAK)\b`:
//!   refuses, because a retried test is two runs of one test and a timed-out one is a kill, not a
//!   failure (AC-331)
//! - `SUMMARY` `^\s+Summary \[\s*[0-9.]+s\] (\d+) tests? run: (\d+) passed(?: \((\d+) slow\))?(?:, (\d+) failed)?, (\d+) skipped$`;
//!   a line that starts like a `Summary` and does not match refuses
//!
//! The rules are N4's, carried over: [`NEXTEST_PINNED`] lists the versions with a golden and any
//! other is refused by name (capability `nextest-lines@<version>`); the one `Starting` count equals
//! the `PASS`/`FAIL` records; the `Summary`'s run, passed and failed equal the tally; exit 0 is
//! green and 100 a test failure, each agreeing with the records. *(D21-62 B.)* Exit 104 over a stream
//! carrying the pinned line `error: creating test list failed` and no status line is
//! [`ParseRefusal::ListFailed`]; any other exit than 0 or 100 refuses as
//! [`ParseRefusal::UnexpectedExit`] and is never tallied (an empty selection exits 4). *(s8c
//! refuting round 6.)* Exit 96 over the pinned line *"error: failed to parse test binary arguments
//! `<args>`: arguments are unsupported"* and no status line is [`ParseRefusal::ArgumentsRejected`],
//! naming the arguments nextest rejected: a selection argument is refused by name, never dropped
//! (record T = ADR-0169 decision 4).

use crate::libtest_lines::{ParseRefusal, Tally, digits, numbered, seconds_then};

/// The `cargo nextest --version` releases a golden exists for. The stage-open install is the pin
/// (D21-48 X15); a bump moves this list, `ci.yml`'s install line and a new golden in one PR.
pub const NEXTEST_PINNED: [&str; 1] = ["0.9.146"];

/// The pinned list-failure line (D21-62 B, measured on `0.9.146`).
const LIST_FAILED: &str = "error: creating test list failed";

/// The pinned rejected-argument line (s8c refuting round 6, measured on `0.9.146`, exit 96):
/// `error: failed to parse test binary arguments `<args>`: arguments are unsupported`.
fn rejected_args(line: &str) -> Option<&str> {
    line.strip_prefix("error: failed to parse test binary arguments `")?
        .strip_suffix("`: arguments are unsupported")
}

/// `^\s+` then the rest; `None` when the line does not start with whitespace.
fn indented(line: &str) -> Option<&str> {
    let rest = line.trim_start();
    (rest.len() < line.len()).then_some(rest)
}

/// `^\s+Starting (\d+) tests? across (\d+) binar(?:y|ies)(?: \((.+) skipped\))?$`: the count and the
/// binary count.
fn starting(line: &str) -> Option<(u32, u32)> {
    let (count, rest) = digits(indented(line)?.strip_prefix("Starting ")?)?;
    let rest = rest
        .strip_prefix(" tests across ")
        .or_else(|| rest.strip_prefix(" test across "))?;
    let (binaries, rest) = digits(rest)?;
    let rest = rest
        .strip_prefix(" binary")
        .or_else(|| rest.strip_prefix(" binaries"))?;
    let ok = rest.is_empty()
        || rest
            .strip_prefix(" (")
            .and_then(|r| r.strip_suffix(" skipped)"))
            .is_some_and(|m| !m.is_empty());
    ok.then_some((count, binaries))
}

/// `\[\s*[0-9.]+s\] ` (`prefix` = `"["`) or `\[>\s*[0-9.]+s\] ` (`prefix` = `"[>"`), and the rest.
fn elapsed<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = s
        .strip_prefix(prefix)?
        .trim_start_matches(char::is_whitespace);
    let close = rest.find("] ")?;
    seconds_then(&rest[..close], "s").then(|| &rest[close + 2..])
}

/// `(\S+) (.+)$`: the binary id and the test name.
fn binary_and_name(s: &str) -> Option<(&str, &str)> {
    let (binary, name) = s.split_once(' ')?;
    (!binary.is_empty() && !binary.contains(char::is_whitespace) && !name.is_empty())
        .then_some((binary, name))
}

/// `^\s+(PASS|FAIL) \[\s*[0-9.]+s\] \(\s*(\d+)/(\d+)\) (\S+) (.+)$`: pass or fail, binary, name.
fn status(line: &str) -> Option<(bool, &str, &str)> {
    let rest = indented(line)?;
    let (pass, rest) = if let Some(r) = rest.strip_prefix("PASS ") {
        (true, r)
    } else {
        (false, rest.strip_prefix("FAIL ")?)
    };
    let rest = elapsed(rest, "[")?;
    let rest = rest
        .strip_prefix('(')?
        .trim_start_matches(char::is_whitespace);
    let (_k, rest) = digits(rest)?;
    let (_of, rest) = digits(rest.strip_prefix('/')?)?;
    let (binary, name) = binary_and_name(rest.strip_prefix(") ")?)?;
    Some((pass, binary, name))
}

/// `^\s+SLOW \[>\s*[0-9.]+s\] \(\S+\) (\S+) (.+)$`: binary and name.
fn slow(line: &str) -> Option<(&str, &str)> {
    let rest = elapsed(indented(line)?.strip_prefix("SLOW ")?, "[>")?;
    let (counter, rest) = rest.strip_prefix('(')?.split_once(") ")?;
    if counter.is_empty() || counter.contains(char::is_whitespace) {
        return None;
    }
    binary_and_name(rest)
}

/// `^\s+(?:TRY \d+ \S+|RETRY|FLAKY|TIMEOUT|TERMINATING|ABORT|SIGSEGV|SIGABRT|SIGKILL|SIGTERM|LEAK)\b`
fn refused(line: &str) -> bool {
    const WORDS: [&str; 10] = [
        "RETRY",
        "FLAKY",
        "TIMEOUT",
        "TERMINATING",
        "ABORT",
        "SIGSEGV",
        "SIGABRT",
        "SIGKILL",
        "SIGTERM",
        "LEAK",
    ];
    let Some(rest) = indented(line) else {
        return false;
    };
    if let Some(r) = rest.strip_prefix("TRY ") {
        return digits(r)
            .and_then(|(_, r)| r.strip_prefix(' '))
            .is_some_and(|r| r.starts_with(|c: char| !c.is_whitespace()));
    }
    WORDS.iter().any(|w| {
        rest.strip_prefix(w)
            .is_some_and(|r| !r.starts_with(|c: char| c.is_alphanumeric() || c == '_'))
    })
}

struct Summary {
    run: u32,
    passed: u32,
    failed: u32,
    skipped: u32,
}

/// `^\s+Summary \[\s*[0-9.]+s\] (\d+) tests? run: (\d+) passed(?: \((\d+) slow\))?(?:, (\d+) failed)?, (\d+) skipped$`
fn summary(rest: &str) -> Option<Summary> {
    let rest = elapsed(rest.strip_prefix("Summary ")?, "[")?;
    let (run, rest) = digits(rest)?;
    let rest = rest
        .strip_prefix(" tests run: ")
        .or_else(|| rest.strip_prefix(" test run: "))?;
    let (passed, mut rest) = digits(rest)?;
    rest = rest.strip_prefix(" passed")?;
    if let Some(r) = rest.strip_prefix(" (") {
        let (_slow, r) = digits(r)?;
        rest = r.strip_prefix(" slow)")?;
    }
    let mut failed = 0;
    if let Some(r) = rest.strip_prefix(", ")
        && let Some((f, r2)) = digits(r)
        && let Some(r3) = r2.strip_prefix(" failed")
    {
        failed = f;
        rest = r3;
    }
    let (skipped, rest) = digits(rest.strip_prefix(", ")?)?;
    (rest == " skipped").then_some(Summary {
        run,
        passed,
        failed,
        skipped,
    })
}

/// Parse nextest's status stream under `version` (a `cargo nextest --version` release) and its exit
/// code.
///
/// # Errors
///
/// A [`ParseRefusal`] for an unpinned version, a list failure, a retry or kill line, or any breach
/// of the rules above; the caller maps it to `Transport { Decode }`, never to a green or red suite.
pub fn parse(stream: &str, exit_code: i32, version: &str) -> Result<Tally, ParseRefusal> {
    if !NEXTEST_PINNED.contains(&version) {
        return Err(ParseRefusal::Unsupported {
            capability: format!("nextest-lines@{version}"),
        });
    }
    let mut start: Option<(usize, u32, u32)> = None;
    let mut records: Vec<(bool, &str, &str)> = Vec::new();
    let mut notices: Vec<(usize, &str, &str)> = Vec::new();
    let mut total: Option<(usize, Summary)> = None;
    let mut list_failed = false;
    let mut rejected: Option<&str> = None;
    for (no, line) in numbered(stream) {
        if line == LIST_FAILED {
            list_failed = true;
        } else if let Some(args) = rejected_args(line) {
            rejected = Some(args);
        } else if refused(line) {
            return Err(ParseRefusal::RetryOrKill {
                line: no,
                text: line.to_owned(),
            });
        } else if let Some((count, binaries)) = starting(line) {
            if start.is_some() {
                return Err(ParseRefusal::StartingOutOfPlace { line: no });
            }
            start = Some((no, count, binaries));
        } else if let Some(record) = status(line) {
            if start.is_none() {
                return Err(ParseRefusal::StartingOutOfPlace { line: no });
            }
            records.push(record);
        } else if let Some((binary, name)) = slow(line) {
            notices.push((no, binary, name));
        } else if let Some(rest) = indented(line).filter(|r| r.starts_with("Summary ")) {
            let Some(s) = summary(rest) else {
                return Err(ParseRefusal::MalformedSummary { line: no });
            };
            if total.is_some() {
                return Err(ParseRefusal::MalformedSummary { line: no });
            }
            total = Some((no, s));
        }
    }
    if exit_code == 104 && list_failed && records.is_empty() {
        return Err(ParseRefusal::ListFailed {
            version: version.to_owned(),
            exit: exit_code,
        });
    }
    if exit_code == 96
        && records.is_empty()
        && let Some(args) = rejected
    {
        return Err(ParseRefusal::ArgumentsRejected {
            version: version.to_owned(),
            args: args.to_owned(),
        });
    }
    if exit_code != 0 && exit_code != 100 {
        return Err(ParseRefusal::UnexpectedExit { exit_code });
    }
    let Some((start_line, starting_count, binaries)) = start else {
        return Err(ParseRefusal::StartingMissing);
    };
    let recorded = u32::try_from(records.len()).unwrap_or(u32::MAX);
    if recorded != starting_count {
        return Err(ParseRefusal::StartingMismatch {
            line: start_line,
            starting: starting_count,
            records: recorded,
        });
    }
    let passed = u32::try_from(records.iter().filter(|r| r.0).count()).unwrap_or(u32::MAX);
    let failed = recorded - passed;
    let Some((summary_line, s)) = total else {
        return Err(ParseRefusal::SummaryMissing);
    };
    if (s.run, s.passed, s.failed) != (recorded, passed, failed) {
        return Err(ParseRefusal::SummaryDisagrees { line: summary_line });
    }
    if let Some((line, binary, name)) = notices
        .iter()
        .find(|(_, b, n)| !records.iter().any(|(_, rb, rn)| rb == b && rn == n))
    {
        return Err(ParseRefusal::SlowNoticeUnrecorded {
            line: *line,
            name: format!("{binary} {name}"),
        });
    }
    if exit_code == 100 && failed == 0 {
        return Err(ParseRefusal::UnattributedFailure { exit_code });
    }
    if exit_code == 0 && failed > 0 {
        return Err(ParseRefusal::ExitDisagrees { exit_code, failed });
    }
    Ok(Tally {
        passed,
        failed,
        ignored: s.skipped,
        blocks: binaries,
        foreign_lines: 0,
        first_foreign: None,
    })
}
