# `pack-min/cards/` — and one obligation this stage RAISES rather than discharges

The cards `run`'s step (8) binder reads, one JSON file per role name
(`.redkiln/cards/<role>.json`). `pack-min` declares one role, `implementer`, and so ships one card.

## 🛑 `fulfils` IS NOT A KEY A CARD MAY CARRY, AND §7d.13 ASSUMES IT IS

This card was authored with `"fulfils": ["implementer"]`, because §7d.13 names `fulfils` as the key
Slice 4's `doctor/roles.rs` resolves a role through. **A card carrying it does not load.**

`rk_adapter_claude::card::CardFile` is `#[serde(deny_unknown_fields)]` over exactly seven names —
`description`, `prompt`, `tools`, `cache_profile`, `permission_mode`, `model`, `effort` — and the
last two are declared *"ONLY so a leak is refused by name rather than as an unknown key"*
(`crates/rk-adapter-claude/src/card.rs`). There is no eighth name, and `fulfils` is refused as an
unknown field.

So the key was removed here, and **the obligation is RAISED, not settled**: Slice 4 must either
widen `CardFile` or carry the role→card map somewhere other than the card. Neither is Slice 3's to
decide — §7d.1 puts the card record's shape outside this stage, and widening a
`deny_unknown_fields` record in `rk-adapter-claude` would re-open Checkpoint 7c for a fixture.

**What resolves the role today**: nothing in this fixture. The role's own table in `pack.toml`
(`[role.implementer]`) carries `family`, `model` and `effort`, and `run`'s binder reads the card by
FILE NAME — `<role>.json` — so the mapping is positional rather than declared. That is sufficient
for Slice 3 and is exactly what Slice 4's reader will have to replace.

## The tier is the ROLE's, and a card that states it is refused

This card carries no `model` and no `effort` on purpose. `Binder::bind` injects the role's tier
(AC-212; AC-118), and a card declaring one is a `ConfigError::TierLeak` refused by member name. The
one source is `[role.<name>]` in the pack — see the 🛑 note on that table in `pack.toml`.
