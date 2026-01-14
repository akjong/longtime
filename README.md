# LongTime

Multi-timezone Time Management Tool

A modern multi-timezone time management application available as both a **Terminal UI (TUI)** and a **Web Application**, built with Rust.

Chinese version: [README_zh](/README_zh.md)

## Features

- **Multi-timezone Support**: Simultaneously display current time in multiple time zones
- **Work Hours Display**: Show configured work time ranges for each time zone
- **Work Status Indicator**: Visually indicate whether each time zone is within working hours
- **Time Adjustment**: Manually adjust time in any time zone, with other time zones updating synchronously
- **Two Interfaces**: Choose between Terminal UI or Web Application
- **Configuration Sharing**: Share timezone configurations via URL (Web version)
- **Keyboard Shortcuts**: Quick navigation and time adjustment

## Quick Start

### Prerequisites

- Rust 1.80+ with `wasm32-unknown-unknown` target
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) (for Web version)
- [Bun](https://bun.sh/) (for Tailwind CSS)

### Installation

```bash
# Clone the repository
git clone https://github.com/akjong/longtime.git
cd longtime

# Install dependencies for Web version
bun install
cargo install cargo-leptos
rustup target add wasm32-unknown-unknown
```

### Running the TUI Version

```bash
# Using Just
just dev-tui

# Or directly with cargo
cargo run -p longtime-tui -- -c timezones.toml
```

### Running the Web Version

```bash
# Using Just (recommended)
just dev-web

# Or directly with cargo-leptos
cd bin/web && cargo leptos watch
```

Then open **<http://127.0.0.1:3000**> in your browser.

### Building for Production

```bash
# Build all workspace crates
just build

# Build Web version for production
just build-web
```

### Available Commands

```bash
just help  # Show all available commands
```

---

## TUI Version

### Configuration

The TUI version uses a `timezones.toml` configuration file:

Example configuration file:

```toml
# Multi-timezone configuration file

[[timezones]]
name = "Beijing"
timezone = "Asia/Shanghai"
work_hours = { start = "09:00", end = "18:00" }

[[timezones]]
name = "New_York"
timezone = "America/New_York"
work_hours = { start = "09:00", end = "17:00" }

[[timezones]]
name = "London"
timezone = "Europe/London"
work_hours = { start = "09:00", end = "17:30" }

[[timezones]]
name = "Tokyo"
timezone = "Asia/Tokyo"
work_hours = { start = "09:30", end = "18:30" }

[[timezones]]
name = "Sydney"
timezone = "Australia/Sydney"
work_hours = { start = "09:00", end = "17:00" }
```

Configuration item description:

- `name`: Time zone display name
- `timezone`: Time zone identifier (conforming to IANA time zone database format)
- `work_hours`: Work time range, including `start` (start time) and `end` (end time)

## Usage

### Interface Navigation

After starting the program, you'll see a list containing information for all configured time zones. Each time zone entry displays:

- Time zone name
- Current time (format: YYYY-MM-DD HH:MM:SS)
- Work hours range
- Current work status (work hours/non-work hours)

### Keyboard Shortcuts

| Key | Function |
|------|------|
| `↑` (Up Arrow) | Select previous time zone |
| `↓` (Down Arrow) | Select next time zone |
| `←` (Left Arrow) | Adjust time backward by 30 minutes |
| `→` (Right Arrow) | Adjust time forward by 30 minutes |
| `q` | Exit program |

---

## Web Version

The Web version provides a modern, responsive interface built with **Leptos** and **TailwindCSS v4**.

### Features

- **Real-time Updates**: Time updates every second automatically
- **12/24 Hour Toggle**: Switch between time formats
- **Add/Edit/Delete Timezones**: Manage timezones directly in the browser
- **Time Travel**: Adjust displayed time with +/- 15min or +/- 1hr buttons
- **Play/Pause**: Freeze time display for comparison
- **URL Sharing**: Generate shareable links with your configuration
- **LocalStorage**: Configuration persists across browser sessions

### Keyboard Shortcuts (Web)

| Key | Function |
|------|------|
| `←` or `h` | Adjust time backward by 15 minutes |
| `→` or `l` | Adjust time forward by 15 minutes |
| `r` | Reset time to current |
| `Space` | Toggle play/pause |
| `Escape` | Close modal dialog |

### URL Sharing

Click the **Share** button to copy a URL with your current configuration. Send it to teammates to share your timezone setup instantly.

---

## Configuration Format

### TUI Configuration (TOML)

The TUI version reads from `timezones.toml`. Add new timezones like this:

```toml
[[timezones]]
name = "Singapore"
timezone = "Asia/Singapore"
work_hours = { start = "09:00", end = "18:00" }
```

### Web Configuration

The Web version stores configuration in **LocalStorage** and supports URL sharing. Configuration is managed through the UI - no file editing required.

### Supported Time Zones

This tool uses the `chrono-tz` library and supports all IANA time zone identifiers:

- `Asia/Shanghai` (China Beijing Time)
- `America/New_York` (US Eastern Time)
- `Europe/London` (UK London Time)
- `Asia/Tokyo` (Japan Tokyo Time)
- `Australia/Sydney` (Australia Sydney Time)
- `Europe/Paris` (France Paris Time)

Full list: [IANA Time Zone Database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)

---

## Technical Architecture

### Workspace Structure

```text
longtime/
├── crates/core/       # Shared business logic (config, time calculations)
├── bin/tui/           # Terminal UI application (ratatui)
├── bin/web/           # Web application (Leptos + Tailwind)
└── timezones.toml     # TUI configuration file
```

### Core Libraries

| Library | Purpose |
|---------|---------|
| **leptos** | Reactive web framework (CSR mode) |
| **ratatui** | Terminal UI framework |
| **chrono-tz** | Timezone calculations |
| **gloo-storage** | LocalStorage for Web |
| **tailwindcss v4** | CSS styling for Web |

---

## FAQ

### Q: The TUI won't start, showing configuration file errors?

A: Ensure `timezones.toml` is in the current directory and follows valid TOML syntax.

### Q: How to restore to current time?

A: **TUI**: Restart the program. **Web**: Click the Reset button or press `r`.

### Q: How do I share my timezone setup?

A: **Web version only**: Click the Share button to copy a URL with your configuration encoded.

---

## License

MIT
