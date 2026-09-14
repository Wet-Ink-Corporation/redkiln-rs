//! **AC-050.** `Rigor` is ordinal and admits **comparison only**. A rank reversal in a weighted sum
//! is the measured failure (catalog §2.3; F27: verdict 2/4 items flaky over repeated draws), so
//! *"pack or engine code attempting to add, average, or otherwise arithmetically combine rigor
//! values"* must **not compile**.
//!
//! This file must FAIL to compile. If it ever compiles, rigor has become arithmetic and the skip at
//! `min_rigor = recommended` against a stage running `optional` is an arithmetic result rather than
//! a logged row.

use rk_kernel::rigor::Rigor;

fn main() {
    // No `Add`.
    let _sum = Rigor::Optional + Rigor::Mandatory;
    // No `Into<f64>`, so no averaging.
    let _avg: f64 = Rigor::Recommended.into();
    // No `Sum`, so no folding a column of them.
    let _total: Rigor = [Rigor::Optional, Rigor::Mandatory].into_iter().sum();
}
