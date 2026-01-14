//! Time controls component
//!
//! Provides controls for adjusting the time offset and pausing/resuming updates.

use leptos::prelude::*;

use crate::state::AppState;

/// Reset/Refresh SVG icon
#[component]
fn ResetIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
        <path d="M3 3v5h5" />
      </svg>
    }
}

/// Play SVG icon
#[component]
fn PlayIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polygon points="5 3 19 12 5 21 5 3" />
      </svg>
    }
}

/// Pause SVG icon
#[component]
fn PauseIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <rect width="4" height="16" x="6" y="4" />
        <rect width="4" height="16" x="14" y="4" />
      </svg>
    }
}

/// Time controls component
#[component]
pub fn TimeControls() -> impl IntoView {
    let state = expect_context::<AppState>();

    // Format offset for display
    let offset_display = {
        let state = state.clone();
        move || {
            let offset_secs = state.time_offset.get();
            if offset_secs == 0 {
                "[ NOW ]".to_string()
            } else {
                let hours = offset_secs / 3600;
                let minutes = (offset_secs.abs() % 3600) / 60;
                if minutes == 0 {
                    format!("[{hours:+}h]")
                } else {
                    format!("[{hours:+}h {minutes:02}m]")
                }
            }
        }
    };

    view! {
      <footer class="sticky bottom-0 py-3 px-4 border-t bg-surface-alt border-primary/30 backdrop-blur-sm">
        <div class="container flex gap-2 justify-center items-center mx-auto sm:gap-4">
          // Reset button
          <button
            on:click={
              let state = state.clone();
              move |_| state.reset_time()
            }
            class="flex gap-1 items-center text-sm btn-terminal"
            title="Reset to current time"
          >
            <ResetIcon />
            <span class="hidden sm:inline">"Reset"</span>
          </button>

          // Time adjustment buttons
          <div class="flex gap-1 items-center">
            <button
              on:click={
                let state = state.clone();
                move |_| state.adjust_time(-60)
              }
              class="font-mono text-sm btn-terminal"
              title="-1 hour"
            >
              "-1h"
            </button>
            <button
              on:click={
                let state = state.clone();
                move |_| state.adjust_time(-15)
              }
              class="font-mono text-sm btn-terminal"
              title="-15 minutes"
            >
              "-15m"
            </button>

            // Current offset display
            <div class="py-2 px-4 font-mono text-center min-w-28 text-primary text-glow">
              {offset_display}
            </div>

            <button
              on:click={
                let state = state.clone();
                move |_| state.adjust_time(15)
              }
              class="font-mono text-sm btn-terminal"
              title="+15 minutes"
            >
              "+15m"
            </button>
            <button
              on:click={
                let state = state.clone();
                move |_| state.adjust_time(60)
              }
              class="font-mono text-sm btn-terminal"
              title="+1 hour"
            >
              "+1h"
            </button>
          </div>

          // Play/Pause button
          <button
            on:click={
              let state = state.clone();
              move |_| state.toggle_running()
            }
            class="flex gap-1 items-center text-sm btn-terminal"
            title={
              let state = state.clone();
              move || if state.is_running.get() { "Pause" } else { "Resume" }
            }
          >
            {
              let state = state.clone();
              move || {
                if state.is_running.get() {
                  view! {
                    <PauseIcon />
                    <span class="hidden sm:inline">"Pause"</span>
                  }
                    .into_any()
                } else {
                  view! {
                    <PlayIcon />
                    <span class="hidden sm:inline">"Play"</span>
                  }
                    .into_any()
                }
              }
            }
          </button>
        </div>
      </footer>
    }
}
