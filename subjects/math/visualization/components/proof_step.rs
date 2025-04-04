// Module: src/formalize_v2/subjects/math/visualization/components/proof_step.rs
// Component for displaying a single proof step

#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::theorem::core::ProofState;
#[cfg(feature = "theorem_visualizer")]
use crate::formalize_v2::subjects::math::visualization::components::common::*;

#[cfg(feature = "theorem_visualizer")]
mod proof_step_impl {
    use crate::formalize_v2::subjects::math::theorem::core::ProofState;
    use crate::formalize_v2::subjects::math::visualization::components::common::*;

    #[component]
    fn PathHeaderEl(#[prop(into, optional)] path: Option<String>) -> impl IntoView {
        let path_text = path.unwrap_or_default();
        if path.is_some() {
            view! { <span class="step-id">{path_text}</span> }
        } else {
            view! { <span class="step-id-empty"></span> }
        }
    }

    #[component]
    fn JustificationHeaderEl(
        #[prop(into, optional)] justification: Option<String>,
    ) -> impl IntoView {
        let justification_text = justification.unwrap_or_default();
        if justification.is_some() {
            view! { <span class="step-justification">{justification_text}</span> }
        } else {
            view! { <span class="step-justification-empty"></span> }
        }
    }

    #[component]
    fn VariablesView(#[prop(into)] state: ProofState) -> impl IntoView {
        if state.value_variables.is_empty() {
            view! {
                <div class="variables-section no-variables">
                    <h5>"No Variables"</h5>
                    <p class="empty-message">"No variables defined."</p>
                </div>
            }
        } else {
            view! {
                <div class="variables-section">
                    <h5>"Variables"</h5>
                    <ul class="variable-list">
                        {state.value_variables.iter().map(|var| {
                            view! {
                                <li>
                                    <span class="var-name">{var.variable.clone()}</span>
                                    <span class="var-value">{format!("{:?}", var.value)}</span>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                </div>
            }
        }
    }

    /// Main component for displaying a proof step
    #[component]
    pub fn ProofStep(#[prop(into)] state: ProofState) -> impl IntoView {
        // Extract data from the proof state
        let path = state.path.clone();
        let justification = state.justification.clone();
        let statement = format!("{:?}", state.statement);

        view! {
            <div class="proof-step">
                <div class="step-header">
                    <PathHeaderEl path={path} />
                    <JustificationHeaderEl justification={justification} />
                </div>

                <div class="step-content">
                    <div class="statement">
                        <pre>{statement}</pre>
                    </div>

                    <VariablesView state={state} />
                </div>
            </div>
        }
    }
}

#[cfg(feature = "theorem_visualizer")]
pub use proof_step_impl::ProofStep;
