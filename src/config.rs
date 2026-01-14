//! Configuration handling for timezone data
//!
//! This module defines the configuration structure and handles
//! parsing of the TOML configuration file containing timezone information.

use chrono::NaiveTime;
use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};

/// The main configuration struct that holds all timezone information
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// List of timezone configurations
    pub timezones: Vec<TimezoneConfig>,
    /// Whether to use 12-hour format (default: false)
    #[serde(default)]
    pub use_12h_format: bool,
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = ConfigLoader::builder();

        // Determine the config source
        let config_source = if let Some(path) = config_path {
            File::with_name(path)
        } else {
            // Default path: ~/.config/longtime/config.toml
            let home = dirs::home_dir().ok_or("Could not find home directory")?;
            let default_path = home.join(".config").join("longtime").join("config.toml");
            // If the default config file doesn't exist, we might want to handle it gracefully
            // or let the config crate error out. Here we let it error out if not found,
            // or we could add a default empty config or built-in defaults.
            // For now, we assume the user wants it to fail or work as before if file is missing.
            // Actually, File::from(path).required(true) is default.
            File::from(default_path)
        };

        let config = builder.add_source(config_source).build()?;

        let app_config: Config = config.try_deserialize()?;
        Ok(app_config)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_hours_parsing() {
        let wh = WorkHours {
            start: "09:00".to_string(),
            end: "17:00".to_string(),
        };

        assert_eq!(
            wh.start_time(),
            Some(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        );
        assert_eq!(
            wh.end_time(),
            Some(NaiveTime::from_hms_opt(17, 0, 0).unwrap())
        );
    }

    #[test]
    fn test_invalid_work_hours() {
        let wh = WorkHours {
            start: "25:00".to_string(),
            end: "invalid".to_string(),
        };

        assert_eq!(wh.start_time(), None);
        assert_eq!(wh.end_time(), None);
    }
}
