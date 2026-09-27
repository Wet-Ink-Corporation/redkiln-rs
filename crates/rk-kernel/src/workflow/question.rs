//! **The ADR-0063 question registry** — what a gate may be answered with, and what an answer must
//! carry (ADR-0129 decision 3; Step 7d Slice 3, the fourth of §7d.1's eight kernel edits; RT-29).
//!
//! ADR-0129 decision 3: *"A `pub` registry type lands in **`rk-kernel` at Step 7d** — a question id,
//! its option ids, and per option the artifact kinds it `attaches` — with the **default question
//! built in**: options `approve` and `changes-requested`, neither attaching anything. A gate step
//! that names no question is the default question."*
//!
//! **Why the kernel and not the CLI.** ADR-0063's Decision is a sentence about the kernel:
//! *"the kernel refuses to close the mark unless that attachment is supplied."* A registry that
//! lived in the composition root would make that refusal the root's, and a second caller — a future
//! API, a replay — could close a mark the record says must stay open. The CLI validates too, and
//! first, so an operator gets a usage error instead of a kernel refusal; but the CLI's check is a
//! courtesy and this one is the rule.
//!
//! **Pack-declared questions are Step 7e's.** At 7d the only question that exists in a file is the
//! default one, built in below; a non-default question exists **in-process only**, constructed by a
//! test. That is why [`QuestionRegistry::with_question`] and [`QuestionRegistry::bind`] are public
//! and why nothing here reads TOML: 7e's pack loader will call the same two seams.
//!
//! *(dated 2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* ~~"Pack-declared questions
//! are Step 7e's"~~: no 7e loader was built (ADR-0156 blocker 2: `.redkiln/processes/` was empty).
//! The loader is `s8d`'s (D21-43: *"The loader and `.redkiln/processes/` are **s8d**'s."*) and it
//! is `rk_cli::processes`, which reads `.redkiln/processes/*.toml` and calls the same two seams.
//! This module still reads no TOML: `rk-kernel` opens no files.

use std::collections::BTreeMap;

/// The question every gate resolves to unless a pack binds it otherwise.
pub const DEFAULT_QUESTION: &str = "default";

/// The two options the default question declares, in the order a reader meets them.
pub const DEFAULT_OPTIONS: [&str; 2] = ["approve", "changes-requested"];

/// One artifact a human supplied when answering.
///
/// **The kernel validates by `kind` and stores by `path`**, and holds neither as a filesystem type:
/// `rk-kernel` opens no files (AC-001's sibling rule), so a `PathBuf` here would imply a capability
/// this crate does not have. The composition root converts from its own `Attachment`, which does
/// carry a `PathBuf`, at the boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    /// The artifact kind name, matched against the chosen option's `attaches`.
    pub kind: String,
    /// Where the root came from, carried opaquely for the record.
    pub path: String,
}

/// Why an option, or the attachments presented with it, were refused.
///
/// Three variants and they are deliberately distinct: `ac410_…` asserts the attachment arms and
/// says *"never `UnknownOption`"*, so collapsing them would make that assertion unfalsifiable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionError {
    /// The question does not declare this option (ADR-0129 decision 3's *"An option the gate's
    /// question does not declare is refused"*).
    UnknownOption { question: String, option: String },
    /// An attachment was presented whose kind no declared option admits (ADR-0129 decision 4).
    AttachmentNotAdmitted { kind: String },
    /// The chosen option declares `attaches = <Kind>` and no attachment of that kind was supplied.
    /// **This is ADR-0063's refusal**, and it is the kernel's.
    AttachmentMissing { kind: String },
}

impl std::fmt::Display for OptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOption { question, option } => write!(
                f,
                "the question `{question}` does not declare the option `{option}`"
            ),
            Self::AttachmentNotAdmitted { kind } => write!(
                f,
                "no declared option admits an attachment of kind `{kind}`"
            ),
            Self::AttachmentMissing { kind } => write!(
                f,
                "the chosen option attaches `{kind}` and no attachment of that kind was supplied; \
                 the mark stays open (ADR-0063)"
            ),
        }
    }
}

impl std::error::Error for OptionError {}

/// One question: its options, and per option the artifact kinds it attaches.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Question {
    options: BTreeMap<String, Vec<String>>,
    /// *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* The question's text, as
    /// its pack file declares it. `None` for the built-in default and for an in-process question a
    /// test constructs. Carried for the reader; nothing here decides on it.
    text: Option<String>,
}

impl Question {
    /// A question declaring `options`, none of which attaches anything.
    #[must_use]
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            options: options
                .into_iter()
                .map(|o| (o.into(), Vec::new()))
                .collect(),
            text: None,
        }
    }

    /// The question with its declared text (R5 = ADR-0177 decision 7).
    #[must_use]
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// The declared text, when the question has one.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// Declare that `option` attaches `kinds`.
    #[must_use]
    pub fn attaching<S, I, K>(mut self, option: S, kinds: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = K>,
        K: Into<String>,
    {
        self.options
            .insert(option.into(), kinds.into_iter().map(Into::into).collect());
        self
    }

    /// The option ids this question declares, sorted.
    #[must_use]
    pub fn options(&self) -> Vec<&str> {
        self.options.keys().map(String::as_str).collect()
    }

    /// The kinds `option` attaches, or `None` if the question does not declare it.
    #[must_use]
    pub fn attaches(&self, option: &str) -> Option<&[String]> {
        self.options.get(option).map(Vec::as_slice)
    }
}

/// Question ids to questions, plus the gate-kind bindings that resolve one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionRegistry {
    questions: BTreeMap<String, Question>,
    /// Gate kind → question id. A kind with no binding resolves to [`DEFAULT_QUESTION`], which is
    /// ADR-0129 decision 3's *"A gate step that names no question is the default question."*
    bindings: BTreeMap<String, String>,
}

impl Default for QuestionRegistry {
    /// The registry as it exists with no pack loaded: **exactly** the default question, its two
    /// options, neither attaching anything.
    fn default() -> Self {
        Self {
            questions: BTreeMap::from([(
                DEFAULT_QUESTION.to_owned(),
                Question::new(DEFAULT_OPTIONS),
            )]),
            bindings: BTreeMap::new(),
        }
    }
}

impl QuestionRegistry {
    /// Add a question. **The seam 7e's pack loader will call**; at 7d its only callers are tests.
    #[must_use]
    pub fn with_question<S: Into<String>>(mut self, id: S, question: Question) -> Self {
        self.questions.insert(id.into(), question);
        self
    }

    /// Bind a gate kind to a question id — the resolution seam.
    ///
    /// The default binding is the absence of one: a kind nobody bound resolves to the default
    /// question, so `unknown_facet:*` gates answer `approve` / `changes-requested` without any pack
    /// having said so.
    #[must_use]
    pub fn bind<K: Into<String>, Q: Into<String>>(mut self, gate_kind: K, question_id: Q) -> Self {
        self.bindings.insert(gate_kind.into(), question_id.into());
        self
    }

    /// Which question a gate of this kind is answered with.
    #[must_use]
    pub fn question_for(&self, gate_kind: &str) -> &str {
        self.bindings
            .get(gate_kind)
            .map_or(DEFAULT_QUESTION, String::as_str)
    }

    /// The question under `id`, if the registry declares one.
    #[must_use]
    pub fn question(&self, id: &str) -> Option<&Question> {
        self.questions.get(id)
    }

    /// *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177 decision 8, proposed.)* Has a pack
    /// BOUND this gate kind to a question? A catalog `gate` step's kind is its `question`
    /// (ADR-0145), and the loader binds every declared question id as its own kind, so a gate step
    /// whose question the pack never declared is unbound here. The default binding (the absence of
    /// one) is for the selection's `unknown_facet:*` gates, never for a catalog gate step: `run`
    /// refuses an unbound catalog gate kind by name.
    #[must_use]
    pub fn binds_kind(&self, gate_kind: &str) -> bool {
        self.bindings.contains_key(gate_kind)
    }

    /// Every question id the registry declares, the built-in default included, sorted.
    #[must_use]
    pub fn question_ids(&self) -> Vec<&str> {
        self.questions.keys().map(String::as_str).collect()
    }

    /// **Is this option admissible for this question, with these attachments?**
    ///
    /// Three refusals, checked in an order that is itself a decision: the option first (an
    /// attachment argument about an option that does not exist is noise), then the attachments
    /// presented but not admitted, then the attachments declared but not supplied. An operator who
    /// got all three at once would have to guess which to fix first.
    ///
    /// # Errors
    /// [`OptionError`] naming the option or the kind. Never a generic refusal: the CLI turns each
    /// of these into a usage error with zero events, and it can only do that if it can tell them
    /// apart.
    pub fn validate(
        &self,
        question_id: &str,
        option_id: &str,
        attachments: &[Attachment],
    ) -> Result<(), OptionError> {
        let question =
            self.questions
                .get(question_id)
                .ok_or_else(|| OptionError::UnknownOption {
                    question: question_id.to_owned(),
                    option: option_id.to_owned(),
                })?;
        let attaches = question
            .attaches(option_id)
            .ok_or_else(|| OptionError::UnknownOption {
                question: question_id.to_owned(),
                option: option_id.to_owned(),
            })?;
        Self::check_attachments(attaches, attachments)
    }

    /// **The attachment rule alone** — ADR-0063's sentence, and the only half the KERNEL enforces.
    ///
    /// The two records split this deliberately, and it took a broken build to notice. ADR-0129
    /// decision 3: *"An option the gate's question does not declare is refused **at the CLI
    /// boundary** as a usage error with zero events."* ADR-0063's Decision, by contrast, is about
    /// what must be SUPPLIED: *"the kernel refuses to close the mark unless that attachment is
    /// supplied."* So an unknown option is the composition root's refusal — exit `2`, before any
    /// event — and a missing attachment is the kernel's.
    ///
    /// Conflating them puts an `UnknownOption` refusal inside `gate::answer`, where it refuses every
    /// verdict the default question does not name. Measured: that turned four `gates.rs` tests red
    /// whose verdict is `"proceed"`, against §7d.15's *"their assertions stand unchanged"* — which is
    /// what surfaced the distinction. An option this registry does not declare attaches nothing, so
    /// there is nothing here to require, and the kernel does not second-guess the boundary.
    ///
    /// # Errors
    /// [`OptionError::AttachmentNotAdmitted`] or [`OptionError::AttachmentMissing`]. Never
    /// [`OptionError::UnknownOption`] — that one is the CLI's.
    pub fn validate_attachments(
        &self,
        question_id: &str,
        option_id: &str,
        attachments: &[Attachment],
    ) -> Result<(), OptionError> {
        let attaches = self
            .questions
            .get(question_id)
            .and_then(|q| q.attaches(option_id))
            .unwrap_or(&[]);
        Self::check_attachments(attaches, attachments)
    }

    fn check_attachments(
        attaches: &[String],
        attachments: &[Attachment],
    ) -> Result<(), OptionError> {
        for supplied in attachments {
            if !attaches.contains(&supplied.kind) {
                return Err(OptionError::AttachmentNotAdmitted {
                    kind: supplied.kind.clone(),
                });
            }
        }
        for required in attaches {
            if !attachments.iter().any(|a| &a.kind == required) {
                return Err(OptionError::AttachmentMissing {
                    kind: required.clone(),
                });
            }
        }
        Ok(())
    }
}
