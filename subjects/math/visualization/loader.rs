// Module: src/formalize_v2/subjects/math/visualization/loader.rs
// Logic to load theorem definitions from the math module

use std::collections::HashMap;

use crate::formalize_v2::subjects::math::theorem::core::{MathContext, Theorem};
use crate::formalize_v2::subjects::math::theories::groups::theorems as group_theorems;
use crate::formalize_v2::subjects::math::visualization::models::{
    MathLibrary, TheoremVisualization, TheoryVisualization,
};

/// Loads all theorems from the math module
pub fn load_math_library() -> MathLibrary {
    let mut theories = HashMap::new();
    let mut theorems_by_theory = HashMap::new();

    // Load group theory theorems
    let group_theory = load_group_theory();
    theorems_by_theory.insert(
        "group_theory".to_string(),
        group_theory
            .theorems
            .iter()
            .map(|t| t.name.clone())
            .collect(),
    );
    theories.insert("group_theory".to_string(), group_theory);

    // Load set theory theorems
    let set_theory = load_set_theory();
    theorems_by_theory.insert(
        "set_theory".to_string(),
        set_theory.theorems.iter().map(|t| t.name.clone()).collect(),
    );
    theories.insert("set_theory".to_string(), set_theory);

    // Return the complete library
    MathLibrary {
        theories,
        theorems_by_theory,
    }
}

/// Loads all group theory theorems
pub fn load_group_theory() -> TheoryVisualization {
    // Collect all theorems defined in the group theory module
    let mut theorems = Vec::new();

    // Load the inverse uniqueness theorem
    theorems.push(group_theorems::prove_inverse_uniqueness());

    // Load the identity uniqueness theorem
    theorems.push(group_theorems::prove_identity_uniqueness_with_syntax_trees());

    // Load the inverse product rule theorem
    theorems.push(group_theorems::prove_inverse_product_rule());

    // Load the abelian squared criterion theorem
    theorems.push(group_theorems::prove_abelian_squared_criterion());

    // Load the Lagrange's theorem
    theorems.push(group_theorems::prove_lagrange_theorem());

    // Create and return the theory visualization
    TheoryVisualization {
        name: "Group Theory".to_string(),
        context: MathContext::GroupTheory,
        description: "Theorems from abstract group theory.".to_string(),
        theorems,
    }
}

/// Loads all set theory theorems
pub fn load_set_theory() -> TheoryVisualization {
    // Collect all theorems defined in the set theory module
    let theorems = Vec::new(); // No set theory theorems implemented yet

    // Create and return the theory visualization
    TheoryVisualization {
        name: "Set Theory".to_string(),
        context: MathContext::SetTheory,
        description: "Theorems from Zermelo-Fraenkel set theory with choice (ZFC).".to_string(),
        theorems,
    }
}

/// Gets the visualization for a specific theorem
pub fn get_theorem_visualization(theorem: &Theorem) -> TheoremVisualization {
    // Create a basic visualization for the theorem
    TheoremVisualization::new(theorem.clone())
}

/// Extracts all proof branches from a theorem
pub fn extract_proof_branches(theorem: &Theorem) -> Vec<String> {
    // This is a simplified version - in a real implementation, we would extract
    // the actual proof branches from the theorem's proof structure

    // For now, just return a simple structure based on the theorem name
    vec!["main".to_string()]
}
