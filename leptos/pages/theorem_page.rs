use crate::leptos::components::theorem::TheoremViewer;
use leptos::prelude::*;
use leptos_router::{hooks::use_params_map, *};

#[component]
pub fn TheoremPage() -> impl IntoView {
    let params = use_params_map();
    let theorem_id = create_memo(move |_| {
        params.with(|p| p.get("id").map(|s| s.to_string()).unwrap_or_default())
    });

    view! {
        <div class="theorem-page">
            <h1>"Theorem: " {theorem_id}</h1>
            <TheoremViewer id=move || theorem_id.get() />
        </div>
    }
}
