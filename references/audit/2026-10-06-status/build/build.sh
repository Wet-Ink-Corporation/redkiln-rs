#!/bin/sh
cd "<scratchpad>/audit-wt"
export CARGO_TARGET_DIR="<scratchpad>/audit-target"
J=8
step(){ name=$1; shift; echo "=== $name: $*"; start=$(date +%s); "$@" > "<scratchpad>/audit/log-$name.txt" 2>&1; rc=$?; echo "=== $name rc=$rc secs=$(($(date +%s)-start))"; }
step fmt cargo fmt --all --check
step build cargo build --workspace --all-targets -j $J
step clippy cargo clippy --workspace --all-targets -j $J
step test cargo test --workspace --no-fail-fast -j $J
step conformance cargo test -p rk-ports --features conformance -j $J
step doc cargo doc --workspace --no-deps -j $J
echo DONE
