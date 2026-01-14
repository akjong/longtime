//! Configuration data structures for timezone management
//!
//! This module defines the configuration structures used to represent
//! timezone information and work hours settings.

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

/// The main configuration struct that holds all timezone information
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    /// List of timezone configurations
    pub timezones: Vec<TimezoneConfig>,
    /// Whether to use 12-hour format (default: false)
    #[serde(default)]
    pub use_12h_format: bool,
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

/// Configuration for a single timezone
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TimezoneConfig {
    /// Display name for the timezone
    pub name: String,
    /// IANA timezone identifier (e.g., "America/New_York")
    pub timezone: String,
    /// Work hours configuration
    pub work_hours: WorkHours,
}

/// Work hours configuration for a timezone
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
    pub fn start_time(&self) -> Option<NaiveTime> {
        NaiveTime::parse_from_str(&self.start, "%H:%M").ok()
    }

    /// Parses the end time string into a NaiveTime object
    ///
    /// # Returns
    ///
    /// * `Option<NaiveTime>` - The parsed time or None if parsing fails
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

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.timezones.len(), 3);
        assert!(!config.use_12h_format);
        assert_eq!(config.timezones[0].name, "Shanghai");
        assert_eq!(config.timezones[1].name, "London");
        assert_eq!(config.timezones[2].name, "New York");
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
