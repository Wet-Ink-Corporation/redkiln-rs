//! **AC-337 — mutation feasibility is declared, and an absent declaration is a configuration
//! error** (`rk-port-toolchain`, a module of `rk-ports`; filtered here by Checkpoint 7b-ii's third
//! loop). *"GIVEN a toolchain pack WHEN it is loaded THEN it declares mutation feasibility
//! explicitly and an absent declaration is a configuration error AND where it declares mutation
//! infeasible, an AC records `mutation: not-feasible(reason)` which is not `pinned`."*
//!
//! What this measures: the declaration TYPE and its three refusals (absent; infeasible with no
//! reason; feasible with a reason that means nothing), and that an infeasible declaration maps to a
//! ledger status which does NOT permit exit. What it does not: the pack loader that reads the
//! `[mutation]` table (`rk-kernel`'s `select::pack`, exercised by the kernel's AC-337 fixture-pack
//! test) and the `doctor` rule-table row (7d).
//!
//! Red first: written against a `toolchain.rs` with no feasibility type — did not compile; then,
//! with the type in, red on its own claim against a reverted mutant that read an absent declaration
//! as `Feasible` (the configuration error became a silent default — the failure AC-337 names).

use rk_ports::toolchain::{FeasibilityError, MutationFeasibility};
use rk_types::artifact::ledger::LedgerStatus;

#[test]
fn ac337_mutation_feasibility_is_declared() {
    // absent → a configuration error, never a default
    assert_eq!(
        MutationFeasibility::declared(None, None),
        Err(FeasibilityError::Undeclared),
        "an absent declaration is a configuration error"
    );
    assert_eq!(
        MutationFeasibility::declared(None, Some("binary assets")),
        Err(FeasibilityError::Undeclared),
        "a reason without the declaration is still no declaration"
    );
    // infeasible needs its reason — `not-feasible(reason)` is the register's shape
    assert_eq!(
        MutationFeasibility::declared(Some(false), None),
        Err(FeasibilityError::InfeasibleWithoutReason)
    );
    assert_eq!(
        MutationFeasibility::declared(Some(false), Some("   ")),
        Err(FeasibilityError::InfeasibleWithoutReason),
        "a blank reason is no reason"
    );
    // feasible carries none
    assert_eq!(
        MutationFeasibility::declared(Some(true), None),
        Ok(MutationFeasibility::Feasible)
    );
    assert_eq!(
        MutationFeasibility::declared(Some(true), Some("why would there be one")),
        Err(FeasibilityError::FeasibleWithReason)
    );
    // the declared-infeasible pack: its ACs record `not-feasible(reason)`, which is NOT pinned
    let infeasible = MutationFeasibility::declared(
        Some(false),
        Some("the Unreal asset toolchain has no mutation harness (ADR-0008)"),
    )
    .expect("infeasible with a reason");
    let status = infeasible
        .ledger_status()
        .expect("an infeasible toolchain fixes every AC's mutation-path status");
    assert!(
        matches!(status, LedgerStatus::NotFeasible { .. }),
        "{status:?}"
    );
    assert_ne!(status, LedgerStatus::Pinned);
    assert!(
        !status.permits_exit(),
        "not-feasible is not pinned and does not let the stage exit (AC-346; AC-357 routes it to a human)"
    );
    // a feasible toolchain fixes nothing — the flip predicate decides per AC (AC-334)
    assert_eq!(MutationFeasibility::Feasible.ledger_status(), None);
    // the error is total and each refusal names itself
    for e in [
        FeasibilityError::Undeclared,
        FeasibilityError::InfeasibleWithoutReason,
        FeasibilityError::FeasibleWithReason,
    ] {
        assert!(!e.to_string().is_empty());
    }
}
