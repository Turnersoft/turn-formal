// Module: src/formalize_v2/subjects/math/visualization/wasm.rs
// WASM bindings for the Theorem Visualizer

#[cfg(feature = "theorem_visualizer")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::app::TheoremVisualizerApp;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::common::*;

/// Initialize the Theorem Visualizer app
#[wasm_bindgen(start)]
#[cfg(feature = "theorem_visualizer")]
pub fn start() {
    // Set up the panic hook for better error messages
    console_error_panic_hook::set_once();

    // Enable console logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("Theorem Visualizer starting up...");

    // Mount the app to body
    mount_to_body(|| {
        view! { <TheoremVisualizerApp /> }
    });
}

/// Make the app available to JS
#[wasm_bindgen]
#[cfg(feature = "theorem_visualizer")]
pub fn init() {
    start();
}
