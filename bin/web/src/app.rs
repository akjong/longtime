//! Root application component
//!
//! This module contains the main App component that serves as the
//! entry point for the Leptos application.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{
    components::{ConfigModal, Header, TimeControls, TimezoneList},
    state::AppState,
    storage::load_initial_config,
};

/// Root application component
#[component]
pub fn App() -> impl IntoView {
    // Load initial configuration from URL, LocalStorage, or defaults
    let config = load_initial_config();

    // Create application state
    let state = AppState::new(config);

    // Provide state to all child components via context
    provide_context(state.clone());

    // Set up time update interval (every second)
    let state_for_interval = state.clone();
    Effect::new(move || {
        use gloo_timers::callback::Interval;

        let state = state_for_interval.clone();
        let _interval = Interval::new(1000, move || {
            if state.is_running.get() {
                // Trigger a re-render by updating the tick counter
                state.tick.update(|t| *t += 1);
            }
        });

        // Keep interval alive by storing it
        // Note: In a real app, you'd want to clean this up properly
        std::mem::forget(_interval);
    });

    // Set up keyboard shortcuts
    let state_for_keyboard = state.clone();
    Effect::new(move || {
        use wasm_bindgen::closure::Closure;

        let state = state_for_keyboard.clone();
        let handler =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
                // Skip if modal is open (except for Escape)
                let modal_open = state.show_config_modal.get();

                // Skip if focus is on an input element (allow typing)
                if let Some(target) = event.target()
                    && let Ok(element) = target.dyn_into::<web_sys::HtmlElement>()
                {
                    let tag = element.tag_name().to_lowercase();
                    if (tag == "input" || tag == "select" || tag == "textarea")
                        && event.key() != "Escape"
                    {
                        // Only handle Escape in inputs
                        return;
                    }
                }

                match event.key().as_str() {
                    "Escape" => {
                        // Close modal if open
                        if modal_open {
                            state.show_config_modal.set(false);
                            state.editing_index.set(None);
                            event.prevent_default();
                        }
                    }
                    "ArrowLeft" | "h" => {
                        // Decrease time by 15 minutes
                        if !modal_open {
                            state.time_offset.update(|offset| *offset -= 15 * 60);
                            event.prevent_default();
                        }
                    }
                    "ArrowRight" | "l" => {
                        // Increase time by 15 minutes
                        if !modal_open {
                            state.time_offset.update(|offset| *offset += 15 * 60);
                            event.prevent_default();
                        }
                    }
                    "r" => {
                        // Reset time offset
                        if !modal_open {
                            state.time_offset.set(0);
                            event.prevent_default();
                        }
                    }
                    " " => {
                        // Toggle play/pause
                        if !modal_open {
                            state.is_running.update(|running| *running = !*running);
                            event.prevent_default();
                        }
                    }
                    _ => {}
                }
            });

        // Add event listener to window
        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
        }

        // Keep closure alive
        handler.forget();
    });

    // Apply theme class to body based on dark_mode state
    let state_for_theme = state.clone();
    Effect::new(move || {
        let is_dark = state_for_theme.dark_mode.get();
        if let Some(document) = web_sys::window().and_then(|w| w.document())
            && let Some(body) = document.body()
        {
            let _ = body.class_list().remove_1("light");
            if !is_dark {
                let _ = body.class_list().add_1("light");
            }
        }
    });

    view! {
      <div class="flex relative flex-col min-h-screen font-mono bg-surface text-text-primary">
        // Scanline effect overlay
        <div class="scanlines"></div>

        <Header />
        <main class="container relative z-10 flex-1 py-6 px-4 mx-auto">
          <TimezoneList />
        </main>
        <TimeControls />
        <ConfigModal />
      </div>
    }
}
