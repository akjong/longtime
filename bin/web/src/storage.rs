//! Storage and URL handling utilities
//!
//! This module provides functions for persisting configuration to LocalStorage
//! and encoding/decoding configuration for URL sharing.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use gloo_storage::{LocalStorage, Storage};
use longtime_core::Config;

/// LocalStorage key for configuration
const STORAGE_KEY: &str = "longtime_config";

/// Save configuration to LocalStorage
pub fn save_config(config: &Config) {
    let _ = LocalStorage::set(STORAGE_KEY, config);
}

/// Load configuration from LocalStorage
pub fn load_config_from_storage() -> Option<Config> {
    LocalStorage::get(STORAGE_KEY).ok()
}

/// Clear configuration from LocalStorage
#[allow(dead_code)]
pub fn clear_config() {
    LocalStorage::delete(STORAGE_KEY);
}

/// Encode configuration to a URL-safe Base64 string
pub fn encode_config_to_url(config: &Config) -> String {
    let json = serde_json::to_string(config).unwrap_or_default();
    URL_SAFE_NO_PAD.encode(json.as_bytes())
}

/// Decode configuration from a URL-safe Base64 string
pub fn decode_config_from_url(encoded: &str) -> Option<Config> {
    let bytes = URL_SAFE_NO_PAD.decode(encoded).ok()?;
    let json = String::from_utf8(bytes).ok()?;
    serde_json::from_str(&json).ok()
}

/// Generate a shareable URL with the current configuration
pub fn generate_share_url(config: &Config) -> String {
    let encoded = encode_config_to_url(config);
    let base_url = get_base_url();
    format!("{base_url}?config={encoded}")
}

/// Get the base URL without query parameters
fn get_base_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().href().ok())
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or("")
        .to_string()
}

/// Get query parameter value from the current URL
fn get_query_param(key: &str) -> Option<String> {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|search| {
            let search = search.trim_start_matches('?');
            for pair in search.split('&') {
                let mut parts = pair.splitn(2, '=');
                if let (Some(k), Some(v)) = (parts.next(), parts.next())
                    && k == key
                {
                    return Some(v.to_string());
                }
            }
            None
        })
}

/// Load initial configuration from URL, LocalStorage, or defaults
///
/// Priority:
/// 1. URL query parameter (?config=<Base64>)
/// 2. LocalStorage
/// 3. Default configuration
pub fn load_initial_config() -> Config {
    // Check URL first (for sharing)
    if let Some(encoded) = get_query_param("config")
        && let Some(config) = decode_config_from_url(&encoded)
    {
        // Save to LocalStorage and return
        save_config(&config);
        return config;
    }

    // Check LocalStorage
    if let Some(config) = load_config_from_storage() {
        return config;
    }

    // Return default
    Config::default()
}

/// Copy text to clipboard
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or("No window")?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to copy to clipboard".to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let config = Config::default();
        let encoded = encode_config_to_url(&config);
        let decoded = decode_config_from_url(&encoded);

        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), config);
    }
}
