use crate::leptos::components::theory::TheoryTree;
use leptos::prelude::*;

#[component]
pub fn TheoryPage() -> impl IntoView {
    view! {
        <div class="theory-page">
            <h1>"Mathematical Theories"</h1>
            <p>"Browse the complete collection of formal mathematical theories organized by subject area."</p>
            <div class="theory-browser">
                <TheoryTree />
            </div>
        </div>
    }
}
