// Module: src/formalize_v2/subjects/math/visualization/components/theorem_list.rs
// Component for displaying a list of theorems

#[cfg(feature = "theorem_visualizer")]
mod theorem_list_impl {
    use crate::formalize_v2::subjects::math::visualization::components::common::*;
    use std::collections::HashMap;

    use crate::formalize_v2::subjects::math::theorem::core::Theorem;
    use crate::formalize_v2::subjects::math::visualization::models::TheoryVisualization;

    #[component]
    fn NoTheorySelectedView() -> impl IntoView {
        view! {
            <div class="no-theory-selected">
                <p>"Please select a theory to view its theorems."</p>
            </div>
        }
    }

    #[component]
    fn TheoremItemView(
        #[prop(into)] theorem: Theorem,
        #[prop(into)] on_select: Callback<Theorem>,
    ) -> impl IntoView {
        view! {
            <li class="theorem-item" on:click=move |_| on_select.run(theorem.clone())>
                <span class="theorem-name">{theorem.name.clone()}</span>
            </li>
        }
    }

    #[component]
    fn TheoryContentView(
        #[prop(into)] theory: TheoryVisualization,
        #[prop(into)] on_select: Callback<Theorem>,
    ) -> impl IntoView {
        view! {
            <div class="theory-content-inner">
                <div class="theory-info">
                    <h3>{theory.name.clone()}</h3>
                    <p>{theory.description.clone()}</p>
                </div>
                <ul class="theorem-items">
                    {theory.theorems.iter().map(|theorem| {
                        view! {
                            <TheoremItemView
                                theorem={theorem.clone()}
                                on_select={on_select}
                            />
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </div>
        }
    }

    #[component]
    pub fn TheoremList(
        #[prop(into)] theory: Signal<Option<TheoryVisualization>>,
        #[prop(into)] on_select: Callback<Theorem>,
    ) -> impl IntoView {
        view! {
            <div class="theorem-list">
                <h2>"Theorems"</h2>
                <div class="theory-content">
                    {move || match theory.get() {
                        Some(t) => view! { <TheoryContentView theory={t} on_select={on_select} /> },
                        None => view! { <NoTheorySelectedView /> }
                    }}
                </div>
            </div>
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
pub use theorem_list_impl::TheoremList;
