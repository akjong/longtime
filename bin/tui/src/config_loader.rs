//! Configuration file loading for the TUI application
//!
//! This module handles loading configuration from TOML files,
//! while the core data structures are defined in `longtime-core`.

use config::{Config as ConfigLoader, File};
use longtime_core::Config;

/// Load configuration from a file path
///
/// # Arguments
///
/// * `config_path` - Optional path to the config file. If None, uses default location.
///
/// # Returns
///
/// * `Result<Config, Box<dyn std::error::Error>>` - The loaded configuration or an error
///
/// # Default Path
///
/// If no path is provided, the default location is `~/.config/longtime/config.toml`
pub fn load_config(config_path: Option<&str>) -> Result<Config, Box<dyn std::error::Error>> {
    let builder = ConfigLoader::builder();

    // Determine the config source
    let config_source = if let Some(path) = config_path {
        File::with_name(path)
    } else {
        // Default path: ~/.config/longtime/config.toml
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        let default_path = home.join(".config").join("longtime").join("config.toml");
        File::from(default_path)
    };

    let config = builder.add_source(config_source).build()?;

    let app_config: Config = config.try_deserialize()?;
    Ok(app_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_with_invalid_path() {
        let result = load_config(Some("/nonexistent/path/config.toml"));
        assert!(result.is_err());
    }
}
