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

use super::{Ctx, Finding, Severity, roles};

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
/// ✅ **Cycle 5 filled every `check`.** The shared `unbuilt` placeholder cycle 3 pointed all
/// thirty-eight rows at is **deleted**, and the compiler is what said so: with the last row wired
/// it became dead code, exactly as the `#[expect]` on it predicted. `ac415_…` now runs each
/// fixture through **its own rule only** and requires at least one finding of any severity, so
/// AC-415's *"a rule with no rejecting fixture fails the build"* is an assertion rather than a
/// name join.
pub static RULES: &[Rule] = &[
    Rule {
        id: "RK-D-GATE-OUTPUTS",
        severity: Severity::Error,
        check: gate_outputs,
    },
    Rule {
        id: "RK-D-HOOKS",
        severity: Severity::Error,
        check: hooks,
    },
    Rule {
        id: "RK-D-STORE-ID",
        severity: Severity::Error,
        check: store_id,
    },
    Rule {
        id: "RK-D-STRAY-STORE",
        severity: Severity::Error,
        check: stray_store,
    },
    Rule {
        id: "RK-D-ROLE-UNRESOLVED",
        severity: Severity::Error,
        check: role_unresolved,
    },
    Rule {
        id: "RK-D-ROLE-AMBIGUOUS",
        severity: Severity::Error,
        check: role_ambiguous,
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
        check: dangling_role,
    },
    Rule {
        id: "RK-D-DANGLING-SLOT",
        severity: Severity::Error,
        check: dangling_slot,
    },
    Rule {
        id: "RK-D-DANGLING-ATTACH",
        severity: Severity::Error,
        check: dangling_attach,
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
        check: facet_unknown,
    },
    Rule {
        id: "RK-D-FACET-VALUE",
        severity: Severity::Error,
        check: facet_value,
    },
    Rule {
        id: "RK-D-FACET-NEW",
        severity: Severity::Error,
        check: facet_new,
    },
    Rule {
        id: "RK-D-FACET-FACT",
        severity: Severity::Error,
        check: facet_fact,
    },
    Rule {
        id: "RK-D-ADAPTER",
        severity: Severity::Error,
        check: adapter,
    },
    Rule {
        id: "RK-D-UNREAD",
        severity: Severity::Warning,
        check: unread,
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
        check: headroom_margin,
    },
    Rule {
        id: "RK-D-REBUILT-TIER2",
        severity: Severity::Ok,
        check: rebuilt_tier2,
    },
    Rule {
        id: "RK-D-WORKTREE-ORPHAN",
        severity: Severity::Error,
        check: worktree_orphan,
    },
    Rule {
        id: "RK-D-NETWORK-FS",
        severity: Severity::Error,
        check: network_fs,
    },
    Rule {
        id: "RK-D-CREDENTIAL-RETAINED",
        severity: Severity::Error,
        check: credential_retained,
    },
    Rule {
        id: "RK-D-REVIEW-REEXEC",
        severity: Severity::Error,
        check: review_reexec,
    },
    Rule {
        id: "RK-D-QUARANTINE-LIVE",
        severity: Severity::Warning,
        check: quarantine_live,
    },
    Rule {
        id: "RK-D-SINK-COUNT",
        severity: Severity::Error,
        check: sink_count,
    },
    Rule {
        id: "RK-D-ADVISORY-LEDGER",
        severity: Severity::Error,
        check: advisory_ledger,
    },
    Rule {
        id: "RK-D-CORPUS-SHA",
        severity: Severity::Error,
        check: corpus_sha,
    },
    Rule {
        id: "RK-D-VERIFICATION-POLICY",
        severity: Severity::Error,
        check: verification_policy,
    },
    Rule {
        id: "RK-D-EXECUTED-LEDGER",
        severity: Severity::Ok,
        check: executed_ledger,
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
    // 🛑 THE FIRST WHITESPACE TOKEN, NOT THE WHOLE LINE. Measured 2026-09-21: the installed CLI
    // prints `2.1.278 (Claude Code)` while the pin is the bare `2.1.278` form, so a comparison of
    // the whole line never matches and the row would fire on EVERY box, at every version. This is
    // the same normalisation Checkpoint 7d's Halt 2 applies with `${pin%% *}`.
    let reported = |v: &str| v.split_whitespace().next().unwrap_or("").to_owned();
    match ctx.probe.version(program) {
        Ok(v) if reported(&v) == pin => Vec::new(),
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
                reported(&v)
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

// ============================================================================ batch 3: the roles
// ============================================================================ rows, and AC-409
//
// Five rows read `.redkiln/roles.toml`, `skills/` and `slots.toml` against the enabled packs'
// `[role.*]` and `[slot.*]` tables. The sixth reads the loader's own typed diagnostic.

/// Every role name the enabled packs declare, across their `[role.*]` tables.
fn declared_roles(ctx: &Ctx) -> std::collections::BTreeSet<String> {
    packs(ctx)
        .iter()
        .filter_map(|(_, pack)| table(pack, "role"))
        .flat_map(|t| t.keys().cloned())
        .collect()
}

/// Every slot name the enabled packs declare, across their `[slot.*]` tables.
fn declared_slots(ctx: &Ctx) -> std::collections::BTreeSet<String> {
    packs(ctx)
        .iter()
        .filter_map(|(_, pack)| table(pack, "slot"))
        .flat_map(|t| t.keys().cloned())
        .collect()
}

/// Every role a loaded activity's step NOMINATES, in catalog order.
///
/// ADR-0014: *"an activity nominates a ROLE … never a card"*. This is the demand side; the cards in
/// `roles.toml` are the supply side, and `RK-D-ROLE-UNRESOLVED` is the join between them.
fn nominated_roles(ctx: &Ctx) -> Vec<String> {
    let mut out: Vec<String> = ctx
        .catalog
        .catalog
        .activities()
        .iter()
        .flat_map(|a| a.steps.iter().filter_map(|s| s.role.clone()))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// **`RK-D-ROLE-UNRESOLVED`** — AC-416: *"a role resolves to zero cards"*.
///
/// **Scoped to NOMINATED roles.** A role that no step names and no card fulfils is not a defect —
/// it is a role nobody asked for. The demand comes from the catalog, which is why this row is
/// silent over a pack whose catalog did not load: there is nothing nominating anything.
fn role_unresolved(ctx: &Ctx) -> Vec<Finding> {
    let declarations = roles::read_declarations(&ctx.redkiln());
    nominated_roles(ctx)
        .into_iter()
        .filter(|role| {
            matches!(
                roles::resolve(&declarations, role),
                roles::Resolution::Unresolved
            )
        })
        .map(|role| {
            finding(
                "RK-D-ROLE-UNRESOLVED",
                Severity::Error,
                format!(
                    "role `{role}` is nominated by a step and no card in `roles.toml` fulfils it — \
                     the activity cannot be bound"
                ),
            )
        })
        .collect()
}

/// **`RK-D-ROLE-AMBIGUOUS`** — AC-416: *"or to two cards at equal selector specificity"*.
///
/// 🛑 **Scoped to DECLARED roles, not nominated ones**, and the difference is what makes
/// `ac416_`'s control work. An ambiguity is a property of the card set alone: two cards contending
/// at the same specificity are unorderable whether or not a step names the role today, and the
/// first step that does would pick arbitrarily. Scoping this to nominated roles would also leave
/// the control role in the fixture unexamined, and the control is the only thing standing between
/// this rule and *"a role with two cards"*.
fn role_ambiguous(ctx: &Ctx) -> Vec<Finding> {
    let declarations = roles::read_declarations(&ctx.redkiln());
    let mut roles_declared: Vec<String> = declarations
        .cards
        .iter()
        .flat_map(|c| c.fulfils.iter().cloned())
        .collect();
    roles_declared.sort();
    roles_declared.dedup();

    roles_declared
        .into_iter()
        .filter_map(|role| match roles::resolve(&declarations, &role) {
            roles::Resolution::Ambiguous(files) => Some(finding(
                "RK-D-ROLE-AMBIGUOUS",
                Severity::Error,
                format!(
                    "role `{role}` resolves to {} cards at equal selector specificity ({}) — \
                     nothing orders them",
                    files.len(),
                    files.join(", ")
                ),
            )),
            roles::Resolution::Resolved(_) | roles::Resolution::Unresolved => None,
        })
        .collect()
}

/// **`RK-D-DANGLING-ROLE`** — AC-420: a card `fulfils` a role no enabled pack declares.
fn dangling_role(ctx: &Ctx) -> Vec<Finding> {
    let declared = declared_roles(ctx);
    roles::read_declarations(&ctx.redkiln())
        .cards
        .into_iter()
        .flat_map(|c| {
            let file = c.file.clone();
            c.fulfils
                .into_iter()
                .filter(|r| !declared.contains(r))
                .map(move |role| {
                    finding(
                        "RK-D-DANGLING-ROLE",
                        Severity::Error,
                        format!(
                            "`roles.toml` has `{file}` fulfilling role `{role}`, which no enabled \
                             pack declares"
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// **`RK-D-DANGLING-ATTACH`** — AC-420: a skill `attach.roles` names a role no enabled pack
/// declares.
fn dangling_attach(ctx: &Ctx) -> Vec<Finding> {
    let declared = declared_roles(ctx);
    roles::read_declarations(&ctx.redkiln())
        .skills
        .into_iter()
        .flat_map(|(name, attaches)| {
            attaches
                .into_iter()
                .filter(|r| !declared.contains(r))
                .map(move |role| {
                    finding(
                        "RK-D-DANGLING-ATTACH",
                        Severity::Error,
                        format!(
                            "`skills/{name}` attaches to role `{role}`, which no enabled pack \
                             declares"
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// **`RK-D-DANGLING-SLOT`** — AC-420: a slot filler names a slot no enabled pack declares.
fn dangling_slot(ctx: &Ctx) -> Vec<Finding> {
    let declared = declared_slots(ctx);
    let Ok(src) = std::fs::read_to_string(ctx.redkiln().join("slots.toml")) else {
        return Vec::new();
    };
    let Ok(doc) = toml::from_str::<toml::Value>(&src) else {
        return Vec::new();
    };
    let Some(fillers) = doc.get("filler").and_then(toml::Value::as_array) else {
        return Vec::new();
    };
    fillers
        .iter()
        .filter_map(|f| f.get("slot")?.as_str())
        .filter(|slot| !declared.contains(*slot))
        .map(|slot| {
            finding(
                "RK-D-DANGLING-SLOT",
                Severity::Error,
                format!(
                    "`slots.toml` has a filler for slot `{slot}`, which no enabled pack declares"
                ),
            )
        })
        .collect()
}

/// **`RK-D-GATE-OUTPUTS`** — AC-409 as amended (C-01 RA-M-05).
///
/// 🛑 **It reads the loader's own typed diagnostic and re-derives nothing.** Slice 4 cycle 1 gave
/// `Catalog::from_sources` the AC-409 refusal and `catalog::load_diagnostic` the vocabulary to
/// report it apart from every other `Shape` — `DiagnosticKind::GateOutputs`. That discriminant
/// exists **for this row**: a rule that recognised the refusal by matching text inside
/// `Shape.detail` would break the first time the message was reworded, and one that walked the
/// catalog itself would be a second implementation of a predicate `run` already enforces, free to
/// disagree with it.
///
/// So this row fires exactly when `run` would have refused to load the graph, which is what makes
/// AC-409's WHEN — *"when the configuration is loaded (and when `doctor` runs over it)"* — one
/// predicate read from two sides rather than two predicates that happen to agree.
fn gate_outputs(ctx: &Ctx) -> Vec<Finding> {
    ctx.catalog
        .diagnostics
        .iter()
        .filter(|d| d.kind == rk_kernel::catalog::DiagnosticKind::GateOutputs)
        .map(|d| {
            finding(
                "RK-D-GATE-OUTPUTS",
                Severity::Error,
                format!(
                    "{d} — a gate answer populates the step's `answer` and binds nothing \
                     downstream (AC-409)"
                ),
            )
        })
        .collect()
}

// ==================================================================== batch 4: the git-directory
// ==================================================================== and store rows
//
// Six ids reading the common directory or the store handle the root opened.

/// **`RK-D-HOOKS`** — AC-412: a `.git/hooks/` file carrying redkiln's own marker.
///
/// 🛑 **The recogniser is the MARKER, not the file's existence** (RT-51). The rejected alternative
/// — flagging any non-`*.sample` file in `hooks/` — would fire on every operator hook in every
/// repository, and `doctor/clean/` carries one precisely so that alternative is red. `.sample`
/// files are git's own shipped examples and are skipped before anything is read.
fn hooks(ctx: &Ctx) -> Vec<Finding> {
    const MARKER: &str = "# redkiln-hook";
    let dir = ctx.common_dir.join("hooks");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut paths: Vec<std::path::PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .filter(|p| p.extension().is_none_or(|x| x != "sample"))
        .collect();
    paths.sort();
    paths
        .iter()
        .filter(|p| std::fs::read_to_string(p).is_ok_and(|text| text.contains(MARKER)))
        .map(|p| {
            let name = p
                .file_name()
                .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
            finding(
                "RK-D-HOOKS",
                Severity::Error,
                format!(
                    "`hooks/{name}` carries the `{MARKER}` marker. AC-412: `init` installs zero \
                     hooks, so a marked hook is one this tool wrote and no longer owns"
                ),
            )
        })
        .collect()
}

/// **`RK-D-STORE-ID`** — AC-413, AC-369: a store belonging to another repository.
///
/// 🛑 **It renders the handle the root carried in and re-derives nothing.** §7d.8 makes
/// `Ctx.store` an `Option<Result<…>>` *"carried in and not refused before the table, so
/// `RK-D-STORE-ID` renders it as its row"*. The identity check lives inside `SqliteStore::open`
/// (AC-179), and a second comparison here could disagree with the one that actually governs
/// whether a row is ever read.
///
/// **`None` is not a finding.** It is the `--rebuild` path, where the root deliberately holds no
/// handle, and a repository with no store yet is not a repository with the wrong one.
fn store_id(ctx: &Ctx) -> Vec<Finding> {
    match &ctx.store {
        None | Some(Ok(_)) => Vec::new(),
        Some(Err(e)) => vec![finding("RK-D-STORE-ID", Severity::Error, e.to_string())],
    }
}

/// Every linked worktree's directory under `<common-dir>/worktrees/`, in name order.
///
/// Read off the FILESYSTEM and not through `rk-git`: `GitOp` has no `worktree list` op and this
/// stage does not edit that crate (§7d.8's own note on the orphan row).
fn linked_worktrees(ctx: &Ctx) -> Vec<(String, std::path::PathBuf)> {
    let Ok(entries) = std::fs::read_dir(ctx.common_dir.join("worktrees")) else {
        return Vec::new();
    };
    let mut out: Vec<(String, std::path::PathBuf)> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter_map(|p| {
            let name = p.file_name()?.to_string_lossy().into_owned();
            Some((name, p))
        })
        .collect();
    out.sort();
    out
}

/// **`RK-D-STRAY-STORE`** — AC-370: a store under a linked worktree's own gitdir.
///
/// The store belongs to the common dir. One under `worktrees/<name>/redkiln/` is a second store
/// nothing will ever reconcile, so **presence is the whole finding** — it is not opened, and
/// whether it is a valid store does not change that it should not be there.
fn stray_store(ctx: &Ctx) -> Vec<Finding> {
    linked_worktrees(ctx)
        .into_iter()
        .filter(|(_, path)| path.join("redkiln").join("store.sqlite3").is_file())
        .map(|(name, _)| {
            finding(
                "RK-D-STRAY-STORE",
                Severity::Error,
                format!(
                    "linked worktree `{name}` carries `redkiln/store.sqlite3` under its own \
                     gitdir; the store belongs to the common dir"
                ),
            )
        })
        .collect()
}

/// **`RK-D-WORKTREE-ORPHAN`** — ADR-0105 clauses 6/7 (RT-45): a worktree entry whose working tree
/// is gone.
fn worktree_orphan(ctx: &Ctx) -> Vec<Finding> {
    linked_worktrees(ctx)
        .into_iter()
        .filter_map(|(name, path)| {
            let target = std::fs::read_to_string(path.join("gitdir")).ok()?;
            let target = target.trim();
            // The entry points at the worktree's own `.git` FILE; the working tree is its parent.
            let gone = !std::path::Path::new(target).exists();
            gone.then(|| {
                finding(
                    "RK-D-WORKTREE-ORPHAN",
                    Severity::Error,
                    format!(
                        "worktree entry `{name}` points at `{target}`, which does not exist — the \
                         working tree is gone and the entry was never pruned"
                    ),
                )
            })
        })
        .collect()
}

/// **`RK-D-NETWORK-FS`** — ADR-0105 clauses 6/7 (RT-45): the common dir is on a network
/// filesystem.
///
/// The recogniser is the path's FORM: a UNC path (`\\\\server\\share`) on Windows, or a mount under
/// a network prefix elsewhere. It is not a probe — asking the operating system what a filesystem is
/// would be a syscall per run for a fact the path already carries, and on a share that is slow or
/// absent the probe is the thing that hangs.
fn network_fs(ctx: &Ctx) -> Vec<Finding> {
    let shown = ctx.common_dir.display().to_string();
    let unc = shown.starts_with("\\\\") || shown.starts_with("//");
    let net_mount = ["/net/", "/mnt/net/", "/Volumes/"]
        .iter()
        .any(|p| shown.starts_with(p));
    if unc || net_mount {
        vec![finding(
            "RK-D-NETWORK-FS",
            Severity::Error,
            format!(
                "the common dir is at `{shown}`, which is on a network filesystem. Locking and \
                 rename semantics there are not the ones the store assumes (ADR-0105)"
            ),
        )]
    } else {
        Vec::new()
    }
}

/// **`RK-D-CREDENTIAL-RETAINED`** — ADR-0105 clauses 6/7 (RT-45): a credential in a retained
/// stream.
///
/// 🛑 **It scans EVERY string in every payload, not one named member.** ADR-0105 clause 7 keeps
/// argv and env as digests precisely so a secret cannot land there, which leaves the retained
/// streams as the one place it still can — and a rule that read only `stdout` would miss the same
/// token in `stderr`, in a nested object, or in a member a later record adds.
///
/// **The finding never quotes what it found.** A rule that printed the secret back would copy it
/// into wherever the report goes, which is the defect it exists to report.
fn credential_retained(ctx: &Ctx) -> Vec<Finding> {
    /// The shapes a credential takes in these streams. Prefixes, because the remainder is the
    /// secret and nothing here should hold one.
    const SHAPES: [&str; 2] = ["sk-ant-", "Bearer "];

    fn scan(value: &serde_json::Value, hit: &mut bool) {
        match value {
            serde_json::Value::String(s) => {
                if SHAPES.iter().any(|p| s.contains(p)) {
                    *hit = true;
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| scan(v, hit)),
            serde_json::Value::Object(map) => map.values().for_each(|v| scan(v, hit)),
            _ => {}
        }
    }

    ctx.journal
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            let mut hit = false;
            scan(&row.payload, &mut hit);
            hit.then(|| {
                finding(
                    "RK-D-CREDENTIAL-RETAINED",
                    Severity::Error,
                    format!(
                        "journal row {i} (`{}`) carries a credential-shaped string in its retained \
                         payload. The value is NOT reproduced here; read the row to rotate it",
                        row.kind
                    ),
                )
            })
        })
        .collect()
}

// ================================================================================ batch 5: the
// ================================================================================ facet rows
//
// Four ids. Two of them are the kernel's own rule reported under a doctor id; two are `doctor`'s,
// because the kernel's violation vocabulary has no member for either.

/// Collect a predicate's field-path ROOTS by parsing it and walking the tree.
///
/// 🛑 **Parsed, never scanned.** `crate::registry`'s own reason, transcribed because it applies
/// here unchanged: `when = "'text' in change"` reads `change`, and a substring search for a facet's
/// name would also match the word inside a quoted literal. `rk_kernel::registry::collect_paths` is
/// private, so the walk is written here over the public `Expr`.
fn roots_of(expr: &rk_kernel::predicate::Expr, out: &mut std::collections::BTreeSet<String>) {
    use rk_kernel::predicate::Expr as E;
    match expr {
        E::Path(path) => {
            // The ROOT is the first dotted segment: `item.kind` reads `item`.
            let root = path.split('.').next().unwrap_or(path);
            out.insert(root.to_owned());
        }
        E::Or(a, b) | E::And(a, b) => {
            roots_of(a, out);
            roots_of(b, out);
        }
        E::Not(a) | E::Group(a) | E::Len(a) => roots_of(a, out),
        E::Cmp { left, right, .. } => {
            roots_of(left, out);
            roots_of(right, out);
        }
        E::In { left, list } => {
            roots_of(left, out);
            roots_of(list, out);
        }
        E::Member { literal, set } => {
            roots_of(literal, out);
            roots_of(set, out);
        }
        E::List(items) => items.iter().for_each(|i| roots_of(i, out)),
        E::Quant { source, body, .. } => {
            roots_of(source, out);
            roots_of(body, out);
        }
        E::Ref(_) | E::Lit(_) => {}
    }
}

/// Every predicate site beside the registry, plus the step ids that are lawfully not facets.
///
/// Two carriers, and **both**, because `facet_registry_rule`'s own doc records what reading one of
/// them lets through: *"a rule reading only `require` rows would pass a catalog whose
/// `applicable_if` names an unregistered facet, which is `binary_change`'s measured case."*
fn predicate_sites(ctx: &Ctx) -> rk_kernel::registry::PredicateSites {
    use rk_kernel::registry::{PredicateSite, SiteKind};

    let mut sites = Vec::new();
    let mut step_ids = std::collections::BTreeSet::new();

    for activity in ctx.catalog.catalog.activities() {
        for step in &activity.steps {
            step_ids.insert(step.id.clone());
        }
        if let Some(predicate) = &activity.applicable_if {
            let mut keys = std::collections::BTreeSet::new();
            if let Ok(parsed) = rk_kernel::predicate::parse(predicate) {
                roots_of(&parsed.expr, &mut keys);
            }
            sites.push(PredicateSite {
                kind: SiteKind::ApplicableIf,
                owner: activity.id.clone(),
                predicate: predicate.clone(),
                keys,
            });
        }
    }

    for (_, pack) in packs(ctx) {
        let Some(stages) = table(&pack, "stage") else {
            continue;
        };
        for (stage, body) in stages {
            let Some(rows) = body.get("require").and_then(toml::Value::as_array) else {
                continue;
            };
            for row in rows {
                let Some(predicate) = row.get("when").and_then(toml::Value::as_str) else {
                    continue;
                };
                let mut keys = std::collections::BTreeSet::new();
                if let Ok(parsed) = rk_kernel::predicate::parse(predicate) {
                    roots_of(&parsed.expr, &mut keys);
                }
                sites.push(PredicateSite {
                    kind: SiteKind::RequireWhen,
                    owner: stage.clone(),
                    predicate: predicate.to_owned(),
                    keys,
                });
            }
        }
    }

    rk_kernel::registry::PredicateSites::new(sites, step_ids)
}

/// Every violation the kernel's own registry rule reports, or an empty list when it passes.
fn registry_violations(ctx: &Ctx) -> Vec<rk_kernel::facet_registry::EntryShapeViolation> {
    let Some(registry) = &ctx.registry else {
        return Vec::new();
    };
    rk_kernel::facet_registry::facet_registry_rule(registry, &predicate_sites(ctx))
        .err()
        .unwrap_or_default()
}

/// **`RK-D-FACET-UNKNOWN`** — AC-425, AC-072: a predicate naming a facet the registry does not
/// declare.
///
/// 🛑 **The kernel's rule is the predicate, reported under a doctor id.** `facet_registry_rule` is
/// what `lint --facet-provenance` runs and what a pack load enforces; a second implementation here
/// would be free to disagree with the one that governs, and AC-072's *"an unregistered facet can
/// feed no row"* would then be two different propositions.
fn facet_unknown(ctx: &Ctx) -> Vec<Finding> {
    use rk_kernel::facet_registry::EntryShapeViolation as V;
    registry_violations(ctx)
        .into_iter()
        .filter_map(|v| match v {
            V::UnregisteredFacet { .. } => Some(finding(
                "RK-D-FACET-UNKNOWN",
                Severity::Error,
                format!("{v}"),
            )),
            _ => None,
        })
        .collect()
}

/// **`RK-D-FACET-FACT`** — AC-425, AC-071, AC-520: a derived facet with no named `deriver`.
///
/// ADR-0089 ships the `deriver` field **plus a record requirement**; a derived facet with neither
/// is what this row reports. Also the kernel's own violation, under its own id.
fn facet_fact(ctx: &Ctx) -> Vec<Finding> {
    use rk_kernel::facet_registry::EntryShapeViolation as V;
    registry_violations(ctx)
        .into_iter()
        .filter_map(|v| match v {
            V::DerivedMissingField { .. } => {
                Some(finding("RK-D-FACET-FACT", Severity::Error, format!("{v}")))
            }
            _ => None,
        })
        .collect()
}

/// One declared facet value as a predicate would spell it.
///
/// `toml_subset::Value` implements no `Display` [measured 2026-09-21], and inventing one for a
/// closed type in a crate this stage may not edit is not available. Only the three scalar shapes a
/// facet's `values` list can hold are rendered; an array or a table there is a registry-shape
/// defect and `facet_registry_rule`'s own row, not this one.
fn declared_value_text(value: &rk_kernel::toml_subset::Value) -> String {
    use rk_kernel::toml_subset::Value as V;
    match value {
        V::Str(s) => s.clone(),
        V::Int(i) => i.to_string(),
        V::Bool(b) => b.to_string(),
        V::Float(_) | V::Array(_) | V::Table(_) => String::new(),
    }
}

/// **`RK-D-FACET-VALUE`** — AC-425: a predicate comparing a **declared** facet against a value
/// outside its declared set.
///
/// 🛑 **This is `doctor`'s own predicate and not the kernel's**, because the kernel's violation
/// vocabulary has no member for it [measured 2026-09-21: `EntryShapeViolation`'s nine variants are
/// registry-SHAPE defects — an unregistered facet, a missing domain, a non-token source — and none
/// compares a predicate's literal against a facet's declared values]. The facet here IS declared,
/// which is exactly what separates this row from `RK-D-FACET-UNKNOWN`.
///
/// It walks the parsed tree for `<facet> == <literal>` and `<facet> in [<literals>]`, never a
/// scan: `roots_of`'s reason again, one level down.
fn facet_value(ctx: &Ctx) -> Vec<Finding> {
    use rk_kernel::predicate::{Expr as E, Literal};

    fn literal_text(expr: &E) -> Option<String> {
        match expr {
            E::Lit(Literal::Bare(s) | Literal::Str(s) | Literal::Num(s)) => Some(s.clone()),
            E::Lit(Literal::True) => Some("true".to_owned()),
            E::Lit(Literal::False) => Some("false".to_owned()),
            _ => None,
        }
    }

    fn walk(expr: &E, out: &mut Vec<(String, String)>) {
        match expr {
            E::Cmp { left, right, .. } => {
                if let (E::Path(p), Some(v)) = (left.as_ref(), literal_text(right)) {
                    out.push((p.split('.').next().unwrap_or(p).to_owned(), v));
                }
                if let (E::Path(p), Some(v)) = (right.as_ref(), literal_text(left)) {
                    out.push((p.split('.').next().unwrap_or(p).to_owned(), v));
                }
                walk(left, out);
                walk(right, out);
            }
            E::In { left, list } => {
                if let (E::Path(p), E::List(items)) = (left.as_ref(), list.as_ref()) {
                    let root = p.split('.').next().unwrap_or(p).to_owned();
                    for item in items {
                        if let Some(v) = literal_text(item) {
                            out.push((root.clone(), v));
                        }
                    }
                }
            }
            E::Or(a, b) | E::And(a, b) => {
                walk(a, out);
                walk(b, out);
            }
            E::Not(a) | E::Group(a) | E::Len(a) => walk(a, out),
            E::Member { set, .. } => walk(set, out),
            E::List(items) => items.iter().for_each(|i| walk(i, out)),
            E::Quant { source, body, .. } => {
                walk(source, out);
                walk(body, out);
            }
            E::Path(_) | E::Ref(_) | E::Lit(_) => {}
        }
    }

    let Some(registry) = &ctx.registry else {
        return Vec::new();
    };
    let declared: std::collections::BTreeMap<String, Vec<String>> = registry
        .facets()
        .map(|f| {
            (
                f.name.clone(),
                f.values.iter().map(declared_value_text).collect(),
            )
        })
        .collect();

    let mut out = Vec::new();
    for site in predicate_sites(ctx).sites() {
        let Ok(parsed) = rk_kernel::predicate::parse(&site.predicate) else {
            continue;
        };
        let mut pairs = Vec::new();
        walk(&parsed.expr, &mut pairs);
        for (facet, value) in pairs {
            // An UNDECLARED facet is `RK-D-FACET-UNKNOWN`'s row, not this one. A declared facet
            // with no declared values constrains nothing and is not compared against.
            let Some(values) = declared.get(&facet) else {
                continue;
            };
            if !values.is_empty() && !values.iter().any(|v| v == &value) {
                out.push(finding(
                    "RK-D-FACET-VALUE",
                    Severity::Error,
                    format!(
                        "{} compares `{facet}` against `{value}`, which is outside its declared \
                         values {values:?}",
                        site.site()
                    ),
                ));
            }
        }
    }
    out
}

/// **`RK-D-FACET-NEW`** — AC-425, ADR-0089: a registry entry declared since the last pinned
/// reading.
///
/// 🛑 **Also `doctor`'s own, and for a reason the kernel could not have.** A new facet is neither
/// unregistered nor wrongly valued — it is one **nothing has scored**, and that is a fact about two
/// readings of the registry rather than about one. `.redkiln/facets.pinned.toml` is the last
/// reading; a repository with none has nothing to compare and the row is silent, which is the
/// state a repository is in before anyone pins.
fn facet_new(ctx: &Ctx) -> Vec<Finding> {
    let Some(registry) = &ctx.registry else {
        return Vec::new();
    };
    let Ok(src) = std::fs::read_to_string(ctx.redkiln().join("facets.pinned.toml")) else {
        return Vec::new();
    };
    let Ok(pinned) = rk_kernel::registry::Registry::from_toml(&src) else {
        return Vec::new();
    };
    let known: std::collections::BTreeSet<&str> =
        pinned.facets().map(|f| f.name.as_str()).collect();
    registry
        .facets()
        .filter(|f| !known.contains(f.name.as_str()))
        .map(|f| {
            finding(
                "RK-D-FACET-NEW",
                Severity::Error,
                format!(
                    "facet `{}` is declared in `facets.toml` and absent from the pinned reading — \
                     nothing has scored it (ADR-0089)",
                    f.name
                ),
            )
        })
        .collect()
}

// =============================================================== batch 6: the journal and record
// =============================================================== rows, and the last two
//
// Nine ids. Seven fold `Ctx.journal`; two read a file beside it.

/// Every journal row of one wire kind, in journal order.
fn of_kind<'a>(ctx: &'a Ctx, wire: &str) -> Vec<&'a super::Row> {
    ctx.journal.iter().filter(|r| r.kind == wire).collect()
}

/// **`RK-D-HEADROOM-MARGIN`** — AC-220's doctor half, a **warning**.
///
/// **The LAST reading**, not the maximum and not any: AC-220's GIVEN is *"the last recorded
/// `five_hour` utilisation"*. Utilisation is in ticks out of a hundred, so *inside the margin* is
/// `100 − ticks <= margin`.
fn headroom_margin(ctx: &Ctx) -> Vec<Finding> {
    const LIMIT: i64 = 100;
    let Some(last) = of_kind(ctx, "rate_limit_reading")
        .last()
        .and_then(|r| r.payload.get("windows")?.get("five_hour")?.as_i64())
    else {
        return Vec::new();
    };
    packs(ctx)
        .iter()
        .filter_map(|(name, pack)| {
            let margin = table(pack, "margins")?
                .get("five_hour")
                .and_then(toml::Value::as_integer)?;
            (LIMIT - last <= margin).then(|| {
                finding(
                    "RK-D-HEADROOM-MARGIN",
                    Severity::Warning,
                    format!(
                        "the last recorded `five_hour` utilisation is {last} of {LIMIT}, inside \
                         pack `{name}`'s declared margin of {margin} ticks"
                    ),
                )
            })
        })
        .collect()
}

/// **`RK-D-REVIEW-REEXEC`** — AC-290 (D18-19): a `reproduced` finding with no re-execution record.
///
/// **The class is what makes the record mandatory**, and the record is what discharges it. An
/// `advisory` finding owes none, so the class is read first and `re_execution_id` second — a rule
/// keyed on either alone reports the wrong set.
fn review_reexec(ctx: &Ctx) -> Vec<Finding> {
    of_kind(ctx, "review_block")
        .into_iter()
        .filter(|r| r.str_field("computed_class") == Some("reproduced"))
        .filter(|r| {
            r.payload
                .get("re_execution_id")
                .is_none_or(serde_json::Value::is_null)
        })
        .map(|r| {
            finding(
                "RK-D-REVIEW-REEXEC",
                Severity::Error,
                format!(
                    "finding `{}` is classed `reproduced` and carries no `re_execution_id` — the \
                     class is a claim that it was re-run, and nothing records the run",
                    r.str_field("finding_id").unwrap_or("<unnamed>")
                ),
            )
        })
        .collect()
}

/// **`RK-D-QUARANTINE-LIVE`** — AC-306 (D18-19), a **warning**: live quarantines with their ages.
///
/// 🛑 **The age is `now − until` off `Ctx::now`, never a clock read** (§7d.8). A quarantine is live
/// when it was granted, not later removed or expired, and its `until` has not passed.
fn quarantine_live(ctx: &Ctx) -> Vec<Finding> {
    let closed: std::collections::BTreeSet<&str> = ["quarantine_removed", "quarantine_expired"]
        .iter()
        .flat_map(|k| of_kind(ctx, k))
        .filter_map(|r| r.str_field("finding_id"))
        .collect();

    of_kind(ctx, "quarantine_granted")
        .into_iter()
        .filter_map(|r| {
            let id = r.str_field("finding_id")?;
            if closed.contains(id) {
                return None;
            }
            let until = r.payload.get("until")?.as_i64()?;
            let age = ctx.now - until;
            (age < 0).then(|| {
                finding(
                    "RK-D-QUARANTINE-LIVE",
                    Severity::Warning,
                    format!(
                        "quarantine on `{id}` is live: it has {} seconds left (`until` is {until}, \
                         now is {})",
                        -age, ctx.now
                    ),
                )
            })
        })
        .collect()
}

/// **`RK-D-SINK-COUNT`** — AC-310 (D18-19).
///
/// AC-310's own replay invariant is `findings_in == blocks + sink_records`. The findings come from
/// the review's `Findings` artifact row, the blocks from `review_block` and the sinks from
/// `advisory_created`; a shortfall is the criterion's *"and none is dropped"* read off the journal.
///
/// 🛑 **The `findings` member on the artifact row is a READING** — RT-48 already routes these rows'
/// payloads and records that a dated design §5 note widening them is OWED. Nothing else in the
/// journal carries a review's finding count.
fn sink_count(ctx: &Ctx) -> Vec<Finding> {
    let findings_in: usize = of_kind(ctx, "artifact.write")
        .into_iter()
        .filter(|r| r.str_field("kind") == Some("Findings"))
        .filter_map(|r| Some(r.payload.get("findings")?.as_array()?.len()))
        .sum();
    if findings_in == 0 {
        return Vec::new();
    }
    let blocks = of_kind(ctx, "review_block").len();
    let sinks = of_kind(ctx, "advisory_created").len();
    let accounted = blocks + sinks;
    if findings_in > accounted {
        vec![finding(
            "RK-D-SINK-COUNT",
            Severity::Error,
            format!(
                "a completed review recorded {findings_in} findings and accounts for \
                 {accounted} ({blocks} blocked, {sinks} sunk) — {} was dropped rather than \
                 persisted to the advisory sink",
                findings_in - accounted
            ),
        )]
    } else {
        Vec::new()
    }
}

/// **`RK-D-ADVISORY-LEDGER`** — AC-530 (D18-19).
///
/// AC-530's invariant is `live_sink == created − retired − dismissed`. **The live count is the
/// store's and not the journal's**, so what is checked here is the half the journal can settle by
/// itself: every retirement and dismissal names a finding the sink actually held, and names it
/// once. A retirement of something never created makes the arithmetic unreachable rather than
/// merely wrong, and needs no second count to trust.
fn advisory_ledger(ctx: &Ctx) -> Vec<Finding> {
    let created: std::collections::BTreeSet<&str> = of_kind(ctx, "advisory_created")
        .into_iter()
        .filter_map(|r| r.str_field("finding_id"))
        .collect();
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for wire in ["advisory_retired", "advisory_dismissed"] {
        for row in of_kind(ctx, wire) {
            let Some(id) = row.str_field("finding_id") else {
                continue;
            };
            if !created.contains(id) {
                out.push(finding(
                    "RK-D-ADVISORY-LEDGER",
                    Severity::Error,
                    format!(
                        "`{wire}` names finding `{id}`, which the sink never held — \
                         `live_sink == created - retired - dismissed` cannot hold"
                    ),
                ));
            } else if !seen.insert(id) {
                out.push(finding(
                    "RK-D-ADVISORY-LEDGER",
                    Severity::Error,
                    format!("finding `{id}` leaves the live sink twice"),
                ));
            }
        }
    }
    out
}

/// **`RK-D-CORPUS-SHA`** — AC-314 (D18-19): a regression record whose corpus sha resolves nowhere.
///
/// 🛑 **The record is `.redkiln/regression.toml`, a FILE.** §7d.8 sketches the fixture as an
/// `events.jsonl` row and calls the corpus path *"a `ConfigFile` key"*; measured 2026-09-21
/// neither can carry it — `EventKind::ALL` is closed at 38 with no regression kind, and
/// `ConfigFile` is `deny_unknown_fields` over three tables with no corpus key. 7e's schema may
/// re-point it.
///
/// **Resolution is asked of git, not pattern-matched.** A 40-hex string is not a commit; only a
/// repository can say whether it names one.
fn corpus_sha(ctx: &Ctx) -> Vec<Finding> {
    let Ok(src) = std::fs::read_to_string(ctx.redkiln().join("regression.toml")) else {
        return Vec::new();
    };
    let Ok(doc) = toml::from_str::<toml::Value>(&src) else {
        return Vec::new();
    };
    let Some(sha) = doc.get("corpus_sha").and_then(toml::Value::as_str) else {
        return Vec::new();
    };
    // `rev-parse --verify` is the read that answers this, through `rk-git`'s own wrapper — the
    // one place `std::process::Command` is permitted (ADR-0077; Checkpoint 6's ban).
    let resolves = rk_git::git_exec::run(
        std::path::Path::new(&ctx.worktree),
        [
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{sha}^{{commit}}"),
        ],
    )
    .is_ok();
    if resolves {
        Vec::new()
    } else {
        vec![finding(
            "RK-D-CORPUS-SHA",
            Severity::Error,
            format!(
                "the regression record names corpus sha `{sha}`, which resolves to no commit — \
                 the run cannot be reproduced against the corpus it claims"
            ),
        )]
    }
}

/// **`RK-D-VERIFICATION-POLICY`** — AC-455 (D18-19).
///
/// **The POLICY decides, not the spec** (S-MIG decision 1, which AC-455 cites). A spec with
/// `verification = "None"` is lawful under a pack that requires none — the criterion's own second
/// clause — so this row reads the pair and never the spec alone.
fn verification_policy(ctx: &Ctx) -> Vec<Finding> {
    let requiring: std::collections::BTreeSet<String> = packs(ctx)
        .iter()
        .filter_map(|(_, pack)| table(pack, "verification"))
        .flat_map(|t| {
            t.iter()
                .filter(|(_, v)| v.as_str() == Some("required"))
                .map(|(stage, _)| stage.clone())
                .collect::<Vec<_>>()
        })
        .collect();
    if requiring.is_empty() {
        return Vec::new();
    }

    let dir = ctx.redkiln().join("specs");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut paths: Vec<std::path::PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort();
    paths
        .iter()
        .filter_map(|p| {
            let doc: toml::Value = toml::from_str(&std::fs::read_to_string(p).ok()?).ok()?;
            let stage = doc.get("stage")?.as_str()?;
            if !requiring.contains(stage) {
                return None;
            }
            (doc.get("verification")?.as_str()? == "None").then(|| {
                finding(
                    "RK-D-VERIFICATION-POLICY",
                    Severity::Error,
                    format!(
                        "spec for `{}` declares `verification = \"None\"` at stage `{stage}`, \
                         which an enabled pack's policy requires",
                        doc.get("item")
                            .and_then(toml::Value::as_str)
                            .unwrap_or("<unnamed>")
                    ),
                )
            })
        })
        .collect()
}

/// **`RK-D-UNREAD`** — AC-426 / AC-472 / AC-147, a **warning**: an artifact kind written and never
/// read over the last N runs.
///
/// **N is the repository's own key** (`[doctor] unread_window_runs`, required with no default —
/// AC-426), never a constant. The predicate is kind-per-STAGE (D-08): a kind read at one stage and
/// not another is unread at the second.
///
/// 🛑 **The window is a COUNT OF RUNS a kind has gone unread, not a slice of recent history**,
/// and `ac426_ac472_…`'s own fixtures are what fix that reading: 49 runs are **not** flagged and
/// 51 are, at `unread_window_runs = 50`, and **neither** is at 60. A rule that flagged anything
/// unread within the last N runs would flag both logs at both settings, and the 49/51 pair would
/// discriminate nothing. The comparison is `>=`: at N the kind has gone unread N times.
///
/// Its own tests are `ac426_ac472_unread_window_49_not_flagged_51_flagged` and
/// `ac426_rule_fire_leaves_write_set_unchanged`, at cycle 8. The predicate lands here because
/// `ac415_` needs every one of the thirty-eight rows to reject its fixture.
fn unread(ctx: &Ctx) -> Vec<Finding> {
    let window = i64::from(ctx.config.doctor.unread_window_runs);
    let run_of = |r: &super::Row| r.payload.get("run").and_then(serde_json::Value::as_i64);
    let key = |r: &super::Row| {
        Some((
            r.str_field("stage")?.to_owned(),
            r.str_field("kind")?.to_owned(),
        ))
    };

    // Which (stage, kind) pairs were READ at all, and in how many distinct runs each was WRITTEN.
    let read: std::collections::BTreeSet<(String, String)> = of_kind(ctx, "artifact.read")
        .into_iter()
        .filter_map(key)
        .collect();
    let mut written: std::collections::BTreeMap<(String, String), std::collections::BTreeSet<i64>> =
        std::collections::BTreeMap::new();
    for row in of_kind(ctx, "artifact.write") {
        let (Some(k), Some(run)) = (key(row), run_of(row)) else {
            continue;
        };
        written.entry(k).or_default().insert(run);
    }

    written
        .into_iter()
        .filter(|(k, _)| !read.contains(k))
        .filter(|(_, runs)| i64::try_from(runs.len()).unwrap_or(i64::MAX) >= window)
        .map(|((stage, kind), runs)| {
            finding(
                "RK-D-UNREAD",
                Severity::Warning,
                format!(
                    "artifact kind `{kind}` has been written at stage `{stage}` in {} runs and \
                     read in none; the window is {window}",
                    runs.len()
                ),
            )
        })
        .collect()
}

/// **`RK-D-REBUILT-TIER2`** — the table's one `ok`-severity row (RT-56).
///
/// 🛑 **An `ok` finding is a finding.** Tier-2 emptiness after a rebuild is *expected*, and a row
/// that reported it by staying silent would be indistinguishable from one that did not run.
/// `ac415_` counts findings of any severity, so this satisfies the per-id join; `doctor/clean/`
/// carries no sidecar, so the row is SILENT there rather than `ok`, which is what keeps the clean
/// run clean.
///
/// Its own test is `ac397_rebuilt_store_tier2_emptiness_is_ok_expected`, at cycle 9. **The
/// sidecar's home is RT-56 — ROUTED, not decided** — and a dated design note naming how `doctor`
/// learns a store was rebuilt is still OWED (§7d.18).
fn rebuilt_tier2(ctx: &Ctx) -> Vec<Finding> {
    if ctx.common_dir.join("redkiln").join("rebuilt").is_file() {
        vec![finding(
            "RK-D-REBUILT-TIER2",
            Severity::Ok,
            "the `rebuilt` sidecar is present: tier-2 emptiness is expected after rebuild"
                .to_owned(),
        )]
    } else {
        Vec::new()
    }
}

/// **`RK-D-EXECUTED-LEDGER`** — AC-532; ADR-0114 decision 2's `doctor` half.
///
/// > *"`redkiln doctor` prints the never-executed count and list per pack, and `redkiln lint`
/// > warns — never fails"* (ADR-0114 `## Decision` clause 2).
///
/// # Why the row is built at Step 7e
///
/// `s7b-kernel.md` §7b.15's ARMED table routed the ledger's `doctor` half to **Step 7d**, and 7d
/// did not build it. Measured 2026-09-21, before this row existed: a case-insensitive `executed`
/// grep over all of `crates/rk-cli/` returned one hit — unrelated prose in a test — and
/// [`rk_kernel::runtime::ledger::fold_executed`] had **zero** non-test callers in the workspace.
/// Checkpoint 7e asserts that `doctor` prints a never-executed line, so the row lands here.
///
/// # 🛑 THE FOLD IS THE KERNEL'S, RE-USED AND NOT RE-IMPLEMENTED
///
/// [`super::Row`] exists because `rk_types::event::Event` implements no `Deserialize` and
/// `rk-types` is a crate this stage may not edit — so a journal cannot be *parsed* into `Event`.
/// It can still be *built* into one: `Event` carries two public fields and a `const fn new`. So
/// this rule maps `Row -> Event` and calls the kernel's fold, rather than writing a second fold
/// over `Row`. A second fold would agree with the kernel's today and drift from it silently — and
/// the kernel's carries two readings a re-implementation would have to re-derive and could get
/// wrong: `first`/`last` are **journal (append) order, not `at` order** (AC-532, D15-4(ii)), and a
/// row missing `run_id`, `item` or `at` is **refused, never defaulted to a zero or an epoch**.
///
/// A row whose `kind` is outside the closed 38 is skipped here, not refused: `Row::kind` returns
/// `None` for it, an unknown wire name is another rule's subject, and the ledger's subject is the
/// `activity_committed` rows alone.
///
/// # 🛑 "PER PACK" IS REPORTED PER PACK OVER A CATALOG THAT IS NOT PARTITIONED BY PACK
///
/// ADR-0114 says *"per pack"*. The installed catalog is **one directory** every enabled pack draws
/// on — `.redkiln/catalog/`, read once by `load_diagnostic_dir` — and no pack declares an activity
/// *set*. So there is no per-pack partition of the catalog to report against, and this rule emits
/// one finding **per enabled pack** naming that pack, each over the shared catalog. That is a
/// reading, written into the code that takes it (the house model is `src/select.rs`:494-499), and
/// the record that would fix the partition is **OWED**: nothing in the tree says which activities
/// a pack owns.
///
/// # The severity is `Ok`, and the id deliberately omits "never-executed"
///
/// Never-executed is **not a defect** — ADR-0114: *"a doctor row, never a lint failure"* — and
/// Checkpoint 7e requires `doctor` to exit **0** on a v0 tree where every shipped activity is
/// unrun. `RK-D-REBUILT-TIER2` is the table's precedent for an `Ok` finding reporting an expected
/// state.
///
/// The id is `RK-D-EXECUTED-LEDGER` and **not** `RK-D-NEVER-EXECUTED`: Checkpoint 7e greps
/// `doctor`'s stdout for `never[- ]executed`, and the latter id would satisfy that grep from the
/// bare `<id> ok` line a rule prints when it finds **nothing** — a gate satisfied by the name of
/// what it means to measure. With this id the grep matches only a line the rule actually reported.
///
/// A fold refusal is `Warning`, not `Error`: this rule's subject is the ledger report, and failing
/// the whole `doctor` run on a corrupt journal row belongs to a rule whose subject is journal
/// integrity. It is reported here rather than swallowed.
fn executed_ledger(ctx: &Ctx) -> Vec<Finding> {
    const ID: &str = "RK-D-EXECUTED-LEDGER";

    let events: Vec<rk_types::event::Event> = ctx
        .journal
        .iter()
        .filter_map(|row| {
            row.kind()
                .map(|kind| rk_types::event::Event::new(kind, row.payload.clone()))
        })
        .collect();

    let ledger = match rk_kernel::runtime::ledger::fold_executed(&events) {
        Ok(ledger) => ledger,
        Err(e) => {
            return vec![finding(
                ID,
                Severity::Warning,
                format!(
                    "the executed ledger could not be folded and no never-executed count is \
                     reported: {e}"
                ),
            )];
        }
    };

    // An activity that declares no `version` has no `activity_id@version` key, so no
    // `activity_committed` row can be keyed to it — it reads never-executed, which is what it is.
    // The missing `version` is AC-010's subject, not this rule's.
    let activities = ctx.catalog.catalog.activities();
    let never: Vec<&str> = activities
        .iter()
        .filter(|a| {
            !a.id_at_version()
                .and_then(|key| ledger.get(&key).map(|e| !e.never()))
                .unwrap_or(false)
        })
        .map(|a| a.id.as_str())
        .collect();

    // Nothing unrun is nothing to report: the rule stays silent over a tree whose every activity
    // has executed, which is what keeps `doctor/clean/` clean and makes the fixture below a
    // rejecting one rather than a second clean control.
    if never.is_empty() {
        return Vec::new();
    }

    let detail = |pack: &str| {
        format!(
            "{pack}{} of {} shipped activities never executed (`first` is None; \
             ADR-0114 decision 2's ledger is kernel-written on `activity_committed`, so at v0 it \
             is absent and this is the expected reading): {}",
            never.len(),
            activities.len(),
            never.join(", ")
        )
    };

    let packs = packs(ctx);
    if packs.is_empty() {
        // No enabled pack, so "per pack" has nothing to range over. The count is still reported —
        // a silent rule here would be an absence reported as a pass.
        return vec![finding(ID, Severity::Ok, detail(""))];
    }
    packs
        .iter()
        .map(|(name, _)| finding(ID, Severity::Ok, detail(&format!("pack `{name}`: "))))
        .collect()
}
