//! Application state management
//!
//! This module defines the reactive state used throughout the application,
//! including timezone configuration, time offset, and UI state.

use chrono::{DateTime, Duration, Utc};
use leptos::prelude::*;
use longtime_core::Config;

/// Main application state
///
/// This struct contains all reactive signals used by the application.
/// It is provided via Leptos context to all child components.
#[derive(Clone)]
pub struct AppState {
    /// Current timezone configuration
    pub config: RwSignal<Config>,
    /// Time offset for time-travel simulation (in seconds)
    pub time_offset: RwSignal<i64>,
    /// Whether time is auto-updating
    pub is_running: RwSignal<bool>,
    /// Whether to show the configuration modal
    pub show_config_modal: RwSignal<bool>,
    /// Index of timezone being edited (None for adding new)
    pub editing_index: RwSignal<Option<usize>>,
    /// Currently selected timezone index (for reference calculations)
    pub selected_index: RwSignal<usize>,
    /// Tick counter to trigger time updates
    pub tick: RwSignal<u64>,
    /// Dark mode state (true = dark, false = light)
    pub dark_mode: RwSignal<bool>,
}

impl AppState {
    /// Create a new AppState with the given configuration
    pub fn new(config: Config) -> Self {
        // Load dark mode preference from localStorage or default to true (dark)
        let dark_mode = web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("longtime_dark_mode").ok().flatten())
            .map(|v| v != "false")
            .unwrap_or(true);

        Self {
            config: RwSignal::new(config),
            time_offset: RwSignal::new(0),
            is_running: RwSignal::new(true),
            show_config_modal: RwSignal::new(false),
            editing_index: RwSignal::new(None),
            selected_index: RwSignal::new(0),
            tick: RwSignal::new(0),
            dark_mode: RwSignal::new(dark_mode),
        }
    }

    /// Get the current time with offset applied
    pub fn current_time(&self) -> DateTime<Utc> {
        // Read tick to create dependency for reactivity
        let _ = self.tick.get();
        Utc::now() + Duration::seconds(self.time_offset.get())
    }

    /// Adjust time offset by the given number of minutes
    pub fn adjust_time(&self, minutes: i64) {
        self.time_offset.update(|offset| *offset += minutes * 60);
    }

    /// Reset time offset to zero
    pub fn reset_time(&self) {
        self.time_offset.set(0);
    }

    /// Toggle whether time is running
    pub fn toggle_running(&self) {
        self.is_running.update(|running| *running = !*running);
    }

    /// Open modal to add a new timezone
    pub fn open_add_modal(&self) {
        self.editing_index.set(None);
        self.show_config_modal.set(true);
    }

    /// Open modal to edit an existing timezone
    pub fn open_edit_modal(&self, index: usize) {
        self.editing_index.set(Some(index));
        self.show_config_modal.set(true);
    }

    /// Close the configuration modal
    pub fn close_modal(&self) {
        self.show_config_modal.set(false);
        self.editing_index.set(None);
    }

    /// Delete a timezone at the given index
    pub fn delete_timezone(&self, index: usize) {
        self.config.update(|config| {
            if index < config.timezones.len() {
                config.timezones.remove(index);
            }
        });
        // Trigger storage save
        crate::storage::save_config(&self.config.get());
    }

    /// Toggle 12/24 hour format
    pub fn toggle_format(&self) {
        self.config.update(|config| {
            config.use_12h_format = !config.use_12h_format;
        });
        crate::storage::save_config(&self.config.get());
    }

    /// Toggle dark/light mode
    pub fn toggle_theme(&self) {
        self.dark_mode.update(|dark| *dark = !*dark);
        // Save preference to localStorage
        if let Some(window) = web_sys::window()
            && let Ok(Some(storage)) = window.local_storage()
        {
            let _ = storage.set_item(
                "longtime_dark_mode",
                if self.dark_mode.get() {
                    "true"
                } else {
                    "false"
                },
            );
        }
    }
}
