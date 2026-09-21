# `clean/` - the positive control every per-row test runs second

A configuration **every rule passes**. It is the one named exemption from `ac415_`'s set equality
(`s7d` §7d.13): it is a directory under `doctor/` with no rule of the same name, and that is
lawful here and nowhere else.

## Why a clean fixture is not decoration

Every per-row test runs its rule over its own fixture **and over this one**, asserting no finding.
Without the second half a rule that fired unconditionally would pass every per-row test in the
slice - the shape `s7d` §7d.8 names: *"the second half is what makes a rule that always fires
red."*

## 🛑 The hook here is an OPERATOR hook, and that is the discriminating byte

`dot-git/hooks/pre-commit` is a real hook with **no `# redkiln-hook` marker** and a `# fork-join`
comment. `RK-D-HOOKS` recognises redkiln's own hooks by that marker (RT-51), and the alternative
RT-51 rejected - flagging any non-`*.sample` file in `hooks/` - is **red on this run**. A `clean/`
with no hooks at all could not tell the two recognisers apart, which is the whole reason this file
is here rather than omitted.

## What it carries, and why each piece

A complete `ConfigFile`, one enabled pack, one activity, one card with its role sidecar entry, a
facet registry, and the operator hook above. Every rule that reads one of those finds it
well-formed. **No `rebuilt` sidecar**, deliberately: `RK-D-REBUILT-TIER2` emits its `ok`-severity
finding only when the sidecar is present, so its absence here keeps the row silent rather than
merely non-erroring.
