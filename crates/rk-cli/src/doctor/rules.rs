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
        check: timeout,
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
        check: dead_def,
    },
    Rule {
        id: "RK-D-UNFILLED-SLOT",
        severity: Severity::Warning,
        check: unfilled_slot,
    },
    Rule {
        id: "RK-D-ENV-DENYLIST",
        severity: Severity::Error,
        check: env_denylist,
    },
    Rule {
        id: "RK-D-CLI-VERSION",
        severity: Severity::Error,
        check: cli_version,
    },
    Rule {
        id: "RK-D-TTL",
        severity: Severity::Error,
        check: ttl,
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
        check: margin,
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

// ============================================================================ batch 2: the pack
// ============================================================================ and catalog rows
//
// Four families, five ids, each reading the enabled pack's own TOML or the catalog beside it.
//
// 🛑 **NONE OF THESE READS THROUGH `verbs::run::PackExtras`, AND §7d.8 PRESCRIBES THAT IT DOES.**
// §7d.8: *"`RK-D-TIMEOUT` reads per-activity `timeout_secs` through `rk-cli`'s own `PackExtras`
// (absent → `Missing`; non-integer → `NonNumeric`)"*. Measured 2026-09-21, that cannot be built:
//
//   * `ActivityExtras.timeout_secs` is a **required `u64`**, so an activity table declaring none
//     and one declaring `"900"` are the SAME failure — `serde` refuses the whole document either
//     way, and the two cases the row exists to tell apart become indistinguishable;
//   * that failure is not local. `PackExtras` also carries `margins` and `cache`, both with
//     required members, so ONE absent `timeout_secs` would silence `RK-D-MARGIN` and `RK-D-TTL`
//     as well — three rows blinded by one defect;
//   * and a pack declaring `[cache] prompt_ttl` alone refuses for the same reason, which is
//     exactly `RK-D-TTL`'s own rejecting fixture.
//
// So the rows below read the pack as `toml::Value`. **This is `CardView`'s asymmetry a second
// time, and it is worth naming once**: a loader refuses a document so that nothing downstream
// reads a half-parsed one, while a linter must read the parts that ARE well formed in order to
// report the part that is not. A doctor rule built on a loader's type reports nothing precisely
// when there is most to report. A dated §7d.8 amendment records it.

/// Every enabled pack's document, by name, in `config.packs.enabled` order.
///
/// An unreadable or unparseable pack is **skipped**, on [`cards`]' rule: each row below says one
/// thing about a pack it could read, and *"this file is not a pack"* belongs to whichever row a
/// record gives it.
fn packs(ctx: &Ctx) -> Vec<(String, toml::Value)> {
    ctx.config
        .packs
        .enabled
        .iter()
        .filter_map(|name| {
            let path = ctx.pack_files.get(name)?;
            let text = std::fs::read_to_string(path).ok()?;
            let doc: toml::Value = toml::from_str(&text).ok()?;
            Some((name.clone(), doc))
        })
        .collect()
}

/// A pack's `[<table>]`, when it has one.
fn table<'a>(pack: &'a toml::Value, name: &str) -> Option<&'a toml::Table> {
    pack.get(name).and_then(toml::Value::as_table)
}

/// **`RK-D-TIMEOUT`** — AC-419, AC-097's doctor half.
///
/// **Every activity the catalog declares owes a `timeout_secs`**, so an activity with no
/// `[activity.<id>]` table at all is `Missing` exactly as one whose table omits the key is: a
/// declaration that is absent and a declaration that is empty are the same absence to a run that
/// needs the number. The two arms the row distinguishes are absent and *present but not an
/// integer*, which is what the loader's type could not tell apart.
fn timeout(ctx: &Ctx) -> Vec<Finding> {
    let packs = packs(ctx);
    ctx.catalog
        .catalog
        .activities()
        .iter()
        .filter_map(|a| {
            let declared = packs
                .iter()
                .find_map(|(_, pack)| table(pack, "activity")?.get(&a.id)?.get("timeout_secs"));
            match declared {
                None => Some(finding(
                    "RK-D-TIMEOUT",
                    Severity::Error,
                    format!(
                        "activity `{}` declares no `timeout_secs`: no enabled pack carries `[activity.{}] timeout_secs` (Missing)",
                        a.id, a.id
                    ),
                )),
                Some(v) if v.as_integer().is_none() => Some(finding(
                    "RK-D-TIMEOUT",
                    Severity::Error,
                    format!(
                        "activity `{}` declares `timeout_secs = {v}`, which is not an integer (NonNumeric)",
                        a.id
                    ),
                )),
                Some(_) => None,
            }
        })
        .collect()
}

/// **`RK-D-CLI-VERSION`** — AC-423, D-11: the version of the harness binary **as installed on this
/// box**, against the pin this workspace was tested at.
///
/// It reads the probe seam rather than a file, because that is the only way to learn what is
/// installed. A probe that cannot answer is reported too: an unprobeable harness is as much a
/// configuration fact as a mismatched one, and a rule that stayed silent on it would be silent
/// exactly when the binary is missing.
fn cli_version(ctx: &Ctx) -> Vec<Finding> {
    let program = ctx
        .config
        .adapters
        .harness_binary
        .as_deref()
        .unwrap_or("claude");
    let pin = rk_adapter_claude::config::TESTED_CLI_VERSION;
    match ctx.probe.version(program) {
        Ok(v) if v.trim() == pin => Vec::new(),
        Ok(v) if v.trim().is_empty() => vec![finding(
            "RK-D-CLI-VERSION",
            Severity::Error,
            format!("`{program} --version` printed nothing; the tested pin is `{pin}`"),
        )],
        Ok(v) => vec![finding(
            "RK-D-CLI-VERSION",
            Severity::Error,
            format!(
                "the installed `{program}` reports `{}`; this workspace is tested at `{pin}`",
                v.trim()
            ),
        )],
        Err(e) => vec![finding(
            "RK-D-CLI-VERSION",
            Severity::Error,
            format!("`{program}` could not be probed: {e}"),
        )],
    }
}

/// **`RK-D-TTL`** — AC-424, ADR-0113, AC-210's THEN.
///
/// AC-210 names **both** keys and one value: `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and**
/// `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h`. So a pack owes two declarations, and each owes the
/// recorded value — the constant is read from `rk_adapter_claude::config::CACHE_TTL` and copied
/// nowhere, on `env_denylist`'s reasoning.
fn ttl(ctx: &Ctx) -> Vec<Finding> {
    let want = rk_adapter_claude::config::CACHE_TTL;
    let mut out = Vec::new();
    for (name, pack) in packs(ctx) {
        let cache = table(&pack, "cache");
        for key in ["prompt_ttl", "subagent_prompt_ttl"] {
            match cache.and_then(|c| c.get(key)).and_then(toml::Value::as_str) {
                Some(v) if v == want => {}
                Some(v) => out.push(finding(
                    "RK-D-TTL",
                    Severity::Error,
                    format!(
                        "pack `{name}` declares `[cache] {key} = \"{v}\"`; AC-210 names `{want}`"
                    ),
                )),
                None => out.push(finding(
                    "RK-D-TTL",
                    Severity::Error,
                    format!("pack `{name}` declares no `[cache] {key}`; AC-210 names both keys"),
                )),
            }
        }
    }
    out
}

/// **`RK-D-MARGIN`** — AC-377's doctor half (`s7c` ARMED): a declared margin under **two** ticks.
///
/// The floor is two and the comparison is `< 2`, so a margin of two is lawful. The test's own name
/// is what fixes it — *margin under two ticks* — and a rule keyed on `<= 2` or `< 1` would be
/// wrong in opposite directions at exactly the value the fixture sits on.
fn margin(ctx: &Ctx) -> Vec<Finding> {
    const FLOOR: i64 = 2;
    let mut out = Vec::new();
    for (name, pack) in packs(ctx) {
        let Some(margins) = table(&pack, "margins") else {
            continue;
        };
        for window in ["five_hour", "seven_day"] {
            if let Some(ticks) = margins.get(window).and_then(toml::Value::as_integer)
                && ticks < FLOOR
            {
                out.push(finding(
                    "RK-D-MARGIN",
                    Severity::Error,
                    format!(
                        "pack `{name}` declares `[margins] {window} = {ticks}`, under the floor of \
                         {FLOOR} ticks"
                    ),
                ));
            }
        }
    }
    out
}

/// **`RK-D-DEAD-DEF`** — AC-421, a **warning**: a definition referenced by no enabled pack.
///
/// The join is the activity's `stage` against the union of every enabled pack's `[policy] stages`.
/// A warning and not an error, on AC-057's standing warning class: a dead definition costs nothing
/// at run time, and §7d.8's *"warnings never move the code"* keeps the exit at `0`.
fn dead_def(ctx: &Ctx) -> Vec<Finding> {
    let mut reachable: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (_, pack) in packs(ctx) {
        if let Some(stages) = table(&pack, "policy")
            .and_then(|p| p.get("stages"))
            .and_then(toml::Value::as_table)
        {
            reachable.extend(stages.keys().cloned());
        }
    }
    if reachable.is_empty() {
        // No enabled pack declares a policy at all. Every activity is then unreachable for a
        // reason that is not the activity's, and reporting all of them would bury it.
        return Vec::new();
    }
    ctx.catalog
        .catalog
        .activities()
        .iter()
        .filter(|a| !reachable.contains(&a.stage))
        .map(|a| {
            finding(
                "RK-D-DEAD-DEF",
                Severity::Warning,
                format!(
                    "activity `{}` declares stage `{}`, which no enabled pack's `[policy] stages` \
                     names — nothing can reach it",
                    a.id, a.stage
                ),
            )
        })
        .collect()
}

/// **`RK-D-UNFILLED-SLOT`** — AC-421, a **warning**: a declared slot neither filled nor optional.
///
/// `required = true` is the discriminator: a slot the pack marks optional is a slot the pack said
/// may stay empty, and reporting it would make the marking meaningless.
fn unfilled_slot(ctx: &Ctx) -> Vec<Finding> {
    let filled: std::collections::BTreeSet<String> =
        std::fs::read_to_string(ctx.redkiln().join("slots.toml"))
            .ok()
            .and_then(|src| toml::from_str::<toml::Value>(&src).ok())
            .and_then(|doc| {
                Some(
                    doc.get("filler")?
                        .as_array()?
                        .iter()
                        .filter_map(|f| f.get("slot")?.as_str().map(ToOwned::to_owned))
                        .collect(),
                )
            })
            .unwrap_or_default();

    let mut out = Vec::new();
    for (name, pack) in packs(ctx) {
        let Some(slots) = table(&pack, "slot") else {
            continue;
        };
        for (slot, body) in slots {
            let required = body
                .get("required")
                .and_then(toml::Value::as_bool)
                .unwrap_or(false);
            if required && !filled.contains(slot) {
                out.push(finding(
                    "RK-D-UNFILLED-SLOT",
                    Severity::Warning,
                    format!(
                        "pack `{name}` declares `[slot.{slot}] required = true` and no filler in \
                         `slots.toml` names it"
                    ),
                ));
            }
        }
    }
    out
}
