//! **The question registry's loader** — `.redkiln/processes/*.toml` into the kernel's
//! `QuestionRegistry` (`s8d` §8d.6; R5 = ADR-0177, proposed). *(Added 2026-09-26, `s8d` build,
//! package P7.)*
//!
//! ADR-0129 decision 3 built the registry type in `rk-kernel` and said *"**Pack-declared questions**
//! (a question in a process pack's TOML with its own options and `attaches`) are Step 7e's, loaded
//! into the same type."* No 7e loader was built: ADR-0156's blocker 2 measured `.redkiln/processes/`
//! empty, so every gate was answerable only as the default question. D21-43 gives the loader and the
//! directory to `s8d` alone (*"The loader and `.redkiln/processes/` are **s8d**'s."*); `s8j` authors
//! the human-root questions into the same directory and `s8g` loads through this function. There is
//! no second loader.
//!
//! **What it reads.** Every `*.toml` file directly under `<root>/.redkiln/processes/`, in sorted
//! order. It does **not** recurse (a subdirectory is not read, so D21-44's `.redkiln/worktrees/` is
//! never under it), and a file with another extension is not read. An absent directory is the
//! built-in default registry and nothing else.
//!
//! **The schema (R5 decision 7).** Each `[question."<id>"]` table declares:
//!
//! | key | value |
//! | --- | --- |
//! | `text` | the question, a non-empty string |
//! | `options` | a non-empty list; each an option id, or `{ id = "<option>", attaches = "<Kind>" }` (a kind or a list of kinds) |
//! | `binds` | optional: a list of further gate kinds this question answers (N3's `unknown_facet:change`) |
//!
//! Any other key, a duplicate option, an id declared in two files, or a gate kind bound twice is
//! refused by name. Each question binds **its own id** as a gate kind, because a catalog `gate`
//! step's kind is its `question` (ADR-0145) — which is what lets `run` refuse, by name, a gate step
//! whose question no file declares ([`undeclared_gate_kind`]).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_kernel::workflow::question::{Question, QuestionRegistry};

/// The directory the loader reads, under a worktree root: `<root>/.redkiln/processes`.
#[must_use]
pub fn processes_dir(root: &Path) -> PathBuf {
    root.join(".redkiln").join("processes")
}

/// Why the question files were refused. Each variant names the file and what it could not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessesError {
    /// The directory or a file could not be read.
    Read { path: String, detail: String },
    /// A file is not TOML, or a question table is not the schema's shape.
    Shape { path: String, detail: String },
    /// A question id declared in two files, or twice-bound gate kind.
    Duplicate { path: String, detail: String },
}

impl std::fmt::Display for ProcessesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read { path, detail } | Self::Duplicate { path, detail } => {
                write!(f, "{path}: {detail}")
            }
            Self::Shape { path, detail } => write!(
                f,
                "{path}: {detail} (the question schema: `text`, `options`, optional `binds`; R5 = \
                 ADR-0177 decision 7)"
            ),
        }
    }
}

impl std::error::Error for ProcessesError {}

/// **Load every question under `<root>/.redkiln/processes/`** into the kernel's registry, over the
/// built-in default question (ADR-0129 decision 3).
///
/// # Errors
/// [`ProcessesError`] naming the file and the refusal; nothing is loaded in part.
pub fn load(root: &Path) -> Result<QuestionRegistry, ProcessesError> {
    let dir = processes_dir(root);
    let mut registry = QuestionRegistry::default();
    if !dir.is_dir() {
        return Ok(registry);
    }
    let entries = std::fs::read_dir(&dir).map_err(|e| ProcessesError::Read {
        path: dir.display().to_string(),
        detail: e.to_string(),
    })?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|e| ProcessesError::Read {
                path: dir.display().to_string(),
                detail: e.to_string(),
            })?
            .path();
        // Not recursive, and `*.toml` only.
        if path.is_file() && path.extension().is_some_and(|x| x == "toml") {
            files.push(path);
        }
    }
    files.sort();

    // question id -> the file that declared it; gate kind -> (question, file).
    let mut declared: BTreeMap<String, String> = BTreeMap::new();
    let mut bound: BTreeMap<String, (String, String)> = BTreeMap::new();
    for path in files {
        let shown = path.display().to_string();
        let src = std::fs::read_to_string(&path).map_err(|e| ProcessesError::Read {
            path: shown.clone(),
            detail: e.to_string(),
        })?;
        let shape = |detail: String| ProcessesError::Shape {
            path: shown.clone(),
            detail,
        };
        let doc: toml::Table = toml::from_str(&src).map_err(|e| shape(e.to_string()))?;
        for key in doc.keys() {
            if key != "question" {
                return Err(shape(format!(
                    "declares the top-level key `{key}`; a question file declares `[question.\"<id>\"]` tables only"
                )));
            }
        }
        let Some(questions) = doc.get("question") else {
            continue;
        };
        let questions = questions
            .as_table()
            .ok_or_else(|| shape("`question` is not a table".to_owned()))?;
        for (id, table) in questions {
            let (question, binds) = question_from(id, table).map_err(&shape)?;
            if let Some(first) = declared.get(id) {
                return Err(ProcessesError::Duplicate {
                    path: shown.clone(),
                    detail: format!("the question `{id}` is already declared by {first}"),
                });
            }
            declared.insert(id.clone(), shown.clone());
            registry = registry.with_question(id.clone(), question);
            for kind in std::iter::once(id.clone()).chain(binds) {
                if let Some((other, file)) = bound.get(&kind) {
                    return Err(ProcessesError::Duplicate {
                        path: shown.clone(),
                        detail: format!(
                            "the gate kind `{kind}` is bound to `{id}` and already to `{other}` \
                             (by {file}); a gate kind is answered by one question"
                        ),
                    });
                }
                bound.insert(kind.clone(), (id.clone(), shown.clone()));
                registry = registry.bind(kind, id.clone());
            }
        }
    }
    Ok(registry)
}

/// **The kernel's own questions, over a loaded registry** — the refused landing's
/// (`landing.refused-path`: `exclude` | `abort`; ADR-0188 decision 6, proposed). The kernel opens
/// that gate, as it opens the selection's `unknown_facet:*` gates, so no pack file declares its
/// question; `answer` validates against the registry this returns. A pack file that declares or
/// binds the kernel's kind is refused by name, never silently replaced. [`load`] itself is
/// unchanged, so an absent directory still loads the default question alone.
/// *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E.)*
///
/// # Errors
/// [`ProcessesError::Duplicate`] when a pack file already declares or binds the kind.
pub fn with_kernel_questions(
    root: &Path,
    registry: QuestionRegistry,
) -> Result<QuestionRegistry, ProcessesError> {
    let kind = rk_kernel::runtime::tier_one::LANDING_REFUSED_KIND;
    if registry.binds_kind(kind) || registry.question(kind).is_some() {
        return Err(ProcessesError::Duplicate {
            path: processes_dir(root).display().to_string(),
            detail: format!(
                "the gate kind `{kind}` is the kernel's own (the refused landing's gate, ADR-0188), \
                 and a pack file may not declare or bind it"
            ),
        });
    }
    Ok(registry
        .with_question(kind, rk_kernel::runtime::tier_one::landing_question())
        .bind(kind, kind))
}

/// One `[question."<id>"]` table: the question, and the further gate kinds it binds.
fn question_from(id: &str, table: &toml::Value) -> Result<(Question, Vec<String>), String> {
    let table = table
        .as_table()
        .ok_or_else(|| format!("`question.\"{id}\"` is not a table"))?;
    for key in table.keys() {
        if !["text", "options", "binds"].contains(&key.as_str()) {
            return Err(format!(
                "`question.\"{id}\"` declares `{key}`, which is not one of `text`, `options`, `binds`"
            ));
        }
    }
    if id.trim().is_empty() {
        return Err("a question id is empty".to_owned());
    }
    let text = table
        .get("text")
        .and_then(toml::Value::as_str)
        .filter(|t| !t.trim().is_empty())
        .ok_or_else(|| format!("`question.\"{id}\"` declares no non-empty `text`"))?;
    let options = table
        .get("options")
        .and_then(toml::Value::as_array)
        .filter(|o| !o.is_empty())
        .ok_or_else(|| format!("`question.\"{id}\"` declares no non-empty `options` list"))?;
    let mut ids: Vec<String> = Vec::new();
    let mut attaching: Vec<(String, Vec<String>)> = Vec::new();
    for option in options {
        let (option_id, attaches) = option_from(id, option)?;
        if ids.contains(&option_id) {
            return Err(format!(
                "`question.\"{id}\"` declares the option `{option_id}` twice"
            ));
        }
        ids.push(option_id.clone());
        if !attaches.is_empty() {
            attaching.push((option_id, attaches));
        }
    }
    let mut question = Question::new(ids).with_text(text);
    for (option, kinds) in attaching {
        question = question.attaching(option, kinds);
    }
    let binds = match table.get("binds") {
        None => Vec::new(),
        Some(value) => value
            .as_array()
            .ok_or_else(|| format!("`question.\"{id}\"`'s `binds` is not a list"))?
            .iter()
            .map(|k| {
                k.as_str()
                    .filter(|k| !k.trim().is_empty())
                    .map(str::to_owned)
                    .ok_or_else(|| {
                        format!(
                            "`question.\"{id}\"`'s `binds` holds a member that is not a gate kind"
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?,
    };
    Ok((question, binds))
}

/// One option: a bare id, or `{ id, attaches }` with `attaches` a kind or a list of kinds.
fn option_from(id: &str, option: &toml::Value) -> Result<(String, Vec<String>), String> {
    if let Some(bare) = option.as_str() {
        if bare.trim().is_empty() {
            return Err(format!("`question.\"{id}\"` declares an empty option"));
        }
        return Ok((bare.to_owned(), Vec::new()));
    }
    let table = option.as_table().ok_or_else(|| {
        format!("`question.\"{id}\"` declares an option that is neither an id nor a table")
    })?;
    for key in table.keys() {
        if key != "id" && key != "attaches" {
            return Err(format!(
                "`question.\"{id}\"` declares an option with `{key}`; an option is `{{ id, attaches }}`"
            ));
        }
    }
    let option_id = table
        .get("id")
        .and_then(toml::Value::as_str)
        .filter(|o| !o.trim().is_empty())
        .ok_or_else(|| format!("`question.\"{id}\"` declares an option table with no `id`"))?;
    let attaches = match table.get("attaches") {
        None => Vec::new(),
        Some(toml::Value::String(kind)) => vec![kind.clone()],
        Some(toml::Value::Array(kinds)) => kinds
            .iter()
            .map(|k| {
                k.as_str().map(str::to_owned).ok_or_else(|| {
                    format!("`question.\"{id}\"`'s option `{option_id}` attaches a non-kind")
                })
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => {
            return Err(format!(
                "`question.\"{id}\"`'s option `{option_id}` declares `attaches` that is neither a kind nor a list"
            ));
        }
    };
    Ok((option_id.to_owned(), attaches))
}

/// **The first gate step of `activity` whose question no file declares.**
///
/// The `run` refusal R5 decision 8 names: a catalog `gate` step's kind is its `question` (ADR-0145),
/// and a kind the pack never bound would be answerable only as the built-in default question, which
/// is not the question the activity asks. `(step id, question)`.
#[must_use]
pub fn undeclared_gate_kind<'a>(
    registry: &QuestionRegistry,
    activity: &'a rk_kernel::catalog::Activity,
) -> Option<(&'a str, &'a str)> {
    activity.steps.iter().find_map(|step| {
        if step.kind != rk_kernel::catalog::StepKind::Gate {
            return None;
        }
        let question = step.question.as_deref()?;
        (!registry.binds_kind(question)).then_some((step.id.as_str(), question))
    })
}
