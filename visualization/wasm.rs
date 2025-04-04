//! WASM entry point for the formalize_v2 visualization system
//!
//! This module handles initializing the application for WebAssembly

#[cfg(feature = "theorem_visualizer")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::visualization::app::VisualizeApp;

/// Initialize the application when loaded as WebAssembly
#[wasm_bindgen(start)]
#[cfg(feature = "theorem_visualizer")]
pub fn start() {
    // Set up the panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize console logging
    #[cfg(debug_assertions)]
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize console logger");

    #[cfg(not(debug_assertions))]
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize console logger");

    log::info!("Formalize v2 Visualization starting up...");

    // Mount the app to the document body
    leptos::mount_to_body(|| {
        leptos::view! { <VisualizeApp/> }
    });
}

/// Entry point for calling from JavaScript
#[wasm_bindgen]
#[cfg(feature = "theorem_visualizer")]
pub fn init() {
    start();
}
