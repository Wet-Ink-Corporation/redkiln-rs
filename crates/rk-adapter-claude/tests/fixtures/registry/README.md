# registry/ — AC-238 / AC-489 / AC-492's must-fail registries (s7c §7c.7, §7c.17)

Each file is a registry that `rk_adapter_claude::registry::load` must **refuse**, and the test that reads
it says which member or token the refusal must name. The shape is the one every adapter declares (RT-24);
these are the Claude crate's copies of `rk-adapter-sqlite`'s six, keyed on the `dispatch` row.

| file | what is wrong | criterion |
| --- | --- | --- |
| `bare_row.json` | a `dispatch` row carrying only `capability` — no `verified_on`, no `probe` | AC-489: every member is required (the register's literal instrument) |
| `undated_row.json` | `capability` and `probe` but no `verified_on` | AC-489 / AC-238: the refusal names `verified_on` — a `#[serde(default)]` on the date would let an undated entry load |
| `probeless_row.json` | `capability` and `verified_on` but no `probe` | AC-489: the refusal names `probe` — a defaulted probe would make an absent member look like a `Probe::None` |
| `unknown_member.json` | a complete row plus a member the row type does not declare (`adopted`) | AC-489: `deny_unknown_fields` |
| `probe_unknown_member.json` | a `recorded` probe carrying a member the variant does not declare | AC-489: `deny_unknown_fields` reaches the probe |
| `reads_minds.json` | a well-formed row whose `capability` is a token the canonical vocabulary does not define | AC-492: an unknown token fails, and is never mapped to a default |

The shipped registry is the crate's own (`src/registry.rs`); none of these is read at runtime.
