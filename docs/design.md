# LongTime Web Version - Design Document

## 1. Overview

### 1.1 Project Goal

Extend the existing LongTime TUI application into a web application, enabling users to manage and view multi-timezone information directly in their browser.

### 1.2 Core Principles

1. **Logic Reuse**: Extract core business logic (time calculations, timezone handling, work hours detection) into a shared `core` crate usable by both TUI and Web.
2. **CSR-First Architecture**: Use Leptos in Client-Side Rendering (CSR) mode with WASM. All time calculations run in the browser with zero backend dependencies.
3. **Configuration-Driven**: Reuse `TimezoneConfig` structure. Support URL query parameters for sharing and LocalStorage for persistence.
4. **Modern UI**: TailwindCSS v4 for responsive, accessible design that mirrors the TUI's clean aesthetic with enhanced interactivity.

### 1.3 Technology Stack

| Layer | Technology |
|-------|------------|
| Core Logic | Rust (shared crate) |
| Web Framework | Leptos 0.8+ (CSR mode) |
| Build Tool | `cargo-leptos` (NOT trunk) |
| Styling | TailwindCSS v4 |
| JS Runtime | Bun (for CSS build) |
| State Persistence | LocalStorage (`gloo-storage`) |
| URL Handling | `leptos_router`, Base64 encoding |

---

## 2. Architecture

### 2.1 Workspace Structure

```text
longtime/
├── Cargo.toml              # Workspace manifest
├── Justfile                # Task automation
├── package.json            # Bun workspace root
├── bun.lockb               # Single lockfile
├── timezones.toml          # Example configuration
├── docs/
│   ├── design.md
│   └── tasks.md
├── crates/
│   └── core/               # Shared business logic
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── config.rs   # Config, TimezoneConfig, WorkHours
│           └── time.rs     # is_work_hours, time diff calculations
├── bin/
│   ├── tui/                # Terminal interface (refactored)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── app.rs
│   │       └── ui.rs
│   └── web/                # Leptos web application
│       ├── Cargo.toml      # Contains [package.metadata.leptos]
│       ├── package.json    # TailwindCSS dependencies
│       ├── index.html
│       ├── style/
│       │   └── main.css    # Tailwind v4 directives
│       └── src/
│           ├── main.rs     # CSR entry point
│           ├── app.rs      # Root component
│           ├── state.rs    # AppState, signals
│           ├── storage.rs  # LocalStorage helpers
│           └── components/
│               ├── mod.rs
│               ├── header.rs
│               ├── timezone_card.rs
│               ├── timezone_list.rs
│               ├── time_controls.rs
│               └── config_modal.rs
└── assets/                 # Static assets (favicon, images)
```

### 2.2 Crate Dependency Graph

```text
┌─────────────────────────────────────────────┐
│                 Workspace                    │
├─────────────────────────────────────────────┤
│  crates/core  ←──────┬──────────────────────┤
│      │               │                      │
│      ▼               ▼                      │
│  bin/tui         bin/web                    │
│  (ratatui)       (leptos + wasm)            │
└─────────────────────────────────────────────┘
```

---

## 3. Core Library Design (`crates/core`)

### 3.1 Public API

```rust
// crates/core/src/lib.rs
pub mod config;
pub mod time;

pub use config::{Config, TimezoneConfig, WorkHours};
pub use time::{is_work_hours, calculate_time_difference, TimeDisplayInfo};
```

### 3.2 Data Structures

```rust
// crates/core/src/config.rs
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub timezones: Vec<TimezoneConfig>,
    #[serde(default)]
    pub use_12h_format: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimezoneConfig {
    pub name: String,
    pub timezone: String,  // IANA identifier
    pub work_hours: WorkHours,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkHours {
    pub start: String,  // "HH:MM"
    pub end: String,    // "HH:MM"
}

impl WorkHours {
    pub fn start_time(&self) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(&self.start, "%H:%M").ok()
    }

    pub fn end_time(&self) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(&self.end, "%H:%M").ok()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timezones: vec![
                TimezoneConfig {
                    name: "Shanghai".to_string(),
                    timezone: "Asia/Shanghai".to_string(),
                    work_hours: WorkHours {
                        start: "09:00".to_string(),
                        end: "18:00".to_string(),
                    },
                },
                TimezoneConfig {
                    name: "London".to_string(),
                    timezone: "Europe/London".to_string(),
                    work_hours: WorkHours {
                        start: "09:00".to_string(),
                        end: "17:30".to_string(),
                    },
                },
                TimezoneConfig {
                    name: "New York".to_string(),
                    timezone: "America/New_York".to_string(),
                    work_hours: WorkHours {
                        start: "09:00".to_string(),
                        end: "17:00".to_string(),
                    },
                },
            ],
            use_12h_format: false,
        }
    }
}
```

### 3.3 Time Calculation Functions

```rust
// crates/core/src/time.rs
use chrono::{DateTime, Utc, Offset};
use chrono_tz::Tz;
use std::str::FromStr;

use crate::config::TimezoneConfig;

#[derive(Debug, Clone)]
pub struct TimeDisplayInfo {
    pub time: String,
    pub date: String,
    pub diff_hours: f64,
    pub is_working: bool,
}

/// Check if current time falls within work hours for a timezone
pub fn is_work_hours(now: DateTime<Utc>, config: &TimezoneConfig) -> bool {
    let Ok(tz) = Tz::from_str(&config.timezone) else {
        return false;
    };

    let local_time = now.with_timezone(&tz);
    let naive_time = local_time.time();

    match (config.work_hours.start_time(), config.work_hours.end_time()) {
        (Some(start), Some(end)) => naive_time >= start && naive_time <= end,
        _ => false,
    }
}

/// Calculate time difference in hours between two timezone offsets
pub fn calculate_time_difference(
    now: DateTime<Utc>,
    tz_str: &str,
    reference_offset_seconds: i32,
) -> Option<f64> {
    let tz = Tz::from_str(tz_str).ok()?;
    let local_time = now.with_timezone(&tz);
    let current_offset = local_time.offset().fix().local_minus_utc();
    let diff_seconds = current_offset - reference_offset_seconds;
    Some(diff_seconds as f64 / 3600.0)
}

/// Get comprehensive display info for a timezone
pub fn get_time_display_info(
    now: DateTime<Utc>,
    config: &TimezoneConfig,
    reference_offset_seconds: i32,
    use_12h_format: bool,
) -> Option<TimeDisplayInfo> {
    let tz = Tz::from_str(&config.timezone).ok()?;
    let local_time = now.with_timezone(&tz);

    let time_format = if use_12h_format { "%I:%M %p" } else { "%H:%M" };
    let time = local_time.format(time_format).to_string();
    let date = local_time.format("%Y-%m-%d").to_string();

    let current_offset = local_time.offset().fix().local_minus_utc();
    let diff_hours = (current_offset - reference_offset_seconds) as f64 / 3600.0;

    let is_working = is_work_hours(now, config);

    Some(TimeDisplayInfo {
        time,
        date,
        diff_hours,
        is_working,
    })
}
```

---

## 4. Web Frontend Design (`bin/web`)

### 4.1 State Management

Using Leptos Signals for reactive state:

```rust
// bin/web/src/state.rs
use chrono::{DateTime, Duration, Utc};
use leptos::prelude::*;
use longtime_core::{Config, TimezoneConfig};

#[derive(Clone)]
pub struct AppState {
    /// Current configuration (timezones list)
    pub config: RwSignal<Config>,
    /// Current time offset for time-travel simulation
    pub time_offset: RwSignal<Duration>,
    /// Whether time is auto-updating
    pub is_running: RwSignal<bool>,
    /// Show configuration modal
    pub show_config_modal: RwSignal<bool>,
    /// Currently selected timezone index (for reference)
    pub selected_index: RwSignal<usize>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: RwSignal::new(config),
            time_offset: RwSignal::new(Duration::zero()),
            is_running: RwSignal::new(true),
            show_config_modal: RwSignal::new(false),
            selected_index: RwSignal::new(0),
        }
    }

    pub fn current_time(&self) -> DateTime<Utc> {
        Utc::now() + self.time_offset.get()
    }
}
```

### 4.2 Configuration Loading Priority

```text
┌─────────────────────────────────────────────────────────────┐
│                  Configuration Load Order                    │
├─────────────────────────────────────────────────────────────┤
│  1. URL Query Parameter (?config=<Base64_JSON>)             │
│     └── If present: decode and use (sharing use case)       │
│                                                              │
│  2. LocalStorage (key: "longtime_config")                   │
│     └── If present: deserialize and use                     │
│                                                              │
│  3. Default Configuration                                    │
│     └── Fallback: Shanghai, London, New York                │
└─────────────────────────────────────────────────────────────┘
```

### 4.3 URL Sharing Mechanism

```rust
// bin/web/src/storage.rs
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use longtime_core::Config;

pub fn encode_config_to_url(config: &Config) -> String {
    let json = serde_json::to_string(config).unwrap_or_default();
    URL_SAFE_NO_PAD.encode(json.as_bytes())
}

pub fn decode_config_from_url(encoded: &str) -> Option<Config> {
    let bytes = URL_SAFE_NO_PAD.decode(encoded).ok()?;
    let json = String::from_utf8(bytes).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn generate_share_url(config: &Config) -> String {
    let encoded = encode_config_to_url(config);
    let base_url = web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or("")
        .to_string();
    format!("{base_url}?config={encoded}")
}
```

### 4.4 LocalStorage Persistence

```rust
// bin/web/src/storage.rs
use gloo_storage::{LocalStorage, Storage};
use longtime_core::Config;

const STORAGE_KEY: &str = "longtime_config";

pub fn save_config(config: &Config) {
    let _ = LocalStorage::set(STORAGE_KEY, config);
}

pub fn load_config() -> Option<Config> {
    LocalStorage::get(STORAGE_KEY).ok()
}

pub fn clear_config() {
    LocalStorage::delete(STORAGE_KEY);
}
```

---

## 5. UI Components

### 5.1 Component Hierarchy

```text
<App>
├── <Header>
│   ├── Logo
│   ├── ShareButton
│   └── SettingsButton
├── <TimezoneList>
│   └── <TimezoneCard> (×N)
│       ├── City Name
│       ├── Current Time (large)
│       ├── Date
│       ├── Time Difference Badge
│       ├── Work Status Indicator
│       └── Action Buttons (Edit/Delete)
├── <TimeControls>
│   ├── Reset Button
│   ├── Play/Pause Toggle
│   └── Time Offset Slider/Stepper
└── <ConfigModal> (conditional)
    ├── Timezone Selector (IANA dropdown)
    ├── Work Hours Input
    └── Save/Cancel Buttons
```

### 5.2 Responsive Layout Strategy

| Viewport | Layout |
|----------|--------|
| Mobile (<640px) | Single column, stacked cards |
| Tablet (640-1024px) | 2-column grid |
| Desktop (>1024px) | 3-4 column grid |

### 5.3 TailwindCSS v4 Configuration

```css
/* bin/web/style/main.css */
@import "tailwindcss";

@theme {
    --color-working: oklch(0.75 0.18 145);   /* Green */
    --color-off: oklch(0.65 0.2 25);          /* Red */
    --color-primary: oklch(0.8 0.15 85);      /* Yellow */
    --color-surface: oklch(0.2 0.02 260);     /* Dark gray */
    --color-surface-alt: oklch(0.25 0.02 260);

    --font-mono: "JetBrains Mono", ui-monospace, monospace;

    --radius-card: 0.75rem;
    --shadow-card: 0 4px 6px -1px rgb(0 0 0 / 0.3);
}
```

---

## 6. Build Configuration

### 6.1 Cargo-Leptos Metadata

```toml
# bin/web/Cargo.toml
[package]
name = "longtime-web"
version.workspace = true
edition.workspace = true

[dependencies]
longtime-core = { path = "../../crates/core" }
leptos.workspace = true
leptos_router.workspace = true
leptos_meta.workspace = true
chrono.workspace = true
chrono-tz.workspace = true
serde.workspace = true
serde_json.workspace = true
base64.workspace = true
gloo-storage = "0.3"
gloo-timers = "0.3"
web-sys = { version = "0.3", features = ["Clipboard", "Navigator", "Window", "Location"] }
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"

[package.metadata.leptos]
output-name = "longtime"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "style/main.css"
assets-dir = "../../assets"
site-addr = "127.0.0.1:3000"
reload-port = 3001
bin-features = []
lib-features = []
```

### 6.2 Justfile Commands

```just
# Development
dev-tui:
    cargo run -p longtime-tui -- -c timezones.toml

dev-web:
    cd bin/web && cargo leptos watch

build-web:
    cd bin/web && cargo leptos build --release

# CSS (if manual build needed)
build-css:
    cd bin/web && bun run build:css
```

---

## 7. Data Flow Diagrams

### 7.1 Time Update Flow

```text
┌──────────────────────────────────────────────────────────────┐
│                    Time Update Cycle                          │
├──────────────────────────────────────────────────────────────┤
│                                                               │
│  setInterval(1s)                                             │
│       │                                                       │
│       ▼                                                       │
│  if is_running.get() {                                       │
│       │                                                       │
│       ▼                                                       │
│    Trigger re-render via signal dependency                   │
│       │                                                       │
│       ▼                                                       │
│    current_time = Utc::now() + time_offset.get()            │
│       │                                                       │
│       ▼                                                       │
│    For each TimezoneCard:                                    │
│      └── get_time_display_info(current_time, config, ...)   │
│  }                                                            │
│                                                               │
└──────────────────────────────────────────────────────────────┘
```

### 7.2 Configuration Change Flow

```text
User Action (Add/Edit/Delete Timezone)
        │
        ▼
    config.update(|c| { ... })
        │
        ├────────────────────┐
        ▼                    ▼
    UI Re-renders     save_config(&config)
                            │
                            ▼
                      LocalStorage
```

---

## 8. Error Handling

### 8.1 Invalid Timezone

- Display fallback card with error indicator
- Log warning to console
- Do not crash the application

### 8.2 LocalStorage Errors

- Silently fail on write errors (storage full, private browsing)
- Return `None` on read errors, fall back to defaults

### 8.3 URL Decode Errors

- Log warning
- Ignore invalid `?config=` parameter
- Fall back to LocalStorage or defaults

---

## 9. Accessibility Considerations

1. **Keyboard Navigation**: All interactive elements focusable via Tab
2. **ARIA Labels**: Cards announce timezone name, time, and work status
3. **Color Contrast**: Working/Off indicators use high-contrast colors + icon/text labels
4. **Reduced Motion**: Respect `prefers-reduced-motion` for animations

---

## 10. Future Enhancements (Out of Scope)

- [ ] PWA support with offline caching
- [ ] Server-side rendering (SSR) for SEO
- [ ] Multiple saved configurations (profiles)
- [ ] Calendar integration for meeting scheduling
- [ ] Dark/Light theme toggle
