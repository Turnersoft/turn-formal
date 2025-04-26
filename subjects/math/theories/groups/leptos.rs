use super::super::super::super::math::formalism::core::{ProofGoal, Theorem};

use super::super::super::super::math::formalism::relations::MathRelation;
use leptos::prelude::*;

use super::theorems::{
    prove_abelian_squared_criterion, prove_identity_uniqueness_with_syntax_trees,
    prove_inverse_product_rule, prove_inverse_uniqueness, prove_lagrange_theorem,
};

/// Component that lists all group theory theorems
#[component]
pub fn GroupTheorems() -> impl IntoView {
    view! {
        <div class="group-theorems">
            <h2 class="section-title">"Group Theory Theorems"</h2>
            <div class="theorems-list">
                <TheoremDisplay
                    theorem_name="Group Inverse Uniqueness".to_string()
                    theorem_fn=prove_inverse_uniqueness
                />
                <TheoremDisplay
                    theorem_name="Group Identity Uniqueness".to_string()
                    theorem_fn=prove_identity_uniqueness_with_syntax_trees
                />
                <TheoremDisplay
                    theorem_name="Group Inverse Product Rule".to_string()
                    theorem_fn=prove_inverse_product_rule
                />
                <TheoremDisplay
                    theorem_name="Abelian Group Squared Criterion".to_string()
                    theorem_fn=prove_abelian_squared_criterion
                />
                <TheoremDisplay
                    theorem_name="Lagrange's Theorem".to_string()
                    theorem_fn=prove_lagrange_theorem
                />
            </div>
        </div>
    }
}

/// Component to display a single theorem
#[component]
fn TheoremDisplay<F>(theorem_name: String, theorem_fn: F) -> impl IntoView
where
    F: Fn() -> Theorem + 'static + Clone,
{
    // Get the theorem by calling the function directly
    let theorem_fn = theorem_fn.clone();
    let theorem = theorem_fn();

    // Store theorem in a signal for reactivity
    let theorem = create_rw_signal(theorem);

    // Create a signal to track whether the theorem details are expanded
    let (expanded, set_expanded) = create_signal(false);
    let toggle_expanded = move |_| set_expanded.update(|expanded| *expanded = !*expanded);

    view! {
        <div class="theorem-container">
            <div class="theorem-header"
                 on:click=toggle_expanded>
                <h3 class="theorem-title">{theorem_name}</h3>
                <span class="expander">{move || if expanded.get() { "▼" } else { "▶" }}</span>
            </div>

            <Show
                when=move || expanded.get()
                fallback=|| view! {}
            >
                {move || {
                    let current_theorem = theorem.get();
                    view! { <TheoremDetails theorem=current_theorem /> }
                }}
            </Show>
        </div>
    }
}

/// Component to display theorem details including statement and proof
#[component]
fn TheoremDetails(theorem: Theorem) -> impl IntoView {
    view! {
        <div class="theorem-details">
            <div class="theorem-statement">
                <h4>"Statement"</h4>
                <MathRelationDisplay relation=theorem.goal.statement.clone() />
            </div>

            <div class="theorem-proof">
                <h4>"Proof"</h4>
                <ProofStateDisplay state=theorem.goal />
            </div>
        </div>
    }
}

/// Component to display a mathematical relation
#[component]
fn MathRelationDisplay(relation: MathRelation) -> impl IntoView {
    // Format the relation as a debug string for display since MathRelation doesn't implement Display
    let formatted = match &relation {
        MathRelation::Equal { left, right, .. } => format!("{:?} = {:?}", left, right),
        MathRelation::Implies(a, b) => format!("{:?} ⟹ {:?}", a, b),
        MathRelation::Equivalent(a, b) => format!("{:?} ⟺ {:?}", a, b),
        MathRelation::And(relations) => {
            let formatted_relations: Vec<String> =
                relations.iter().map(|r| format!("{:?}", r)).collect();
            formatted_relations.join(" ∧ ")
        }
        _ => format!("{:?}", relation),
    };

    view! {
        <div class="math-relation">
            {formatted}
        </div>
    }
}

/// Component to display a proof state
#[component]
fn ProofStateDisplay(state: ProofGoal) -> impl IntoView {
    // Get the goal text from the state (no longer using justification)
    let goal_text = format!("Goal: {:?}", state.statement);

    view! {
        <div class="proof-state">
            <MathRelationDisplay relation=state.statement />
        </div>
    }
}

/// CSS styles for group theory components
#[component]
pub fn GroupStyles() -> impl IntoView {
    view! {
        <style>
            ".group-theorems {{
                padding: 1rem;
                max-width: 800px;
                margin: 2rem auto;
                font-family: 'Source Serif Pro', serif;
            }}
            
            .section-title {{ 
                font-size: 1.75rem;
                color: #2c3e50;
                border-bottom: 2px solid #eee;
                padding-bottom: 0.5rem;
                margin-bottom: 1.5rem;
                font-weight: 600;
            }}
            
            .theorems-list {{ 
                display: flex;
                flex-direction: column;
                gap: 1.5rem;
            }}
            
            .theorem-container {{ 
                border: 1px solid #dfdfdf;
                border-radius: 4px;
                overflow: hidden;
                margin-bottom: 1.5rem;
            }}
            
            .theorem-header {{ 
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 0.75rem 1rem;
                background-color: #f8f8f8;
                cursor: pointer;
                transition: background-color 0.2s;
            }}
            
            .theorem-header:hover {{ 
                background-color: #f0f0f0;
            }}
            
            .theorem-title {{ 
                margin: 0;
                font-size: 1.2rem;
                color: #0056b3;
                font-family: 'Source Code Pro', monospace;
                font-weight: 600;
            }}
            
            .expander {{ 
                font-size: 1.2rem;
                color: #7f8c8d;
            }}
            
            .theorem-details {{ 
                padding: 1rem;
                border-top: 1px solid #dfdfdf;
            }}
            
            .theorem-statement, .theorem-proof {{ 
                margin-bottom: 1.5rem;
            }}
            
            .theorem-statement h4, .theorem-proof h4 {{ 
                color: #5c5c5c;
                font-size: 1rem;
                margin-top: 0;
                margin-bottom: 0.8rem;
                font-weight: 600;
            }}
            
            .math-relation {{ 
                font-family: 'Source Serif Pro', serif;
                padding: 0.8rem;
                background-color: #f8f8f8;
                border-radius: 4px;
                margin-bottom: 0.8rem;
                overflow-x: auto;
                color: #333;
                line-height: 1.5;
            }}
            
            .proof-state {{ 
                margin-top: 0.8rem;
                padding: 0.5rem;
                border-left: 3px solid #0056b3;
                background-color: rgba(0, 86, 179, 0.05);
            }}
            
            .interactive-theorems {{
                margin-top: 2rem;
                border-top: 1px solid #dfdfdf;
                padding-top: 1.5rem;
            }}
            
            .interactive-theorems h2 {{
                font-size: 1.5rem;
                color: #2c3e50;
                margin-bottom: 1rem;
            }}
            
            .interactive-theorems p {{
                color: #5c5c5c;
                margin-bottom: 1.5rem;
            }}"
        </style>
    }
}
