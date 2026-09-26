//! `wsbin`, the workspace fixture's `[[bin]]` target (s8c §8c.3, §8c.4): the binary the `smoke`
//! handler runs checklist rows against and the `package` handler hashes. std only.
//!
//! - `wsbin --version` prints `wsbin 0.0.0` and exits 0;
//! - `wsbin --exit <n>` exits with status `n` and prints nothing;
//! - anything else prints `wsbin ok` and exits 0.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") => println!("wsbin 0.0.0"),
        Some("--exit") => {
            let code = args.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(1);
            std::process::exit(code);
        }
        _ => println!("wsbin ok"),
    }
}
