# Estimation history

kb:estimation-history — read by t-shirt-sizing/size (s8c §8c.4; ADR-0170 decision 8, proposed). Authored
2026-09-26. Each line below restates a passage named in `source:`; nothing here is authored from memory.

source: references/research/2026-09-07-activity-catalog-raw.md, the sizing rows (appetite-check, t-shirt-sizing,
relative-estimation, reference-class-forecast) and the line "Ordered options for sizing (pick one)";
.redkiln/catalog/t-shirt-sizing.toml (its rationale and applicable_if).

## The recorded history

**No estimate has been recorded.** At authoring no run had sized an item: the lanes do not walk the shipped catalog
before `s8d`. A sizing step that reads this atom reads an empty history, and it says so rather than invent
comparables.

## How the sizing forms use history

- The raw catalog orders them: *"Ordered options for sizing (pick one): `appetite-check` < `t-shirt-sizing` <
  `relative-estimation` < `reference-class-forecast`."*
- `t-shirt-sizing`: *"Coarse S/M/L sizing per slice"*. Its rationale: *"Coarse sizing is enough for small items;
  larger stories fall through to relative-estimation by the size facet."* It applies only when `size < high`.
- `reference-class-forecast`: *"Estimate from historical distributions of similar items"*. It is the form that
  needs a recorded history, and it sits above `t-shirt-sizing` in the order.
