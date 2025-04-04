// Module: src/formalize_v2/subjects/math/visualization/components/theorem_detail.rs
// Component for displaying detailed information about a theorem

#[cfg(feature = "theorem_visualizer")]
mod theorem_detail_impl {
    use crate::formalize_v2::subjects::math::theorem::core::Theorem;
    use crate::formalize_v2::subjects::math::visualization::components::common::*;
    use crate::formalize_v2::subjects::math::visualization::components::proof_step::ProofStep;
    use crate::formalize_v2::subjects::math::visualization::loader;
    use crate::formalize_v2::subjects::math::visualization::models::TheoremVisualization;

    #[component]
    fn ContentWrapper(#[prop(into)] theorem: Option<Theorem>) -> impl IntoView {
        match theorem {
            Some(thm) => {
                let viz = loader::get_theorem_visualization(&thm);
                view! { <TheoremContentView viz={viz} /> }
            }
            None => view! { <NoTheoremSelectedView /> },
        }
    }

    /// Renders a placeholder when no theorem is selected
    #[component]
    fn NoTheoremSelectedView() -> impl IntoView {
        view! {
            <div class="no-theorem-selected">
                <p>"Please select a theorem to view its details."</p>
            </div>
        }
    }

    /// Renders the branches of a proof
    #[component]
    fn ProofBranchListView(#[prop(into)] branches: Vec<String>) -> impl IntoView {
        view! {
            <ul class="branch-list">
                {branches.iter().map(|branch| {
                    view! {
                        <li class="branch-item">{branch.clone()}</li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        }
    }

    /// Component that displays the visualization of a proof
    #[component]
    fn ProofVisualizationView(#[prop(into)] viz: TheoremVisualization) -> impl IntoView {
        // Extract the proof state from the theorem for initial state display
        let initial_state = viz.theorem.initial_proof_state.clone();

        // Extract proof branches
        let branches = loader::extract_proof_branches(&viz.theorem);

        view! {
            <div class="proof-visualization">
                <div class="proof-branches">
                    <h4>"Proof Structure"</h4>
                    <ProofBranchListView branches={branches} />
                </div>
                <div class="proof-steps">
                    <h4>"Proof Steps"</h4>
                    <div class="initial-state">
                        <h5>"Initial State"</h5>
                        <ProofStep state={initial_state} />
                    </div>
                    <div class="remaining-steps">
                        <p>"The complete proof steps would be shown here in a real implementation."</p>
                    </div>
                </div>
            </div>
        }
    }

    /// Component that displays the full theorem content when selected
    #[component]
    fn TheoremContentView(#[prop(into)] viz: TheoremVisualization) -> impl IntoView {
        view! {
            <div class="theorem-content-wrapper">
                <div class="theorem-header">
                    <h2>{viz.theorem.name.clone()}</h2>
                    <p class="theorem-description">{viz.theorem.description.clone()}</p>
                </div>
                <div class="theorem-statement">
                    <h3>"Statement"</h3>
                    <div class="statement-content">
                        <pre>{format!("{:?}", viz.theorem.initial_proof_state.statement)}</pre>
                    </div>
                </div>
                <div class="theorem-proof">
                    <h3>"Proof"</h3>
                    <div class="proof-structure">
                        <ProofVisualizationView viz={viz} />
                    </div>
                </div>
            </div>
        }
    }

    #[component]
    pub fn TheoremDetail(#[prop(into)] theorem: Signal<Option<Theorem>>) -> impl IntoView {
        view! {
            <div class="theorem-detail">
                <div class="theorem-content">
                    <Suspense>
                        {move || view! { <ContentWrapper theorem={theorem.get()} /> }}
                    </Suspense>
                </div>
            </div>
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
pub use theorem_detail_impl::TheoremDetail;
