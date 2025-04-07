use crate::leptos::components::repository_reader::MathItem;
use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys;

/// Helper function to simulate timeout
fn set_timeout<F>(f: F, delay: i32)
where
    F: FnOnce() + 'static,
{
    let window = web_sys::window().expect("no global `window` exists");
    let closure = Closure::once(f);
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            delay,
        )
        .expect("Failed to set timeout");
    closure.forget();
}

/// Component to view mathematical content like theorems, proofs, etc.
#[component]
pub fn ContentViewer() -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let (found_item, set_found_item) = create_signal::<Option<MathItem>>(None);
    // Use a hardcoded path for simplicity
    let current_path = "/theorems/pythagoras".to_string();

    create_effect(move |_| {
        set_loading.set(true);
        set_found_item.set(None);

        // Simulate fetching data
        set_timeout(
            move || {
                // Simulate finding an item - using the correct MathItem structure
                let found = Some(MathItem::Theorem {
                    name: "Pythagoras".to_string(),
                    path: "/theorems/pythagoras".to_string(),
                });
                set_found_item.set(found);
                set_loading.set(false);
            },
            1000,
        );
    });

    view! {
        <div class="content-viewer">
            <h1>"Content Viewer"</h1>
            <div class="content-status">
                {move || {
                    if loading.get() {
                        "Loading...".to_string()
                    } else if let Some(item) = found_item.get() {
                        match item {
                            MathItem::Theorem { name, .. } => format!("Theorem: {}", name),
                            MathItem::Definition { name, .. } => format!("Definition: {}", name),
                            MathItem::Theory { name, .. } => format!("Theory: {}", name),
                        }
                    } else {
                        "Content not found".to_string()
                    }
                }}
            </div>
        </div>
    }
}
