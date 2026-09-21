//! **The role → card resolver `rk-cli` authors for `doctor`** — AC-416, `s7d` §7d.8's row.
//!
//! *"A role resolves to zero cards, or to two cards at equal selector specificity"* — yielding
//! [`Resolution::Unresolved`] or [`Resolution::Ambiguous`], the two `RK-D-ROLE-*` rows. The same
//! reader serves `RK-D-DANGLING-ROLE` / `-SLOT` / `-ATTACH` (AC-420), which ask the opposite
//! question: a reference to a role no enabled pack declares.
//!
//! # 🛑 This is NOT the binder's resolver and NOT the kernel's
//!
//! §7d.8 says so in terms — *"**not** the binder's error variants and **not** the kernel's
//! resolver"*. `rk_kernel::select::Pack::require_family_for_role` answers a different question
//! (which model family a role runs on) and `Binder::bind` answers a third (which card a *bound
//! node* takes). `doctor`'s question is about the pack's declarations alone, with nothing selected
//! and nothing bound, which is what makes it configuration-time and zero-dispatch (AC-098).
//!
//! # The field set is a FLOOR, and 7e may re-point it
//!
//! §7d.13 fixes exactly what these fixtures spell: *"a card file's `fulfils: [<role>]` and a skill
//! file's `attach.roles: [<role>]`, both under the pack's `cards/` and `skills/` directories — 7e's
//! pack schema may re-point it."* So the two structs below carry those keys and **no more**, and
//! `#[serde(deny_unknown_fields)]` is deliberately NOT used: the pack schema is Step 7e's, a card
//! carries fields this reader has no business refusing (`tools`, `model`, `effort` — the
//! `RK-D-CARD-TOOLS` and `RK-D-TIER-LEAK` rows read those, and they are cycle 5's), and a reader
//! that refused them would turn every real pack into a parse error. `ConfigFile` denies unknown
//! fields because it owns its whole schema; this reader owns one key of someone else's.
//!
//! # 🚧 CYCLE 2: the types and the signature are real, [`resolve`] returns the empty answer
//!
//! Not `todo!()` — this crate's rule, `verbs/mod.rs`'s header. Cycle 5 fills it, and the family
//! test's red is then *"the rule found no card where the fixture declares one"*, which is
//! behavioural.

use std::path::Path;

use serde::Deserialize;

/// A pack's `cards/<name>.toml`, read for **one key**. See the header on why unknown fields are
/// admitted rather than denied.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct AuthoredCard {
    /// The roles this card fulfils. Absent is an empty list and **not** an error: a card that
    /// declares none is a card no role resolves to, which is `RK-D-ROLE-UNRESOLVED`'s input, not a
    /// malformed file.
    #[serde(default)]
    pub fulfils: Vec<String>,
}

/// A pack's `skills/<name>.toml`, read for `attach.roles`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct AuthoredSkill {
    /// `[attach]`. Absent is the empty attachment, on `fulfils`' reasoning.
    #[serde(default)]
    pub attach: Attach,
}

/// `[attach]` on a skill file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Attach {
    /// The roles this skill attaches to.
    #[serde(default)]
    pub roles: Vec<String>,
}

/// What a role resolved to.
///
/// **Three arms and the two defects are distinct**, because §7d.8 gives them distinct ids:
/// `RK-D-ROLE-UNRESOLVED` and `RK-D-ROLE-AMBIGUOUS` are separate rows with separate fixtures, and
/// AC-415 joins each id to its own directory. One `Err` covering both would make the two rows one
/// rule wearing two names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// Exactly one card at the winning specificity — the lawful case.
    Resolved(String),
    /// Zero cards fulfil the role. `RK-D-ROLE-UNRESOLVED`.
    Unresolved,
    /// Two or more cards at **equal** selector specificity. Carries them so the finding can name
    /// them: a report saying only *"ambiguous"* leaves the operator to find the pair.
    Ambiguous(Vec<String>),
}

/// Every card and skill declaration a pack root holds, read once.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Declarations {
    /// `(card name, the roles it fulfils)`, in path order.
    pub cards: Vec<(String, Vec<String>)>,
    /// `(skill name, the roles it attaches to)`, in path order.
    pub skills: Vec<(String, Vec<String>)>,
}

/// Read a pack root's `cards/` and `skills/` directories.
///
/// **An absent directory is an empty list, never an error.** A pack with no `skills/` has declared
/// no skills; that is the input to `RK-D-DANGLING-ATTACH` finding nothing, not a refusal. A pack
/// root that does not exist at all is a different fact and is `RK-D-DEAD-DEF`'s.
///
/// 🚧 Cycle 2 — the signature is real; cycle 5 reads the files.
#[must_use]
pub fn read_declarations(pack_root: &Path) -> Declarations {
    let _ = pack_root;
    Declarations::default()
}

/// Resolve one role against a pack's declarations.
///
/// 🚧 Cycle 2 — the signature is real; cycle 5 fills it. It returns [`Resolution::Unresolved`],
/// which is the honest answer over the empty [`Declarations`] this cycle's reader produces: no card
/// declares anything, so no role resolves. Cycle 5's red is the family test finding no card where
/// its fixture declares one.
#[must_use]
#[expect(
    clippy::missing_const_for_fn,
    reason = "const only because the cycle-2 body is empty — cycle 5 walks the declarations and               builds an `Ambiguous` vector. The expectation goes UNFULFILLED when that lands,               which is what tells that cycle to delete this attribute."
)]
pub fn resolve(declarations: &Declarations, role: &str) -> Resolution {
    let _ = (declarations, role);
    Resolution::Unresolved
}
