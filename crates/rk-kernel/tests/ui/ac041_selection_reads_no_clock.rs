//! **AC-041.** *"the four digests and their expanded values are its only inputs AND a call that
//! would need any of them fails to compile rather than reading them at runtime."*
//!
//! This file must FAIL to compile. Its subject is the **surface**: `select` takes a
//! pack, a catalog, two registries, a stage name and an item, and there is no parameter, no builder
//! and no setter through which a clock, an environment, a path or a random source can be handed in.
//! A caller that tries to pass one is a type error.
//!
//! # This case is the SURFACE half, and the other half is named rather than assumed
//!
//! A `trybuild` fixture **cannot fail on a lint, only on a compile error** — the root
//! `clippy.toml`:10-13 says so in terms, for exactly this reason (PIA-030/PIA-031). So the ban on
//! `SystemTime::now`, `rand`, `std::fs` and `std::env` *inside* `rk-kernel::select` is NOT pinned
//! here: it is pinned by that module's `#![deny(clippy::disallowed_methods)]` together with the
//! committed positive control `rk-kernel/select-clock-reintroduce`, whose whole purpose is to show
//! the attribute BITES rather than merely being declared.

use rk_kernel::select::{select, Item, Pack};

fn main() {
    let pack = Pack::default();
    let catalog = rk_kernel::catalog::Catalog::default();
    let artifacts = rk_kernel::select::ArtifactRegistry::default();
    let facets = rk_kernel::registry::Registry::from_toml("").unwrap();
    let item = Item::new("X-1", "story");
    // There is no seventh parameter, and there never will be one that takes a clock.
    let _ = select(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        std::time::SystemTime::now(),
    );
}
