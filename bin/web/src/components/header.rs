//! Header component
//!
//! Displays the application title, share button, and settings controls.

use leptos::prelude::*;

use crate::{state::AppState, storage::generate_share_url};

/// Clock SVG icon
#[component]
fn ClockIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="text-primary"
      >
        <circle cx="12" cy="12" r="10" />
        <polyline points="12 6 12 12 16 14" />
      </svg>
    }
}

/// Plus SVG icon
#[component]
fn PlusIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="12" y1="5" x2="12" y2="19" />
        <line x1="5" y1="12" x2="19" y2="12" />
      </svg>
    }
}

/// Share/Link SVG icon
#[component]
fn ShareIcon() -> impl IntoView {
    view! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
        <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
      </svg>
    }
}

/// Sun SVG icon (for light mode)
#[component]
fn SunIcon() -> impl IntoView {
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
        <circle cx="12" cy="12" r="5" />
        <line x1="12" y1="1" x2="12" y2="3" />
        <line x1="12" y1="21" x2="12" y2="23" />
        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
        <line x1="1" y1="12" x2="3" y2="12" />
        <line x1="21" y1="12" x2="23" y2="12" />
        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
      </svg>
    }
}

/// Moon SVG icon (for dark mode)
#[component]
fn MoonIcon() -> impl IntoView {
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
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    }
}

/// Header component with title and action buttons
#[component]
pub fn Header() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
      <header class="sticky top-0 z-10 py-3 px-4 border-b bg-surface-alt border-primary/30 backdrop-blur-sm">
        <div class="container flex justify-between items-center mx-auto">
          // Logo and title
          <div class="flex gap-3 items-center">
            <ClockIcon />
            <h1 class="font-mono text-xl font-bold tracking-wider text-primary">
              <span class="text-primary/60">">"</span>
              "LongTime"
              <span class="terminal-cursor">"_"</span>
            </h1>
          </div>

          // Action buttons
          <div class="flex gap-2 items-center">
            // 12/24h toggle
            <button
              on:click={
                let state = state.clone();
                move |_| state.toggle_format()
              }
              class="font-mono text-sm btn-terminal"
              title="Toggle 12/24 hour format"
            >
              {
                let state = state.clone();
                move || if state.config.get().use_12h_format { "12h" } else { "24h" }
              }
            </button>

            // Add timezone button
            <button
              on:click={
                let state = state.clone();
                move |_| state.open_add_modal()
              }
              class="flex gap-1 items-center text-sm btn-primary"
              title="Add timezone"
            >
              <PlusIcon />
              <span class="hidden sm:inline">"Add"</span>
            </button>

            // Share button
            <button
              on:click={
                let state = state.clone();
                move |_| {
                  let config = state.config.get();
                  let url = generate_share_url(&config);
                  leptos::task::spawn_local(async move {
                    if crate::storage::copy_to_clipboard(&url).await.is_ok() {
                      let _ = web_sys::window()
                        .and_then(|w| w.alert_with_message("Link copied to clipboard!").ok());
                    }
                  });
                }
              }
              class="flex gap-1 items-center text-sm btn-terminal"
              title="Copy shareable link"
            >
              <ShareIcon />
              <span class="hidden sm:inline">"Share"</span>
            </button>

            // Theme toggle button
            <button
              on:click={
                let state = state.clone();
                move |_| state.toggle_theme()
              }
              class="p-2 btn-terminal"
              title="Toggle dark/light mode"
            >
              {
                let state = state.clone();
                move || {
                  if state.dark_mode.get() {
                    view! { <SunIcon /> }.into_any()
                  } else {
                    view! { <MoonIcon /> }.into_any()
                  }
                }
              }
            </button>
          </div>
        </div>
      </header>
    }
}
