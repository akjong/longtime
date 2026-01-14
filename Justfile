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