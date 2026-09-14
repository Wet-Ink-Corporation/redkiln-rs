//! **AC-061's second AND.** *"`RequiredSet` is constructible only inside the selection crate …, so
//! no agent-returned value can reach it."* ADR-0099 clause 5 makes *"inside the selection crate"*
//! mean the `select` MODULE, and §7b.4:431 fixes the constructor at `pub(in crate::select)`.
//!
//! This file must FAIL to compile. If it ever compiles, an activity's typed output could carry a
//! required-artifact set and the deviation channel becomes control flow by the back door — the
//! risk row §5 names, and the one F37 measured as closed only by disuse (deviation offered 71
//! times, used 0).

use rk_kernel::select::RequiredSet;

fn main() {
    // `RequiredSet::new` is `pub(in crate::select)`.
    let _r = RequiredSet::new(std::collections::BTreeSet::new());
}
