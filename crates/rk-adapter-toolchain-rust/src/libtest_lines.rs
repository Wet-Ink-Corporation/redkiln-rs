//! The strict libtest-line parser, pinned per toolchain version.
//!
//! s8c §8c.1; D21-43 N4: *"A strict parser over libtest's `test <name> ... ok|FAILED` and `test result:` lines, pinned by a golden
//! fixture per toolchain version."* and *"Unparseable output is Unverifiable, never green."*; record
//! T = ADR-0169 decision 3, proposed).
//!
//! One pure function, [`parse`], over the run child's stdout FILE (never stderr, so s7c's `build.rs`
//! decoy cannot reach it). The rules, numbered as §8c.1 numbers them:
//!
//! 1. **Pinned per version.** [`PINNED`] lists the `rustc -V` releases with a golden under
//!    `tests/fixtures/libtest/<version>/`. Any other version is refused by name
//!    ([`ParseRefusal::Unsupported`], capability `libtest-lines@<version>`), never parsed on trust.
//! 2. **Block-framed.** A block opens on `^running (\d+) tests?$`. Its RECORD region runs to the
//!    first empty line. A record is `^test (.+) \.\.\. (ok|FAILED|ignored(, .*)?)$`, and the region
//!    must hold exactly as many records as the header states. libtest's slow notice,
//!    `^test (.+) has been running for over 60 seconds$`, is skipped, and must name a test the same
//!    block records. *(D21-62 A, provisional.)* Any other non-empty line in the region is FOREIGN:
//!    skipped, never counted, and tallied in [`Tally::foreign_lines`] with the first one kept as
//!    evidence. A header or a summary inside the region is a protocol line, not a foreign one, and
//!    refuses. The block closes on the full summary grammar
//!    `^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in [0-9.]+s$`;
//!    the lines between the region and the summary (`failures:`, captured output) are skipped. A
//!    record-grammar line outside a block is never counted.
//! 3. **Self-consistent.** Each summary's verdict, `passed`, `failed` and `ignored` must equal its
//!    block's records. A header without a summary, or a summary without a header, refuses.
//! 4. **Multi-binary.** The tally is the sum over blocks (doctest blocks included).
//! 5. **Against `--list`** is the adapter's check (record T decision 1's invariant), not this
//!    function's: it has no `--list` count.
//! 6. **Exit-code agreement.** A non-zero exit with no failed record refuses as
//!    [`ParseRefusal::UnattributedFailure`]; a zero exit with a failed record refuses too.
//!
//! The grammars are matched by hand, character for character, so the crate takes no `regex`
//! dependency; each matcher's doc names the expression it implements.

/// The `rustc -V` releases a golden exists for: `rust-toolchain.toml`'s channel. A pin bump adds a
/// golden and an entry here in the PR that moves the pin.
pub const PINNED: [&str; 1] = ["1.98.1"];

/// The first foreign line a parse met: its 1-based line number and its text (D21-62 A's evidence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignLine {
    pub line: usize,
    pub text: String,
}

/// What a run's lines prove.
///
/// Shared with [`crate::nextest_lines`], so `test` returns the same outcome whichever runner ran. On the libtest path `blocks` counts result blocks and `ignored`
/// is libtest's `ignored`; on the nextest path they are the `Starting` line's binary count and the
/// `Summary` line's `skipped` (nextest does not tell an ignored test from a filtered one).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tally {
    pub passed: u32,
    pub failed: u32,
    pub ignored: u32,
    pub blocks: u32,
    /// *(D21-62 A.)* Non-empty lines inside a record region that match no protocol grammar. A count
    /// from the parser, never a parsed prose value; always 0 on the nextest path.
    pub foreign_lines: u32,
    pub first_foreign: Option<ForeignLine>,
}

/// Why a stream is Unverifiable. The adapter maps every variant to `Err(Transport { Decode })`,
/// naming the refusal ([`ParseRefusal::name`]) and the version: never `Ok` with `failed = 0`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseRefusal {
    #[error("unsupported: {capability} (no golden pins this version)")]
    Unsupported { capability: String },
    #[error("line {line}: the block header states {header} records and its region holds {records}")]
    RecordCountMismatch {
        line: usize,
        header: u32,
        records: u32,
    },
    #[error("line {line}: a slow-test notice names `{name}`, which the run never records")]
    SlowNoticeUnrecorded { line: usize, name: String },
    #[error("line {line}: the summary disagrees with the records it closes")]
    SummaryDisagrees { line: usize },
    #[error("line {line}: a summary with no block header open")]
    SummaryWithoutHeader { line: usize },
    #[error("line {line}: a block header whose block never closes on a summary")]
    HeaderWithoutSummary { line: usize },
    #[error("exit {exit_code} with no failed record: a failure no record names")]
    UnattributedFailure { exit_code: i32 },
    #[error("exit {exit_code} over {failed} failed records")]
    ExitDisagrees { exit_code: i32, failed: u32 },
    #[error("cargo-nextest {version} exited {exit}: creating test list failed")]
    ListFailed { version: String, exit: i32 },
    #[error("cargo-nextest exited {exit_code}, which is neither a green (0) nor a red (100) run")]
    UnexpectedExit { exit_code: i32 },
    /// *(s8c refuting round 6.)* nextest refused selection arguments it does not accept (exit 96),
    /// naming them in its pinned line; `args` is its backticked text, as printed.
    #[error("cargo-nextest {version} does not accept the selection argument(s) `{args}` (exit 96)")]
    ArgumentsRejected { version: String, args: String },
    #[error("line {line}: a retry or kill status is never counted: {text}")]
    RetryOrKill { line: usize, text: String },
    #[error("line {line}: `Starting` states {starting} tests and the run records {records}")]
    StartingMismatch {
        line: usize,
        starting: u32,
        records: u32,
    },
    #[error("line {line}: a second `Starting` line, or a status line before the first")]
    StartingOutOfPlace { line: usize },
    #[error("the run has no `Starting` line")]
    StartingMissing,
    #[error("the run has no `Summary` line")]
    SummaryMissing,
    #[error("line {line}: a `Summary` line outside the pinned grammar")]
    MalformedSummary { line: usize },
}

impl ParseRefusal {
    /// The refusal's name, as the adapter's `Transport { Decode }` detail carries it.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Unsupported { .. } => "Unsupported",
            Self::RecordCountMismatch { .. } => "RecordCountMismatch",
            Self::SlowNoticeUnrecorded { .. } => "SlowNoticeUnrecorded",
            Self::SummaryDisagrees { .. } => "SummaryDisagrees",
            Self::SummaryWithoutHeader { .. } => "SummaryWithoutHeader",
            Self::HeaderWithoutSummary { .. } => "HeaderWithoutSummary",
            Self::UnattributedFailure { .. } => "UnattributedFailure",
            Self::ExitDisagrees { .. } => "ExitDisagrees",
            Self::ListFailed { .. } => "ListFailed",
            Self::UnexpectedExit { .. } => "UnexpectedExit",
            Self::ArgumentsRejected { .. } => "ArgumentsRejected",
            Self::RetryOrKill { .. } => "RetryOrKill",
            Self::StartingMismatch { .. } => "StartingMismatch",
            Self::StartingOutOfPlace { .. } => "StartingOutOfPlace",
            Self::StartingMissing => "StartingMissing",
            Self::SummaryMissing => "SummaryMissing",
            Self::MalformedSummary { .. } => "MalformedSummary",
        }
    }
}

/// The lines of a stream, 1-based, each without its terminator (`\n`, or `\r\n`).
pub(crate) fn numbered(stream: &str) -> impl Iterator<Item = (usize, &str)> {
    stream.lines().enumerate().map(|(i, l)| (i + 1, l))
}

/// A run of ASCII digits as a `u32`, and the rest; `None` on no digit or on overflow.
pub(crate) fn digits(s: &str) -> Option<(u32, &str)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    Some((s[..end].parse().ok()?, &s[end..]))
}

/// `[0-9.]+` followed by `tail`, to the end of the string.
pub(crate) fn seconds_then(s: &str, tail: &str) -> bool {
    let end = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    end > 0 && &s[end..] == tail
}

/// `^running (\d+) tests?$`
fn header(line: &str) -> Option<u32> {
    let (n, rest) = digits(line.strip_prefix("running ")?)?;
    matches!(rest, " test" | " tests").then_some(n)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Ok,
    Failed,
    Ignored,
}

/// `^test (.+) \.\.\. (ok|FAILED|ignored(, .*)?)$`, with `(.+)` greedy: the RIGHTMOST ` ... `
/// whose tail is a verdict separates the name from it.
fn record(line: &str) -> Option<(&str, Verdict)> {
    let body = line.strip_prefix("test ")?;
    let mut end = body.len();
    while let Some(at) = body[..end].rfind(" ... ") {
        let tail = &body[at + " ... ".len()..];
        let verdict = match tail {
            "ok" => Some(Verdict::Ok),
            "FAILED" => Some(Verdict::Failed),
            t if t == "ignored" || t.starts_with("ignored, ") => Some(Verdict::Ignored),
            _ => None,
        };
        if let Some(v) = verdict
            && at > 0
        {
            return Some((&body[..at], v));
        }
        end = at;
    }
    None
}

/// `^test (.+) has been running for over 60 seconds$`
fn slow_notice(line: &str) -> Option<&str> {
    let name = line
        .strip_prefix("test ")?
        .strip_suffix(" has been running for over 60 seconds")?;
    (!name.is_empty()).then_some(name)
}

struct Summary {
    ok: bool,
    passed: u32,
    failed: u32,
    ignored: u32,
}

/// `^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in [0-9.]+s$`
fn summary(line: &str) -> Option<Summary> {
    let rest = line.strip_prefix("test result: ")?;
    let (ok, rest) = if let Some(r) = rest.strip_prefix("ok. ") {
        (true, r)
    } else {
        (false, rest.strip_prefix("FAILED. ")?)
    };
    let (passed, rest) = digits(rest)?;
    let (failed, rest) = digits(rest.strip_prefix(" passed; ")?)?;
    let (ignored, rest) = digits(rest.strip_prefix(" failed; ")?)?;
    let (_measured, rest) = digits(rest.strip_prefix(" ignored; ")?)?;
    let (_filtered, rest) = digits(rest.strip_prefix(" measured; ")?)?;
    let rest = rest.strip_prefix(" filtered out; finished in ")?;
    seconds_then(rest, "s").then_some(Summary {
        ok,
        passed,
        failed,
        ignored,
    })
}

/// One open block: its header, its records and its slow notices.
struct Block<'a> {
    header_line: usize,
    header: u32,
    passed: u32,
    failed: u32,
    ignored: u32,
    names: Vec<&'a str>,
    notices: Vec<(usize, &'a str)>,
}

impl Block<'_> {
    const fn records(&self) -> u32 {
        self.passed + self.failed + self.ignored
    }
}

enum State<'a> {
    Outside,
    Region(Block<'a>),
    AwaitSummary(Block<'a>),
}

/// Rule 2's region close: the header count, and every slow notice names a recorded test.
fn close_region(block: &Block<'_>) -> Result<(), ParseRefusal> {
    if block.records() != block.header {
        return Err(ParseRefusal::RecordCountMismatch {
            line: block.header_line,
            header: block.header,
            records: block.records(),
        });
    }
    if let Some((line, name)) = block.notices.iter().find(|(_, n)| !block.names.contains(n)) {
        return Err(ParseRefusal::SlowNoticeUnrecorded {
            line: *line,
            name: (*name).to_owned(),
        });
    }
    Ok(())
}

/// Rule 3: the summary agrees with the block's records, verdict included.
const fn check_summary(s: &Summary, block: &Block<'_>, line: usize) -> Result<(), ParseRefusal> {
    let agrees = s.passed == block.passed
        && s.failed == block.failed
        && s.ignored == block.ignored
        && s.ok == (block.failed == 0);
    if agrees {
        Ok(())
    } else {
        Err(ParseRefusal::SummaryDisagrees { line })
    }
}

/// Parse a libtest run child's stdout under `version` (a `rustc -V` release) and its exit code.
///
/// # Errors
///
/// A [`ParseRefusal`] for an unpinned version or any breach of the rules above; the caller maps it
/// to `Transport { Decode }`, never to a green or a red suite.
pub fn parse(stdout: &str, exit_code: i32, version: &str) -> Result<Tally, ParseRefusal> {
    if !PINNED.contains(&version) {
        return Err(ParseRefusal::Unsupported {
            capability: format!("libtest-lines@{version}"),
        });
    }
    let mut tally = Tally::default();
    let mut state = State::Outside;
    for (no, line) in numbered(stdout) {
        state = match state {
            State::Outside => {
                if let Some(h) = header(line) {
                    State::Region(Block {
                        header_line: no,
                        header: h,
                        passed: 0,
                        failed: 0,
                        ignored: 0,
                        names: Vec::new(),
                        notices: Vec::new(),
                    })
                } else if summary(line).is_some() {
                    return Err(ParseRefusal::SummaryWithoutHeader { line: no });
                } else {
                    State::Outside
                }
            }
            State::Region(mut block) => {
                if line.is_empty() {
                    close_region(&block)?;
                    State::AwaitSummary(block)
                } else if let Some((name, verdict)) = record(line) {
                    match verdict {
                        Verdict::Ok => block.passed += 1,
                        Verdict::Failed => block.failed += 1,
                        Verdict::Ignored => block.ignored += 1,
                    }
                    block.names.push(name);
                    State::Region(block)
                } else if let Some(name) = slow_notice(line) {
                    block.notices.push((no, name));
                    State::Region(block)
                } else if header(line).is_some() {
                    return Err(ParseRefusal::HeaderWithoutSummary {
                        line: block.header_line,
                    });
                } else if summary(line).is_some() {
                    return Err(ParseRefusal::SummaryWithoutHeader { line: no });
                } else {
                    // D21-62 A: a FOREIGN line, skipped and never counted, but tallied.
                    tally.foreign_lines += 1;
                    if tally.first_foreign.is_none() {
                        tally.first_foreign = Some(ForeignLine {
                            line: no,
                            text: line.to_owned(),
                        });
                    }
                    State::Region(block)
                }
            }
            State::AwaitSummary(block) => {
                if let Some(s) = summary(line) {
                    check_summary(&s, &block, no)?;
                    tally.passed += block.passed;
                    tally.failed += block.failed;
                    tally.ignored += block.ignored;
                    tally.blocks += 1;
                    State::Outside
                } else if header(line).is_some() {
                    return Err(ParseRefusal::HeaderWithoutSummary {
                        line: block.header_line,
                    });
                } else {
                    State::AwaitSummary(block)
                }
            }
        };
    }
    match state {
        State::Outside => {}
        State::Region(block) | State::AwaitSummary(block) => {
            return Err(ParseRefusal::HeaderWithoutSummary {
                line: block.header_line,
            });
        }
    }
    if exit_code != 0 && tally.failed == 0 {
        return Err(ParseRefusal::UnattributedFailure { exit_code });
    }
    if exit_code == 0 && tally.failed > 0 {
        return Err(ParseRefusal::ExitDisagrees {
            exit_code,
            failed: tally.failed,
        });
    }
    Ok(tally)
}
