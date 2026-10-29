# Format and lint, applying fixes
fix:
    dx fmt --all-code
    cargo clippy --fix --allow-dirty --target wasm32-unknown-unknown
    cargo clippy --fix --allow-dirty --no-default-features --features server
    dx check

# Check formatting and lints without changing anything (for CI)
check:
    cargo fmt --check
    dx fmt --check
    cargo clippy --target wasm32-unknown-unknown -- -D warnings
    cargo clippy --no-default-features --features server -- -D warnings
    dx check
