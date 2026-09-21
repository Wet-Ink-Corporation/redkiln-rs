//! **The rule table, and it IS data** — AC-415's own words: *"the `doctor` rule table, which is
//! data with a stable id per rule"*. `s7d` §7d.8.
//!
//! The shape is the stage file's, transcribed: `Rule { id: &'static str, severity: Severity, check:
//! fn(&Ctx) -> Vec<Finding> }` and `pub static RULES: &[Rule]`, run in order by `verbs/doctor.rs`.
//!
//! # Why `fn` and not `Box<dyn Fn>`
//!
//! A plain function pointer is what makes the table a `static` at all: a `&[Rule]` in read-only
//! memory with no allocation and no initialisation order. It is also what makes *"the table is
//! data"* checkable rather than aspirational — a rule cannot capture a `Ctx`, a clock or a store
//! handle, because a `fn` item captures nothing. Everything a rule reads it reads through its one
//! parameter, which is the property `ac414_…` depends on when it builds a `Ctx` itself and runs the
//! table over it.
//!
//! # 🚧 CYCLE 3 FILLED THE TABLE. IT DID NOT END THE VACUITY, AND THE DIFFERENCE IS THE POINT
//!
//! §7d.14: *"over an empty rule table AC-414's per-fixture join, AC-415's per-id join and SM-121's
//! dispatch count"* are all **vacuous**. Cycle 2 landed [`RULES`] empty and claimed none of the
//! three. Cycle 3 grew it to all thirty-eight ids and landed
//! `doctor_rule_table_covers_every_register_named_row` — the twenty-nine register-named ids as a
//! floor (RA-M-01) — **watched red on the empty table first**, on *"`RULES` is empty"*, which is
//! the vacuity stated as a failure rather than as a comment.
//!
//! **What that test buys, and what it does not.** It buys the floor: no criterion in the register's
//! §9, §12 or §14 names a rule id the table lacks. It does **not** buy AC-414, AC-415 or SM-121,
//! and none of the three is claimed yet — every `check` is [`unbuilt`] until cycle 5, and a table
//! of thirty-eight rules that all find nothing satisfies a per-**id** join exactly as an empty one
//! does. A floor over ids cannot see that, because ids are all it reads. What sees it is cycle 4's
//! `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through **its own rule
//! only** and demands a finding — the first assertion in this slice that a rule which does nothing
//! cannot pass.
//!
//! **Thirty-eight, not thirty-two.** §7d.8 and §7d.13 both say thirty-eight and an independent
//! count off the table's first column agrees (39 `RK-D-*` tokens less the prose prefix
//! `RK-D-DANGLING-` of the compound cell `RK-D-DANGLING-{ROLE,SLOT,ATTACH}`). §7d.16 item 4 said
//! *thirty-two* and was corrected in the stage file on 2026-09-20: D18-19 moved the table 32 → 38
//! on 2026-09-19 and that item was not updated with it.

use super::{Ctx, Finding, Severity};

/// The placeholder every `check` points at until cycle 5 writes its predicate.
///
/// 🚧 **One shared function and not thirty-eight identical stubs**, because thirty-eight bodies
/// that all return nothing are thirty-eight places to forget the same thing, and because the thing
/// that catches a row still pointing here is not a reader: it is cycle 4's
/// `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through its own rule
/// alone and demands a finding. A row left on this function fails it **by id**.
#[expect(
    clippy::missing_const_for_fn,
    reason = "const only because the cycle-3 placeholder returns nothing — cycle 5 replaces every               reference to it with a predicate that reads the `Ctx` and allocates. The expectation               goes UNFULFILLED when the last row leaves it, which is what says this function can               be deleted."
)]
fn unbuilt(_: &Ctx) -> Vec<Finding> {
    Vec::new()
}

/// One row of the table.
#[derive(Debug, Clone, Copy)]
pub struct Rule {
    /// The stable `RK-D-*` id. `&'static str` because AC-415 joins it against a fixture
    /// **directory name**, and a per-run `String` would make that join a comparison of two things
    /// built at run time rather than of a table against the tree.
    pub id: &'static str,
    /// What a finding from this rule weighs.
    pub severity: Severity,
    /// The check. Reads its `Ctx` and nothing else; returns every finding it has, never the first.
    pub check: fn(&Ctx) -> Vec<Finding>,
}

/// **The table** — the thirty-eight ids of `s7d` §7d.8, in its own row order, which is the order
/// `verbs/doctor.rs` evaluates them in.
///
/// **Not transcribed by hand.** The rows were walked out of §7d.8's table's first column, with its
/// compound cells split, by a script that refused to write unless the walk yielded exactly 38
/// distinct ids — so the table here and the table in the runbook cannot drift through a
/// transcription step nobody re-ran. Halt 7: count it off the table, and re-count after any edit.
///
/// **Severities, counted off the same walk** [measured 2026-09-21]: **32** `Error`, **5**
/// `Warning` (`RK-D-DEAD-DEF`, `RK-D-UNFILLED-SLOT`, `RK-D-UNREAD`, `RK-D-HEADROOM-MARGIN`,
/// `RK-D-QUARANTINE-LIVE`) and **1** `Ok` (`RK-D-REBUILT-TIER2`). The three ids of the compound
/// `WORKTREE-ORPHAN / NETWORK-FS / CREDENTIAL-RETAINED` cell are `Error` because design §8.4 fixes
/// them so and the cell says the design outranks the runbook (ADR-0092 clause 2.5).
///
/// 🚧 **Cycle 3: every `check` is [`unbuilt`]** — the ids and their severities are real, the
/// predicates are cycle 5's. `doctor_rule_table_covers_every_register_named_row` is green on this
/// table; **AC-414, AC-415 and SM-121 are still not claimed**, because a table of rules that find
/// nothing satisfies a per-id join exactly as an empty one does. What ends that is cycle 4's
/// `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through **its own rule
/// only** and requires at least one finding — so a row still pointing at [`unbuilt`] when cycle 5
/// is done is red there, by id.
pub static RULES: &[Rule] = &[
    Rule {
        id: "RK-D-GATE-OUTPUTS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-HOOKS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-STORE-ID",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-STRAY-STORE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ROLE-UNRESOLVED",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ROLE-AMBIGUOUS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CARD-TOOLS",
        severity: Severity::Error,
        check: card_tools,
    },
    Rule {
        id: "RK-D-CARD-TOOLS-ABSENT",
        severity: Severity::Error,
        check: card_tools_absent,
    },
    Rule {
        id: "RK-D-TIER-LEAK",
        severity: Severity::Error,
        check: tier_leak,
    },
    Rule {
        id: "RK-D-TIMEOUT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-ROLE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-SLOT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-ATTACH",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DEAD-DEF",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-UNFILLED-SLOT",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ENV-DENYLIST",
        severity: Severity::Error,
        check: env_denylist,
    },
    Rule {
        id: "RK-D-CLI-VERSION",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-TTL",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-UNKNOWN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-VALUE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-NEW",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-FACT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ADAPTER",
        severity: Severity::Error,
        check: adapter,
    },
    Rule {
        id: "RK-D-UNREAD",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-AGENTS-WITHOUT-AGENT",
        severity: Severity::Error,
        check: agents_without_agent,
    },
    Rule {
        id: "RK-D-CACHE-PROFILE",
        severity: Severity::Error,
        check: cache_profile,
    },
    Rule {
        id: "RK-D-MARGIN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-HEADROOM-MARGIN",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-REBUILT-TIER2",
        severity: Severity::Ok,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-WORKTREE-ORPHAN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-NETWORK-FS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CREDENTIAL-RETAINED",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-REVIEW-REEXEC",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-QUARANTINE-LIVE",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-SINK-COUNT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ADVISORY-LEDGER",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CORPUS-SHA",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-VERIFICATION-POLICY",
        severity: Severity::Error,
        check: unbuilt,
    },
];

/// Evaluate every rule in table order, into a [`super::Report`].
///
/// **Every rule is evaluated and every rule is recorded as evaluated**, whether or not it found
/// anything — §7d.8's stdout is *"one line per **evaluated** rule — fired or not"*, and the
/// checkpoint block's FLOOR loop greps the register-named ids off a **clean** repository. A loop
/// that recorded only the rules that fired would make that loop find nothing on a green tree.
///
/// **It takes the harness so a dispatch is attributable.** [`crate::no_dispatch::NoDispatchHarness`]
/// panics naming the rule that reached it, and the only way it can know the rule is to be told
/// before each check. AC-414's instrument is that panic; this is what makes its message useful.
#[must_use]
pub fn evaluate(ctx: &Ctx, harness: &crate::no_dispatch::NoDispatchHarness) -> super::Report {
    let mut report = super::Report::default();
    for rule in RULES {
        harness.entering(rule.id);
        report.evaluated.push(rule.id);
        report.findings.extend((rule.check)(ctx));
    }
    report
}

// ============================================================================ batch 1: the card
// ============================================================================ and config rows
//
// Six families, seven ids. Each reads the `ConfigFile`, the captured environment, or a card file.
// Nothing here folds a journal or parses a predicate, which is why they land together.

/// One card as **`doctor`** reads it — permissive on purpose.
///
/// 🛑 **Not `rk_adapter_claude::card::CardFile`**, and the reason is what these rules are for.
/// `CardFile` is `#[serde(deny_unknown_fields)]`, so a card carrying one unknown key fails to
/// parse — and a rule built on it would then report **nothing** about a card that also omits
/// `StructuredOutput`. The defect would hide behind the typo. A doctor reader that refuses less
/// than the loader finds more, which is the asymmetry between a linter and a loader.
#[derive(Debug, Clone, Default, serde::Deserialize)]
struct CardView {
    #[serde(default)]
    description: String,
    #[serde(default)]
    tools: Option<Vec<String>>,
    #[serde(default)]
    cache_profile: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    effort: Option<String>,
}

/// Every `.redkiln/cards/*.json`, by file name, in path order.
///
/// **An unreadable or unparseable card is SKIPPED, not reported here.** Each row below says one
/// thing about a card it could read; *"this file is not a card"* is a different finding and belongs
/// to whichever row a record gives it. Inventing it here would put a defect under an id whose
/// fixture does not carry it, and `ac415_` would then pass for the wrong reason.
fn cards(ctx: &Ctx) -> Vec<(String, CardView)> {
    let dir = ctx.redkiln().join("cards");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut paths: Vec<std::path::PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    paths
        .iter()
        .filter_map(|p| {
            let name = p.file_name()?.to_string_lossy().into_owned();
            let text = std::fs::read_to_string(p).ok()?;
            let card: CardView = serde_json::from_str(&text).ok()?;
            Some((name, card))
        })
        .collect()
}

/// One finding, spelled once so every row reads the same shape.
const fn finding(id: &'static str, severity: Severity, detail: String) -> Finding {
    Finding {
        id,
        severity,
        detail,
    }
}

/// **`RK-D-ADAPTER`** — AC-486, AC-438's doctor clause.
///
/// It calls `AdapterSet::resolve` and reports the refusal **as a finding**, rather than carrying a
/// second copy of the shipped-adapter list. §7d.8 forbids `doctor` calling it *as a startup
/// refusal* — that is `run`'s step (4) — not calling it at all, and the difference is why this id
/// is observable on the real binary: a `doctor` that exited on the refusal would print no row.
///
/// **Only the `UnknownAdapter` arm.** `resolve` also refuses a denylisted environment, which is
/// [`env_denylist`]'s row; a rule reporting both would fire twice over one defect and make each
/// fixture reject for two reasons.
fn adapter(ctx: &Ctx) -> Vec<Finding> {
    match crate::adapters::AdapterSet::resolve(&ctx.config, &ctx.env) {
        Ok(_) | Err(crate::adapters::Refusal::DenylistedEnv { .. }) => Vec::new(),
        Err(refusal) => vec![finding(
            "RK-D-ADAPTER",
            Severity::Error,
            refusal.to_string(),
        )],
    }
}

/// **`RK-D-ENV-DENYLIST`** — AC-422, AC-488.
///
/// The one denylist is `rk_adapter_claude::env_denylist`, read here and copied nowhere: AC-422's
/// source ban over `crates/rk-cli/src` exists precisely to keep a second list out of this crate,
/// and a rule that spelled the names would be the thing the ban forbids.
fn env_denylist(ctx: &Ctx) -> Vec<Finding> {
    rk_adapter_claude::env_denylist::first_denylisted(ctx.env.as_map()).map_or_else(
        Vec::new,
        |name| {
            vec![finding(
                "RK-D-ENV-DENYLIST",
                Severity::Error,
                format!(
                    "the captured environment carries `{name}`, which is refused: a dispatch runs \
                     on the subscription session and an API-key-mode fallback is not an allowed \
                     degradation path (ADR-0010)"
                ),
            )]
        },
    )
}

/// **`RK-D-CARD-TOOLS`** — AC-417, AC-205, AC-493's doctor halves: a card whose `tools` list omits
/// `StructuredOutput`.
///
/// **A card with NO `tools` key is not this row** — it is [`card_tools_absent`], and the two are
/// separate ids because the remedy differs: an allowlist to edit, versus no allowlist at all.
fn card_tools(ctx: &Ctx) -> Vec<Finding> {
    cards(ctx)
        .into_iter()
        .filter_map(|(name, card)| {
            let tools = card.tools?;
            (!tools.iter().any(|t| t == "StructuredOutput")).then(|| {
                finding(
                    "RK-D-CARD-TOOLS",
                    Severity::Error,
                    format!(
                        "`cards/{name}` declares tools {tools:?}, which omit `StructuredOutput` — \
                         the effective tool set is the card's declared list and nothing else, so \
                         the activity cannot return a structured result"
                    ),
                )
            })
        })
        .collect()
}

/// **`RK-D-CARD-TOOLS-ABSENT`** — AC-417, AC-205: a card with no `tools:` key at all.
fn card_tools_absent(ctx: &Ctx) -> Vec<Finding> {
    cards(ctx)
        .into_iter()
        .filter(|(_, card)| card.tools.is_none())
        .map(|(name, _)| {
            finding(
                "RK-D-CARD-TOOLS-ABSENT",
                Severity::Error,
                format!(
                    "`cards/{name}` declares no `tools` key. `CardFile.tools` is optional, so the \
                     card loads — the absence is a configuration fact and not a parse failure"
                ),
            )
        })
        .collect()
}

/// **`RK-D-CACHE-PROFILE`** — ADR-0113's doctor half (`s7c` ARMED).
fn cache_profile(ctx: &Ctx) -> Vec<Finding> {
    cards(ctx)
        .into_iter()
        .filter(|(_, card)| card.cache_profile.is_none())
        .map(|(name, _)| {
            finding(
                "RK-D-CACHE-PROFILE",
                Severity::Error,
                format!("`cards/{name}` declares no `cache_profile`"),
            )
        })
        .collect()
}

/// **`RK-D-AGENTS-WITHOUT-AGENT`** — AC-203's doctor half (`s7c` ARMED): a card whose binding
/// yields no agent name.
///
/// The name comes off `description`, and `CardFile.description` is a `String` — an empty one is a
/// **value**, so the card loads and the bound `--agents` object has no name to key on. Trimmed
/// before the test, because a description of one space is as nameless as one of none.
fn agents_without_agent(ctx: &Ctx) -> Vec<Finding> {
    cards(ctx)
        .into_iter()
        .filter(|(_, card)| card.description.trim().is_empty())
        .map(|(name, _)| {
            finding(
                "RK-D-AGENTS-WITHOUT-AGENT",
                Severity::Error,
                format!(
                    "`cards/{name}` has an empty `description`, so its bound `--agents` object has \
                     no name to key on"
                ),
            )
        })
        .collect()
}

/// **`RK-D-TIER-LEAK`** — AC-418, AC-212's doctor half: *"any card or slot filler also sets a model
/// or effort"*.
///
/// **Both carriers, because the clause names both.** The tier is the role's (`[role.<name>]` in the
/// pack); a card or a filler that states one has taken a decision the pack owns. A rule that read
/// cards alone would pass a repository whose leak is in `slots.toml`.
fn tier_leak(ctx: &Ctx) -> Vec<Finding> {
    let mut out: Vec<Finding> = cards(ctx)
        .into_iter()
        .filter_map(|(name, card)| {
            let member = if card.model.is_some() {
                "model"
            } else if card.effort.is_some() {
                "effort"
            } else {
                return None;
            };
            Some(finding(
                "RK-D-TIER-LEAK",
                Severity::Error,
                format!("`cards/{name}` sets `{member}`; the tier is the role's"),
            ))
        })
        .collect();

    if let Ok(src) = std::fs::read_to_string(ctx.redkiln().join("slots.toml"))
        && let Ok(doc) = toml::from_str::<toml::Value>(&src)
        && let Some(fillers) = doc.get("filler").and_then(toml::Value::as_array)
    {
        for (i, f) in fillers.iter().enumerate() {
            for member in ["model", "effort"] {
                if f.get(member).is_some() {
                    out.push(finding(
                        "RK-D-TIER-LEAK",
                        Severity::Error,
                        format!(
                            "`slots.toml`'s filler #{i} sets `{member}`; the tier is the role's"
                        ),
                    ));
                }
            }
        }
    }
    out
}
