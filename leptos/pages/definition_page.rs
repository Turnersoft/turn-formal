use crate::leptos::components::definition::DefinitionViewer;
use leptos::prelude::*;
use leptos_router::{hooks::use_params_map, *};

#[component]
pub fn DefinitionPage() -> impl IntoView {
    let params = use_params_map();
    let definition_id = create_memo(move |_| {
        params.with(|p| p.get("id").map(|s| s.to_string()).unwrap_or_default())
    });

    view! {
        <div class="definition-page">
            <h1>"Definition: " {definition_id}</h1>
            <DefinitionViewer id=move || definition_id.get() />
        </div>
    }
}
