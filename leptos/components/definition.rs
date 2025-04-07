use crate::leptos::components::repository_reader::{MathItem, RepositoryData};
use leptos::prelude::*;

/// Component to view a definition that exists
#[component]
fn DefinitionFound(name: String, path: String) -> impl IntoView {
    view! {
        <div class="definition-container">
            <h2>{name}</h2>
            <div class="definition-content">
                <p>"Path: " {path}</p>
                <div class="definition-statement">
                    <p>"Definition details would be rendered here..."</p>
                </div>
                <div class="definition-examples">
                    <h3>"Examples"</h3>
                    <p>"Example usage would be rendered here..."</p>
                </div>
            </div>
        </div>
    }
}

/// Component to view when a definition is not found
#[component]
fn DefinitionNotFound() -> impl IntoView {
    view! {
        <div class="definition-container">
            <h2>"Definition not found"</h2>
            <div class="definition-content">
                <p>"The requested definition could not be found."</p>
                <div class="definition-statement">
                    <p>"No details available."</p>
                </div>
                <div class="definition-examples">
                    <h3>"Examples"</h3>
                    <p>"No examples available."</p>
                </div>
            </div>
        </div>
    }
}

/// Component to view a specific definition
#[component]
pub fn DefinitionViewer<F>(id: F) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    let repo_data = use_context::<RepositoryData>().expect("Repository data should be provided");

    // Create a signal to store the found definition info
    let (definition_info, set_definition_info) = create_signal(None::<(String, String)>);

    // Effect to find the definition based on the id
    create_effect(move |_| {
        let data = repo_data.0.get();
        let path = id();
        let mut found_definition = None;

        // Function to recursively find a definition by path
        fn find_definition(item: &MathItem, target_path: &str) -> Option<(String, String)> {
            match item {
                MathItem::Definition { name, path } if path == target_path => {
                    Some((name.clone(), path.clone()))
                }
                MathItem::Theory { children, .. } => {
                    for child in children {
                        if let Some(result) = find_definition(child, target_path) {
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
            if let Some(def) = find_definition(item, &path) {
                found_definition = Some(def);
                break;
            }
        }

        set_definition_info.set(found_definition);
    });

    view! {
        <div class="definition-viewer">
            <Show
                when=move || definition_info.get().is_some()
                fallback=|| {
                    view! {
                        <div class="definition-container">
                            <h2>"Definition not found"</h2>
                            <div class="definition-content">
                                <p>"The requested definition could not be found."</p>
                                <div class="definition-statement">
                                    <p>"No details available."</p>
                                </div>
                                <div class="definition-examples">
                                    <h3>"Examples"</h3>
                                    <p>"No examples available."</p>
                                </div>
                            </div>
                        </div>
                    }
                }
            >
                {move || {
                    let (name, path) = definition_info.get().unwrap();
                    view! {
                        <div class="definition-container">
                            <h2>{name}</h2>
                            <div class="definition-content">
                                <p>"Path: " {path}</p>
                                <div class="definition-statement">
                                    <p>"Definition details would be rendered here..."</p>
                                </div>
                                <div class="definition-examples">
                                    <h3>"Examples"</h3>
                                    <p>"Example usage would be rendered here..."</p>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
