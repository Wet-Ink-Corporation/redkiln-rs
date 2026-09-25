// The no-dispatch fake `claude` for runbook checkpoints (D21-47). runbook/tools/nodispatch.sh copies this file,
// fills RK_PIN and RK_LOG, and compiles it to `claude.exe` (Windows) or `claude` (POSIX). It must be COMPILED: a Rust
// `Command::new("claude")` on Windows resolves only `claude.exe` from PATH, never a shell script, so a script fake
// leaves the real binary to answer (s8h review #1).
//
// `--version` alone prints the pin, so doctor's RK-D-CLI-VERSION and Checkpoint 7d's Halt 2 line meet the pinned
// version. Any other call is appended to the call log and exits 97, so a spawn that would have dispatched is refused
// and seen.
use std::io::Write;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() == 1 && a[0] == "--version" {
        println!("RK_PIN (Claude Code)");
        return;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("RK_LOG")
        .expect("the fake claude's call log");
    let _ = writeln!(f, "{:?}", a);
    std::process::exit(97);
}
