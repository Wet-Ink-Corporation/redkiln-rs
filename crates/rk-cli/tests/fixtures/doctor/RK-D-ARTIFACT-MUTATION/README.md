# `RK-D-ARTIFACT-MUTATION` - the rejecting fixture

**Severity:** `error` · **Pins:** AC-159 (`s8f` §8f.9; ADR-0189 decision 13, proposed) - an
unrecorded artifact mutation.

**The defect.** `.redkiln/artifacts/RK-S0001/Spec.md` is the layout-C rendering of `Spec.json`, and
`events.jsonl`'s one `artifact.write` records the sidecar's digest. ONE managed file is edited
out-of-band: the structured `acceptance` section of `Spec.md` gains an element, and no store
write record accounts for it. The rule recomputes the frontmatter's and each structured
section's digest through `rk-store`'s locator and reports exactly that one section.

This is the CHECKOUT's own `.redkiln/artifacts/` arm (no `worktree_assigned` row names a run
tree here, because a materialised fixture's absolute path is not known when it is written).
The through-the-store control is `doctor_artifact::a_rewrite_through_the_store_is_not_reported`;
this fixture adds no `#[test]` to `doctor.rs` (§8f.9).

Generated 2026-09-29 by S8F-P8's `S8F-P8-gen-fixtures.py`, LF only; `doctor_artifact::an_out_of_band_structured_edit_is_reported_as_an_unrecorded_mutation`
asserts the rule names this one section and no other, so every other byte is the renderer's.
