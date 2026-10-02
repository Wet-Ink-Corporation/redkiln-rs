# `RK-D-CARD-GRANT` - the rejecting fixture

**Severity:** `error` · **Pins:** `s8h` §8h.10 (D21-42 21c; Record A = ADR-0197, proposed) - "a card tool outside the (g) union `{Read, Glob, Grep, Edit, Write, StructuredOutput}`, or a shipped card mode outside `{default, acceptEdits}`"

*(Added 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments.)*

**The defects.** Six cards, one defect each:

| card | the defect |
| --- | --- |
| `bash.json` | grants `Bash`, a command channel |
| `powershell.json` | grants `PowerShell`, the same class as `Bash` (`CLAUDE_CODE_USE_POWERSHELL_TOOL` in the installed binary) |
| `webfetch.json` | grants `WebFetch` |
| `websearch.json` | grants `WebSearch` |
| `mcp.json` | grants `mcp__x__y`, an MCP tool |
| `plan-mode.json` | carries `permission_mode = "plan"`, a CLI mode outside the shipped pair |

The five tool cards carry `permission_mode = "default"`, so each fires the tool half only, and the
mode card grants `StructuredOutput` alone, so it fires the mode half only. The row is a UNION
allowlist: it names no list of forbidden tools that could go stale, and `Bash`, `PowerShell`,
`Web*` and `mcp__*` appear in its message as examples only.

## How this directory is used

`ac415_every_rule_id_has_a_rejecting_fixture` and the row's own tests,
`doctor::rk_d_card_grant_refuses_a_tool_outside_the_least_privilege_set` and
`doctor::rk_d_card_grant_refuses_a_mode_outside_the_shipped_pair`, **materialise** it into a
`tempfile` git repository before running the rule (`s7d` §7d.13). Nothing runs against the tracked
copy.

## What this directory is NOT

It is not a runnable repository for `redkiln run`. It is the smallest tree that makes this one rule
fire; `doctor/clean/` and the shipped `.redkiln/cards/` are the configurations it passes.
