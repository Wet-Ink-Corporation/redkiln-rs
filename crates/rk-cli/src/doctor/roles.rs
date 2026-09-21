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
//! # 🛑 `fulfils` IS IN A SIDECAR AND NOT ON THE CARD, AND THAT IS MEASURED RATHER THAN CHOSEN
//!
//! §7d.13 fixes the floor as *"a card file's `fulfils: [<role>]` and a skill file's
//! `attach.roles: [<role>]`, both under the pack's `cards/` and `skills/` directories — 7e's pack
//! schema may re-point it."* **The card half cannot be built as written**, and the skill half can.
//!
//! Measured 2026-09-21: `rk_adapter_claude::card::CardFile` is `#[serde(deny_unknown_fields)]` over
//! exactly seven names — `description`, `prompt`, `tools`, `cache_profile`, `permission_mode`,
//! `model`, `effort` — and `fulfils` is refused as an unknown field. `s7d` §7d.1 permits **one
//! closed crate edited** (`rk-kernel`), so widening `CardFile` is foreclosed; and a card carrying
//! `fulfils` would be a shape no production pack could use, since `run`'s binder loads every card
//! through that type.
//!
//! **Slice 3 hit this and handed it to this slice by name**, in
//! `crates/rk-cli/tests/fixtures/pack-min/cards/README.md`: *"Slice 4 must either widen `CardFile`
//! or carry the role→card map somewhere other than the card."* This is the second, taken on the
//! operator's decision of 2026-09-21: **`.redkiln/roles.toml`**, read here and by nothing else.
//!
//! **`selector` is not decoration.** AC-416's second half is *"two cards at **equal selector
//! specificity**"*. A bare list of card names has no specificity, so `RK-D-ROLE-AMBIGUOUS` would be
//! a row that could never fire and `ac415_`'s rejecting fixture for it could not exist.
//!
//! The **skill** half is spelled exactly as §7d.13 writes it, because nothing outside this module
//! reads skills at all [measured 2026-09-21] — no closed-crate schema collides with it.
//!
//! `#[serde(deny_unknown_fields)]` is deliberately NOT used on either type: the pack schema is Step
//! 7e's and may add keys this reader has no business refusing. `ConfigFile` denies unknown fields
//! because it owns its whole schema; this reader owns two keys of someone else's.
//!
//! # 🚧 CYCLE 2/4: the types and the signatures are real, [`resolve`] returns the empty answer
//!
//! Not `todo!()` — this crate's rule, `verbs/mod.rs`'s header. Cycle 5 fills it, and the family
//! test's red is then *"the rule found no card where the fixture declares one"*, which is
//! behavioural.

use std::path::Path;

use serde::Deserialize;

/// `.redkiln/roles.toml` — the role→card map. See the header on why it is not on the card.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct RolesFile {
    /// The `[[card]]` entries. Absent is an empty list and **not** an error: a repository that
    /// declares no card is one where every role is `Unresolved`, which is a finding rather than a
    /// malformed file.
    #[serde(default)]
    pub card: Vec<CardEntry>,
}

/// One `[[card]]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CardEntry {
    /// The card's path, relative to `.redkiln/` — `cards/<role>.json` in the ordinary case, which
    /// is also the file name `run`'s binder resolves positionally.
    pub file: String,
    /// The roles this card fulfils.
    #[serde(default)]
    pub fulfils: Vec<String>,
    /// The selector this card is chosen under. **Two cards fulfilling one role at the SAME
    /// selector is AC-416's ambiguity**; at different selectors the more specific wins.
    #[serde(default)]
    pub selector: String,
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

/// Every card and skill declaration a repository holds, read once.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Declarations {
    /// `.redkiln/roles.toml`'s entries, in declared order.
    pub cards: Vec<CardEntry>,
    /// `(skill name, the roles it attaches to)`, in path order.
    pub skills: Vec<(String, Vec<String>)>,
}

/// Read a repository's `.redkiln/roles.toml` and `.redkiln/skills/`.
///
/// **An absent file or directory is an empty list, never an error.** A repository with no
/// `skills/` has declared no skills; that is the input to `RK-D-DANGLING-ATTACH` finding nothing,
/// not a refusal. A `.redkiln/` that does not exist at all is a different fact and the verb
/// refuses on it before any rule runs.
#[must_use]
pub fn read_declarations(redkiln_dir: &Path) -> Declarations {
    let cards = std::fs::read_to_string(redkiln_dir.join("roles.toml"))
        .ok()
        .and_then(|src| toml::from_str::<RolesFile>(&src).ok())
        .map(|f| f.card)
        .unwrap_or_default();

    let mut skills = Vec::new();
    if let Ok(entries) = std::fs::read_dir(redkiln_dir.join("skills")) {
        let mut paths: Vec<std::path::PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        paths.sort();
        for path in paths {
            let Some(name) = path.file_name().map(|n| n.to_string_lossy().into_owned()) else {
                continue;
            };
            let Ok(src) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(skill) = toml::from_str::<AuthoredSkill>(&src) else {
                continue;
            };
            skills.push((name, skill.attach.roles));
        }
    }
    Declarations { cards, skills }
}

/// How specific a selector is: the number of comma-separated clauses it carries.
///
/// 🛑 **A READING, and 7e's pack schema may re-point it.** Nothing in any record defines selector
/// specificity, and AC-416 needs an ordering to mean *"two cards at EQUAL specificity"* at all.
/// Clause count is the reading taken here: `"stage:review"` is one clause and
/// `"stage:review,kind:bug"` is two, so the second wins and the role resolves. It is stated rather
/// than assumed because the alternative — treating every selector as equally specific — makes the
/// word *equal* do no work and turns `RK-D-ROLE-AMBIGUOUS` into *"a role with two cards"*, which
/// `ac416_`'s own control is written to catch.
#[must_use]
pub fn specificity(selector: &str) -> usize {
    selector
        .split(',')
        .filter(|clause| !clause.trim().is_empty())
        .count()
}

/// Resolve one role against a pack's declarations.
///
/// 🚧 Cycle 2 — the signature is real; cycle 5 fills it. It returns [`Resolution::Unresolved`],
/// which is the honest answer over the empty [`Declarations`] this cycle's reader produces: no card
/// declares anything, so no role resolves. Cycle 5's red is the family test finding no card where
/// its fixture declares one.
#[must_use]
pub fn resolve(declarations: &Declarations, role: &str) -> Resolution {
    let candidates: Vec<&CardEntry> = declarations
        .cards
        .iter()
        .filter(|c| c.fulfils.iter().any(|r| r == role))
        .collect();

    let Some(most) = candidates.iter().map(|c| specificity(&c.selector)).max() else {
        return Resolution::Unresolved;
    };
    // Only the cards at the WINNING specificity contend. A less specific card beside a more
    // specific one is not an ambiguity — it is the ordering doing its job.
    let mut winners: Vec<String> = candidates
        .iter()
        .filter(|c| specificity(&c.selector) == most)
        .map(|c| c.file.clone())
        .collect();
    winners.sort();

    match winners.len() {
        0 => Resolution::Unresolved,
        1 => Resolution::Resolved(winners.remove(0)),
        _ => Resolution::Ambiguous(winners),
    }
}
