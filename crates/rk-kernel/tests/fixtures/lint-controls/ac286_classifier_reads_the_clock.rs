//! **AC-286's positive control** for the ban on `std::fs` / `std::time` / `std::env` /
//! `SystemTime` / `Instant` inside the classifier module. NOT compiled; the source rule
//! `module_boundary::classifier_purity_rule` and Checkpoint 7b-ii's grep must both fire on it.

pub fn classify(repro: &Repro, observation: &Observation) -> EvidenceClass {
    let now = std::time::SystemTime::now();
    let salt = std::env::var("REVIEW_SALT").unwrap_or_default();
    let prior = std::fs::read_to_string(".redkiln/last-class").ok();
    let t0 = std::time::Instant::now();
    let _ = (now, salt, prior, t0, repro, observation);
    EvidenceClass::Advisory
}
