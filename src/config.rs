//! Configuration handling for timezone data
//!
//! This module defines the configuration structure and handles
//! parsing of the TOML configuration file containing timezone information.

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

/// The main configuration struct that holds all timezone information
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// List of timezone configurations
    pub timezones: Vec<TimezoneConfig>,
}

/// Configuration for a single timezone
#[derive(Debug, Deserialize, Serialize)]
pub struct TimezoneConfig {
    /// Display name for the timezone
    pub name: String,
    /// IANA timezone identifier (e.g., "America/New_York")
    pub timezone: String,
    /// Work hours configuration
    pub work_hours: WorkHours,
}

/// Work hours configuration for a timezone
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkHours {
    /// Start time of work hours (format: "HH:MM")
    pub start: String,
    /// End time of work hours (format: "HH:MM")
    pub end: String,
}

impl WorkHours {
    /// Parses the start time string into a NaiveTime object
    ///
    /// # Returns
    ///
    /// * `Option<NaiveTime>` - The parsed time or None if parsing fails
    #[allow(dead_code)]
    pub fn start_time(&self) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(&self.start, "%H:%M").ok()
    }

    /// Parses the end time string into a NaiveTime object
    ///
    /// # Returns
    ///
    /// * `Option<NaiveTime>` - The parsed time or None if parsing fails
    #[allow(dead_code)]
    pub fn end_time(&self) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(&self.end, "%H:%M").ok()
    }
}
