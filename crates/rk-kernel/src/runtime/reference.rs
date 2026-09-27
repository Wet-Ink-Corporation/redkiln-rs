//! **The complete reference resolver — `runtime::reference`** (`s8d` §8d.3; R2 = ADR-0173, written
//! `proposed`).
//!
//! ADR-0136's forbids: *"The resolver is one mechanism with one record, or it is absent."* This
//! module is that one mechanism. It is **not** `crate::resolver`, which is AC-494's capability
//! resolver (*"a missing capability is `StepError::Unsupported`"*) and stays as it is (m11).
//!
//! # What it resolves (ADR-0040's closed set; ADR-0173 decision 1)
//!
//! | form | resolves to | absent or unresolvable |
//! | --- | --- | --- |
//! | `${<step>.outputs[.path]}`, `${<step>}` | the step's bound output in this activation, at the current iteration path | a mandatory ref to a step that has not run refuses by name (step id and reference); `${…?}` is a `null` value (ADR-0032) |
//! | `${<this or a later step>.outputs?}` | the value bound in the **previous** frame ([`Scope::bind_previous`]) | `null` on the first pass (ADR-0042, ADR-0055) |
//! | `${a?} ?? ${b}` | the left operand unless it is `null`, else the right (ADR-0034) | the right operand resolves like any other |
//! | `${<gate>.answer}`, `${<gate>.attachments.<Kind>}` | the gate's answered option / admitted attachment (ADR-0045) | `.outputs` on a gate refuses by name |
//! | `${item}` | the current `for_each` element | outside a `for_each`, refuses by name |
//! | `${step.outputs[*].field}` | one value per element, and a list-valued field concatenated in element order (D21-48 X5) | a non-list output refuses by name |
//! | `${toolchain.<key>}` | `s8c`'s value table (D21-42 4a) | refused by name |
//! | `artifact:Kind[.field]` | the journalled `emit` binding, through ONE [`ArtifactReadSeam`] call ([`read_artifact_input`]) | mandatory refuses; `?` is `null`; neither calls the seam |
//! | `kb:`, `checklist:`, `toolchain:<feed>[?]` | the key's row in `s8c`'s tables (D21-43 N14) | undeclared refuses even with `?`; a mandatory declared key with no content refuses |
//! | literals | themselves | — |
//!
//! 🛑 **An unresolvable reference is never a value.** A [`RefError`] names the step id AND the
//! reference (ADR-0136's forbids), and it is never `Tri::Null`, `Tri::False` or a skip.
//! `rigor.rs`'s `null_out_refs` is a static lint's treatment and has no caller on this path.
//!
//! 🛑 **An absent `${…?}` binds a `null` VALUE and is never rewritten into the literal `null`**
//! (ADR-0172 clause 3; ADR-0173's forbids). [`bind_refs`] replaces each `${…}` of a predicate by a
//! PATH bound to the resolved value, so `${a?} == 'proceed'` over an absent `a` reads `null`, not
//! the presence test ADR-0172 gives the literal.
//!
//! # What this module does NOT do (named, not claimed)
//!
//! It pushes no iteration frame. [`Scope::bind_previous`] and [`Scope::set_element`] are the entries
//! the loop executor (`for_each`, `repeat`, `on_false` loop-backs; R2 decision 5 and R3) binds
//! through, and the walk in `super::step` binds only the current frame.
//!
//! *(2026-09-26, `s8d` build, package P5.)* The loop executor is built, in `super::step`'s walk:
//! it opens each frame through [`Scope::open_frame`] (a loop-back, a `repeat` pass, an escalate
//! re-entry) and binds each `for_each` element through [`Scope::set_element`]; this module still
//! pushes no frame itself.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use rk_types::error::StepError;
use rk_types::event::Event;

use crate::catalog::{EmitArtifact, InputValue, Step, StepKind};
use crate::predicate::{self, Context, Expr};
use crate::workflow::memo::{SourceValue, ToolchainValues, VerbRegistry};

type Json = serde_json::Value;

// =================================================================================================
// The grammar.
// =================================================================================================

/// One segment of a field path: `.name` or `[N]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Seg {
    Field(String),
    Index(usize),
}

/// Which member of a step a `${<step>…}` reference reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Member {
    /// `.outputs`, the step's bound output.
    Outputs,
    /// `.answer`, a gate step's answered option (ADR-0045).
    Answer,
    /// `.attachments.<Kind>`, a gate step's admitted attachment (ADR-0129 decision 4).
    Attachment(String),
    /// A bare `${<step>}`: the step itself, as an `emit`'s evidence reference reads it.
    Whole,
}

/// One parsed step-input source (ADR-0040's closed set).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// `${<step>…}`.
    StepRef {
        step: String,
        member: Member,
        path: Vec<Seg>,
        /// `[*]` and the field projected from each element (ADR-0035).
        star: Option<Vec<Seg>>,
        optional: bool,
    },
    /// `${item}`, the current `for_each` element.
    Item,
    /// `${toolchain.<key>}`.
    ToolchainKey(String),
    /// `<left> ?? <right>` (ADR-0034).
    Coalesce(Box<Self>, Box<Self>),
    /// `artifact:Kind[.field][?]`.
    Artifact {
        kind: String,
        path: Vec<Seg>,
        optional: bool,
    },
    /// `kb:<atom>`, `checklist:<id>` or `toolchain:<feed>`, with its `?`.
    Declared { key: String, optional: bool },
    /// A literal, itself.
    Literal(Json),
}

/// A source string outside the grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarError {
    pub text: String,
    pub why: String,
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not in ADR-0040's step input grammar: {}",
            self.text, self.why
        )
    }
}

impl std::error::Error for GrammarError {}

/// Parse one `inputs` value. A scalar literal is itself.
///
/// # Errors
/// [`GrammarError`] for a string outside the grammar.
pub fn parse_input(value: &InputValue) -> Result<Source, GrammarError> {
    match value {
        InputValue::Text(text) => parse_source(text),
        InputValue::Int(n) => Ok(Source::Literal(Json::from(*n))),
        InputValue::Bool(b) => Ok(Source::Literal(Json::from(*b))),
    }
}

/// Parse one source string.
///
/// # Errors
/// [`GrammarError`] naming why the string is outside ADR-0040's grammar.
pub fn parse_source(text: &str) -> Result<Source, GrammarError> {
    let err = |why: &str| GrammarError {
        text: text.to_owned(),
        why: why.to_owned(),
    };
    if let Some((left, right)) = text.split_once(" ?? ") {
        let left = parse_single(left.trim()).map_err(|e| err(&e))?;
        let right = parse_single(right.trim()).map_err(|e| err(&e))?;
        return Ok(Source::Coalesce(Box::new(left), Box::new(right)));
    }
    parse_single(text).map_err(|e| err(&e))
}

fn parse_single(text: &str) -> Result<Source, String> {
    if let Some(rest) = text.strip_prefix("${") {
        let inner = rest
            .strip_suffix('}')
            .ok_or_else(|| "a `${` reference is not closed by `}` at the end".to_owned())?;
        if inner.contains(['{', '}', '$']) {
            return Err("a reference holds exactly one `${…}`".to_owned());
        }
        return parse_ref(inner);
    }
    if text.contains("${") {
        return Err(
            "a `${…}` inside a literal is interpolation, which the grammar does not have"
                .to_owned(),
        );
    }
    if let Some(rest) = text.strip_prefix("artifact:") {
        return parse_artifact(rest);
    }
    for prefix in ["kb:", "checklist:", "toolchain:"] {
        if let Some(rest) = text.strip_prefix(prefix) {
            let (bare, optional) = rest.strip_suffix('?').map_or((rest, false), |b| (b, true));
            if bare.is_empty() || !bare.bytes().all(ident_byte) {
                return Err(format!("`{prefix}` names no key"));
            }
            return Ok(Source::Declared {
                key: format!("{prefix}{bare}"),
                optional,
            });
        }
    }
    Ok(Source::Literal(Json::String(text.to_owned())))
}

const fn ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

fn parse_ref(inner: &str) -> Result<Source, String> {
    let (body, optional) = inner
        .strip_suffix('?')
        .map_or((inner, false), |b| (b, true));
    if body == "item" {
        return Ok(Source::Item);
    }
    if let Some(key) = body.strip_prefix("toolchain.") {
        if key.is_empty()
            || !key
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err("`${toolchain.<key>}` names no key".to_owned());
        }
        return Ok(Source::ToolchainKey(key.to_owned()));
    }
    let name_end = body.find(['.', '[']).unwrap_or(body.len());
    let step = &body[..name_end];
    if step.is_empty() || !step.bytes().all(ident_byte) {
        return Err("a reference names no step".to_owned());
    }
    let rest = &body[name_end..];
    let (member, path, star) = if rest.is_empty() {
        (Member::Whole, Vec::new(), None)
    } else if let Some(r) = rest.strip_prefix(".outputs") {
        if let Some(after) = r.strip_prefix("[*]") {
            let field = parse_path(after)?;
            if field.is_empty() {
                return Err("`[*]` projects a FIELD of each element, and none is named".to_owned());
            }
            (Member::Outputs, Vec::new(), Some(field))
        } else {
            if !(r.is_empty() || r.starts_with('.') || r.starts_with('[')) {
                return Err(format!("`{rest}` is not `.outputs` followed by a path"));
            }
            (Member::Outputs, parse_path(r)?, None)
        }
    } else if rest == ".answer" {
        (Member::Answer, Vec::new(), None)
    } else if let Some(kind) = rest.strip_prefix(".attachments.") {
        if kind.is_empty() || !kind.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err("`.attachments.` names no artifact kind".to_owned());
        }
        (Member::Attachment(kind.to_owned()), Vec::new(), None)
    } else {
        return Err(format!(
            "`{rest}` is none of `.outputs[…]`, `.answer` and `.attachments.<Kind>`"
        ));
    };
    Ok(Source::StepRef {
        step: step.to_owned(),
        member,
        path,
        star,
        optional,
    })
}

fn parse_artifact(rest: &str) -> Result<Source, String> {
    let (body, optional) = rest.strip_suffix('?').map_or((rest, false), |b| (b, true));
    let kind_end = body.find(['.', '[']).unwrap_or(body.len());
    let kind = &body[..kind_end];
    if kind.is_empty()
        || !kind.starts_with(|c: char| c.is_ascii_uppercase())
        || !kind.bytes().all(|b| b.is_ascii_alphanumeric())
    {
        return Err("`artifact:` names no artifact kind".to_owned());
    }
    Ok(Source::Artifact {
        kind: kind.to_owned(),
        path: parse_path(&body[kind_end..])?,
        optional,
    })
}

fn parse_path(mut s: &str) -> Result<Vec<Seg>, String> {
    let mut out = Vec::new();
    while !s.is_empty() {
        if let Some(rest) = s.strip_prefix('.') {
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            if end == 0 {
                return Err("a `.` in a field path names no field".to_owned());
            }
            out.push(Seg::Field(rest[..end].to_owned()));
            s = &rest[end..];
        } else if let Some(rest) = s.strip_prefix('[') {
            let end = rest
                .find(']')
                .ok_or_else(|| "a `[` in a field path is not closed".to_owned())?;
            let index = rest[..end]
                .parse::<usize>()
                .map_err(|_| format!("`[{}]` is not an index", &rest[..end]))?;
            out.push(Seg::Index(index));
            s = &rest[end + 1..];
        } else {
            return Err(format!("`{s}` is not a field path"));
        }
    }
    Ok(out)
}

fn render_path(path: &[Seg]) -> String {
    path.iter()
        .map(|s| match s {
            Seg::Field(f) => format!(".{f}"),
            Seg::Index(i) => format!("[{i}]"),
        })
        .collect()
}

fn at_path<'a>(mut value: &'a Json, path: &[Seg]) -> Option<&'a Json> {
    for seg in path {
        value = match seg {
            Seg::Field(f) => value.get(f.as_str())?,
            Seg::Index(i) => value.get(*i)?,
        };
    }
    Some(value)
}

/// Every step name a source reads (`${<step>…}` only), for the grammar census.
#[must_use]
pub fn step_names(source: &Source) -> Vec<&str> {
    match source {
        Source::StepRef { step, .. } => vec![step.as_str()],
        Source::Coalesce(l, r) => {
            let mut v = step_names(l);
            v.extend(step_names(r));
            v
        }
        _ => Vec::new(),
    }
}

// =================================================================================================
// The scope and the environment.
// =================================================================================================

/// A gate step's answer as the walk binds it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GateAnswer {
    /// The answered option (`gate_answered`'s `verdict`).
    pub option: String,
    /// The admitted attachments, by kind.
    pub attachments: BTreeMap<String, Json>,
}

/// **What one activation has bound, at its current iteration path.**
#[derive(Debug, Clone, Default)]
pub struct Scope {
    order: Vec<(String, StepKind)>,
    current: Option<usize>,
    outputs: BTreeMap<String, Json>,
    previous: BTreeMap<String, Json>,
    answers: BTreeMap<String, GateAnswer>,
    /// A gate's answer from the PREVIOUS frame, read by a licensed forward `.answer` ref (an
    /// escalate gate's answer, read by the step its re-entry resumes into).
    previous_answers: BTreeMap<String, GateAnswer>,
    ran: BTreeSet<String>,
    element: Option<Json>,
    /// *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* Each `tool` step's `run`,
    /// by step id, so a route can find the step whose verb it reads (`verdict_record` reads the
    /// `Workspace` the activation's `review_clone` step committed).
    runs: BTreeMap<String, String>,
    /// *(2026-09-27, `s8d` build, package F3-D2174; D21-74 ruling 2; R2 = ADR-0173 decision 9 as
    /// amended, proposed.)* Every kind an `emit` step of this activation binds: what the activation
    /// PRODUCES. `"affected"` reads the run branch's diff inside the producer of `Changeset`.
    emits: BTreeSet<String>,
}

impl Scope {
    /// An empty scope over an activation's steps, in declaration order.
    #[must_use]
    pub fn new(steps: &[Step]) -> Self {
        Self {
            order: steps.iter().map(|s| (s.id.clone(), s.kind)).collect(),
            runs: steps
                .iter()
                .filter_map(|s| s.run.clone().map(|run| (s.id.clone(), run)))
                .collect(),
            emits: steps
                .iter()
                .filter(|s| s.kind == StepKind::Emit)
                .filter_map(|s| s.emit.as_ref())
                .flat_map(|e| e.artifacts.keys().cloned())
                .collect(),
            ..Self::default()
        }
    }

    /// *(F3-D2174; D21-74 ruling 2.)* Whether an `emit` step of this activation binds `kind` —
    /// whether the activation is `kind`'s producer.
    #[must_use]
    pub fn emits(&self, kind: &str) -> bool {
        self.emits.contains(kind)
    }

    /// *(P7; R5 = ADR-0177 decision 5.)* The current-frame output of the LAST declared step whose
    /// `run` names a verb `is_verb` accepts, with that step's id. `None` when no such step has
    /// bound an output in this frame.
    #[must_use]
    pub fn output_of_verb(&self, is_verb: impl Fn(&str) -> bool) -> Option<(&str, &Json)> {
        self.order.iter().rev().find_map(|(id, _)| {
            let run = self.runs.get(id)?;
            let verb = crate::workflow::memo::verb_of(run)?;
            if !is_verb(verb) {
                return None;
            }
            self.outputs.get(id).map(|v| (id.as_str(), v))
        })
    }

    /// The step being resolved for: a reference to it or to a later step reads the previous frame.
    pub fn enter(&mut self, step: &str) {
        self.current = self.index(step);
    }

    fn index(&self, step: &str) -> Option<usize> {
        self.order.iter().position(|(id, _)| id == step)
    }

    /// Bind a step's output in the current frame (and mark it run).
    pub fn bind_output(&mut self, step: &str, value: Json) {
        self.ran.insert(step.to_owned());
        self.outputs.insert(step.to_owned(), value);
    }

    /// The step's output in the current frame.
    #[must_use]
    pub fn output(&self, step: &str) -> Option<&Json> {
        self.outputs.get(step)
    }

    /// Mark a step run in the current frame without an output (a `check`, or a tool step whose port
    /// method returns none).
    pub fn mark_ran(&mut self, step: &str) {
        self.ran.insert(step.to_owned());
    }

    /// Bind a gate step's answer.
    pub fn bind_answer(&mut self, step: &str, answer: GateAnswer) {
        self.ran.insert(step.to_owned());
        self.answers.insert(step.to_owned(), answer);
    }

    /// Bind a step's value from the PREVIOUS frame of the cycle that re-enters the current step —
    /// what a self-reference (ADR-0055) and a licensed forward reference (ADR-0042) read. The loop
    /// executor's entry; the walk binds none.
    pub fn bind_previous(&mut self, step: &str, value: Json) {
        self.previous.insert(step.to_owned(), value);
    }

    /// The current `for_each` element, bound by the loop executor.
    pub fn set_element(&mut self, element: Option<Json>) {
        self.element = element;
    }

    /// *(2026-09-26, `s8d` build, package P5.)* No step is being resolved for: every reference
    /// reads the CURRENT frame. The loop executor evaluates a `repeat`'s `until` so, after the
    /// body's last step, where the body's own outputs are this pass's (D21-48 X6).
    pub const fn leave(&mut self) {
        self.current = None;
    }

    /// *(2026-09-26, `s8d` build, package P5.)* Forget one step's current-frame output, so a
    /// `for_each` element never reads its sibling's.
    pub fn unbind(&mut self, step: &str) {
        self.outputs.remove(step);
        self.ran.remove(step);
    }

    /// **Open a new frame over the steps `from..=to`** (by declaration index) — a loop-back, a
    /// `repeat` pass or an escalate re-entry (*(2026-09-26, `s8d` build, package P5; R3, R2
    /// decision 5)*). Each such step's current-frame binding (output, gate answer, `ran` mark)
    /// becomes its PREVIOUS-frame binding, and a step that bound nothing in the frame now closing
    /// has no previous value (it is `null` to an optional ref). A self or forward ref therefore
    /// reads the frame that re-entered it (ADR-0055; ADR-0042), never the current one; a step
    /// before `from` keeps its binding, since the frame does not re-run it.
    pub fn open_frame(&mut self, from: usize, to: usize) {
        let ids: Vec<String> = self
            .order
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= from && *i <= to)
            .map(|(_, (id, _))| id.clone())
            .collect();
        for id in ids {
            match self.outputs.remove(&id) {
                Some(value) => {
                    self.previous.insert(id.clone(), value);
                }
                None => {
                    self.previous.remove(&id);
                }
            }
            match self.answers.remove(&id) {
                Some(answer) => {
                    self.previous_answers.insert(id.clone(), answer);
                }
                None => {
                    self.previous_answers.remove(&id);
                }
            }
            self.ran.remove(&id);
        }
    }
}

/// **The one read seam's call** (N12; R2 decision 8): what one resolved `artifact:` input read.
#[derive(Debug, Clone, Copy)]
pub struct ArtifactRead<'a> {
    pub item: &'a str,
    /// The READING stage: `doctor`'s `unread` rule groups `artifact.read` rows by `stage` and `kind`.
    pub stage: &'a str,
    pub kind: &'a str,
    /// The reading activation, `activity_id@version`.
    pub reader_activity: &'a str,
    /// The reading step and the input binding it resolved.
    pub reader_step: &'a str,
    pub binding: &'a str,
    /// The activation whose `emit` binding was served, `activity_id@version`.
    pub producer: &'a str,
    /// The value served.
    pub value: &'a Json,
}

/// **The read seam** (N12). `s8f`'s rk-store seam is the ONLY emitter of `artifact.read`
/// (D21-47); [`read_artifact_input`] only calls it, once per resolved input, and never appends a
/// row itself.
pub trait ArtifactReadSeam: Send + Sync {
    /// One resolved `artifact:` input.
    ///
    /// # Errors
    /// The seam's refusal, which refuses the input by name.
    fn read(&self, read: &ArtifactRead<'_>) -> Result<(), String>;
}

/// **The one production value at `s8d`: a named, NON-EMITTING stub.**
///
/// It appends nothing — it holds
/// no store to append to. `s8f` re-points the composition root's value to its rk-store seam, which
/// emits the tier-2 `artifact.read` row (`s8f` §8f.2 item 10).
#[derive(Debug, Clone, Copy, Default)]
pub struct UnwiredReadSeam;

impl ArtifactReadSeam for UnwiredReadSeam {
    fn read(&self, _read: &ArtifactRead<'_>) -> Result<(), String> {
        Ok(())
    }
}

/// What the resolver reads beyond the activation's own scope.
pub struct Env<'a> {
    pub item: &'a str,
    pub stage: &'a str,
    /// The reading activation, `activity_id@version`.
    pub activity: &'a str,
    /// The item's journal, where `emit` bindings and gate answers are read from (tier 2).
    pub events: &'a [Event],
    /// `s8c`'s value table (D21-42 4a).
    pub values: &'a ToolchainValues,
    /// `s8c`'s registry, whose `[kb.*]`, `[checklist.*]` and `[feed.*]` tables N14 names.
    pub registry: Option<&'a VerbRegistry>,
    /// The feed table a `toolchain:<feed>` reads: the item's `toolchain` value.
    pub feed_table: Option<&'a str>,
    /// The directory a declared `kb:`/`checklist:` path is relative to (`.redkiln/`).
    pub sources: Option<&'a Path>,
    /// The read seam.
    pub seam: &'a dyn ArtifactReadSeam,
    /// Call a feed's producing verb (`s8c`'s registry method names it).
    pub feed: &'a dyn Fn(&str) -> Result<Json, String>,
    /// *(2026-09-27, `s8d` build, package F3-D2174; D21-74 ruling 2; R2 = ADR-0173 decision 9 as
    /// amended, proposed.)* The item's run-branch changeset reader, bound by the composition root
    /// ([`super::step::ToolRouting::with_run_branch`]). `None`: inside `Changeset`'s producer,
    /// `"affected"` refuses by name.
    pub run_branch: Option<&'a dyn RunBranch>,
}

/// **The item's run-branch changeset** (D21-74 ruling 2; R2 = ADR-0173 decision 9 as amended,
/// proposed; *added 2026-09-27, `s8d` build, package F3-D2174*).
///
/// Inside the producer of `Changeset`, no `Changeset` is committed while the producer runs, so
/// `"affected"` resolves over the paths the item's run branch changes: R6's range,
/// `HEAD..refs/heads/rk-run/<item>`, read through `rk-git`. `rk-kernel` opens no files and spawns no
/// `git`, so the read is the composition root's (`rk-cli`'s `change_deriver::RunBranchDiff`, the
/// same reader R6's `change` deriver uses), handed in through this seam as review routes are.
pub trait RunBranch: Send + Sync {
    /// The paths `HEAD..refs/heads/rk-run/<item>` changes, in first-seen order. `Ok(None)` is an
    /// EMPTY range: no run branch, or nothing on it past `HEAD`.
    ///
    /// # Errors
    /// A range that exists and could not be read, by name.
    fn changed_paths(&self, item: &str) -> Result<Option<Vec<String>>, String>;
}

/// **An unresolvable reference, by name.** Never `Tri::Null`, `Tri::False` or a skip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefError {
    /// The step whose input (or guard, or `emit` binding) carries the reference.
    pub step: String,
    /// Where the step declares it: an `inputs` key, `when`, `evidence` or `artifacts.<Kind>`.
    pub binding: String,
    /// The reference as written.
    pub reference: String,
    pub why: String,
}

impl RefError {
    /// Where the reference is declared, as a reader finds it in the activity: `inputs.<key>` for
    /// an input, and `when`, `evidence` or `artifacts.<Kind>[.<member>]` as written.
    #[must_use]
    pub fn declared_at(&self) -> String {
        if self.binding == "when"
            || self.binding == "evidence"
            || self.binding == "for_each"
            || self.binding == "repeat.until"
            || self.binding.starts_with("artifacts.")
        {
            self.binding.clone()
        } else {
            format!("inputs.{}", self.binding)
        }
    }
}

impl fmt::Display for RefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the step `{}` cannot resolve its `{}` = `{}`: {}",
            self.step,
            self.declared_at(),
            self.reference,
            self.why
        )
    }
}

impl std::error::Error for RefError {}

impl RefError {
    /// The step's refusal (ADR-0173 decision 1; ADR-0136's forbids).
    #[must_use]
    pub fn to_step_error(&self) -> StepError {
        StepError::Unsupported {
            capability: format!(
                "{self}. An unresolvable reference is refused by name and never read as null, \
                 false or a skip (R2 = ADR-0173 decision 1)"
            ),
        }
    }
}

// =================================================================================================
// Resolution.
// =================================================================================================

struct Site<'s> {
    step: &'s str,
    binding: &'s str,
    reference: &'s str,
}

impl Site<'_> {
    fn refuse(&self, why: impl Into<String>) -> RefError {
        RefError {
            step: self.step.to_owned(),
            binding: self.binding.to_owned(),
            reference: self.reference.to_owned(),
            why: why.into(),
        }
    }
}

/// **Resolve one `inputs` value of one step.**
///
/// # Errors
/// [`RefError`], naming the step and the reference.
pub fn resolve(
    env: &Env<'_>,
    scope: &Scope,
    step: &str,
    binding: &str,
    value: &InputValue,
) -> Result<Json, RefError> {
    let reference = match value {
        InputValue::Text(t) => t.clone(),
        InputValue::Int(n) => n.to_string(),
        InputValue::Bool(b) => b.to_string(),
    };
    let site = Site {
        step,
        binding,
        reference: &reference,
    };
    let source = parse_input(value).map_err(|e| site.refuse(e.why))?;
    resolve_source(env, scope, &site, &source)
}

/// Resolve one reference as written (a `${…}` of a `when` guard or an `emit` binding).
///
/// # Errors
/// As [`resolve`].
pub fn resolve_text(
    env: &Env<'_>,
    scope: &Scope,
    step: &str,
    binding: &str,
    text: &str,
) -> Result<Json, RefError> {
    resolve(
        env,
        scope,
        step,
        binding,
        &InputValue::Text(text.to_owned()),
    )
}

/// Every `inputs` binding of `step`, resolved, in key order.
///
/// # Errors
/// The first binding that does not resolve.
pub fn resolve_inputs(
    env: &Env<'_>,
    scope: &Scope,
    step: &Step,
) -> Result<BTreeMap<String, Json>, RefError> {
    let mut out = BTreeMap::new();
    for (binding, value) in &step.inputs {
        out.insert(
            binding.clone(),
            resolve(env, scope, &step.id, binding, value)?,
        );
    }
    Ok(out)
}

fn resolve_source(
    env: &Env<'_>,
    scope: &Scope,
    site: &Site<'_>,
    source: &Source,
) -> Result<Json, RefError> {
    match source {
        Source::Literal(v) => Ok(v.clone()),
        Source::Item => scope.element.clone().ok_or_else(|| {
            site.refuse(
                "`${item}` is bound only inside a `for_each`, and this step runs outside one",
            )
        }),
        Source::ToolchainKey(key) => env
            .values
            .resolve(key)
            .map(|n| Json::from(n.get()))
            .map_err(|r| site.refuse(r.to_string())),
        // ADR-0034: the left operand unless it is `null`, otherwise the right operand.
        Source::Coalesce(left, right) => {
            let value = resolve_source(env, scope, site, left)?;
            if value.is_null() {
                resolve_source(env, scope, site, right)
            } else {
                Ok(value)
            }
        }
        Source::StepRef {
            step,
            member,
            path,
            star,
            optional,
        } => resolve_step_ref(scope, site, step, member, path, star.as_deref(), *optional),
        Source::Artifact {
            kind,
            path,
            optional,
        } => read_artifact_input(
            env,
            &ArtifactInput {
                step: site.step,
                binding: site.binding,
                reference: site.reference,
                kind,
                path,
                optional: *optional,
            },
        ),
        Source::Declared { key, optional } => resolve_declared(env, site, key, *optional),
    }
}

/// A step kind's catalog token (`agent`, `tool`, `check`, `gate`, `emit`).
#[must_use]
pub(crate) const fn kind_token(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Agent => "agent",
        StepKind::Tool => "tool",
        StepKind::Check => "check",
        StepKind::Gate => "gate",
        StepKind::Emit => "emit",
    }
}

fn resolve_step_ref(
    scope: &Scope,
    site: &Site<'_>,
    target: &str,
    member: &Member,
    path: &[Seg],
    star: Option<&[Seg]>,
    optional: bool,
) -> Result<Json, RefError> {
    let Some(index) = scope.index(target) else {
        return Err(site.refuse(format!("`{target}` names no step of this activation")));
    };
    let kind = scope.order[index].1;
    let absent = |why: String| {
        if optional {
            Ok(Json::Null)
        } else {
            Err(site.refuse(why))
        }
    };
    match member {
        Member::Answer | Member::Attachment(_) => {
            if kind != StepKind::Gate {
                return Err(site.refuse(format!(
                    "`.answer` and `.attachments` are a gate step's members, and `{target}` is a \
                     `{}` step (ADR-0045)",
                    kind_token(kind)
                )));
            }
            // A later gate's answer (the escalate re-entry's licence, D21-48 X8) is the PREVIOUS
            // frame's, `null` first — as a later step's output is (P5).
            let answers = if scope.current.is_some_and(|c| index >= c) {
                &scope.previous_answers
            } else {
                &scope.answers
            };
            let Some(answer) = answers.get(target) else {
                return absent(format!("the gate step `{target}` has not been answered"));
            };
            match member {
                Member::Attachment(k) => answer.attachments.get(k).map_or_else(
                    || {
                        absent(format!(
                            "the gate step `{target}` admitted no `{k}` attachment"
                        ))
                    },
                    |v| Ok(v.clone()),
                ),
                _ => Ok(Json::String(answer.option.clone())),
            }
        }
        Member::Outputs => {
            if kind == StepKind::Gate {
                return Err(site.refuse(format!(
                    "`.outputs` on the gate step `{target}`: a gate answer populates its `answer` \
                     and binds nothing downstream (ADR-0045; AC-409)"
                )));
            }
            // A self-reference, or a reference to a LATER step, reads the previous frame (ADR-0055;
            // ADR-0042): never the current frame's value, and `null` on the first pass.
            let feedback = scope.current.is_some_and(|c| index >= c);
            let frame = if feedback {
                &scope.previous
            } else {
                &scope.outputs
            };
            let Some(value) = frame.get(target) else {
                return absent(if feedback {
                    format!(
                        "`{target}` has no previous frame; a self or feedback reference is `null` \
                         on the first pass, so it must carry `?` (ADR-0042)"
                    )
                } else {
                    format!("the step `{target}` has not run in this activation")
                });
            };
            if let Some(field) = star {
                let Json::Array(elements) = value else {
                    return Err(site.refuse(format!(
                        "`[*]` projects a fan-out's list, and `{target}`'s output is not one \
                         (ADR-0035)"
                    )));
                };
                // D21-48 X5: concatenation in element order.
                let mut out = Vec::new();
                for element in elements {
                    match at_path(element, field) {
                        Some(Json::Array(items)) => out.extend(items.iter().cloned()),
                        Some(v) => out.push(v.clone()),
                        None => {
                            return absent(format!(
                                "an element of `{target}`'s output has no field `{}`",
                                render_path(field)
                            ));
                        }
                    }
                }
                return Ok(Json::Array(out));
            }
            at_path(value, path).map_or_else(
                || {
                    absent(format!(
                        "`{target}`'s output has no field `{}`",
                        render_path(path)
                    ))
                },
                |v| Ok(v.clone()),
            )
        }
        Member::Whole => {
            if scope.ran.contains(target) {
                Ok(serde_json::json!({ "step": target, "kind": kind_token(kind) }))
            } else {
                absent(format!(
                    "the step `{target}` has not run in this activation"
                ))
            }
        }
    }
}

fn resolve_declared(
    env: &Env<'_>,
    site: &Site<'_>,
    key: &str,
    optional: bool,
) -> Result<Json, RefError> {
    let Some(registry) = env.registry else {
        return Err(site.refuse(
            "no verb registry is bound, so `kb:`, `checklist:` and `toolchain:<feed>` have no \
             declared table to resolve through (D21-43 N14)",
        ));
    };
    let table = if key.starts_with("toolchain:") {
        let Some(table) = env.feed_table else {
            return Err(site.refuse(
                "no toolchain feed table is selected, so the feed cannot be looked up and is \
                 never guessed (D21-43 N14)",
            ));
        };
        table
    } else {
        ""
    };
    let input = if optional {
        format!("{key}?")
    } else {
        key.to_owned()
    };
    match registry.resolve_source(table, &input) {
        Err(refusal) => Err(site.refuse(refusal.to_string())),
        Ok(SourceValue::Null) => Ok(Json::Null),
        Ok(SourceValue::FeedVerb(verb)) => (env.feed)(&verb)
            .map_err(|e| site.refuse(format!("its producing verb `{verb}` refused: {e}"))),
        Ok(SourceValue::Path(path)) => {
            let Some(root) = env.sources else {
                return Err(site.refuse(format!(
                    "`{key}` is declared at `{path}`, and no pack directory is bound to read it \
                     from"
                )));
            };
            let bytes = std::fs::read(root.join(&path)).map_err(|e| {
                site.refuse(format!("`{key}`'s declared `{path}` is unreadable: {e}"))
            })?;
            let content = String::from_utf8(bytes.clone())
                .map_err(|_| site.refuse(format!("`{key}`'s declared `{path}` is not UTF-8")))?;
            Ok(serde_json::json!({
                "key": key,
                "path": path,
                "content": content,
                "digest": rk_types::digest::sha256(&bytes).as_str(),
            }))
        }
    }
}

/// One `artifact:` input, as [`read_artifact_input`] reads it.
#[derive(Debug, Clone, Copy)]
pub struct ArtifactInput<'a> {
    pub step: &'a str,
    pub binding: &'a str,
    pub reference: &'a str,
    pub kind: &'a str,
    pub path: &'a [Seg],
    pub optional: bool,
}

/// The committed `emit` binding of `kind` for `item`: the latest completion row carrying it, and the
/// activation that produced it.
#[must_use]
pub fn emit_binding_of<'a>(
    events: &'a [Event],
    item: &str,
    kind: &str,
) -> Option<(&'a Json, &'a str)> {
    events
        .iter()
        .rev()
        .filter(|e| super::activity::is_completion_row(e))
        .filter(|e| e.payload.get("item").and_then(Json::as_str) == Some(item))
        .find_map(|e| {
            let value = e.payload.get("artifacts")?.get(kind)?;
            let producer = e
                .payload
                .get("activity_id_at_version")
                .and_then(Json::as_str)
                .unwrap_or_default();
            Some((value, producer))
        })
}

/// **The one call site of the read seam** (N12; R2 decision 8).
///
/// It serves the committed `emit` binding of the activation that produced `kind` for this item,
/// read from the journal, and calls [`ArtifactReadSeam::read`] exactly ONCE for an input it
/// resolves. An optional input that resolves `null` makes no call, and neither does a refused
/// one. **It appends no row, at this stage or later**: the seam is the emitter.
///
/// # Errors
/// [`RefError`] for a mandatory input with no binding or no such field, or the seam's refusal.
pub fn read_artifact_input(env: &Env<'_>, input: &ArtifactInput<'_>) -> Result<Json, RefError> {
    let site = Site {
        step: input.step,
        binding: input.binding,
        reference: input.reference,
    };
    let absent = |why: String| {
        if input.optional {
            Ok(Json::Null)
        } else {
            Err(site.refuse(why))
        }
    };
    let Some((bound, producer)) = emit_binding_of(env.events, env.item, input.kind) else {
        return absent(format!(
            "no committed `emit` binding of `{}` for the item `{}` is in the journal",
            input.kind, env.item
        ));
    };
    let Some(value) = at_path(bound, input.path) else {
        return absent(format!(
            "the committed `{}` binding has no field `{}`",
            input.kind,
            render_path(input.path)
        ));
    };
    env.seam
        .read(&ArtifactRead {
            item: env.item,
            stage: env.stage,
            kind: input.kind,
            reader_activity: env.activity,
            reader_step: input.step,
            binding: input.binding,
            producer,
            value,
        })
        .map_err(|e| site.refuse(format!("the read seam refused it: {e}")))?;
    Ok(value.clone())
}

// =================================================================================================
// Predicates: a `check`'s context, and a guard's references.
// =================================================================================================

/// A JSON value as the evaluator reads it.
#[must_use]
pub fn json_to_value(value: &Json) -> predicate::Value {
    match value {
        Json::Null => predicate::Value::Null,
        Json::Bool(b) => predicate::Value::Bool(*b),
        Json::Number(n) => n.as_i64().map_or_else(
            || predicate::Value::Str(n.to_string()),
            predicate::Value::Int,
        ),
        Json::String(s) => predicate::Value::Str(s.clone()),
        Json::Array(items) => predicate::Value::List(items.iter().map(json_to_value).collect()),
        Json::Object(map) => predicate::Value::Record(
            map.iter()
                .map(|(k, v)| (k.clone(), json_to_value(v)))
                .collect(),
        ),
    }
}

/// Bind `name`, and every dotted field path under it, to a JSON value.
pub fn bind_json(ctx: &mut Context, name: &str, value: &Json) {
    ctx.bind(name, json_to_value(value));
    if let Json::Object(map) = value {
        for (k, v) in map {
            bind_json(ctx, &format!("{name}.{k}"), v);
        }
    }
}

fn collect_paths<'e>(expr: &'e Expr, out: &mut Vec<&'e str>) {
    match expr {
        Expr::Path(p) => out.push(p),
        Expr::Or(a, b) | Expr::And(a, b) => {
            collect_paths(a, out);
            collect_paths(b, out);
        }
        Expr::Cmp { left, right, .. } => {
            collect_paths(left, out);
            collect_paths(right, out);
        }
        Expr::In { left, list } => {
            collect_paths(left, out);
            collect_paths(list, out);
        }
        Expr::Member { literal, set } => {
            collect_paths(literal, out);
            collect_paths(set, out);
        }
        Expr::Not(a) | Expr::Group(a) | Expr::Len(a) => collect_paths(a, out),
        Expr::List(items) => items.iter().for_each(|i| collect_paths(i, out)),
        Expr::Quant { source, .. } => collect_paths(source, out),
        Expr::Ref(_) | Expr::Lit(_) => {}
    }
}

/// **A `check`'s context.**
///
/// The selection's context, with each of the step's own `inputs` bound
/// over it (`s8d` §8d.3: *"a `Context` built from its own `inputs` table"*). A binding whose value
/// is `null` — an absent `${…?}` — also binds `null` at every field path under it that `expr`
/// reads, so the field of an absent optional input reads `null` (ADR-0032), never an unknown field.
#[must_use]
pub fn check_context(
    ctx: &Context,
    inputs: &BTreeMap<String, Json>,
    expr: Option<&Expr>,
) -> Context {
    let mut out = ctx.clone();
    for (name, value) in inputs {
        bind_json(&mut out, name, value);
    }
    if let Some(expr) = expr {
        let mut paths = Vec::new();
        collect_paths(expr, &mut paths);
        for path in paths {
            let null_root = inputs.iter().any(|(name, v)| {
                v.is_null()
                    && path
                        .strip_prefix(name.as_str())
                        .is_some_and(|r| r.starts_with('.'))
            });
            if null_root {
                out.bind(path, predicate::Value::Null);
            }
        }
    }
    out
}

/// **A guard's (or an `until`'s) references, bound.**
///
/// Each `${…}` of `expr` is resolved through
/// `resolve` and replaced by a PATH bound to its value — never by the literal `null` (ADR-0172
/// clause 3), so `${a?} == 'x'` over an absent `a` reads `null`.
///
/// # Errors
/// The first reference that does not resolve.
pub fn bind_refs(
    expr: &Expr,
    ctx: &Context,
    resolve_ref: &mut dyn FnMut(&str) -> Result<Json, RefError>,
) -> Result<(Expr, Context), RefError> {
    let mut out = ctx.clone();
    let rewritten = rewrite(expr, &mut out, resolve_ref)?;
    Ok((rewritten, out))
}

fn rewrite(
    expr: &Expr,
    ctx: &mut Context,
    resolve_ref: &mut dyn FnMut(&str) -> Result<Json, RefError>,
) -> Result<Expr, RefError> {
    let mut go = |e: &Expr, ctx: &mut Context| rewrite(e, ctx, resolve_ref);
    Ok(match expr {
        Expr::Ref(text) => {
            let value = resolve_ref(text)?;
            ctx.bind(text, json_to_value(&value));
            Expr::Path(text.clone())
        }
        Expr::Or(a, b) => Expr::Or(Box::new(go(a, ctx)?), Box::new(go(b, ctx)?)),
        Expr::And(a, b) => Expr::And(Box::new(go(a, ctx)?), Box::new(go(b, ctx)?)),
        Expr::Not(a) => Expr::Not(Box::new(go(a, ctx)?)),
        Expr::Group(a) => Expr::Group(Box::new(go(a, ctx)?)),
        Expr::Len(a) => Expr::Len(Box::new(go(a, ctx)?)),
        Expr::Cmp { op, left, right } => Expr::Cmp {
            op: *op,
            left: Box::new(go(left, ctx)?),
            right: Box::new(go(right, ctx)?),
        },
        Expr::In { left, list } => Expr::In {
            left: Box::new(go(left, ctx)?),
            list: Box::new(go(list, ctx)?),
        },
        Expr::Member { literal, set } => Expr::Member {
            literal: Box::new(go(literal, ctx)?),
            set: Box::new(go(set, ctx)?),
        },
        Expr::List(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(go(item, ctx)?);
            }
            Expr::List(out)
        }
        Expr::Quant {
            quantifier,
            var,
            source,
            body,
        } => Expr::Quant {
            quantifier: *quantifier,
            var: var.clone(),
            source: Box::new(go(source, ctx)?),
            body: Box::new(go(body, ctx)?),
        },
        Expr::Path(_) | Expr::Lit(_) => expr.clone(),
    })
}

// =================================================================================================
// `emit` binding (R2 decision 7).
// =================================================================================================

/// What an `emit` step bound: each provided kind, and the evidence refs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Emitted {
    pub artifacts: BTreeMap<String, Json>,
    pub evidence: Vec<Json>,
}

/// **Bind an `emit` step.**
///
/// Each key of its `artifacts` table — a reference, or a composing table —
/// to the provided kind, and its `evidence` list to the evidence refs (an absent `?` ref is
/// omitted). `None` for an `emit` that declares neither (a fixture's).
///
/// # Errors
/// The first reference that does not resolve.
pub fn bind_emit(env: &Env<'_>, scope: &Scope, step: &Step) -> Result<Option<Emitted>, RefError> {
    let Some(binding) = &step.emit else {
        return Ok(None);
    };
    let mut out = Emitted::default();
    for (kind, artifact) in &binding.artifacts {
        let value = match artifact {
            EmitArtifact::Ref(text) => {
                resolve_text(env, scope, &step.id, &format!("artifacts.{kind}"), text)?
            }
            EmitArtifact::Compose(members) => {
                let mut object = serde_json::Map::new();
                for (member, value) in members {
                    let binding = format!("artifacts.{kind}.{member}");
                    object.insert(
                        member.clone(),
                        resolve(env, scope, &step.id, &binding, value)?,
                    );
                }
                Json::Object(object)
            }
        };
        out.artifacts.insert(kind.clone(), value);
    }
    for text in &binding.evidence {
        let value = resolve_text(env, scope, &step.id, "evidence", text)?;
        if !value.is_null() {
            out.evidence.push(value);
        }
    }
    Ok(Some(out))
}

// =================================================================================================
// Test selection, the resolver side (D21-42 4c; R2 decision 9).
// =================================================================================================

/// The selection a `run` repro names: the tokens of `command` after a `--` separator.
///
/// **The ONE selection parser** (`s8d` §8d.3): moved here from `rk_cli::review_pass`, which
/// re-exports it with `pub use` so its path and its tests are unchanged. `rk-kernel` cannot depend
/// on `rk-cli`, and a second parser in the kernel is forbidden (ADR-0173).
///
/// 🛑 **`None` rather than a guess, and `None` rather than an empty selection.** `Repro.command` is
/// a shell string and `Toolchain::test` takes a selection, and no record maps one to the other; the
/// narrowest rule that makes AC-288's four cells computable from what a `Repro` actually carries is
/// the explicit `--` filter. A command without one is refused into `Unverifiable { cannot_run }`,
/// which AC-287 forbids from blocking — and an empty selection is refused outright, because
/// `ac332_empty_selection_is_never_green` is the standing finding that an empty suite is not a pass.
#[must_use]
pub fn selection_of(command: &str) -> Option<Vec<String>> {
    let (_, tail) = command.split_once(" -- ")?;
    let selection: Vec<String> = tail
        .split_whitespace()
        .map(str::to_owned)
        .filter(|t| !t.is_empty())
        .collect();
    (!selection.is_empty()).then_some(selection)
}

/// What a `toolchain:test` step's `inputs.selection` resolved to, before it becomes a port
/// `Selection` (the kernel maps it; `rk-ports` owns the type).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionPlan {
    /// Named tests, from a `${…}` value or through [`selection_of`].
    Filter(Vec<String>),
    /// `"affected"`: the item's committed `Changeset` paths (`s8c`'s package-level mapping) —
    /// or, inside the producer of `Changeset`, the paths its run branch changes (D21-74 ruling 2).
    Affected(Vec<String>),
    /// A selection the adapter declares or refuses by name (`"e2e"`).
    Named(String),
}

/// **Resolve a `toolchain:test` step's selection.**
///
/// An unresolvable selection refuses by name and
/// never reaches the port as an empty one (ADR-0148's reason stands: *"`cargo test affected`, which
/// matches no test and exits 0"*).
///
/// # Errors
/// [`RefError`] naming the step and its `inputs.selection`.
pub fn selection_plan(
    env: &Env<'_>,
    scope: &Scope,
    step: &Step,
    inputs: &BTreeMap<String, Json>,
) -> Result<SelectionPlan, RefError> {
    let refuse = |reference: &str, why: String| RefError {
        step: step.id.clone(),
        binding: "selection".to_owned(),
        reference: reference.to_owned(),
        why,
    };
    let Some(raw) = step.inputs.get("selection") else {
        return Err(refuse(
            "inputs.selection",
            "the `toolchain:test` step declares no `inputs.selection`, and an empty selection is \
             never run (ADR-0148)"
                .to_owned(),
        ));
    };
    let text = raw.as_text().unwrap_or_default();
    match parse_input(raw) {
        Ok(Source::Literal(_)) => {
            return match text {
                // *(2026-09-27, `s8d` build, package F3-D2174; D21-74 ruling 2; R2 = ADR-0173
                // decision 9 as amended, proposed.)* **Inside the producer of `Changeset`**, none is
                // committed while it runs (and one an earlier activation committed is not this
                // one's), so `"affected"` is the run branch's diff, `HEAD..rk-run/<item>`, read
                // through the composition root's reader. Elsewhere, as before.
                "affected" if scope.emits("Changeset") => {
                    let Some(reader) = env.run_branch else {
                        return Err(refuse(
                            "affected",
                            "this activation produces the item's `Changeset`, so `affected` is \
                             the run branch's diff (HEAD..rk-run/<item>), and no run-branch reader \
                             is bound to read it; it is never guessed and never an empty selection"
                                .to_owned(),
                        ));
                    };
                    match reader.changed_paths(env.item) {
                        Ok(Some(paths)) if !paths.is_empty() => Ok(SelectionPlan::Affected(paths)),
                        Ok(_) => Err(refuse(
                            "affected",
                            format!(
                                "this activation produces the item's `Changeset`, and its run \
                                 branch HEAD..rk-run/{} carries no change, so `affected` has no \
                                 paths to map and is never run as an empty selection",
                                env.item
                            ),
                        )),
                        Err(e) => Err(refuse(
                            "affected",
                            format!(
                                "this activation produces the item's `Changeset`, and its run \
                                 branch HEAD..rk-run/{} could not be read: {e}",
                                env.item
                            ),
                        )),
                    }
                }
                "affected" => {
                    let changeset = read_artifact_input(
                        env,
                        &ArtifactInput {
                            step: &step.id,
                            binding: "selection",
                            reference: "affected",
                            kind: "Changeset",
                            path: &[],
                            optional: false,
                        },
                    )?;
                    let paths: Vec<String> = changeset
                        .get("files")
                        .and_then(Json::as_array)
                        .map(|files| {
                            files
                                .iter()
                                .filter_map(Json::as_str)
                                .map(str::to_owned)
                                .collect()
                        })
                        .unwrap_or_default();
                    if paths.is_empty() {
                        return Err(refuse(
                            "affected",
                            "the committed `Changeset` carries no `files`, so `affected` has no \
                             paths to map and is never run as an empty selection"
                                .to_owned(),
                        ));
                    }
                    Ok(SelectionPlan::Affected(paths))
                }
                "e2e" => Ok(SelectionPlan::Named("e2e".to_owned())),
                other => Err(refuse(
                    other,
                    "a literal selection other than `affected` or `e2e` is not a test selection, \
                     and it is never passed through as a test name (ADR-0148's forbids)"
                        .to_owned(),
                )),
            };
        }
        Ok(_) => {}
        Err(e) => return Err(refuse(text, e.why)),
    }
    let tests = match inputs.get("selection") {
        Some(Json::String(command)) => selection_of(command).ok_or_else(|| {
            refuse(
                text,
                format!(
                    "it resolved to `{command}`, which names no `--` test filter, so it selects no \
                     test and is never run as an empty or whole-suite selection"
                ),
            )
        })?,
        Some(Json::Array(items)) => {
            let names: Vec<String> = items
                .iter()
                .filter_map(Json::as_str)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect();
            if names.is_empty() || names.len() != items.len() {
                return Err(refuse(
                    text,
                    "it resolved to a list that is empty or not all test names, and an empty \
                     selection is never run (ADR-0148)"
                        .to_owned(),
                ));
            }
            names
        }
        other => {
            return Err(refuse(
                text,
                format!(
                    "it resolved to {}, which is not a test selection, and an empty selection is \
                     never run (ADR-0148)",
                    other.map_or_else(|| "nothing".to_owned(), ToString::to_string)
                ),
            ));
        }
    };
    Ok(SelectionPlan::Filter(tests))
}
