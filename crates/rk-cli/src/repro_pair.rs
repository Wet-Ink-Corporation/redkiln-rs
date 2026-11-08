//! **The item's committed repro pair, read for `repro_place`** (ADR-0204 decision 2, proposed;
//! added 2026-11-08, s8 repro-pair route).
//!
//! `run` reads `.redkiln/intake/<item>.repro.toml` and `.repro.rs` from the **run tree**, which is
//! the fork commit plus the run's commits, so the bytes are the committed, frozen ones (CODE_PATHS
//! includes `.redkiln`). It parses the TOML into
//! [`ReproPairSpec`](rk_types::repro_pair::ReproPairSpec), validates it into a
//! [`rk_types::repro_pair::ReproPair`], and binds the result as the row input `repro_place.pair`.
//! It always binds the envelope: an absent or invalid pair is refused at `place`, by name, and never
//! at load for every item and stage.
//!
//! Both reads are capped at
//! [`REPRO_PAIR_MAX_BYTES`](rk_types::repro_pair::REPRO_PAIR_MAX_BYTES) (H-13). Each reads one byte past the cap, so
//! "too large" is told apart from "exactly the cap".

use std::io::Read;
use std::path::Path;

use rk_types::repro_pair::{
    REPRO_PAIR_MAX_BYTES, ReproPair, ReproPairError, ReproPairInput, ReproPairSpec,
};

/// The pair for `item` in the run tree at `tree`, as the envelope `repro_place.pair` carries.
///
/// Never fails: every failure is an [`ReproPairInput::Invalid`] or [`ReproPairInput::Absent`]
/// value, refused later by `place`, by name. A missing `.repro.toml` is `Absent`; a `.repro.toml`
/// without its `.repro.rs` is `Invalid`, naming the `.repro.rs`.
#[must_use]
pub fn read(tree: &Path, item: &str) -> ReproPairInput {
    let spec_rel = format!(".redkiln/intake/{item}.repro.toml");
    let source_rel = format!(".redkiln/intake/{item}.repro.rs");
    let spec_bytes = match read_capped(tree, &spec_rel) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            return ReproPairInput::Absent {
                looked_for: vec![spec_rel, source_rel],
            };
        }
        Err(why) => return invalid(spec_rel, why),
    };
    let source_bytes = match read_capped(tree, &source_rel) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            return invalid(
                source_rel,
                format!("the file is missing, and {spec_rel} names a repro"),
            );
        }
        Err(why) => return invalid(source_rel, why),
    };
    let spec = match std::str::from_utf8(&spec_bytes)
        .map_err(|e| format!("not UTF-8: {e}"))
        .and_then(|text| toml::from_str::<ReproPairSpec>(text).map_err(|e| e.to_string()))
    {
        Ok(spec) => spec,
        Err(why) => return invalid(spec_rel, why),
    };
    let source = match String::from_utf8(source_bytes) {
        Ok(source) => source,
        Err(e) => return invalid(source_rel, format!("not UTF-8: {e}")),
    };
    match ReproPair::new(spec, source, &spec_bytes) {
        Ok(pair) => ReproPairInput::Present(pair),
        Err(
            e @ (ReproPairError::SourceTooLarge { .. } | ReproPairError::SourceLacksTest { .. }),
        ) => invalid(source_rel, e.to_string()),
        Err(e) => invalid(spec_rel, e.to_string()),
    }
}

/// [`read`]'s envelope as the JSON row input `repro_place.pair`. A serialisation failure is itself
/// an `Invalid` envelope, so it too is refused at `place`, by name, and never dropped.
#[must_use]
pub fn row_input(tree: &Path, item: &str) -> serde_json::Value {
    serde_json::to_value(read(tree, item)).unwrap_or_else(|e| {
        serde_json::json!({
            "state": "invalid",
            "path": format!(".redkiln/intake/{item}.repro.toml"),
            "why": format!("the repro pair envelope did not serialise: {e}"),
        })
    })
}

/// `Invalid { path, why }`.
const fn invalid(path: String, why: String) -> ReproPairInput {
    ReproPairInput::Invalid { path, why }
}

/// The bytes of `tree/rel`, `Ok(None)` when it does not exist, or the rendered failure (an
/// over-cap file included).
fn read_capped(tree: &Path, rel: &str) -> Result<Option<Vec<u8>>, String> {
    let file = match std::fs::File::open(tree.join(rel)) {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("the file did not open: {e}")),
    };
    let cap = u64::try_from(REPRO_PAIR_MAX_BYTES).map_or(u64::MAX, |cap| cap.saturating_add(1));
    let mut bytes = Vec::new();
    file.take(cap)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("the file did not read: {e}"))?;
    if bytes.len() > REPRO_PAIR_MAX_BYTES {
        return Err(format!(
            "the file is over the {REPRO_PAIR_MAX_BYTES}-byte cap (H-13)"
        ));
    }
    Ok(Some(bytes))
}
