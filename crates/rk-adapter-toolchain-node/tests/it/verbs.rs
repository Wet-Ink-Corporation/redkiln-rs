//! **The Node adapter implements no verb at this stage** (s8c §8c.4; record V = ADR-0170 decision 2,
//! proposed: *"The Node adapter implements none at this stage"*; decision 7: *"The Node adapter
//! refuses `setup` by name at this stage"*). Every verb, the four the Rust adapter handles
//! included, is refused by its bare name.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::collections::BTreeMap;

use rk_ports::toolchain::{StepBound, Toolchain, VerbCall};
use rk_types::error::StepError;

use crate::support::{GENEROUS, adapter};

/// **An undeclared verb is refused by name** -- and on this adapter every verb is undeclared.
#[test]
fn an_undeclared_verb_is_refused_by_name() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let mut inputs = BTreeMap::new();
    inputs.insert(
        "projects".to_owned(),
        serde_json::json!(["crates/rk-adapter-toolchain-node/tests/fixtures/project"]),
    );
    for verb in [
        "setup",
        "smoke",
        "sca_scan",
        "package",
        "item_create",
        "release_tag",
        "mutate",
        "deploy",
    ] {
        match tc.run_verb(&VerbCall {
            verb,
            inputs: &inputs,
            bound: StepBound::Undeclared,
        }) {
            Err(StepError::Unsupported { capability }) => {
                assert_eq!(capability, verb, "refused by the BARE name");
            }
            other => panic!("the Node adapter refuses `{verb}` by name; got {other:?}"),
        }
    }
}
