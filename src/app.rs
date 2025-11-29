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
    /// Whether to show the help modal
    pub show_help: bool,
    /// Search query for filtering timezones
    pub search_query: String,
    /// Whether the user is currently typing a search query
    pub is_searching: bool,
    /// Whether to use 12-hour format
    pub use_12h_format: bool,
}

impl App {
    /// Creates a new application with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration containing timezone information
    pub fn new(config: Config) -> Self {
        let use_12h_format = config.use_12h_format;
        App {
            config: Rc::new(config),
            selected: 0,
            time_offset: Duration::zero(),
            show_help: false,
            search_query: String::new(),
            is_searching: false,
            use_12h_format,
        }
    }

    /// Returns the current configuration
    #[allow(dead_code)]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns the filtered timezones based on search query
    pub fn get_filtered_timezones(&self) -> Vec<(usize, &crate::config::TimezoneConfig)> {
        self.config
            .timezones
            .iter()
            .enumerate()
            .filter(|(_, tz)| {
                self.search_query.is_empty()
                    || tz
                        .name
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                    || tz
                        .timezone
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
            })
            .collect()
    }

    /// Returns the number of configured timezones (filtered)
    pub fn timezone_count(&self) -> usize {
        self.get_filtered_timezones().len()
    }

    /// Gets the current time with the applied offset
    #[allow(dead_code)]
    pub fn current_time(&self) -> DateTime<Utc> {
        Utc::now() + self.time_offset
    }

    /// Toggles the help modal
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Toggles 12/24 hour format
    pub fn toggle_format(&mut self) {
        self.use_12h_format = !self.use_12h_format;
    }

    /// Enters search mode
    pub fn enter_search(&mut self) {
        self.is_searching = true;
        self.show_help = false;
    }

    /// Exits search mode
    pub fn exit_search(&mut self) {
        self.is_searching = false;
    }

    /// Clears search query
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.selected = 0;
    }

    /// Appends a character to the search query
    pub fn append_search(&mut self, c: char) {
        self.search_query.push(c);
        self.selected = 0; // Reset selection when filtering changes
    }

    /// Removes the last character from the search query
    pub fn backspace_search(&mut self) {
        self.search_query.pop();
        self.selected = 0;
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

    /// Resets the time offset to zero
    pub fn reset_time(&mut self) {
        self.time_offset = Duration::zero();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{TimezoneConfig, WorkHours};

    fn create_test_config() -> Config {
        Config {
            timezones: vec![
                TimezoneConfig {
                    name: "Test1".to_string(),
                    timezone: "UTC".to_string(),
                    work_hours: WorkHours {
                        start: "09:00".to_string(),
                        end: "17:00".to_string(),
                    },
                },
                TimezoneConfig {
                    name: "Test2".to_string(),
                    timezone: "UTC".to_string(),
                    work_hours: WorkHours {
                        start: "09:00".to_string(),
                        end: "17:00".to_string(),
                    },
                },
            ],
            use_12h_format: false,
        }
    }

    #[test]
    fn test_app_initialization() {
        let config = create_test_config();
        let app = App::new(config);
        assert_eq!(app.selected, 0);
        assert_eq!(app.time_offset, Duration::zero());
        assert_eq!(app.timezone_count(), 2);
    }

    #[test]
    fn test_navigation() {
        let config = create_test_config();
        let mut app = App::new(config);

        app.next();
        assert_eq!(app.selected, 1);

        app.next();
        assert_eq!(app.selected, 0);

        app.previous();
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn test_time_adjustment() {
        let config = create_test_config();
        let mut app = App::new(config);

        app.adjust_time_forward(60);
        assert_eq!(app.time_offset, Duration::minutes(60));

        app.adjust_time_backward(30);
        assert_eq!(app.time_offset, Duration::minutes(30));

        app.reset_time();
        assert_eq!(app.time_offset, Duration::zero());
    }

    #[test]
    fn test_search() {
        let config = create_test_config();
        let mut app = App::new(config);

        app.enter_search();
        assert!(app.is_searching);

        app.append_search('T');
        app.append_search('e');
        app.append_search('s');
        app.append_search('t');
        app.append_search('1');

        assert_eq!(app.search_query, "Test1");
        assert_eq!(app.timezone_count(), 1);

        app.backspace_search();
        assert_eq!(app.search_query, "Test");
        assert_eq!(app.timezone_count(), 2);

        app.clear_search();
        assert_eq!(app.search_query, "");
        assert_eq!(app.timezone_count(), 2);
    }
}
