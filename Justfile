# Run TUI application
dev-tui:
    cargo run -p longtime-tui -- -c timezones.toml

# Run Web application (development)
dev-web:
    cd bin/web && cargo leptos watch

# Build Web application (release)
build-web:
    cd bin/web && cargo leptos build --release
    cp bin/web/index.html target/site/index.html
    cp target/site/pkg/longtime.wasm target/site/pkg/longtime_bg.wasm

# Deploy Web application to Cloudflare Worker
deploy-worker: build-web
    cd bin/web && npx wrangler deploy

# Build all workspace crates
build:
    cargo build --workspace

# Format code
format:
    rumdl fmt .
    taplo fmt
    leptosfmt .
    cargo +nightly fmt --all

# Fix markdown issues
fix:
    rumdl check --fix .

# Lint all code
lint:
    rumdl check .
    taplo fmt --check
    cargo +nightly fmt --all -- --check
    leptosfmt . --check
    cargo +nightly clippy --workspace -- -D warnings -A clippy::derive_partial_eq_without_eq -D clippy::unwrap_used -D clippy::uninlined_format_args
    cargo machete

# Run all tests
test:
    cargo test --workspace --all-features

# Test coverage
test-coverage:
    cargo tarpaulin --all-features --workspace --timeout 300

# Check for Chinese characters
check-cn:
    rg --line-number --column "\p{Han}"

# Full CI check
ci: lint test

# Show help
help:
    @echo "Available commands:"
    @echo "  just dev-tui      - Run TUI application"
    @echo "  just dev-web      - Run Web application (dev server)"
    @echo "  just build-web    - Build Web application (release)"
    @echo "  just build        - Build all workspace crates"
    @echo "  just format       - Format all code"
    @echo "  just lint         - Run all linters"
    @echo "  just test         - Run all tests"
    @echo "  just ci           - Full CI check (lint + test)"

# Default recipe to display help
default:
  @just --list

# Run mutation tests with cargo-mutants
mutation:
  cargo mutants

# Clean build artifacts
clean:
  cargo clean

# Generate documentation for the workspace
docs:
  cargo doc --no-deps --open

# Install all required development tools
setup:
  cargo install cargo-mutants
  cargo install cargo-machete
  cargo install cargo-sort
  cargo install typos-cli
  cargo install rumdl

# ============================================================
# Maintenance & Tools
# ============================================================

# Run security audit
audit:
  cargo audit

# Check for outdated dependencies
outdated:
  cargo outdated

# Run clippy with all lints
clippy-all:
  cargo +nightly clippy --all-targets --all-features -- -W clippy::pedantic -W clippy::nursery -W clippy::cargo

# Run tests with nextest
test-nextest:
  cargo nextest run --all-features

# Run miri on unsafe code
miri:
  cargo +nightly miri test --all-features

# Check for unused dependencies
unused-deps:
  cargo shear --check

# Update all dependencies
update-deps:
  cargo update

# Full check (lint + test + build + audit)
full-check: lint test build audit
