use crate::theorem_proof::{Theorem, create_example_theorem};
use leptos::prelude::*;
use std::collections::HashMap;

/// Result of theorem fetching operations
#[derive(Clone)]
pub enum TheoremResult {
    /// Theorem fetch successful
    Success(Theorem),
    /// Theorem not found
    NotFound,
    /// Error occurred during fetch
    Error(String),
}

/// Simple in-memory adapter for theorem data
#[derive(Clone)]
pub struct TheoremAdapter {
    /// Map of theorem IDs to theorems
    theorems: HashMap<String, Theorem>,
}

impl TheoremAdapter {
    /// Create a new theorem adapter with example data
    pub fn new() -> Self {
        let mut theorems = HashMap::new();

        // Add a sample theorem
        let example = create_example_theorem();
        theorems.insert(example.id.clone(), example);

        // Add more sample theorems as needed

        Self { theorems }
    }

    /// Get a theorem by ID
    pub fn get_theorem(&self, id: &str) -> TheoremResult {
        match self.theorems.get(id) {
            Some(theorem) => TheoremResult::Success(theorem.clone()),
            None => TheoremResult::NotFound,
        }
    }

    /// List all available theorems
    pub fn list_theorems(&self) -> Vec<(String, String)> {
        self.theorems
            .iter()
            .map(|(id, theorem)| (id.clone(), theorem.name.clone()))
            .collect()
    }
}

/// Resource for fetching theorem data
#[component]
pub fn TheoremResource<F, T>(
    #[prop(into)] id: MaybeSignal<String>,
    #[prop(into)] children: F,
) -> impl IntoView
where
    F: Fn(Resource<String, TheoremResult>) -> T + 'static,
    T: IntoView,
{
    let theorem_adapter = TheoremAdapter::new();

    // Create a resource that fetches the theorem when the ID changes
    let theorem_resource = create_resource(
        move || id.get(),
        move |theorem_id| {
            let adapter = theorem_adapter.clone();
            async move {
                // In a real app, this would be an async API call
                // For simplicity, we use a synchronous call to our adapter
                adapter.get_theorem(&theorem_id)
            }
        },
    );

    // Call the children function with the resource
    children(theorem_resource)
}

/// Example usage component that displays a theorem
#[component]
pub fn TheoremDisplay(#[prop(into)] id: MaybeSignal<String>) -> impl IntoView {
    let (loading_error, set_loading_error) = create_signal(None::<String>);

    view! {
        <TheoremResource id={id}>
            {move |theorem_resource| {
                view! {
                    <div class="theorem-display">
                        <Suspense fallback=move || {
                            view! { <div class="loading">"Loading theorem..."</div> }
                        }>
                            {move || {
                                theorem_resource.get().map(|result| {
                                    match result {
                                        TheoremResult::Success(theorem) => {
                                            set_loading_error.set(None);
                                            let name = theorem.name.clone();
                                            view! {
                                                <div class="theorem-success">
                                                    <h2>{name}</h2>
                                                    <p>"Theorem loaded successfully!"</p>
                                                    // In a real app, you would render the complete theorem here
                                                </div>
                                            }
                                        },
                                        TheoremResult::NotFound => {
                                            set_loading_error.set(Some("Theorem not found".to_string()));
                                            view! {
                                                <div class="theorem-not-found">
                                                    <h2>"Theorem Not Found"</h2>
                                                    <p>"The requested theorem could not be found."</p>
                                                </div>
                                            }
                                        },
                                        TheoremResult::Error(error) => {
                                            set_loading_error.set(Some(error.clone()));
                                            view! {
                                                <div class="theorem-error">
                                                    <h2>"Error Loading Theorem"</h2>
                                                    <p>{format!("An error occurred: {}", error)}</p>
                                                </div>
                                            }
                                        }
                                    }
                                })
                            }}
                        </Suspense>
                        {move || {
                            loading_error.get().map(|error| {
                                view! {
                                    <div class="error-banner">{error}</div>
                                }
                            })
                        }}
                    </div>
                }
            }}
        </TheoremResource>
    }
}
