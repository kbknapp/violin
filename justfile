default: help

# Get a list of recipes you can run
help:
    just --list

# Install required tools for development
setup: (_cargo-install 'cargo-nextest') (_cargo-install 'typos-cli')

# Run all the checks required for CI to pass
ci: spell-check lint test

# Format the code
fmt:
    cargo fmt --all

# Lint the code
lint:
    cargo clippy --all-targets -- -Dwarnings
    cargo clippy --all-targets --no-default-features -- -Dwarnings
    cargo clippy --all-targets --all-features -- -Dwarnings
    cargo fmt --check

# Run benchmarks
bench $RUSTFLAGS='-Ctarget-cpu=native':
    cargo bench

# Run the test suite
test: (_cargo-install 'cargo-nextest')
    cargo nextest run
    cargo nextest run --no-default-features
    cargo nextest run --all-features

# Check for typos
spell-check: (_cargo-install 'typos-cli')

_cargo-install TOOL:
    cargo install {{TOOL}}
