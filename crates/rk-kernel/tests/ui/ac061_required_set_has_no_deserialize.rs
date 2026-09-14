//! **AC-061's second AND, the other half.** `RequiredSet` *"implements no deserialisation"*.
//!
//! A private constructor alone is not enough: a `Deserialize` impl is a second constructor, and it
//! is the one an agent-returned JSON payload would reach. This file must FAIL to compile.

use rk_kernel::select::RequiredSet;

fn main() {
    let _r: RequiredSet = serde_json::from_str("[]").unwrap();
}
