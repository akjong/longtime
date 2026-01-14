# LongTime Web Version - Task Breakdown

This document contains the detailed, executable tasks for implementing the LongTime web version. Tasks are organized into phases and should be completed sequentially within each phase. Some phases can run in parallel.

---

## Phase 1: Workspace Restructuring ✅ COMPLETED

**Goal**: Reorganize the project into a Cargo workspace with shared core logic.

### Task 1.1: Create Workspace Manifest ✅

**File**: `Cargo.toml` (root)

- [x] Convert root `Cargo.toml` to workspace manifest
- [x] Define workspace members: `["crates/*", "bin/*"]`
- [x] Move all dependencies to `[workspace.dependencies]` with version numbers only
- [x] Define `[workspace.package]` with shared version and edition

**Acceptance Criteria**:

- [x] `cargo check --workspace` passes
- [x] No version numbers in sub-crate Cargo.toml files

---

### Task 1.2: Create Core Crate ✅

**Directory**: `crates/core/`

- [x] Create `crates/core/Cargo.toml`
- [x] Create `crates/core/src/lib.rs` with module re-exports
- [x] Create `crates/core/src/config.rs` with `Config`, `TimezoneConfig`, `WorkHours`
- [x] Create `crates/core/src/time.rs` with `is_work_hours`, `calculate_time_difference`, etc.
- [x] Implement `Default` for `Config` with Shanghai, London, New York
- [x] Add `#[derive(Clone, PartialEq)]` to all structs

**Acceptance Criteria**:

- [x] `cargo test -p longtime-core` passes (11 tests)
- [x] All structs are `Serialize`, `Deserialize`, `Clone`, `PartialEq`

---

### Task 1.3: Refactor TUI Crate ✅

**Directory**: `bin/tui/`

- [x] Create `bin/tui/Cargo.toml` with workspace dependencies
- [x] Move `src/main.rs` → `bin/tui/src/main.rs`
- [x] Move `src/app.rs` → `bin/tui/src/app.rs`
- [x] Move `src/ui.rs` → `bin/tui/src/ui.rs`
- [x] Create `bin/tui/src/config_loader.rs` with file loading logic
- [x] Update imports to use `longtime_core::{Config, TimezoneConfig, WorkHours}`
- [x] Remove old `src/` directory

**Acceptance Criteria**:

- [x] `cargo run -p longtime-tui -- -c timezones.toml` works identically to before
- [x] `cargo test -p longtime-tui` passes (6 tests)

---

### Task 1.4: Update Justfile ✅

**File**: `Justfile`

- [x] Add `dev-tui` command
- [x] Add `build` command for workspace
- [x] Add `help` command
- [x] Update `lint` to use `--workspace`
- [x] Update `test` to use `--workspace`

**Acceptance Criteria**:

- [x] `just dev-tui` launches the TUI application
- [x] `just test` runs all tests (18 total)

---

## Phase 2: Web Project Initialization ✅ COMPLETED

**Goal**: Set up the Leptos web application structure.

### Task 2.1: Create Web Crate Manifest ✅

**File**: `bin/web/Cargo.toml`

- [x] Create manifest with leptos CSR configuration
- [x] Add new dependencies to workspace root `Cargo.toml`
- [x] Configure `[package.metadata.leptos]`

**Acceptance Criteria**:

- [x] `cargo check -p longtime-web` passes

---

### Task 2.2: Create Web Entry Points ✅

**Files**: `bin/web/src/lib.rs`, `bin/web/src/main.rs`

- [x] Create `bin/web/src/lib.rs` with module exports and `hydrate()` function
- [x] Create `bin/web/src/main.rs` as placeholder for CSR mode

**Acceptance Criteria**:

- [x] Compiles without errors

---

### Task 2.3: Setup TailwindCSS v4 ✅

**Files**: `bin/web/package.json`, `bin/web/style/main.css`

- [x] Create `bin/web/package.json` with Tailwind v4 dev dependency
- [x] Create `bin/web/style/main.css` with `@theme` configuration
- [x] Update root `package.json` to include workspace

**Acceptance Criteria**:

- [x] CSS file can be generated with Tailwind

---

### Task 2.4: Create Index HTML ✅

**File**: `bin/web/index.html`

- [x] Create HTML template with proper structure

**Acceptance Criteria**:

- [x] File exists and is valid HTML5

---

### Task 2.5: Create Assets Directory ✅

**Directory**: `assets/`

- [x] Created `bin/web/assets/` directory with favicon placeholder

**Acceptance Criteria**:

- [x] Directory exists

---

### Task 2.6: Implement Core Components ✅

**Directory**: `bin/web/src/components/`

- [x] Create component module structure with `mod.rs`
- [x] Create `header.rs` - Header with logo, 12/24h toggle, share button
- [x] Create `timezone_card.rs` - Individual timezone display card
- [x] Create `timezone_list.rs` - Grid of timezone cards
- [x] Create `time_controls.rs` - Time adjustment controls
- [x] Create `config_modal.rs` - Add/Edit timezone modal

**Acceptance Criteria**:

- [x] All components compile without errors
- [x] `cargo clippy --workspace` passes

---

### Task 2.7: Implement State Management ✅

**File**: `bin/web/src/state.rs`

- [x] Define `AppState` struct with reactive signals
- [x] Implement state methods: `current_time()`, `adjust_time()`, `reset_time()`, etc.
- [x] Implement modal and config management methods

**Acceptance Criteria**:

- [x] State struct is `Clone` and can be shared via context

---

### Task 2.8: Implement Storage Utilities ✅

**File**: `bin/web/src/storage.rs`

- [x] Implement `save_config()` and `load_config_from_storage()` with LocalStorage
- [x] Implement `encode_config_to_url()` and `decode_config_from_url()` with Base64
- [x] Implement `generate_share_url()` and `load_initial_config()`
- [x] Implement `copy_to_clipboard()` async function
- [x] Add test for encode/decode roundtrip

**Acceptance Criteria**:

- [x] `cargo test -p longtime-web` passes (1 test)

---

### Task 2.9: Update Justfile ✅

**File**: `Justfile`

- [x] Add `dev-web` command for cargo-leptos watch
- [x] Add `build-web` command for release builds
- [x] Add `watch-css` command for Tailwind watch mode

**Acceptance Criteria**:

- [x] Commands are defined in Justfile

---

## Phase 2 Verification ✅

- [x] `cargo check --workspace` passes
- [x] `cargo test --workspace` passes (19 tests: 11 core + 6 TUI + 1 web + 1 doctest)
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] All component files compile without errors

---

## Phase 3: Core State & Storage Implementation ✅ COMPLETED

**Goal**: Implement state management and persistence logic.

### Task 3.1: Implement AppState ✅

**File**: `bin/web/src/state.rs`

- [x] Define `AppState` struct with signals:
  - `config: RwSignal<Config>`
  - `time_offset: RwSignal<i64>` (seconds)
  - `is_running: RwSignal<bool>`
  - `show_config_modal: RwSignal<bool>`
  - `editing_index: RwSignal<Option<usize>>`
  - `selected_index: RwSignal<usize>`
  - `tick: RwSignal<u64>` (for reactivity)

- [x] Implement constructor `AppState::new(config: Config) -> Self`

- [x] Implement methods:
  - `current_time(&self) -> DateTime<Utc>`
  - `adjust_time(&self, minutes: i64)`
  - `reset_time(&self)`
  - `toggle_running(&self)`
  - `toggle_format(&self)`
  - `open_add_modal(&self)`
  - `open_edit_modal(&self, index: usize)`
  - `close_modal(&self)`
  - `delete_timezone(&self, index: usize)`

**Acceptance Criteria**:

- [x] All methods compile and work with Leptos signal system
- [x] AppState derives Clone for context sharing

---

### Task 3.2: Implement Storage Module ✅

**File**: `bin/web/src/storage.rs`

- [x] Implement LocalStorage functions:
  - `save_config(config: &Config)`
  - `load_config_from_storage() -> Option<Config>`
  - `clear_config()`

- [x] Implement URL encoding/decoding:
  - `encode_config_to_url(config: &Config) -> String`
  - `decode_config_from_url(encoded: &str) -> Option<Config>`
  - `generate_share_url(config: &Config) -> String`

- [x] Implement clipboard helper:
  - `copy_to_clipboard(text: &str) -> Result<(), String>` (async)

- [x] Implement configuration loader:
  - `load_initial_config() -> Config` (checks URL → LocalStorage → Default)

**Acceptance Criteria**:

- [x] Unit tests pass for encoding/decoding round-trip (1 test)
- [x] Clippy passes

---

## Phase 4: UI Component Development ✅ COMPLETED

**Goal**: Build all Leptos components.

### Task 4.1: Create Root App Component ✅

**File**: `bin/web/src/app.rs`

- [x] Create `App` component:
  - Initialize `AppState` with `load_initial_config()`
  - Provide state via Leptos context
  - Set up 1-second interval for time updates
  - Render layout with Header, TimezoneList, TimeControls, ConfigModal

**Acceptance Criteria**:

- [x] App compiles without errors

---

### Task 4.2: Create Components Module ✅

**File**: `bin/web/src/components/mod.rs`

- [x] Create module file exporting all components:
  - Header, TimezoneCard, TimezoneList, TimeControls, ConfigModal

---

### Task 4.3: Implement Header Component ✅

**File**: `bin/web/src/components/header.rs`

- [x] Create `Header` component with:
  - Logo/Title with Unicode clock icon
  - 12h/24h format toggle button
  - Add timezone button
  - Share button (generates URL, copies to clipboard)

- [x] Style with Tailwind:
  - Sticky header
  - Flexbox layout with spacing
  - Responsive button sizes

**Acceptance Criteria**:

- [x] Component compiles without errors

---

### Task 4.4: Implement TimezoneCard Component ✅

**File**: `bin/web/src/components/timezone_card.rs`

- [x] Create `TimezoneCard` component with props:
  - `config: TimezoneConfig`
  - `index: usize`
  - `reference_offset: i32`

- [x] Display elements:
  - City name (bold) and timezone
  - Current time (large, monospace)
  - Date (smaller)
  - Time difference badge
  - Work status indicator (green/red dot + text)
  - Edit/Delete action buttons

- [x] Style with Tailwind:
  - Card with rounded corners and shadow
  - Hover state for actions

**Acceptance Criteria**:

- [x] Component compiles without errors
- Work status indicator is accurate
- Delete/Edit callbacks fire correctly

---

### Task 4.5: Implement TimezoneList Component ✅

**File**: `bin/web/src/components/timezone_list.rs`

- [x] Create `TimezoneList` component:
  - Get `AppState` from context
  - Calculate reference offset from selected timezone
  - Render grid of `TimezoneCard` components
  - Show empty state when no timezones

- [x] Style with Tailwind:
  - CSS Grid layout: 1 col mobile, 2 cols tablet, 3+ cols desktop
  - Gap between cards

**Acceptance Criteria**:

- [x] List updates reactively when config changes
- [x] Responsive grid layout

---

### Task 4.6: Implement TimeControls Component ✅

**File**: `bin/web/src/components/time_controls.rs`

- [x] Create `TimeControls` component:
  - Reset button (sets offset to zero)
  - Play/Pause toggle (controls `is_running`)
  - Time stepper buttons: -1h, -15m, +15m, +1h
  - Current offset display

- [x] Style with Tailwind:
  - Sticky position at bottom
  - Button group styling

**Acceptance Criteria**:

- [x] Component compiles without errors

---

### Task 4.7: Implement ConfigModal Component ✅

**File**: `bin/web/src/components/config_modal.rs`

- [x] Create `ConfigModal` component:
  - Modal overlay with click-outside-to-close
  - Form fields:
    - Timezone name (text input)
    - IANA timezone (dropdown with common options)
    - Work hours start (time input)
    - Work hours end (time input)
  - Save and Cancel buttons

- [x] Implement modes:
  - "Add" mode: Empty form, creates new TimezoneConfig
  - "Edit" mode: Pre-filled form, updates existing config

- [x] Style with Tailwind:
  - Centered modal with backdrop blur
  - Form layout with labels

**Acceptance Criteria**:

- [x] Component compiles without errors

---

## Phase 5: Integration & Testing ✅

**Goal**: Test the complete application in browser.

### Task 5.1: Install cargo-leptos ✅

- [x] Install cargo-leptos if not present:

  ```bash
  cargo install cargo-leptos
  ```

---

### Task 5.2: Install Bun Dependencies ✅

- [x] Run `bun install` at project root
- [x] Verify `bun.lockb` exists only at root

---

### Task 5.3: Build and Test Web Application ✅

- [x] Run `just dev-web` or `cargo leptos watch -p longtime-web`
- [x] Open browser at <http://127.0.0.1:3000>
- [x] Test features:
  - [x] Time displays correctly for each timezone
  - [x] Time updates every second
  - [x] 12/24h toggle works
  - [x] Add timezone modal works
  - [x] Edit timezone modal works
  - [x] Delete timezone works
  - [x] Time controls (±15m, ±1h) work
  - [x] Play/Pause works
  - [x] Reset to current time works
  - [x] Share URL is generated correctly
  - [x] URL config parameter loads correctly
  - [x] LocalStorage persistence works

---

### Task 5.4: Fix Any Runtime Issues ✅

- [x] Address any JavaScript console errors
- [x] Fix any styling issues
- [x] Ensure responsive design works

---

**Acceptance Criteria**:

- [x] `just dev-web` starts development server with hot reload
- [x] `just build-web` produces optimized WASM bundle

---

## Phase 5.5: Future Improvements (Optional)

These tasks are nice-to-have improvements for future iterations.

### Task 5.5.1: Add Keyboard Shortcuts

**File**: `bin/web/src/app.rs` (or separate `hooks.rs`)

- [ ] Implement keyboard event handler:
  - `←` / `→`: Adjust time ±15 minutes
  - `r`: Reset time
  - `Space`: Toggle play/pause
  - `Esc`: Close modal
  - `?`: Show help (optional)

- [ ] Use `leptos::window_event_listener` or `web-sys` for key events

**Acceptance Criteria**:

- Keyboard shortcuts work when modal is closed
- Shortcuts disabled when input is focused

---

### Task 5.5.2: Implement URL Query Handling

**File**: `bin/web/src/app.rs`

- [x] On app mount:
  - Parse URL query string for `config` parameter
  - If found, decode and apply configuration
  - Clear URL query after applying (optional, for cleaner URL)

- [x] Ensure sharing flow:
  1. User clicks Share
  2. URL with encoded config is generated
  3. URL is copied to clipboard
  4. Toast notification confirms copy

**Acceptance Criteria**:

- Opening shared URL loads correct configuration
- Configuration persists to LocalStorage after loading from URL

---

### Task 5.5.3: Add Loading & Error States

**Files**: Various component files

- [ ] Implement loading state:
  - Show skeleton cards while initializing
  - Graceful handling of slow WASM load

- [ ] Implement error handling:
  - Invalid timezone: Show error in card
  - LocalStorage errors: Log to console, continue
  - URL decode errors: Log warning, use defaults

**Acceptance Criteria**:

- No panics or blank screens on errors
- User can always access default configuration

---

### Task 5.5.4: Accessibility Audit

**Files**: All component files

- [ ] Add ARIA attributes:
  - `aria-label` on buttons without visible text
  - `role="dialog"` on modal
  - `aria-live="polite"` on time display

- [ ] Keyboard navigation:
  - All buttons focusable via Tab
  - Modal traps focus
  - Escape closes modal

- [ ] Color contrast:
  - Verify WCAG AA compliance
  - Add text labels alongside color indicators

**Acceptance Criteria**:

- Pass automated accessibility testing (axe-core or similar)
- Usable with keyboard only

---

### Task 5.5.5: Write Additional Tests

**Files**: `crates/core/src/*.rs`, `bin/web/src/*.rs`

- [x] Core crate tests:
  - `is_work_hours` edge cases (midnight crossing, invalid timezone)
  - `calculate_time_difference` accuracy
  - Config serialization round-trip

- [x] Web crate tests:
  - URL encoding/decoding round-trip
  - LocalStorage mock tests (if possible in WASM)

**Acceptance Criteria**:

- `cargo test --workspace` passes
- >80% coverage on core crate

---

### Task 5.5.6: Documentation

**Files**: Various

- [ ] Update `README.md`:
  - Add Web version section
  - Include screenshot
  - Document URL sharing feature

- [ ] Add inline documentation:
  - Doc comments on all public functions
  - Module-level documentation

- [x] Update `Justfile` with help command:

  ```just
  help:
      @echo "Available commands:"
      @echo "  just dev-tui  - Run TUI application"
      @echo "  just dev-web  - Run Web application (dev)"
      @echo "  just build    - Build all targets"
      @echo "  just test     - Run all tests"
  ```

**Acceptance Criteria**:

- `cargo doc --workspace --open` generates clean documentation
- README reflects current project state

---

## Phase 6: Future Enhancements (Optional)

These tasks are out of scope for initial release but documented for future consideration.

### Task 6.1: Keyboard Shortcuts ✅

- [x] Implement keyboard event handler:
  - `←` / `→` / `h` / `l`: Adjust time ±15 minutes
  - `r`: Reset time
  - `Space`: Toggle play/pause
  - `Esc`: Close modal

### Task 6.2: PWA Support

- [ ] Add `manifest.json`
- [ ] Implement service worker for offline caching
- [ ] Add install prompt

### Task 6.3: Theme Toggle

- [ ] Implement dark/light theme switching
- [ ] Persist preference to LocalStorage
- [ ] Respect `prefers-color-scheme`

### Task 6.4: Profile Management

- [ ] Allow saving multiple named configurations
- [ ] Quick switch between profiles
- [ ] Import/Export as JSON file

### Task 6.5: Accessibility Improvements

- [ ] Add ARIA attributes to all interactive elements
- [ ] Ensure keyboard navigation works
- [ ] Verify WCAG AA color contrast compliance

---

## Summary: Current Progress

| Phase | Status | Tests |
|-------|--------|-------|
| Phase 1: Workspace Restructuring | ✅ COMPLETED | 18 tests |
| Phase 2: Web Project Initialization | ✅ COMPLETED | - |
| Phase 3: Core State & Storage | ✅ COMPLETED | 1 test |
| Phase 4: UI Component Development | ✅ COMPLETED | - |
| Phase 5: Integration & Testing | ✅ COMPLETED | - |
| Phase 6: Future Enhancements | 🔄 IN PROGRESS | - |

**Total Tests: 19 passing** (11 core + 6 TUI + 1 web + 1 doctest)

---

## Next Steps

1. ✅ Install `cargo-leptos` - Done
2. ✅ Run `bun install` at project root for Tailwind - Done
3. ✅ Run `just dev-web` to start development server - Done
4. ✅ Test all features in browser - Done
5. ✅ Keyboard shortcuts implemented - Done
6. Consider implementing remaining Phase 6 enhancements (PWA, themes, profiles)

---

## Checklist Before Deployment

- [x] `cargo check --workspace` passes
- [x] `cargo test --workspace` passes (19 tests)
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] `cargo fmt --all` runs without errors
- [ ] Application runs in browser without console errors
- [ ] All features work as expected
- [ ] README updated with web version instructions
