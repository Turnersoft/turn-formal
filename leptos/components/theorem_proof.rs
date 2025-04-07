use leptos::prelude::*;
use std::collections::HashMap;

/// Represents a mathematical proof step
#[derive(Clone, Debug)]
pub struct ProofStep {
    /// Step number/identifier
    pub id: String,
    /// Description of the step
    pub description: String,
    /// Justification or explanation
    pub justification: Option<String>,
    /// Optional references to previous steps
    pub references: Vec<String>,
}

/// Represents a mathematical proof as a sequence of steps
#[derive(Clone, Debug)]
pub struct Proof {
    /// Title of the proof
    pub title: String,
    /// Steps in the proof
    pub steps: Vec<ProofStep>,
    /// Indicates if the proof is complete
    pub is_complete: bool,
}

/// Represents a mathematical statement
#[derive(Clone, Debug)]
pub struct TheoremStatement {
    /// Informal statement in natural language
    pub informal: String,
    /// Formal statement (if available)
    pub formal: Option<String>,
}

/// Represents a mathematical theorem with statement and proof
#[derive(Clone, Debug)]
pub struct Theorem {
    /// Unique identifier
    pub id: String,
    /// Name of the theorem
    pub name: String,
    /// Statement of the theorem
    pub statement: TheoremStatement,
    /// Proof of the theorem
    pub proof: Proof,
    /// Related theorems (IDs)
    pub related: Vec<String>,
}

/// Component to display a single proof step
#[component]
fn ProofStepView(
    #[prop(into)] step: ProofStep,
    #[prop(into)] step_index: usize,
    #[prop(optional)] highlighted: bool,
) -> impl IntoView {
    let highlighted_class = if highlighted { "highlighted-step" } else { "" };
    let step_clone = step.clone();

    view! {
        <div class={format!("proof-step {}", highlighted_class)}>
            <div class="step-number">{step_index + 1}</div>
            <div class="step-content">
                <p class="step-description">{step_clone.description}</p>
                {step.justification.as_ref().map(|j| view! {
                    <p class="step-justification">{"Justification: "}{j.clone()}</p>
                })}
                {(!step.references.is_empty()).then(|| {
                    let refs = step.references.iter()
                        .map(|r| r.clone())
                        .collect::<Vec<_>>()
                        .join(", ");
                    view! {
                        <p class="step-references">
                            "References: "
                            {refs}
                        </p>
                    }
                })}
            </div>
        </div>
    }
}

/// Component to display a proof
#[component]
pub fn TheoremProof(
    #[prop(into)] proof: Proof,
    #[prop(optional)] initially_expanded: bool,
) -> impl IntoView {
    // State management
    let (is_expanded, set_is_expanded) = create_signal(initially_expanded);
    let (current_step, set_current_step) = create_signal(0);
    let step_count = proof.steps.len();
    let proof = store_value(proof);

    // Event handlers
    let toggle_expanded = move |_| {
        set_is_expanded.update(|val| {
            *val = !*val;
        });
    };

    let go_to_next_step = move |_| {
        if current_step.get() < step_count - 1 {
            set_current_step.update(|s| {
                *s += 1;
            });
        }
    };

    let go_to_prev_step = move |_| {
        if current_step.get() > 0 {
            set_current_step.update(|s| {
                *s -= 1;
            });
        }
    };

    let render_steps = move || {
        let proof_value = proof.get_value();
        let steps = proof_value.steps.clone();

        let current = current_step.get();

        // Create a vector of pairs (step, index) to iterate over
        let indexed_steps: Vec<_> = steps
            .iter()
            .enumerate()
            .map(|(i, step)| (step.clone(), i))
            .collect();

        view! {
            <div class="proof-steps">
                <For
                    each=move || indexed_steps.clone()
                    key=|(step, _)| step.id.clone()
                    let:item
                >
                    {
                        let (step, index) = item;
                        view! {
                            <ProofStepView
                                step={step}
                                step_index={index}
                                highlighted={current == index}
                            />
                        }
                    }
                </For>
            </div>
        }
    };

    let proof_title = proof.get_value().title.clone();
    let is_complete = proof.get_value().is_complete;

    view! {
        <div class="theorem-proof">
            <div class="proof-header">
                <h3>{proof_title}</h3>
                <div class="proof-controls">
                    <button
                        class="toggle-proof-btn"
                        on:click={toggle_expanded}
                    >
                        {move || if is_expanded.get() { "Collapse Proof" } else { "Expand Proof" }}
                    </button>
                </div>
            </div>

            <Show
                when=move || is_expanded.get()
                fallback=|| view! { <p class="proof-collapsed">"Proof is collapsed. Click to expand."</p> }
            >
                <div class="proof-steps-container">
                    <div class="proof-navigation">
                        <button
                            class="nav-btn"
                            on:click={go_to_prev_step}
                            disabled=move || current_step.get() == 0
                        >
                            "Previous"
                        </button>
                        <span class="step-indicator">
                            {"Step "}{move || current_step.get() + 1}{" of "}{step_count}
                        </span>
                        <button
                            class="nav-btn"
                            on:click={go_to_next_step}
                            disabled=move || current_step.get() >= step_count - 1
                        >
                            "Next"
                        </button>
                    </div>

                    {render_steps}

                    {is_complete.then(|| view! {
                        <p class="proof-qed">"Q.E.D."</p>
                    })}
                </div>
            </Show>
        </div>
    }
}

/// Component to render a theorem with its statement and proof
#[component]
pub fn TheoremView(#[prop(into)] theorem: Theorem) -> impl IntoView {
    let theorem = store_value(theorem);
    let name = theorem.get_value().name.clone();
    let statement = theorem.get_value().statement.clone();
    let proof = theorem.get_value().proof.clone();
    let related = theorem.get_value().related.clone();

    let related_items = related
        .iter()
        .map(|id| {
            let id_for_display = id.clone();
            let id_for_link = id.clone();
            view! {
                <li>
                    <a href={format!("/theorem/{}", id_for_link)}>
                        {id_for_display}
                    </a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="theorem-view">
            <h2>{name}</h2>

            <div class="theorem-section">
                <h3>"Statement"</h3>
                <div class="theorem-statement">
                    <p>{statement.informal}</p>
                    {statement.formal.map(|f| view! {
                        <div class="formal-statement">
                            <h4>"Formal Statement:"</h4>
                            <p>{f}</p>
                        </div>
                    })}
                </div>
            </div>

            <div class="theorem-section">
                <TheoremProof
                    proof={proof}
                    initially_expanded={false}
                />
            </div>

            {(!related.is_empty()).then(|| view! {
                <div class="theorem-section">
                    <h3>"Related Theorems"</h3>
                    <ul class="related-theorems">
                        {related_items}
                    </ul>
                </div>
            })}
        </div>
    }
}

// Helper function to create an example theorem for testing
pub fn create_example_theorem() -> Theorem {
    Theorem {
        id: "lagrange_theorem".to_string(),
        name: "Lagrange's Theorem".to_string(),
        statement: TheoremStatement {
            informal: "For all finite groups G, if H is a subgroup of G, then the order of H divides the order of G.".to_string(),
            formal: Some("∀ G: Group, H: Subgroup(G), |H| divides |G|".to_string()),
        },
        proof: Proof {
            title: "Proof of Lagrange's Theorem".to_string(),
            steps: vec![
                ProofStep {
                    id: "step1".to_string(),
                    description: "Let G be a group and H a subgroup of G.".to_string(),
                    justification: None,
                    references: vec![],
                },
                ProofStep {
                    id: "step2".to_string(),
                    description: "Consider the set of left cosets G/H = {gH : g ∈ G}.".to_string(),
                    justification: Some("Definition of left cosets".to_string()),
                    references: vec![],
                },
                ProofStep {
                    id: "step3".to_string(),
                    description: "These cosets partition the group G, meaning each element of G belongs to exactly one coset.".to_string(),
                    justification: Some("Properties of equivalence relations".to_string()),
                    references: vec!["step1".to_string()],
                },
                ProofStep {
                    id: "step4".to_string(),
                    description: "Each coset has the same number of elements as H.".to_string(),
                    justification: Some("Bijection between cosets".to_string()),
                    references: vec!["step2".to_string()],
                },
                ProofStep {
                    id: "step5".to_string(),
                    description: "Therefore, |G| = |G/H| · |H|, which implies that |H| divides |G|.".to_string(),
                    justification: Some("Counting elements in the partition".to_string()),
                    references: vec!["step3".to_string(), "step4".to_string()],
                },
            ],
            is_complete: true,
        },
        related: vec!["cauchys_theorem".to_string(), "sylow_theorems".to_string()],
    }
}
