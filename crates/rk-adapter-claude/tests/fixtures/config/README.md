# config/ — AC-377's fixture configs (s7c §7c.8)

`Margins` reaches the adapter as a `Config` value (ADR-0083; §7c.8) — the adapter parses no pack file — so a
"fixture config" here is the margin block as the pack reader would hand it in, deserialised into
`rk_adapter_claude::config::Margins` (both windows required; no `Default`, no `#[serde(default)]`).

| file | what it is | must |
| --- | --- | --- |
| `no-margin.json` | an empty block — no margin at all | fail to deserialise (AC-377: *"a fixture config with no margin that must fail (no default)"*) |
| `one-window.json` | a `five_hour` margin and no `seven_day` | fail to deserialise (the throttle has exactly two windows — AC-375 — and a margin for one of them is not a margin) |
| `margin-one-tick.json` | `five_hour: 1` — under two ticks | deserialise, then be REFUSED by `Config::new` (AC-377 / AC-220: `margin_ticks < 2`) |
| `margins-ok.json` | 7 / 18 ticks — the register's 0.07 / 0.18 vectors as ticks | deserialise and construct |
