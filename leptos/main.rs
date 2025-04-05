use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;

// Import the App component from the parent crate
use turn_formal::leptos::app::App;

// This main function is necessary for the binary target
// It will be used for non-WASM builds directly
// For WASM builds, it satisfies the compiler but isn't actually called
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        mount_to_body(|| view! { <App /> });
    }
}

// For WebAssembly, we use wasm_bindgen to export our start function
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    // Set up panic hook for better error messages
    _ = console_error_panic_hook::set_once();

    // Mount the App component to the body of the document
    mount_to_body(|| view! { <App /> });
}
