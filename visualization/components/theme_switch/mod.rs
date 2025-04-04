//! Theme switch component for the formalize_v2 visualization system
//!
//! Provides a component to toggle between light and dark themes

#[cfg(feature = "theorem_visualizer")]
use leptos::*;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::themes::Theme;

/// Theme toggle button component
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn ThemeToggle() -> impl IntoView {
    // Get the current theme from context
    let theme = use_context::<ReadSignal<Theme>>().expect("Theme context not found");

    // Get the theme toggle callback from context
    let toggle_theme = use_context::<Callback<(), ()>>().expect("Theme toggle callback not found");

    // Create derived signals for showing the correct icon
    let is_dark_mode = move || theme.get() == Theme::Dark;

    view! {
        <button
            class="theme-toggle"
            on:click=move |_| toggle_theme.call(())
            aria-label=move || if is_dark_mode() { "Switch to light mode" } else { "Switch to dark mode" }
            title=move || if is_dark_mode() { "Switch to light mode" } else { "Switch to dark mode" }
        >
            <div class="icon-container">
                <div class=move || if is_dark_mode() { "icon-hidden" } else { "icon-visible" }>
                    // Sun icon for light mode
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1" x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1" y1="12" x2="3" y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                    </svg>
                </div>
                <div class=move || if is_dark_mode() { "icon-visible" } else { "icon-hidden" }>
                    // Moon icon for dark mode
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </div>
            </div>
        </button>
    }
}
