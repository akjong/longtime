use serde::Deserialize;

// Structure for the work hours in the config file
#[derive(Deserialize, Debug, Clone)]
pub struct WorkHours {
    pub start: String, // Format: "HH:MM"
    pub end: String,   // Format: "HH:MM"
}

// Structure for each timezone entry in the config file
#[derive(Deserialize, Debug, Clone)]
pub struct TimeZoneConfig {
    pub name: String,
    pub timezone: String,
    pub work_hours: WorkHours,
}

// Main configuration structure
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub timezones: Vec<TimeZoneConfig>,
}
