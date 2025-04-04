//! Themes module for the formalize_v2 visualization system
//!
//! Handles theme management, switching between light and dark modes,
//! and provides theme-related components and utilities.

#[cfg(feature = "theorem_visualizer")]
use leptos::*;
#[cfg(feature = "theorem_visualizer")]
use web_sys::Storage;

mod theme;

#[cfg(feature = "theorem_visualizer")]
pub use theme::Theme;

/// Signal key for the current theme
#[cfg(feature = "theorem_visualizer")]
const THEME_STORAGE_KEY: &str = "formalize_v2_theme";

/// Context provider for theme management
#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Try to get the theme from local storage or use the default (light)
    let initial_theme = get_stored_theme().unwrap_or(Theme::Light);

    // Create a signal to track the current theme
    let (current_theme, set_current_theme) = create_signal(initial_theme);

    // Function to toggle the theme
    let toggle_theme = move || {
        set_current_theme.update(|theme| {
            let new_theme = match theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };

            // Store the new theme in local storage
            if let Some(storage) = window().local_storage().ok().flatten() {
                let _ = storage.set_item(THEME_STORAGE_KEY, &new_theme.to_string());
            }

            *theme = new_theme;
        });
    };

    // Provide the theme signals and toggle function to children
    provide_context(current_theme);
    provide_context(create_callback(move |_| toggle_theme()));

    // Apply the theme class to the document body
    create_effect(move |_| {
        let theme_class = current_theme.get().to_class();
        document()
            .body()
            .expect("body should exist")
            .class_list()
            .remove_1("light-theme")
            .expect("should remove light theme class");
        document()
            .body()
            .expect("body should exist")
            .class_list()
            .remove_1("dark-theme")
            .expect("should remove dark theme class");
        document()
            .body()
            .expect("body should exist")
            .class_list()
            .add_1(&theme_class)
            .expect("should add theme class");
    });

    view! { {children()} }
}

/// Utility function to get the stored theme from local storage
#[cfg(feature = "theorem_visualizer")]
fn get_stored_theme() -> Option<Theme> {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage: Storage| storage.get_item(THEME_STORAGE_KEY).ok())
        .flatten()
        .and_then(|theme_str| theme_str.parse().ok())
}

/// Window accessor for web-sys
#[cfg(feature = "theorem_visualizer")]
fn window() -> web_sys::Window {
    web_sys::window().expect("window should exist")
}

/// Document accessor for web-sys
#[cfg(feature = "theorem_visualizer")]
fn document() -> web_sys::Document {
    window().document().expect("document should exist")
}
