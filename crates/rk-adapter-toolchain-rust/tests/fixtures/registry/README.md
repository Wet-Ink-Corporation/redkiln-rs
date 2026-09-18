# registry/ — AC-489 / AC-492 / AC-328's must-fail and must-refuse registries (s7c §7c.17)

Each file is a registry the crate's `registry::load` must **refuse** (the first six), or one whose row
the adapter must refuse to answer from (the last), and the test that reads it says which member or
token the refusal must name:

| file | what is wrong | criterion |
| --- | --- | --- |
| `bare_row.json` | a `change_summary` row carrying only `capability` — no `verified_on`, no `probe` | AC-489: every member is required (the register's literal instrument) |
| `undated_row.json` | a row with `capability` and `probe` but no `verified_on` | AC-489: the refusal names `verified_on` — a `#[serde(default)]` on the date would let an undated row load |
| `probeless_row.json` | a row with `capability` and `verified_on` but no `probe` | AC-489: the refusal names `probe` — a defaulted probe would make an absent member look like a `Probe::None` |
| `unknown_member.json` | a complete row plus a member the row type does not declare (`adopted`) | AC-489: `deny_unknown_fields` |
| `probe_unknown_member.json` | a `recorded` probe carrying a member the variant does not declare | AC-489: `deny_unknown_fields` reaches the probe |
| `reads_minds.json` | a well-formed row whose `capability` is a token the canonical vocabulary does not define | AC-492: an unknown token fails, and is never mapped to a default |
| `invented_probe.json` | a well-formed `affected-selection` row claiming a `recorded` probe that nobody ran | AC-328: a row with an invented probe is refused by the adapter (a decode failure naming the row), never answered as a selection basis |

The shipped registry is the crate's own (`src/registry.rs`); none of these is read at runtime.
