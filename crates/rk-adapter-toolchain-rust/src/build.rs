//! Cargo's structured build stream (s8c §8c.2; record T = ADR-0169 decision 8, proposed; record V =
//! ADR-0170 decisions 4 and 6).
//!
//! `cargo build … --message-format=json` prints one JSON object per line on STDOUT, each with a
//! `reason`. [`read_build_stream`] reads the verdict from it: `status` is the `build-finished`
//! message's `success`, and `errors` counts the `compiler-message` records whose `message.level` is
//! `error`. It is a read of cargo's stable machine format, never of prose (AC-329), and it is strict:
//!
//! - no `build-finished` message is unreadable, whatever the exit code (record T: *"A non-zero exit
//!   with no `build-finished` message is `Err(Transport { Decode })`"*);
//! - a `success` that disagrees with the exit code is unreadable (*"and so is a stream that
//!   disagrees with its exit code"*): `true` needs exit 0, `false` a non-zero exit;
//! - a line that is not a JSON object with a `reason` is unreadable (everything on this stream is
//!   cargo's; a build script's own output is captured by cargo, never passed through).
//!
//! [`executables`] reads the same stream's `compiler-artifact` messages for the executables the
//! build produced, by target name: the `smoke` handler runs rows against them and the `package`
//! handler digests the declared ones (record V decisions 4 and 6).
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::PathBuf;

use rk_types::verb_output::BuildStatus;

/// One parsed stream line.
fn object(line: &str) -> Result<serde_json::Value, String> {
    let value: serde_json::Value = serde_json::from_str(line)
        .map_err(|e| format!("a line of the build stream is not cargo's JSON ({e}): {line:?}"))?;
    if value
        .get("reason")
        .and_then(serde_json::Value::as_str)
        .is_none()
    {
        return Err(format!(
            "a line of the build stream carries no `reason`: {line:?}"
        ));
    }
    Ok(value)
}

/// The build's verdict and its error count, from the stream and the exit code (the module doc).
///
/// # Errors
/// A message naming why the stream is unreadable; the adapter maps it to `Transport { Decode }`.
pub fn read_build_stream(stdout: &str, exit_code: i32) -> Result<(BuildStatus, u32), String> {
    let mut finished: Option<bool> = None;
    let mut errors: u32 = 0;
    for line in stdout.lines().filter(|l| !l.trim().is_empty()) {
        let value = object(line)?;
        match value.get("reason").and_then(serde_json::Value::as_str) {
            Some("build-finished") => {
                let success = value
                    .get("success")
                    .and_then(serde_json::Value::as_bool)
                    .ok_or_else(|| {
                        format!("a build-finished message carries no boolean `success`: {line:?}")
                    })?;
                if finished.replace(success).is_some() {
                    return Err("the build stream carries two build-finished messages".to_owned());
                }
            }
            Some("compiler-message") => {
                let level = value
                    .get("message")
                    .and_then(|m| m.get("level"))
                    .and_then(serde_json::Value::as_str);
                if level == Some("error") {
                    errors = errors.saturating_add(1);
                }
            }
            _ => {}
        }
    }
    match (finished, exit_code) {
        (None, code) => Err(format!(
            "the build stream carries no build-finished message (exit {code}): the build's \
             verdict is unreadable, never a status (record T decision 8)"
        )),
        (Some(true), 0) => Ok((BuildStatus::Passing, errors)),
        (Some(false), code) if code != 0 => Ok((BuildStatus::Failing, errors)),
        (Some(success), code) => Err(format!(
            "the build stream's build-finished success is {success} but cargo exited {code}: the \
             stream disagrees with its exit code (record T decision 8)"
        )),
    }
}

/// The executables the build produced, by target name, from the stream's `compiler-artifact`
/// messages (`target.name` and a non-null `executable`), in stream order.
///
/// # Errors
/// As [`read_build_stream`]'s line reader.
pub fn executables(stdout: &str) -> Result<Vec<(String, PathBuf)>, String> {
    let mut out = Vec::new();
    for line in stdout.lines().filter(|l| !l.trim().is_empty()) {
        let value = object(line)?;
        if value.get("reason").and_then(serde_json::Value::as_str) != Some("compiler-artifact") {
            continue;
        }
        let Some(exe) = value.get("executable").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let name = value
            .get("target")
            .and_then(|t| t.get("name"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                format!("a compiler-artifact with an executable names no target: {line:?}")
            })?;
        out.push((name.to_owned(), PathBuf::from(exe)));
    }
    Ok(out)
}
