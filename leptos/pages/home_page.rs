use crate::leptos::components::visualization::VisualizationPanel;
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <h1>"Formalize Vds2222"</h1>
            <p>"A unified visualization tool for formal mathematics, logic, and foundational theories."</p>
            <VisualizationPanel />
        </div>
    }
}
