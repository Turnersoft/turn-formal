use super::definitions::*;
use leptos::html::ElementChild;
use leptos::prelude::{ClassAttribute, CollectView};
use leptos::*;

/// Leptos components for rendering probability theory concepts

#[component]
pub fn ProbabilitySpaceComponent(#[prop(into)] space: ProbabilitySpace) -> impl IntoView {
    view! {
        <div class="probability-space">
            <h3>"Probability Space"</h3>
            <p>"Type: " {format!("{:?}", std::mem::discriminant(&space))}</p>
        </div>
    }
}

#[component]
pub fn RandomVariableComponent(#[prop(into)] variable: RandomVariable) -> impl IntoView {
    view! {
        <div class="random-variable">
            <h4>"Random Variable"</h4>
            <p>"Type: " {format!("{:?}", variable.variable_type)}</p>
        </div>
    }
}

#[component]
pub fn DistributionComponent(#[prop(into)] distribution: Distribution) -> impl IntoView {
    view! {
        <div class="distribution">
            <h4>"Distribution"</h4>
            <p>"Type: " {format!("{:?}", distribution.distribution_type)}</p>
        </div>
    }
}

#[component]
pub fn TheoremListComponent(#[prop(into)] theorems: Vec<String>) -> impl IntoView {
    view! {
        <div class="probability-theorems">
            <h3>"Probability Theory Theorems"</h3>
            <p>"Number of theorems: " {theorems.len()}</p>
        </div>
    }
}

#[component]
pub fn ProbabilityRelationComponent(#[prop(into)] relation: ProbabilityRelation) -> impl IntoView {
    view! {
        <div class="probability-relation">
            <h4>"Probability Relation"</h4>
            <p>"Relation type: " {format!("{:?}", std::mem::discriminant(&relation))}</p>
        </div>
    }
}

// Helper functions for rendering mathematical expressions
pub fn render_probability_expression(expr: &ProbabilityExpression) -> String {
    match expr {
        ProbabilityExpression::EventProbability { .. } => "P(A)".to_string(),
        ProbabilityExpression::ConditionalProbability { .. } => "P(A|B)".to_string(),
        ProbabilityExpression::ExpectedValue { .. } => "E[X]".to_string(),
        ProbabilityExpression::Variance { .. } => "Var(X)".to_string(),
        ProbabilityExpression::Covariance { .. } => "Cov(X,Y)".to_string(),
        ProbabilityExpression::CharacteristicFunction { .. } => "φ_X(t)".to_string(),
        ProbabilityExpression::MomentGeneratingFunction { .. } => "M_X(t)".to_string(),
        _ => "Mathematical Expression".to_string(),
    }
}

pub fn render_distribution_name(dist_type: &DistributionType) -> String {
    match dist_type {
        DistributionType::Discrete(variant) => match variant {
            DiscreteDistributionVariant::Bernoulli => "Bernoulli".to_string(),
            DiscreteDistributionVariant::Binomial => "Binomial".to_string(),
            DiscreteDistributionVariant::Poisson => "Poisson".to_string(),
            DiscreteDistributionVariant::Geometric => "Geometric".to_string(),
            DiscreteDistributionVariant::UniformDiscrete => "Uniform (Discrete)".to_string(),
            DiscreteDistributionVariant::Hypergeometric => "Hypergeometric".to_string(),
        },
        DistributionType::Continuous(variant) => match variant {
            ContinuousDistributionVariant::Normal => "Normal".to_string(),
            ContinuousDistributionVariant::UniformContinuous => "Uniform (Continuous)".to_string(),
            ContinuousDistributionVariant::Exponential => "Exponential".to_string(),
            ContinuousDistributionVariant::Gamma => "Gamma".to_string(),
            ContinuousDistributionVariant::Beta => "Beta".to_string(),
            ContinuousDistributionVariant::ChiSquared => "Chi-squared".to_string(),
            ContinuousDistributionVariant::StudentT => "Student's t".to_string(),
        },
        DistributionType::Mixed => "Mixed".to_string(),
    }
}

// CSS classes for styling (would be defined elsewhere)
pub fn probability_theory_styles() -> &'static str {
    r#"
    .probability-space {
        border: 1px solid #ccc;
        padding: 10px;
        margin: 10px 0;
        border-radius: 5px;
    }

    .random-variable {
        background-color: #f9f9f9;
        padding: 8px;
        margin: 5px 0;
        border-radius: 3px;
    }

    .distribution {
        background-color: #e9f5ff;
        padding: 8px;
        margin: 5px 0;
        border-radius: 3px;
    }

    .probability-theorems ul {
        list-style-type: disc;
        padding-left: 20px;
    }

    .probability-relation {
        border-left: 3px solid #007acc;
        padding-left: 10px;
        margin: 10px 0;
    }
    "#
}
