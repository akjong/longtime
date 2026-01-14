//! Configuration modal component
//!
//! Provides a modal dialog for adding or editing timezone configurations.

use leptos::prelude::*;
use longtime_core::{TimezoneConfig, WorkHours};

use crate::{state::AppState, storage::save_config};

/// Close/X SVG icon
#[component]
fn CloseIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="18" y1="6" x2="6" y2="18" />
        <line x1="6" y1="6" x2="18" y2="18" />
      </svg>
    }
}

/// Common IANA timezone options for the dropdown
const TIMEZONE_OPTIONS: &[(&str, &str)] = &[
    ("Asia/Shanghai", "Shanghai (UTC+8)"),
    ("Asia/Tokyo", "Tokyo (UTC+9)"),
    ("Asia/Singapore", "Singapore (UTC+8)"),
    ("Asia/Hong_Kong", "Hong Kong (UTC+8)"),
    ("Asia/Seoul", "Seoul (UTC+9)"),
    ("Asia/Kolkata", "Mumbai (UTC+5:30)"),
    ("Asia/Dubai", "Dubai (UTC+4)"),
    ("Europe/London", "London (UTC+0/+1)"),
    ("Europe/Paris", "Paris (UTC+1/+2)"),
    ("Europe/Berlin", "Berlin (UTC+1/+2)"),
    ("Europe/Moscow", "Moscow (UTC+3)"),
    ("America/New_York", "New York (UTC-5/-4)"),
    ("America/Chicago", "Chicago (UTC-6/-5)"),
    ("America/Denver", "Denver (UTC-7/-6)"),
    ("America/Los_Angeles", "Los Angeles (UTC-8/-7)"),
    ("America/Sao_Paulo", "Sao Paulo (UTC-3)"),
    ("Australia/Sydney", "Sydney (UTC+10/+11)"),
    ("Australia/Melbourne", "Melbourne (UTC+10/+11)"),
    ("Pacific/Auckland", "Auckland (UTC+12/+13)"),
    ("UTC", "UTC"),
];

/// Configuration modal component
#[component]
pub fn ConfigModal() -> impl IntoView {
    let state = expect_context::<AppState>();

    // Form state
    let name = RwSignal::new(String::new());
    let timezone = RwSignal::new(String::from("Asia/Shanghai"));
    let work_start = RwSignal::new(String::from("09:00"));
    let work_end = RwSignal::new(String::from("17:00"));

    // Initialize form when modal opens
    {
        let state = state.clone();
        Effect::new(move || {
            if state.show_config_modal.get() {
                if let Some(index) = state.editing_index.get() {
                    // Editing existing timezone
                    let config = state.config.get();
                    if let Some(tz) = config.timezones.get(index) {
                        name.set(tz.name.clone());
                        timezone.set(tz.timezone.clone());
                        work_start.set(tz.work_hours.start.clone());
                        work_end.set(tz.work_hours.end.clone());
                    }
                } else {
                    // Adding new timezone
                    name.set(String::new());
                    timezone.set(String::from("Asia/Shanghai"));
                    work_start.set(String::from("09:00"));
                    work_end.set(String::from("17:00"));
                }
            }
        });
    }

    view! {
      <Show when={
        let state = state.clone();
        move || state.show_config_modal.get()
      }>
        // Backdrop
        <div
          class="modal-backdrop"
          on:click={
            let state = state.clone();
            move |_| state.close_modal()
          }
        ></div>

        // Modal
        <div class="flex fixed inset-0 z-50 justify-center items-center p-4">
          <div
            class="w-full max-w-md modal-content"
            on:click=|e: web_sys::MouseEvent| e.stop_propagation()
          >
            // Header
            <div class="flex justify-between items-center mb-6">
              <h2 class="font-mono text-xl font-bold text-primary">
                <span class="text-primary/50">"$ "</span>
                {
                  let state = state.clone();
                  move || {
                    if state.editing_index.get().is_some() {
                      "edit-timezone"
                    } else {
                      "add-timezone"
                    }
                  }
                }
              </h2>
              <button
                on:click={
                  let state = state.clone();
                  move |_| state.close_modal()
                }
                class="transition-colors text-text-secondary hover:text-primary"
              >
                <CloseIcon />
              </button>
            </div>

            // Form
            <form class="space-y-4" on:submit=|e: web_sys::SubmitEvent| e.prevent_default()>
              // Name input
              <div>
                <label class="block mb-1 font-mono text-sm text-text-secondary">
                  <span class="text-primary/50">"# "</span>
                  "display_name"
                </label>
                <input
                  type="text"
                  class="w-full input-terminal"
                  placeholder="e.g., Shanghai Office"
                  prop:value=move || name.get()
                  on:input=move |e| name.set(event_target_value(&e))
                />
              </div>

              // Timezone select
              <div>
                <label class="block mb-1 font-mono text-sm text-text-secondary">
                  <span class="text-primary/50">"# "</span>
                  "timezone"
                </label>
                <select
                  class="w-full input-terminal"
                  prop:value=move || timezone.get()
                  on:change=move |e| timezone.set(event_target_value(&e))
                >
                  {TIMEZONE_OPTIONS
                    .iter()
                    .map(|(value, label)| {
                      view! { <option value=*value>{*label}</option> }
                    })
                    .collect_view()}
                </select>
              </div>

              // Work hours
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="block mb-1 font-mono text-sm text-text-secondary">
                    <span class="text-primary/50">"# "</span>
                    "work_start"
                  </label>
                  <input
                    type="time"
                    class="w-full input-terminal"
                    prop:value=move || work_start.get()
                    on:input=move |e| work_start.set(event_target_value(&e))
                  />
                </div>
                <div>
                  <label class="block mb-1 font-mono text-sm text-text-secondary">
                    <span class="text-primary/50">"# "</span>
                    "work_end"
                  </label>
                  <input
                    type="time"
                    class="w-full input-terminal"
                    prop:value=move || work_end.get()
                    on:input=move |e| work_end.set(event_target_value(&e))
                  />
                </div>
              </div>

              // Buttons
              <div class="flex gap-3 pt-4">
                <button
                  type="button"
                  on:click={
                    let state = state.clone();
                    move |_| state.close_modal()
                  }
                  class="flex-1 btn-terminal"
                >
                  "Cancel"
                </button>
                <button
                  type="submit"
                  on:click={
                    let state = state.clone();
                    move |_| {
                      let tz_config = TimezoneConfig {
                        name: name.get(),
                        timezone: timezone.get(),
                        work_hours: WorkHours {
                          start: work_start.get(),
                          end: work_end.get(),
                        },
                      };
                      state
                        .config
                        .update(|config| {
                          if let Some(index) = state.editing_index.get() {
                            if index < config.timezones.len() {
                              config.timezones[index] = tz_config;
                            }
                          } else {
                            config.timezones.push(tz_config);
                          }
                        });
                      save_config(&state.config.get());
                      state.close_modal();
                    }
                  }
                  class="flex-1 font-semibold btn-primary"
                >
                  "Save"
                </button>
              </div>
            </form>
          </div>
        </div>
      </Show>
    }
}
