use std::{
    error::Error,
    time::{Duration as StdDuration, Instant},
};

use chrono::{DateTime, Duration, NaiveTime, Utc};
use chrono_tz::Tz;

use crate::config::{Config, WorkHours};

// App state
pub struct App {
    pub config: Config,
    pub selected_index: usize,
    pub offset_minutes: i64, // Offset from real time in minutes
    pub last_tick: Instant,
    pub tick_rate: StdDuration,
}

impl App {
    pub fn new(config: Config) -> App {
        App {
            config,
            selected_index: 0,
            offset_minutes: 0,
            last_tick: Instant::now(),
            tick_rate: StdDuration::from_millis(250),
        }
    }

    // Move selection up
    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    // Move selection down
    pub fn next(&mut self) {
        if self.selected_index < self.config.timezones.len() - 1 {
            self.selected_index += 1;
        }
    }

    // Adjust time forward
    pub fn adjust_time_forward(&mut self) {
        self.offset_minutes += 30; // Adjust by 30-minute increments (was 15)
    }

    // Adjust time backward
    pub fn adjust_time_backward(&mut self) {
        self.offset_minutes -= 30; // Adjust by 30-minute increments (was 15)
    }

    // Get the current time for a specific timezone with the offset applied
    pub fn get_current_time(&self, timezone_str: &str) -> Result<DateTime<Tz>, Box<dyn Error>> {
        let tz: Tz = timezone_str.parse()?;
        let now = Utc::now() + Duration::minutes(self.offset_minutes);
        Ok(now.with_timezone(&tz))
    }

    // Check if a given time is within work hours
    pub fn is_work_hours(&self, time: &DateTime<Tz>, work_hours: &WorkHours) -> bool {
        // Parse work hours
        let start_parts: Vec<&str> = work_hours.start.split(':').collect();
        let end_parts: Vec<&str> = work_hours.end.split(':').collect();

        let start_hour: u32 = start_parts[0].parse().unwrap_or(9);
        let start_min: u32 = start_parts[1].parse().unwrap_or(0);
        let end_hour: u32 = end_parts[0].parse().unwrap_or(17);
        let end_min: u32 = end_parts[1].parse().unwrap_or(0);

        let start_time = NaiveTime::from_hms_opt(start_hour, start_min, 0)
            .expect("Invalid start time hours or minutes");
        let end_time = NaiveTime::from_hms_opt(end_hour, end_min, 0)
            .expect("Invalid end time hours or minutes");

        let current_time = time.time();

        current_time >= start_time && current_time <= end_time
    }
}
