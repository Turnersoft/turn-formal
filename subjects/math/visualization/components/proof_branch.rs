// Module: src/formalize_v2/subjects/math/visualization/components/proof_branch.rs
// Component for displaying a proof branch

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::common::*;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::proof_step::ProofStep;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::models::ProofBranchVisualization;

#[component]
#[cfg(feature = "theorem_visualizer")]
pub fn ProofBranch(#[prop(into)] branch: ProofBranchVisualization) -> impl IntoView {
    view! {
        <div class="proof-branch">
            <div class="branch-header">
                <h4>{format!("Branch: {}", branch.name)}</h4>
            </div>
            <div class="branch-steps">
                {branch.steps.iter().map(|step| {
                    view! {
                        <div class="branch-step">
                            <ProofStep state={step.state.clone()} />
                            <div class="step-tactic">
                                <span class="tactic-label">"Tactic: "</span>
                                <span class="tactic-value">{step.tactic.clone()}</span>
                            </div>
                            <div class="step-status">
                                {if step.is_complete {
                                    view! {
                                        <div class="step-complete">
                                            <span class="complete-tag">"COMPLETE"</span>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="step-incomplete">
                                            <span class="incomplete-tag">"INCOMPLETE"</span>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
