use crate::leptos::components::repository_reader::{MathItem, RepositoryData};
use crate::leptos::components::theorem_adapter;
use crate::leptos::components::theorem_proof::TheoremView;
use leptos::prelude::*;

/// Component to view a theorem that exists
#[component]
fn TheoremFound(name: String, path: String) -> impl IntoView {
    // Create a theorem using our adapter
    let theorem = theorem_adapter::build_simple_theorem(
        &name,
        "This theorem was found in the repository. Details would be loaded from the backend in a real implementation.",
    );

    view! {
        <div class="theorem-container">
            <TheoremView theorem={theorem} />
        </div>
    }
}

/// Component to view when a theorem is not found
#[component]
fn TheoremNotFound() -> impl IntoView {
    view! {
        <div class="theorem-container">
            <h2>"Theorem not found"</h2>
            <div class="theorem-content">
                <p>"The requested theorem could not be found."</p>
                <div class="theorem-statement">
                    <p>"No details available."</p>
                </div>
                <div class="theorem-proof">
                    <h3>"Proof"</h3>
                    <p>"No proof available."</p>
                </div>
            </div>
        </div>
    }
}

/// Component to view a specific theorem
#[component]
pub fn TheoremViewer<F>(id: F) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    let repo_data = use_context::<RepositoryData>().expect("Repository data should be provided");

    // Create a signal to store the found theorem info
    let (theorem_info, set_theorem_info) = create_signal(None::<(String, String)>);

    // Effect to find the theorem based on the id
    create_effect(move |_| {
        let data = repo_data.0.get();
        let path = id();
        let mut found_theorem = None;

        // Function to recursively find a theorem by path
        fn find_theorem(item: &MathItem, target_path: &str) -> Option<(String, String)> {
            match item {
                MathItem::Theorem { name, path } if path == target_path => {
                    Some((name.clone(), path.clone()))
                }
                MathItem::Theory { children, .. } => {
                    for child in children {
                        if let Some(result) = find_theorem(child, target_path) {
                            return Some(result);
                        }
                    }
                    None
                }
                _ => None,
            }
        }

        // Search through all items
        for item in data.values() {
            if let Some(thm) = find_theorem(item, &path) {
                found_theorem = Some(thm);
                break;
            }
        }

        set_theorem_info.set(found_theorem);
    });

    view! {
        <div class="theorem-viewer">
            <Show
                when=move || theorem_info.get().is_some()
                fallback=|| view! { <TheoremNotFound /> }
            >
                {move || {
                    let (name, path) = theorem_info.get().unwrap();
                    view! { <TheoremFound name={name} path={path} /> }
                }}
            </Show>
        </div>
    }
}
