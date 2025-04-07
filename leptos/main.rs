use leptos::mount::mount_to_body;
use leptos::*;
use log;
use turn_formal::leptos::app::App;
use wasm_bindgen::prelude::*;

// This main function is necessary for the binary target
// It will be used for non-WASM builds directly
// For WASM builds, it satisfies the compiler but isn't actually called
fn main() {
    // Initialize console error panic hook for better debugging
    console_error_panic_hook::set_once();

    // Enable logging for WASM
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Starting Turn-Formal application");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        simple_logger::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    }

    mount_to_body(|| view! { <App /> })
}

// For WebAssembly, we use wasm_bindgen to export our start function
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the App component to the body of the document
    main();
}
