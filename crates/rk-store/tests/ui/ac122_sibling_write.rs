//! AC-122's compile-fail case (`s8f` §8f.5; the render record ADR-0189 decision 7). A sibling crate
//! holds `rk-store`'s managed directory and tries to write a byte into it: the write handle is
//! private to `rk_store::render`, so this does not compile.
//! *(Added 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ.)*

fn main() {
    let dir = rk_store::render::ManagedDir::of(std::path::Path::new("."), "item-1");
    let _ = dir.write("Spec.json", b"{}");
}
