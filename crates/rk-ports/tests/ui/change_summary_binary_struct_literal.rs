// `s8i`, ADR-0199 decision 2 (proposed): the `Binary` payload's fields are PRIVATE, so a crate
// outside `rk-ports` cannot build it with a struct literal and must go through its one recorded
// checked constructor, `BinaryChangeSummary::new`. The committed `.stderr` pins the privacy
// reason (E0451) for each of the four fields; a `pub` field drops its line.
//
// Its own case, beside `change_summary_binary_construction_rule.rs`, because rustc reports E0451
// only when type checking succeeded. Pinned by
// `ui::trybuild_pins_the_binary_change_summary_construction_rule`.

use rk_ports::toolchain::{BinaryChangeSummary, ChangeSummary};

fn main() {
    let literal = BinaryChangeSummary {
        assets: Vec::new(),
        captures: Vec::new(),
        tests_report_ref: String::new(),
        build_log_ref: String::new(),
    };
    let _summary = ChangeSummary::Binary(literal);
}
