# Instrument checks — NOT experiment results

Every file here was produced while building and pressure-testing Layer 0 on
2026-09-02, before any pre-registration existed. They are the raw evidence
behind FINDINGS F1–F8 and nothing more.

**None of these is an arm.** They were run with a stale/immature instrument
(missing digest capture, missing MCP isolation, flat cache-creation parsing),
so their usage figures are not comparable with each other or with anything
recorded later. Their `preregistration_sha` points at a commit containing no
prompts or arms, which is precisely why they cannot count.

Kept because F1 (the undocumented `rate_limit_event`), F3 (the nested TTL
split), F6 (0.01 quantisation) and F8 (the ~280-tool, 12-MCP-server default
environment) were all read off these files.
