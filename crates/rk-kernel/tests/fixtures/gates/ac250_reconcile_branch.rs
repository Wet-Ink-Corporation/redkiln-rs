//! **AC-250's lint-fail fixture — the positive control for the no-reconciliation rule.** NOT compiled;
//! not under `tests/ui/` (it is not a compile-fail case); it exists so the architecture lint in
//! `rk_kernel::module_boundary::void_consumer_rule` is proven to FIRE on the shape it forbids. A lint
//! with no fixture is green over a gate path nobody has written (`s7b-ii` §7b-ii.11).
//!
//! ADR-0017's rejected option is literally "reconcile stale answers". This is what that would look
//! like as a fold arm: a consumer of `gate_answer_void` that writes a state transition.

fn apply_reconciling(state: &mut RunState, event: &Event) -> Result<(), FsmError> {
    match event.kind {
        EventKind::GateAnswerVoid => {
            let token = required_digest(event, "gate_answer_void", "token")?;
            // THE RECONCILIATION BRANCH AC-250 FORBIDS: the void answer is re-applied best-effort
            // and the item advances on it.
            if let Some(answer) = state.gates_answered.get(&token).cloned() {
                state.stages_entered.push(answer.verdict.clone());
                state.stage = answer.verdict;
            }
            state.gates_void.insert(token);
        }
        _ => {}
    }
    Ok(())
}
