//! AC-172's instrument: a state that reaches the planner without passing through `reconcile()`
//! must not be **constructible**.
//!
//! This file must FAIL to compile. If it ever compiles, `Reconciled`'s field has been made public
//! or a constructor has been added, and ADR-0109's forbids clause — *"Forbids any code path that
//! reaches the planner without a `Reconciled<State>` produced in that invocation"* — is no longer
//! enforced by anything.

use rk_git::reconcile::State;
use rk_git::reconciled::Reconciled;

/// A planner, standing in for the real one: it accepts only reconciled state.
fn planner(_state: &Reconciled<State>) {}

fn main() {
    let unreconciled = State::default();

    // The whole point: there is no way to get from here to a `Reconciled<State>`.
    let smuggled = Reconciled {
        inner: unreconciled,
    };

    planner(&smuggled);
}
