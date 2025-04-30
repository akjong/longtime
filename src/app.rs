//! Application state and business logic
//!
//! This module contains the core application state and business logic
//! for the longtime application. It manages timezone data, time offset,
//! and application state.

use std::rc::Rc;

use chrono::{DateTime, Duration, Utc};

use crate::config::Config;

/// The main application state
///
/// Contains all runtime state including timezone data,
/// current selection, and time offset for simulation.
#[derive(Debug)]
pub struct App {
    /// Configuration loaded from the TOML file
    config: Rc<Config>,
    /// Currently selected timezone index
    pub selected: usize,
    /// Time offset for simulating different times
    pub time_offset: Duration,
}

impl App {
    /// Creates a new application with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration containing timezone information
    pub fn new(config: Config) -> Self {
        App {
            config: Rc::new(config),
            selected: 0,
            time_offset: Duration::zero(),
        }
    }

    /// Returns the current configuration
    #[allow(dead_code)]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns the number of configured timezones
    pub fn timezone_count(&self) -> usize {
        self.config.timezones.len()
    }

    /// Gets the current time with the applied offset
    #[allow(dead_code)]
    pub fn current_time(&self) -> DateTime<Utc> {
        Utc::now() + self.time_offset
    }

    /// Adjusts the time forward by the specified minutes
    ///
    /// # Arguments
    ///
    /// * `minutes` - Number of minutes to move forward
    pub fn adjust_time_forward(&mut self, minutes: i64) {
        self.time_offset += Duration::minutes(minutes);
    }

    /// Adjusts the time backward by the specified minutes
    ///
    /// # Arguments
    ///
    /// * `minutes` - Number of minutes to move backward
    pub fn adjust_time_backward(&mut self, minutes: i64) {
        self.time_offset -= Duration::minutes(minutes);
    }

    /// Moves the selection to the next timezone
    pub fn next(&mut self) {
        let len = self.timezone_count();
        if len > 0 {
            self.selected = (self.selected + 1) % len;
        }
    }

    /// Moves the selection to the previous timezone
    pub fn previous(&mut self) {
        let len = self.timezone_count();
        if len > 0 {
            self.selected = (self.selected + len - 1) % len;
        }
    }
}
