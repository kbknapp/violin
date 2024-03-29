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

# Check the formatting of the code but don't actually format it
fmt-check:
    cargo fmt --all --check

# Lint the code
lint:
    cargo clippy --all-targets -- -Dwarnings
    cargo clippy --all-targets --no-default-features -- -Dwarnings
    cargo clippy --all-targets --all-features -- -Dwarnings

# Run benchmarks
bench $RUSTFLAGS='-Ctarget-cpu=native':
    cargo bench

# Run the test suite
test TEST_RUNNER='cargo nextest run':
    {{ TEST_RUNNER }}
    {{ TEST_RUNNER }} --no-default-features
    {{ TEST_RUNNER }} --all-features

# Check for typos
spell-check: (_cargo-install 'typos-cli')

_cargo-install TOOL:
    cargo install {{TOOL}}
