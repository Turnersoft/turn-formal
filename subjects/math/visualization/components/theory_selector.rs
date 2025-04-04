// Module: src/formalize_v2/subjects/math/visualization/components/theory_selector.rs
// Component for selecting a mathematical theory

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::common::*;
#[cfg(feature = "theorem_visualizer")]
use std::collections::HashMap;

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::models::TheoryVisualization;

#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn TheorySelector(
    #[prop(into)] theories: Signal<HashMap<String, TheoryVisualization>>,
    #[prop(into)] on_select: Callback<TheoryVisualization>,
) -> impl IntoView {
    // Function to handle when a theory is selected
    let handle_theory_select = move |theory: TheoryVisualization| {
        on_select.run(theory);
    };

    let theory_items = move || {
        theories.get()
            .iter()
            .map(|(_, theory)| {
                let theory_clone = theory.clone();
                let name = theory.name.clone();
                let count = theory.theorems.len();
                
                view! {
                    <li class="theory-item" on:click=move |_| handle_theory_select(theory_clone.clone())>
                        <span class="theory-name">{name}</span>
                        <span class="theory-count">
                            {format!("({} theorems)", count)}
                        </span>
                    </li>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="theory-selector">
            <h2>"Mathematical Theories"</h2>
            <ul class="theory-list">
                {theory_items}
            </ul>
        </div>
    }
}
