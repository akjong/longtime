# longtime — Agent Instructions

## Overview

A multi-timezone time management tool. The Rust workspace holds the TUI in `bin/tui` and the Leptos web client in `bin/web`.

- Layout: a Cargo workspace with `bin/` for the executables and `crates/` for the libraries.
- Toolchain: the default stable toolchain, plus a nightly toolchain for `rustfmt` and `clippy` (the `.rustfmt.toml` config uses unstable options).
- Instruction source: this file is the single source of truth for the repository.

## Non-negotiables

- Surgical edits only: every changed line must trace to the request. No drive-by refactors, no reformatting untouched code, no deleting pre-existing dead code (mention it instead).
- Fix root causes. Never silence a warning, weaken a check, or patch a symptom to make a gate pass.
- Never report completion while a `just` gate below fails. Report the failure verbatim and stop.
- Stop and ask when a request is ambiguous, conflicts with existing conventions, or requires credentials, publishing, or destructive operations.
- When a correction reveals a missing rule, add one specific rule here ("always X for Y", never "be careful with Y"); delete rules that never catch real errors.

## Scope

- `bin/` holds the executable crates: the CLI, server, and app entry points.
- `crates/` holds the reusable library crates. Every crate must work as a standalone library with no hidden coupling.
- `scripts/` holds the operational scripts.
- `docs/` holds the design documents.
- English only in code, comments, docs, and commit messages.

## Execution

- Parallelize aggressively: dispatch subagents for independent research, exploration, and implementation.
- NEVER run two `cargo` commands concurrently, and never overlap file edits with a running `cargo` command (`target/` file lock): finish the edits, wait, then run.
- NEVER use `git worktree`. Edit the current branch in place.
- TDD for behavior changes: failing test first, smallest change to pass, then run `just mutation` and fix surviving mutants.

## Verification loop

After each feature or bug fix, in order; any failure means the task is not done:

```bash
just format
just lint
just test
just mutation
```

## Workspace rules

- Never hand-type dependency versions. Add per-crate deps with `cargo add <crate> -p <crate>`. Shared deps go into root `[workspace.dependencies]` — numeric versions only, no features — and are referenced with `dep = { workspace = true }`.
- Sub-crates inherit `version`/`edition`/`license`/`repository` via `workspace = true`. Inheritance failures are fixed by `just fix` (cargo workspace-inheritance-check).
- `.cargo/config.toml` does not set a `rustc-wrapper`. If you add one, unset it (`RUSTC_WRAPPER=`) whenever clippy runs, because the wrapper breaks `clippy --fix`.
- `.rustfmt.toml` enables unstable rustfmt options, so formatting runs on nightly: `cargo +nightly fmt`. Never hand-format around a `cargo fmt` diff.
- Lint policy lives in `[workspace.lints]` in the root `Cargo.toml`. Prefer `#[expect(lint, reason = "...")]` over `#[allow]`; a bare `#[allow]` silences the lint for the whole item and hides later drift.
- `cargo sort` keeps the dependency lists ordered; cargo machete reports unused dependencies. Both run through `just lint`.

## Dependency choices

Preferred defaults: clap, config, eyre, proptest, serde, thiserror, tokio, tracing, tracing-subscriber, tracing-opentelemetry, opentelemetry-otlp, sqlx, utoipa, arc-swap, hpx, scc, winnow, shadow-rs, ecdysis.

- HTTP: `hpx` with rustls; never `reqwest`.
- Concurrent map/set: `scc`; read-heavy shared state: `arc-swap`; never `dashmap` or `RwLock<HashMap<...>>` on hot paths.
- Parsing: `winnow` or `pest`, not hand-rolled parsers.
- Caches: `moka`, not a hand-rolled LRU.
- Forbidden by default: `anyhow`, `log`, `reqwest`, `dashmap`.

## Code conventions

- Errors: `eyre` in `bin/`, `thiserror` in `crates/`. Concrete error types through `?`; no string errors.
- `unsafe` only with a documented, measurable performance or FFI reason plus a `// SAFETY:` comment.
- sqlx: runtime queries (`query_as` + `FromRow` derive); no compile-time macros by default.
- Config: `config` crate, TOML files. Daemons and servers use `ecdysis` for graceful restart and reload.
- Locks: `parking_lot` for sync locking; release guards before `.await`; `tokio::sync::Mutex` only when a lock spans `.await`; `spawn_blocking` for CPU-bound or blocking work.
- Channels: `tokio::sync::mpsc`/`broadcast` for async; `crossbeam-channel` or `flume` for sync and MPMC; never `std::sync::mpsc`.
- Hot paths: no allocations; `bytes::Bytes` for network buffers; `rkyv`/`zerocopy` for critical serialization; `ahash`/`rustc-hash` for trusted hash keys; `compact_str`/`smol_str` for small strings.
- Tests: unit tests live next to the implementation in `#[cfg(test)]` modules; integration tests live in the crate's `tests/` directory; `proptest` stays inside the ordinary `cargo test` loop.

## Leptos rules

- Build, serve, and bundle through `cargo-leptos` (`just fe-dev`, `just fe-build`); NEVER use `trunk`.
- Leptos configuration lives in the crate-local `[package.metadata.leptos]` block; change it there, not in a global file.
- `leptosfmt` runs inside `just format` and `just lint`; never hand-format a `view!` block.
- Reactivity: `RwSignal` for state, `Memo` for derived values, `Effect` for side effects, `StoredValue` for non-reactive storage. Read a signal by calling it (`count()`).
- Server calls go through `#[server]`, routes through `path!`, and server mutations through `Action` so the client invalidates automatically.
