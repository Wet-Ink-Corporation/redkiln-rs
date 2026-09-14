//! The one canonical form and the one hash — **D3, executed**, and §7b.9(2)'s layering question
//! settled as option A.
//!
//! `rk-kernel` depends on `rk-store` and calls [`rk_store::canon::canonical`]. It does **not** write
//! a second canonical serialiser, because the form is already forced:
//!
//! > *"Object keys are sorted byte-wise … LF only, no trailing newline … Floats are refused."*
//! > — `crates/rk-store/src/canon.rs`
//!
//! ADR-0097 decision 2 component 4 already names that form for step inputs, and AC-040's
//! byte-identity is already measured under it. The hazard the edge exists to close is measured too:
//! `serde_json` carries `preserve_order` **workspace-wide** (root `Cargo.toml`:44), so a plain
//! `to_vec` over an insertion-ordered map leaks construction order into AC-040's assertion — and
//! still passes 100 repetitions of the same call, because repeating a call does not re-order a map.
//!
//! The hash is `rk_types::digest::sha256` / `sha256_lp` — **the one hash ADR-0116 clause 6 admits**
//! (*"No second digest algorithm enters the system"*), carrying ADR-0097 decision 2's big-endian
//! `u32-length || bytes` framing. The framing is load-bearing: without it `("ab","c")` and
//! `("a","bc")` hash identically.

use super::{Catalog, Item, Pack};

pub use rk_store::canon::{CanonError, canonical};
pub use rk_types::digest::Digest;

/// The digest of some bytes, under the one hash.
#[must_use]
pub fn digest_of(bytes: &[u8]) -> Digest {
    rk_types::digest::sha256(bytes)
}

/// The digest of a value **under the canonical serialiser**.
///
/// # Errors
///
/// [`CanonError`] if a float reached it, which `rk-store` refuses by decision rather than by
/// choosing a format.
pub fn canonical_digest(value: &serde_json::Value) -> Result<Digest, CanonError> {
    rk_store::canon::canonical_digest(value)
}

/// `inputs_digest` — `sha256_lp` over AC-041's four digests **in the order the criterion names
/// them**: `(pack digest, catalog digest, context digest, artifact digests)`.
///
/// The four are hashed as a length-prefixed tuple rather than concatenated, for the reason
/// AC-064's 2026-09-13 amendment states about its own three: without the framing, two different
/// tuples whose components absorb each other's boundaries share an identity.
///
/// # Errors
///
/// [`CanonError`] from any of the four canonicalisations.
pub fn inputs_digest(pack: &Pack, catalog: &Catalog, item: &Item) -> Result<Digest, CanonError> {
    let pack_d = canonical_digest(&pack.to_json())?;
    let catalog_d = canonical_digest(&catalog_json(catalog))?;
    let context_d = canonical_digest(&item.context_json())?;
    let artifacts_d = canonical_digest(&item.artifacts_json())?;
    Ok(rk_types::digest::sha256_lp(&[
        pack_d.as_str().as_bytes(),
        catalog_d.as_str().as_bytes(),
        context_d.as_str().as_bytes(),
        artifacts_d.as_str().as_bytes(),
    ]))
}

/// The catalog as a canonical value.
///
/// [`Catalog::from_sources`] and [`Catalog::from_dir`] both sort by activity id, so the *order* is
/// already independent of load order; this function digests the fields steps 1-4 actually read, so
/// a catalog edit that cannot change a selection cannot change the digest either.
pub(in crate::select) fn catalog_json(catalog: &Catalog) -> serde_json::Value {
    serde_json::Value::Array(
        catalog
            .activities()
            .iter()
            .map(|a| {
                serde_json::json!({
                    "id": a.id,
                    "stage": a.stage,
                    "provides": a.provides,
                    "requires": a.requires.iter().map(|r| {
                        serde_json::json!({ "kind": r.kind, "optional": r.optional })
                    }).collect::<Vec<_>>(),
                    "tags": a.tags,
                    "applicable_if": a.applicable_if,
                    "method_rank": a.method_rank,
                    "min_rigor": a.min_rigor,
                    "max_rigor": a.max_rigor,
                    "cost_class": a.cost_class,
                    "family": a.family,
                })
            })
            .collect(),
    )
}
