use crate::leptos::components::theory::TheoryViewer;
use leptos::prelude::*;
use leptos_router::{hooks::use_params_map, *};

#[component]
pub fn TheoryDetailPage() -> impl IntoView {
    let params = use_params_map();
    let theory_id = create_memo(move |_| {
        params.with(|p| p.get("id").map(|s| s.to_string()).unwrap_or_default())
    });

    view! {
        <div class="theory-detail-page">
            <h1>"Theory: " {theory_id}</h1>
            <TheoryViewer id=move || theory_id.get() />
        </div>
    }
}
