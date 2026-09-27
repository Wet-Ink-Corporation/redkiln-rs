test -f .redkiln/config.toml || { echo "FAIL: .redkiln/config.toml absent — Step 7d's init did not run, or did not scaffold it (E6)"; exit 1; }
