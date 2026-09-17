//! **AC-242's positive control** — *"a fixture module containing each forbidden construct that must
//! fail the lint"*. NOT compiled; under `tests/fixtures/lint-controls/` and not `tests/ui/` because a
//! grep control must be an ordinary file carrying the banned tokens, not a compile-fail case
//! (`s7b-ii` §7b-ii.10). `gate_path_purity_rule` run over this text must report every line below.
//!
//! The gate path (`workflow/fsm.rs`, `workflow/gate.rs`) may contain none of these: a wait is a
//! durable mark plus a later read, never a process that survives (golden constraint 2; F22).

fn a_gate_that_waits(deadline: Timestamp) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let (tx, rx) = std::sync::mpsc::channel::<Answer>();
    let answer = rx.recv();
    let _timer = std::thread::spawn(move || tx.send(poll_until(deadline)));
    let now = std::time::SystemTime::now();
    let contents = std::fs::read_to_string(".redkiln/gate");
    let root = std::env::var("REDKILN_ROOT");
}
