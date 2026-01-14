//! LongTime Web Application
//!
//! A client-side rendered (CSR) web application for managing and displaying
//! time information across multiple time zones.

pub mod app;
pub mod components;
pub mod state;
pub mod storage;

use wasm_bindgen::prelude::wasm_bindgen;

/// WASM entry point for client-side rendering
#[wasm_bindgen(start)]
pub fn hydrate() {
    // Set up better panic messages in the browser console
    console_error_panic_hook::set_once();

    // Mount the app to the document body
    leptos::mount::mount_to_body(app::App);
}
