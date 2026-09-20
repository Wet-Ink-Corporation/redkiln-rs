// POSITIVE CONTROL for ADR-0118 c3 — a poisoned store lock halts; it is never recovered, in any spelling (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Each spelling below takes the guard out of a lock whose writer panicked mid-invariant.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_recovery(store: &Mutex<State>) -> MutexGuard<'_, State> {
    store.lock().unwrap_or_else(PoisonError::into_inner)
}
