// `s8a` §8a.2 item 1, §8a.9: `ChangeSummary`'s `Binary` arm is a SEALED slot until S-BIN reports
// at `s8i`. Its payload type is public (so the enum can name it) but carries no public field and
// no public constructor, so a crate outside `rk-ports` cannot build one. This case must fail to
// compile for the PRIVACY reason, and the committed `.stderr` pins that reason.
//
// (Amended 2026-09-25, s8a fix round 7.) The private tuple constructor was the only route tried,
// and rustc's E0423 text does not move when `rk-ports` adds a `Default`, a `From` or a `pub const`,
// so each of those opened the slot with this case still green. Each other route is now its own
// compile error, pinned in the same `.stderr`: `Default`, `From`, a `new` constructor, and
// `Deserialize` on the payload or the enum. Any of them added to `rk-ports` changes the `.stderr`.
//
// `s8i` supersedes this case by name when S-BIN finalises `Binary`.

use rk_ports::toolchain::{BinaryChangeSummary, ChangeSummary};

fn deserializable<T: serde::de::DeserializeOwned>() {}

fn main() {
    let sealed = BinaryChangeSummary(());
    let _summary = ChangeSummary::Binary(sealed);
    let _by_default: BinaryChangeSummary = Default::default();
    let _by_from = BinaryChangeSummary::from(0_u8);
    let _by_new = BinaryChangeSummary::new();
    deserializable::<BinaryChangeSummary>();
    deserializable::<ChangeSummary>();
}
