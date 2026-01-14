//! Time calculation utilities
//!
//! This module provides functions for calculating time differences,
//! determining work hours status, and generating display information
//! for timezones.

use std::str::FromStr;

use chrono::{DateTime, Offset, Utc};
use chrono_tz::Tz;

use crate::config::TimezoneConfig;

/// Information for displaying a timezone's current time
#[derive(Debug, Clone, PartialEq)]
pub struct TimeDisplayInfo {
    /// Formatted time string
    pub time: String,
    /// Formatted date string
    pub date: String,
    /// Time difference in hours from reference timezone
    pub diff_hours: f64,
    /// Whether currently within work hours
    pub is_working: bool,
}

/// Check if current time falls within work hours for a timezone
///
/// # Arguments
///
/// * `now` - Current UTC time to check
/// * `config` - Timezone configuration with work hours
///
/// # Returns
///
/// * `bool` - True if time is within work hours, false otherwise
///
/// # Example
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use longtime_core::{TimezoneConfig, WorkHours, is_work_hours};
///
/// let config = TimezoneConfig {
///     name: "Test".to_string(),
///     timezone: "UTC".to_string(),
///     work_hours: WorkHours {
///         start: "09:00".to_string(),
///         end: "17:00".to_string(),
///     },
/// };
///
/// let working_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
/// assert!(is_work_hours(working_time, &config));
/// ```
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

/// Calculate time difference in hours between a timezone and a reference offset
///
/// # Arguments
///
/// * `now` - Current UTC time
/// * `tz_str` - IANA timezone identifier
/// * `reference_offset_seconds` - Reference timezone offset in seconds
///
/// # Returns
///
/// * `Option<f64>` - Time difference in hours, or None if timezone is invalid
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

/// Get the UTC offset in seconds for a timezone
///
/// # Arguments
///
/// * `now` - Current UTC time
/// * `tz_str` - IANA timezone identifier
///
/// # Returns
///
/// * `Option<i32>` - Offset in seconds, or None if timezone is invalid
pub fn get_timezone_offset(now: DateTime<Utc>, tz_str: &str) -> Option<i32> {
    let tz = Tz::from_str(tz_str).ok()?;
    let local_time = now.with_timezone(&tz);
    Some(local_time.offset().fix().local_minus_utc())
}

/// Get comprehensive display info for a timezone
///
/// # Arguments
///
/// * `now` - Current UTC time
/// * `config` - Timezone configuration
/// * `reference_offset_seconds` - Reference timezone offset for difference calculation
/// * `use_12h_format` - Whether to use 12-hour time format
///
/// # Returns
///
/// * `Option<TimeDisplayInfo>` - Display information, or None if timezone is invalid
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

/// Format time difference as a display string
///
/// # Arguments
///
/// * `diff_hours` - Time difference in hours
///
/// # Returns
///
/// * `String` - Formatted string like "+8", "-5", or "="
pub fn format_time_diff(diff_hours: f64) -> String {
    if diff_hours == 0.0 {
        "=".to_string()
    } else if diff_hours > 0.0 {
        format!("+{diff_hours}")
    } else {
        format!("{diff_hours}")
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::config::WorkHours;

    fn create_test_config(timezone: &str) -> TimezoneConfig {
        TimezoneConfig {
            name: "Test".to_string(),
            timezone: timezone.to_string(),
            work_hours: WorkHours {
                start: "09:00".to_string(),
                end: "17:00".to_string(),
            },
        }
    }

    #[test]
    fn test_is_work_hours_within() {
        let config = create_test_config("UTC");
        // 12:00 UTC is within 09:00-17:00
        let working_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        assert!(is_work_hours(working_time, &config));
    }

    #[test]
    fn test_is_work_hours_outside() {
        let config = create_test_config("UTC");
        // 20:00 UTC is outside 09:00-17:00
        let off_time = Utc.with_ymd_and_hms(2023, 1, 1, 20, 0, 0).unwrap();
        assert!(!is_work_hours(off_time, &config));
    }

    #[test]
    fn test_is_work_hours_invalid_timezone() {
        let config = create_test_config("Invalid/Timezone");
        let now = Utc::now();
        assert!(!is_work_hours(now, &config));
    }

    #[test]
    fn test_calculate_time_difference() {
        let now = Utc.with_ymd_and_hms(2023, 6, 1, 12, 0, 0).unwrap();
        // Shanghai is UTC+8
        let diff = calculate_time_difference(now, "Asia/Shanghai", 0);
        assert_eq!(diff, Some(8.0));
    }

    #[test]
    fn test_get_timezone_offset() {
        let now = Utc.with_ymd_and_hms(2023, 6, 1, 12, 0, 0).unwrap();
        let offset = get_timezone_offset(now, "Asia/Shanghai");
        assert_eq!(offset, Some(8 * 3600)); // 8 hours in seconds
    }

    #[test]
    fn test_get_time_display_info() {
        let now = Utc.with_ymd_and_hms(2023, 6, 1, 4, 0, 0).unwrap(); // 4:00 UTC = 12:00 Shanghai
        let config = create_test_config("Asia/Shanghai");
        let info = get_time_display_info(now, &config, 0, false);

        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.time, "12:00");
        assert_eq!(info.date, "2023-06-01");
        assert_eq!(info.diff_hours, 8.0);
        assert!(info.is_working); // 12:00 is within 09:00-17:00
    }

    #[test]
    fn test_format_time_diff() {
        assert_eq!(format_time_diff(0.0), "=");
        assert_eq!(format_time_diff(8.0), "+8");
        assert_eq!(format_time_diff(-5.0), "-5");
        assert_eq!(format_time_diff(5.5), "+5.5");
    }
}
