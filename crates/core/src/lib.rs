//! LongTime Core Library
//!
//! This crate contains the core business logic for the LongTime multi-timezone
//! time management tool. It provides:
//!
//! - Configuration data structures (`Config`, `TimezoneConfig`, `WorkHours`)
//! - Time calculation utilities (`is_work_hours`, `get_time_display_info`)
//!
//! This crate is designed to be shared between the TUI and Web interfaces,
//! and is compatible with WASM targets.

pub mod config;
pub mod time;

pub use config::{Config, TimezoneConfig, WorkHours};
pub use time::{
    TimeDisplayInfo, calculate_time_difference, format_time_diff, get_time_display_info,
    get_timezone_offset, is_work_hours,
};
