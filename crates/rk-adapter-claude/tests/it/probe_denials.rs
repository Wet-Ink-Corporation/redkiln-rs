//! **The live probes' denial reading** (`s8h` §8h.11 probe 0's pass condition and HALT, probe 1's pass
//! condition 6 and HALT 3; §8h.5's de-duplication by `tool_use_id`), compiled OUTSIDE the `live`
//! feature so its scripted-fixture tests run in every suite while the paid probes stay uncompiled.
//!
//! *(dated 2026-10-02, s8h build, package R1FIX-8h; s8h refuting round 1, spec-conformance major.)*
//! The probes counted channel entries (`typed.len() >= 4`), so one denial carried on both of guard
//! 3's channels counted twice, and their HALT passed as soon as ANY denial was typed. This module
//! reads what §8h.11 prescribes instead:
//! - the denials of both typed channels and of the errored `tool_result`s, de-duplicated by
//!   `tool_use_id`, each with the SET of channels that carried it (`result:-`,
//!   `event:<decision_reason_code or ->`, `tool_result:-`);
//! - each instructed action's target mapped to a denial by `Denial.tool_name` and `Denial.path`:
//!   named on channel (a) or (b), surfaced ONLY as an errored `tool_result` (a HALT), or absent
//!   (INCONCLUSIVE: the model did not attempt it);
//! - HALT per denial: a `tool_result`-only denial that no typed denial covers is a HALT even when
//!   other denials are typed.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use rk_adapter_claude::classify;

/// One denial, de-duplicated by `tool_use_id` across the channels, with every channel that carried it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelledDenial {
    pub tool_use_id: Option<String>,
    pub tool_name: String,
    pub path: String,
    /// `result:-`, `event:<decision_reason_code or ->` and/or `tool_result:-`.
    pub channels: BTreeSet<String>,
}

impl ChannelledDenial {
    /// Carried on guard 3's channel (a) or (b): the only channels that count (§8h.11 HALT 3: *"an
    /// errored `tool_result` alone does not count"*).
    #[must_use]
    pub fn typed(&self) -> bool {
        self.channels
            .iter()
            .any(|c| c.starts_with("result:") || c.starts_with("event:"))
    }

    /// The channels, comma-joined, each in §8h.11's `<channel>:<decision_reason_code or ->` form.
    #[must_use]
    pub fn channel_list(&self) -> String {
        self.channels.iter().cloned().collect::<Vec<_>>().join(",")
    }

    /// Probe 1's `DENIAL_CHANNELS` line for this denial (§8h.11: *"for each denial, the channel that
    /// carried it"*).
    #[must_use]
    pub fn channels_line(&self) -> String {
        format!(
            "DENIAL_CHANNELS={} TOOL={} PATH={} TOOL_USE_ID={}",
            self.channel_list(),
            self.tool_name,
            self.path,
            self.tool_use_id.as_deref().unwrap_or("-")
        )
    }

    /// Probe 0's `DENIAL_CHANNEL=<result|event|tool_result>:<decision_reason_code or ->` line for
    /// this denial (§8h.11), every carrying channel comma-joined.
    #[must_use]
    pub fn channel_line(&self) -> String {
        format!("DENIAL_CHANNEL={}", self.channel_list())
    }
}

/// One instructed action's target: the tools that act on it and its path.
#[derive(Debug, Clone, Copy)]
pub struct Target<'a> {
    pub label: &'a str,
    pub tools: &'a [&'a str],
    pub path: &'a Path,
}

/// What guard 3's channels say about one target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetReading {
    /// Named by a denial on channel (a) or (b), with its channel list.
    Typed(String),
    /// Denied, but surfaced ONLY as an errored `tool_result`: a HALT (§8h.11 HALT 3).
    ToolResultOnly,
    /// No denial names it: INCONCLUSIVE when the target is untouched (§8h.11).
    Absent,
}

/// Fold one channel's carrier of a denial into `out`: an entry with the same `tool_use_id` gains the
/// channel; an entry with no id is never merged by id (it stays its own denial; [`halts`] covers it
/// by tool and path).
fn merge(
    out: &mut Vec<ChannelledDenial>,
    id: Option<String>,
    tool: String,
    path: String,
    channel: String,
) {
    let at = id
        .as_deref()
        .and_then(|i| out.iter().position(|d| d.tool_use_id.as_deref() == Some(i)));
    if let Some(i) = at {
        let d = &mut out[i];
        d.channels.insert(channel);
        if d.tool_name.is_empty() {
            d.tool_name = tool;
        }
        if d.path.is_empty() {
            d.path = path;
        }
    } else {
        out.push(ChannelledDenial {
            tool_use_id: id,
            tool_name: tool,
            path,
            channels: BTreeSet::from([channel]),
        });
    }
}

/// The `content` blocks of a stream line's `message`, when the line is of `kind`.
fn blocks(line: &str, kind: &str) -> Vec<serde_json::Value> {
    line.parse::<serde_json::Value>()
        .ok()
        .filter(|v| v.get("type").and_then(serde_json::Value::as_str) == Some(kind))
        .and_then(|v| {
            v.pointer("/message/content")
                .and_then(serde_json::Value::as_array)
                .cloned()
        })
        .unwrap_or_default()
}

/// Every denial the run surfaced, de-duplicated by `tool_use_id`: channel (a), the result frame's
/// `permission_denials` (`classify::denials_from_result`); channel (b), each `permission_denied`
/// event (`classify::denial_from_event`); and each errored `tool_result` naming a denial, whose tool
/// and path are its `tool_use` block's (the model-facing message, never a detector).
#[must_use]
pub fn collect(
    lines: &[String],
    result_raw: Option<&str>,
    cwd: Option<&Path>,
) -> Vec<ChannelledDenial> {
    let mut out: Vec<ChannelledDenial> = Vec::new();
    if let Some(raw) = result_raw
        && let Ok(v) = raw.parse::<serde_json::Value>()
    {
        for d in classify::denials_from_result(&v, cwd) {
            merge(
                &mut out,
                d.tool_use_id,
                d.tool_name,
                d.path,
                "result:-".to_owned(),
            );
        }
    }
    let mut uses: BTreeMap<String, (String, String)> = BTreeMap::new();
    for line in lines {
        if let Some(d) = classify::denial_from_event(line, cwd) {
            let code = d.decision_reason_code.as_deref().unwrap_or("-").to_owned();
            merge(
                &mut out,
                d.tool_use_id,
                d.tool_name,
                d.path,
                format!("event:{code}"),
            );
        }
        for b in blocks(line, "assistant") {
            let id = b.get("id").and_then(serde_json::Value::as_str);
            let name = b.get("name").and_then(serde_json::Value::as_str);
            if b.get("type").and_then(serde_json::Value::as_str) == Some("tool_use")
                && let (Some(id), Some(name)) = (id, name)
            {
                let path = b
                    .get("input")
                    .map_or_else(String::new, |input| classify::denial_path(name, input, cwd));
                uses.insert(id.to_owned(), (name.to_owned(), path));
            }
        }
        for b in blocks(line, "user") {
            let errored = b.get("is_error").and_then(serde_json::Value::as_bool) == Some(true);
            let text = b
                .get("content")
                .map(ToString::to_string)
                .unwrap_or_default();
            if errored && text.to_ascii_lowercase().contains("denied") {
                let id = b.get("tool_use_id").and_then(serde_json::Value::as_str);
                let (tool, path) = id.and_then(|i| uses.get(i)).cloned().unwrap_or_default();
                merge(
                    &mut out,
                    id.map(str::to_owned),
                    tool,
                    path,
                    "tool_result:-".to_owned(),
                );
            }
        }
    }
    out
}

/// A path as the probes compare it: separators unified, and case-folded on Windows, whose paths
/// are case-insensitive.
fn norm(p: &str) -> String {
    let unified = p.trim().replace('\\', "/");
    if cfg!(windows) {
        unified.to_ascii_lowercase()
    } else {
        unified
    }
}

/// Whether `d` acts on `target`: one of its tools, on its path.
fn names(d: &ChannelledDenial, target: &Target<'_>) -> bool {
    target.tools.contains(&d.tool_name.as_str())
        && norm(&d.path) == norm(&target.path.display().to_string())
}

/// What the denials say about `target`.
#[must_use]
pub fn read_target(denials: &[ChannelledDenial], target: &Target<'_>) -> TargetReading {
    denials
        .iter()
        .find(|d| d.typed() && names(d, target))
        .map_or_else(
            || {
                if denials.iter().any(|d| names(d, target)) {
                    TargetReading::ToolResultOnly
                } else {
                    TargetReading::Absent
                }
            },
            |d| TargetReading::Typed(d.channel_list()),
        )
}

/// The HALTs: each `tool_result`-only denial no typed denial covers (by `tool_use_id`, which
/// [`collect`] already merged, or by the same tool and path when an id is missing on one side).
#[must_use]
pub fn halts(denials: &[ChannelledDenial]) -> Vec<&ChannelledDenial> {
    denials
        .iter()
        .filter(|d| !d.typed())
        .filter(|d| {
            d.tool_name.is_empty()
                || !denials.iter().any(|t| {
                    t.typed() && t.tool_name == d.tool_name && norm(&t.path) == norm(&d.path)
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    fn result_frame(entries: &[(&str, &str, &str)]) -> String {
        let list: Vec<serde_json::Value> = entries
            .iter()
            .map(|(id, tool, path)| {
                serde_json::json!({ "tool_name": tool, "tool_use_id": id, "tool_input": { "file_path": path } })
            })
            .collect();
        serde_json::json!({ "type": "result", "subtype": "success", "permission_denials": list })
            .to_string()
    }

    fn event(id: &str, tool: &str, path: &str, code: &str) -> String {
        serde_json::json!({
            "type": "system",
            "subtype": classify::PERMISSION_DENIED_SUBTYPE,
            "tool_name": tool,
            "tool_use_id": id,
            "tool_input": { "file_path": path },
            "decision_reason_code": code
        })
        .to_string()
    }

    fn tool_use(id: &str, tool: &str, path: &str) -> String {
        serde_json::json!({
            "type": "assistant",
            "message": { "content": [ { "type": "tool_use", "id": id, "name": tool, "input": { "file_path": path } } ] }
        })
        .to_string()
    }

    fn errored_result(id: &str) -> String {
        serde_json::json!({
            "type": "user",
            "message": { "content": [ {
                "type": "tool_result",
                "tool_use_id": id,
                "is_error": true,
                "content": "Permission to use this tool was denied."
            } ] }
        })
        .to_string()
    }

    struct Fixture {
        outside_write: PathBuf,
        outside_read: PathBuf,
        settings: PathBuf,
        cargo: PathBuf,
    }

    fn fixture() -> Fixture {
        Fixture {
            outside_write: PathBuf::from("D:/tmp/rk-out/outside-write.txt"),
            outside_read: PathBuf::from("D:/repo/runs/outside-read.txt"),
            settings: PathBuf::from("D:/repo/.git/settings/child-settings-1-1-ab.json"),
            cargo: PathBuf::from("D:/repo/runs/rk-probe-x/.cargo/rk-probe.txt"),
        }
    }

    const WRITE: &[&str] = &["Write", "Edit"];
    const READ: &[&str] = &["Read"];

    fn targets(f: &Fixture) -> [Target<'_>; 4] {
        [
            Target {
                label: "outside write",
                tools: WRITE,
                path: &f.outside_write,
            },
            Target {
                label: "outside read",
                tools: READ,
                path: &f.outside_read,
            },
            Target {
                label: "settings read",
                tools: READ,
                path: &f.settings,
            },
            Target {
                label: ".cargo write",
                tools: WRITE,
                path: &f.cargo,
            },
        ]
    }

    fn s(p: &Path) -> String {
        p.display().to_string()
    }

    /// The refuting round's scenario: the outside write and the outside read each carried on BOTH
    /// typed channels (four channel entries, so the old `typed.len() >= 4` passed), the `.cargo`
    /// write typed, and the settings-file read surfaced ONLY as an errored `tool_result`. Read as
    /// §8h.11 prescribes: four denials, de-duplicated; three targets named; the settings read a HALT.
    ///
    /// RED (observed 2026-10-02 against the stub `collect`): *"four denials, de-duplicated by
    /// tool_use_id: left: 0 right: 4"*.
    #[test]
    fn a_denial_on_both_typed_channels_counts_once_and_a_tool_result_only_target_halts() {
        let f = fixture();
        let result = result_frame(&[
            ("A", "Write", &s(&f.outside_write)),
            ("B", "Read", &s(&f.outside_read)),
        ]);
        let lines = vec![
            tool_use("A", "Write", &s(&f.outside_write)),
            event("A", "Write", &s(&f.outside_write), "path-outside"),
            errored_result("A"),
            tool_use("B", "Read", &s(&f.outside_read)),
            event("B", "Read", &s(&f.outside_read), "path-outside"),
            tool_use("C", "Write", &s(&f.cargo)),
            event("C", "Write", &s(&f.cargo), "protected"),
            tool_use("D", "Read", &s(&f.settings)),
            errored_result("D"),
        ];
        let denials = collect(&lines, Some(&result), None);
        assert_eq!(
            denials.len(),
            4,
            "four denials, de-duplicated by tool_use_id: {denials:?}"
        );
        let a = denials
            .iter()
            .find(|d| d.tool_use_id.as_deref() == Some("A"))
            .expect("A is read");
        assert_eq!(
            a.channel_list(),
            "event:path-outside,result:-,tool_result:-"
        );
        let t = targets(&f);
        assert_eq!(
            read_target(&denials, &t[0]),
            TargetReading::Typed("event:path-outside,result:-,tool_result:-".to_owned())
        );
        assert_eq!(
            read_target(&denials, &t[1]),
            TargetReading::Typed("event:path-outside,result:-".to_owned())
        );
        assert_eq!(
            read_target(&denials, &t[2]),
            TargetReading::ToolResultOnly,
            "the settings read surfaced only as an errored tool_result"
        );
        assert_eq!(
            read_target(&denials, &t[3]),
            TargetReading::Typed("event:protected".to_owned())
        );
        let h = halts(&denials);
        assert_eq!(h.len(), 1, "one HALT though three denials are typed: {h:?}");
        assert_eq!(h[0].tool_use_id.as_deref(), Some("D"));
        assert_eq!(h[0].path, s(&f.settings));
    }

    /// All four targets typed, each on one channel or two: no HALT, every target named, and the
    /// printed lines carry §8h.11's keys in its form. A Windows-spelled path names the same target.
    ///
    /// RED (observed 2026-10-02 against the stub `collect`): *"four denials: left: 0 right: 4"*.
    #[test]
    fn four_typed_denials_name_every_target_and_print_in_the_stage_files_form() {
        let f = fixture();
        let result = result_frame(&[
            ("A", "Write", &s(&f.outside_write).replace('/', "\\")),
            ("B", "Read", &s(&f.outside_read)),
            ("D", "Read", &s(&f.settings)),
        ]);
        let lines = vec![
            event("C", "Edit", &s(&f.cargo), "protected"),
            errored_result("C"),
        ];
        let denials = collect(&lines, Some(&result), None);
        assert_eq!(denials.len(), 4, "four denials: {denials:?}");
        assert!(halts(&denials).is_empty(), "no HALT");
        for t in &targets(&f) {
            assert!(
                matches!(read_target(&denials, t), TargetReading::Typed(_)),
                "{} is named on channel (a) or (b)",
                t.label
            );
        }
        let c = denials
            .iter()
            .find(|d| d.tool_use_id.as_deref() == Some("C"))
            .expect("C");
        assert_eq!(
            c.channel_line(),
            "DENIAL_CHANNEL=event:protected,tool_result:-"
        );
        let b = denials
            .iter()
            .find(|d| d.tool_use_id.as_deref() == Some("B"))
            .expect("B");
        assert_eq!(
            b.channels_line(),
            format!(
                "DENIAL_CHANNELS=result:- TOOL=Read PATH={} TOOL_USE_ID=B",
                s(&f.outside_read)
            )
        );
    }

    /// A target nobody denied is `Absent` (INCONCLUSIVE, never a pass), a denial of another tool on
    /// the same path does not name it, and an errored `tool_result` whose typed twin lacks an id is
    /// covered by tool and path, not a HALT.
    ///
    /// RED (observed 2026-10-02 against the stub `collect`): *"the typed read is named: left: Absent"*.
    #[test]
    fn an_unattempted_target_is_absent_and_a_typed_twin_without_an_id_covers_its_tool_result() {
        let f = fixture();
        let result = serde_json::json!({
            "type": "result",
            "permission_denials": [ { "tool_name": "Read", "tool_input": { "file_path": s(&f.outside_read) } } ]
        })
        .to_string();
        let lines = vec![
            tool_use("B", "Read", &s(&f.outside_read)),
            errored_result("B"),
        ];
        let denials = collect(&lines, Some(&result), None);
        let t = targets(&f);
        assert_eq!(
            read_target(&denials, &t[1]),
            TargetReading::Typed("result:-".to_owned()),
            "the typed read is named"
        );
        assert!(
            halts(&denials).is_empty(),
            "covered by tool and path: {denials:?}"
        );
        assert_eq!(
            read_target(&denials, &t[0]),
            TargetReading::Absent,
            "never attempted"
        );
        let read_of_a_write_target = Target {
            label: "a Read is not a Write",
            tools: WRITE,
            path: &f.outside_read,
        };
        assert_eq!(
            read_target(&denials, &read_of_a_write_target),
            TargetReading::Absent
        );
    }
}
