# `doctor/` - one rejecting directory per rule id, plus `clean/`

**Thirty-eight rejecting directories and one `clean/`** (`s7d` §7d.8, §7d.13). The count is
thirty-eight and not thirty-two: §7d.16 item 4 said *thirty-two* and was corrected in the stage
file on 2026-09-20 - D18-19 moved the table 32 -> 38 on 2026-09-19 and that item was not updated
with it. §7d.8 and §7d.13 both say thirty-eight, and an independent count off the table's first
column agrees.

`ac415_every_rule_id_has_a_rejecting_fixture` joins these names against `doctor::rules::RULES` in
**both directions**: a rule id with no directory is an error, and a directory with no rule is an
error too - a stale fixture is a lie about the table. `clean/` is the one named exemption.

## The layout is the one the binary reads

Measured 2026-09-21 against `crates/rk-cli/src/verbs/run.rs`:

    .redkiln/config.toml          the ConfigFile
    .redkiln/packs/<name>.toml    a pack is a FLAT FILE, not a directory
    .redkiln/catalog/*.toml       the activities
    .redkiln/cards/<role>.json    one card per role, by FILE NAME
    .redkiln/facets.toml          the fact registry
    .redkiln/items/<item>.toml    item records

`s7d` §7d.13 describes cards and skills as living under *"the pack's `cards/` and `skills/`
directories"*. There is no per-pack directory in the tree: cards are at `.redkiln/cards/` and are
keyed by file name. A dated §7d.13 amendment records this.

## 🛑 `fulfils` IS IN A SIDECAR, NOT ON THE CARD, AND THAT IS MEASURED RATHER THAN CHOSEN

`.redkiln/roles.toml` carries `[[card]] file / fulfils / selector`. §7d.13's floor says a card file
carries `fulfils: [<role>]`, and **it cannot**: `rk_adapter_claude::card::CardFile` is
`#[serde(deny_unknown_fields)]` over exactly seven names - `description`, `prompt`, `tools`,
`cache_profile`, `permission_mode`, `model`, `effort` - and `fulfils` is refused as an unknown
field [measured 2026-09-21]. `s7d` §7d.1 permits **one closed crate edited** (`rk-kernel`), so
widening it is foreclosed.

**Slice 3 hit this and handed it here by name**, in `../pack-min/cards/README.md`: *"Slice 4 must
either widen `CardFile` or carry the role->card map somewhere other than the card."* This is the
second.

`selector` is not decoration either: AC-416's second half is *"two cards at **equal selector
specificity**"*, and a bare list of card names has no specificity, so `RK-D-ROLE-AMBIGUOUS` would
be a row that could never fire.

The **skill** half of the floor is spelled exactly as §7d.13 writes it - `attach.roles` on
`.redkiln/skills/<name>.toml` - because nothing outside `crates/rk-cli/src/doctor/roles.rs` reads
skills at all [measured 2026-09-21], so no closed-crate schema collides with it.

## Provenance

**No corpus byte enters the tree** (AC-465; D18-10). Every file here is hand-authored for Step 7d
Slice 4 on 2026-09-21; nothing is copied from `D:/repos/redkiln` or from `redkiln-corpus`, and no
path under `spikes/` is referenced. The `sk-ant-` string in `RK-D-CREDENTIAL-RETAINED/` and the
`ANTHROPIC_API_KEY` name in `RK-D-ENV-DENYLIST/` are **synthetic**: they are the shapes those two
rules recognise, and neither is a credential.
