//! Timezone list component
//!
//! Displays a grid of timezone cards.

use leptos::prelude::*;
use longtime_core::get_timezone_offset;

use crate::{components::TimezoneCard, state::AppState};

/// Timezone list component
#[component]
pub fn TimezoneList() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {
          let state = state.clone();
          move || {
            let config = state.config.get();
            let now = state.current_time();
            let selected_idx = state.selected_index.get();
            let reference_offset = config
              .timezones
              .get(selected_idx)
              .and_then(|tz| get_timezone_offset(now, &tz.timezone))
              .unwrap_or(0);
            if config.timezones.is_empty() {
              let state = state.clone();

              // Get reference offset from selected timezone

              view! {
                <div class="col-span-full py-12 text-center text-gray-400">
                  <p class="mb-4 text-lg">"No timezones configured"</p>
                  <button
                    on:click=move |_| state.open_add_modal()
                    class="py-2 px-4 rounded-lg transition-colors bg-primary/20 text-primary hover:bg-primary/30"
                  >
                    "+ Add your first timezone"
                  </button>
                </div>
              }
                .into_any()
            } else {
              config
                .timezones
                .iter()
                .enumerate()
                .map(|(index, tz)| {
                  view! {
                    <TimezoneCard config=tz.clone() index=index reference_offset=reference_offset />
                  }
                })
                .collect_view()
                .into_any()
            }
          }
        }
      </div>
    }
}
