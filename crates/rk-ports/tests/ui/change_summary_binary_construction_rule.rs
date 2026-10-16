// `s8i`, ADR-0199 decision 2 (proposed): S-BIN has reported, and `ChangeSummary`'s `Binary` arm
// is no longer a sealed slot. Its payload carries ADR-0199's four fields, every one PRIVATE, and
// it is built only through its ONE recorded checked constructor, `BinaryChangeSummary::new`,
// which refuses a text patch of a `Binary`-classified path (AC-497's AND). This case must fail to
// compile for each other route, and the committed `.stderr` pins each reason: a direct field
// read (the fields are private), `Default`, `From`, and `Deserialize` on the payload or the enum.
// Making a field `pub`, or adding any of those routes to `rk-ports`, changes the `.stderr` and
// turns `ui::trybuild_pins_the_binary_change_summary_construction_rule` red.
//
// The struct literal is its own case, `change_summary_binary_struct_literal.rs`: rustc reports a
// private field in a struct literal (E0451) only when type checking succeeded, so beside these
// errors it would never be reported, and so never pinned.
//
// No route here is spelled through a std item (`Default::default()` names only the trait), so no
// error points into the standard library's source: with the `rust-src` component installed that
// output gains a source excerpt, and the snapshot would differ by machine.
//
// It supersedes, by name, `s8a`'s `tests/ui/change_summary_binary_not_constructible.rs`.

use rk_ports::toolchain::{BinaryChangeSummary, ChangeSummary};

fn deserializable<T: serde::de::DeserializeOwned>() {}

fn from_a_byte<T: From<u8>>() {}

fn main() {
    let built = BinaryChangeSummary::new(Vec::new(), Vec::new(), String::new(), String::new(), "");
    let _read = built.map(|b| b.assets);
    let _by_default: BinaryChangeSummary = Default::default();
    from_a_byte::<BinaryChangeSummary>();
    deserializable::<BinaryChangeSummary>();
    deserializable::<ChangeSummary>();
}
