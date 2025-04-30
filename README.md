# LongTime

Multi-timezone Time Management Tool

A command-line terminal user interface (TUI) tool developed with Rust and ratatui library for managing and displaying time information across multiple time zones.

Chinese version: [README_zh](/README_zh.md)

## Features

- **Multi-timezone Support**: Simultaneously display current time in multiple time zones
- **Work Hours Display**: Show configured work time ranges for each time zone
- **Work Status Indicator**: Visually indicate whether each time zone is within working hours
- **Time Adjustment**: Manually adjust time in any time zone, with other time zones updating synchronously
- **Configuration File Support**: Easily add and manage time zones through TOML configuration file
- **Interactive Interface**: User-friendly interface with keyboard navigation and operations

## Quick Start

### Installation

1. Compile the project:

```bash
cargo build --release
```

2. Run the program:

```bash
cargo run --release
```

Or directly run the compiled binary:

```bash
./target/release/time
```

### Configuration

The program uses a `timezones.toml` configuration file to define time zone information. This file should be placed in the current directory where the program is executed.

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

### Time Adjustment Function

When you adjust time using the left and right arrow keys, time in all time zones updates synchronously. This feature allows you to:

1. View the status of different time zones at specific time points
2. Plan meetings or activities across time zones
3. Estimate work time overlap between different time zones

## Customization and Extension

### Adding New Time Zones

To add a new time zone, simply add a new `[[timezones]]` entry in the `timezones.toml` file:

```toml
[[timezones]]
name = "Singapore"
timezone = "Asia/Singapore"
work_hours = { start = "09:00", end = "18:00" }
```

### Modifying Work Hours

To modify the work hours of an existing time zone, update the `work_hours` value for the corresponding time zone entry:

```toml
[[timezones]]
name = "London"
timezone = "Europe/London"
work_hours = { start = "08:30", end = "16:30" }  # Updated work hours
```

### Supported Time Zone Formats

This tool uses the `chrono-tz` library and supports all time zone identifiers in the IANA time zone database. Common time zone identifiers include:

- `Asia/Shanghai` (China Beijing Time)
- `America/New_York` (US Eastern Time)
- `Europe/London` (UK London Time)
- `Asia/Tokyo` (Japan Tokyo Time)
- `Australia/Sydney` (Australia Sydney Time)
- `Europe/Paris` (France Paris Time)

A complete list of time zones can be found [here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).

## Technical Architecture

This tool uses the following Rust libraries:

- **ratatui**: For creating terminal user interfaces
- **crossterm**: For handling terminal events and control
- **chrono** and **chrono-tz**: For time zone and time calculations
- **serde** and **toml**: For configuration file parsing

## Frequently Asked Questions

### Q: The program won't start, showing configuration file errors?
A: Ensure the `timezones.toml` file is in the current directory where the program is executed and follows the correct TOML syntax format.

### Q: How to restore to the real current time?
A: Exit the program and restart to reset the time offset.

### Q: Can I change the time adjustment step size?
A: Currently, the time adjustment step is fixed at 30 minutes. If you need to modify it, you can edit the `adjust_time_forward` and `adjust_time_backward` functions in the source code.
