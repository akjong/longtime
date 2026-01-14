//! Timezone card component
//!
//! Displays a single timezone with its current time, date, and work status.

use leptos::prelude::*;
use longtime_core::{TimezoneConfig, get_time_display_info};

use crate::state::AppState;

/// Edit/Pencil SVG icon
#[component]
fn EditIcon() -> impl IntoView {
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
        <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
        <path d="m15 5 4 4" />
      </svg>
    }
}

/// Trash/Delete SVG icon
#[component]
fn TrashIcon() -> impl IntoView {
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
        <path d="M3 6h18" />
        <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
        <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
        <line x1="10" y1="11" x2="10" y2="17" />
        <line x1="14" y1="11" x2="14" y2="17" />
      </svg>
    }
}

/// Timezone card component
#[component]
pub fn TimezoneCard(
    /// The timezone configuration
    config: TimezoneConfig,
    /// Index of this timezone in the list
    index: usize,
    /// Reference timezone offset in seconds
    reference_offset: i32,
) -> impl IntoView {
    let state = expect_context::<AppState>();

    // Clone config for the closure
    let config_for_view = config.clone();

    view! {
      <div
        class="cursor-pointer card-terminal group"
        on:click={
          let state = state.clone();
          move |_| state.selected_index.set(index)
        }
      >
        // Header with name and actions
        <div class="flex justify-between items-start mb-3">
          <div>
            <h3 class="font-mono text-lg font-bold text-primary">
              <span class="text-primary/50">"$ "</span>
              {config_for_view.name.clone()}
            </h3>
            <p class="mt-1 font-mono text-xs text-text-secondary">
              <span class="text-primary/40">"# "</span>
              {config_for_view.timezone.clone()}
            </p>
          </div>
          <div class="flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
            <button
              on:click={
                let state = state.clone();
                move |e: web_sys::MouseEvent| {
                  e.stop_propagation();
                  state.open_edit_modal(index);
                }
              }
              class="p-1.5 rounded border border-transparent transition-colors text-text-secondary hover:border-primary/50 hover:text-primary"
              title="Edit timezone"
            >
              <EditIcon />
            </button>
            <button
              on:click={
                let state = state.clone();
                move |e: web_sys::MouseEvent| {
                  e.stop_propagation();
                  state.delete_timezone(index);
                }
              }
              class="p-1.5 rounded border border-transparent transition-colors hover:text-red-400 text-text-secondary hover:border-red-500/50"
              title="Delete timezone"
            >
              <TrashIcon />
            </button>
          </div>
        </div>

        // Time display
        {
          let config = config_for_view.clone();
          let state = state.clone();
          move || {
            let now = state.current_time();
            let use_12h = state.config.get().use_12h_format;
            let info = get_time_display_info(now, &config, reference_offset, use_12h);
            match info {
              Some(info) => {
                let diff_str = if info.diff_hours == 0.0 {
                  "=".to_string()
                } else if info.diff_hours > 0.0 {
                  format!("+{}", info.diff_hours)
                } else {
                  format!("{}", info.diff_hours)
                };

                view! {
                  <div>
                    // Time
                    <div class="mb-2 text-4xl time-display">{info.time}</div>
                    // Date and diff
                    <div class="flex justify-between items-center font-mono text-sm">
                      <span class="text-text-secondary">{info.date}</span>
                      <span class="text-accent">{diff_str}</span>
                    </div>
                    // Work status
                    <div class="flex gap-2 items-center mt-3 font-mono text-sm">
                      <span class=if info.is_working {
                        "status-dot status-online"
                      } else {
                        "status-dot status-offline"
                      }></span>
                      <span class=if info.is_working {
                        "text-working"
                      } else {
                        "text-off"
                      }>{if info.is_working { "[ONLINE]" } else { "[OFFLINE]" }}</span>
                    </div>
                  </div>
                }
                  .into_any()
              }
              None => {
                view! { <div class="font-mono text-red-400">"[ERROR] Invalid timezone"</div> }
                  .into_any()
              }
            }
          }
        }
      </div>
    }
}
