// POSITIVE CONTROL for the signal handler's "nothing but fire the token" scan (`s7d` §7d.10,
// §7d.14's self-control rule).
//
// Never compiled, never linked. `signal_handler_fires_cancel_token` scans `src/signal.rs` for the
// spellings below and must find them HERE first, or a scanner that visited nothing would report a
// clean handler having read no handler at all.
//
// Each line is one of the three things §7d.10 forbids a handler to do, and each has a reason: a
// handler runs on a foreign thread, at an arbitrary point, while the store lock may be held by the
// thread it interrupted. So it must not take that lock, must not print into a half-written line,
// and must not exit past every Drop — including the containment job that kills the child tree.

fn a_handler_that_does_too_much(token: &CancelToken, store: &Arc<Mutex<dyn EventStore>>) {
    eprintln!("interrupted");
    let mut guard = store.lock().expect("held");
    guard.append(cancel_event()).expect("appended");
    token.cancel();
    std::process::exit(130);
}
