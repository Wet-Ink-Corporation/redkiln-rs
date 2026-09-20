// POSITIVE CONTROL for ADR-0118 c5 — the acquisition is the composition root's ONE function (`s7d` §7d.13, §7d.17).
//
// Never compiled, never linked. Checkpoint 7d's `rk_control` greps THIS file with the SAME pattern
// its paired `rk_absent` line uses, through the same comment filter, so that an absence ban is
// never asserted by a scanner that has not been seen to find anything. Every other Mutex::lock in rk-cli is banned so the halt cannot be bypassed by a second acquirer.
//
// The token below is on a NON-COMMENT line, deliberately: the filter drops comment lines before
// counting, so a control carrying it only in this header would prove nothing.

fn a_second_acquirer(store: &Mutex<State>) -> MutexGuard<'_, State> {
    store.lock().expect("held")
}
