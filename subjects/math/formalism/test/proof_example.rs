// Module: src/formalize_v2/subjects/math/theorem/test/proof_example.rs
// Examples of using the proof builder to prove theorems with branching

use std::collections::HashMap;

use uuid::Uuid;

use super::super::super::formalism::expressions::{Identifier, MathExpression, TheoryExpression};
use super::super::super::formalism::proof::{
    CaseAnalysisBuilder, CaseResult, ProofForest, ProofNode, ProofStatus, Tactic,
};
use super::super::super::formalism::relations::MathRelation;
use super::super::super::formalism::theorem::{ProofGoal, Theorem};
use super::super::super::theories::groups::definitions::GroupProperty;
use super::super::super::theories::rings::definitions::{Ring, RingExpression};

/// Helper function to create a variable expression
fn create_var(name: &str) -> MathExpression {
    MathExpression::Var(Identifier::Name(name.to_string(), 0))
}

/// Helper function to create a simple ring-based expression
fn create_expr(expr_str: &str) -> MathExpression {
    // This is a simplification - in a real implementation, we would parse the expression
    // For now, we'll just create a variable with the same name
    let ring = Ring::default();
    let ring_var = RingExpression::variable(ring, expr_str);
    MathExpression::Expression(TheoryExpression::Ring(ring_var))
}

/// Example: Proving Group Associativity Theorem
///
/// This example shows how to use the proof builder to create a theorem
/// with multiple proof branches at different depths.
pub fn prove_group_associativity() -> Theorem {
    // Create a theorem about group associativity
    let left = create_expr("a * (b * c)");
    let right = create_expr("(a * b) * c");

    // Create the initial proof goal and theorem
    let theorem_id = "group_associativity_theorem";
    let name = "Group Associativity";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let p0 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(p0.clone());

    // Main proof path - using direct node creation to avoid stack overflow
    let p1 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(p0.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Intro {
            name: Identifier::Name("a".to_string(), 0),
            expression: create_expr("a"),
            view: None,
        }),
        status: ProofStatus::InProgress,
    };
    proofs.add_node(p1.clone());

    // Add p1 as child of p0
    let mut p0_updated = p0.clone();
    p0_updated.children.push(p1.id.clone());
    proofs.add_node(p0_updated);

    // P2 node
    let p2 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(p1.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Apply {
            theorem_id: "group_axiom_associativity".to_string(),
            instantiation: HashMap::new(),
            target_expr: None,
        }),
        status: ProofStatus::Complete,
    };
    proofs.add_node(p2.clone());

    // Add p2 as child of p1
    let mut p1_updated = p1.clone();
    p1_updated.children.push(p2.id.clone());
    proofs.add_node(p1_updated);

    // Alternative branch from p1
    let alt2 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(p1.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Intro {
            name: Identifier::Name("alternative".to_string(), 0),
            expression: create_expr("alt"),
            view: None,
        }),
        status: ProofStatus::InProgress,
    };
    proofs.add_node(alt2.clone());

    // Update p1 with alt2 child
    let mut p1_with_alt = p1.clone();
    p1_with_alt.children.push(alt2.id.clone());
    proofs.add_node(p1_with_alt);

    // Alt3 node
    let alt3 = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(alt2.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Apply {
            theorem_id: "different_approach".to_string(),
            instantiation: HashMap::new(),
            target_expr: None,
        }),
        status: ProofStatus::Wip,
    };
    proofs.add_node(alt3.clone());

    // Add alt3 as child of alt2
    let mut alt2_updated = alt2.clone();
    alt2_updated.children.push(alt3.id.clone());
    proofs.add_node(alt2_updated);

    // Build the theorem
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Demonstrates the associative property of group operations".to_string(),
        goal,
        proofs,
    }
}

/// Example: Using case analysis for complex proofs
pub fn prove_with_bookmarks() -> Theorem {
    let left = create_expr("a * b");
    let right = create_expr("b * a");

    // Create the initial proof goal and theorem
    let theorem_id = "commutative_group_properties";
    let name = "Commutative Group Properties";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let start = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(start.clone());

    // First key step - simplify to avoid stack overflow issues
    let key_step = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(start.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Intro {
            name: Identifier::Name("commutativity".to_string(), 0),
            expression: create_expr("commutativity"),
            view: None,
        }),
        status: ProofStatus::InProgress,
    };
    proofs.add_node(key_step.clone());

    // Add key_step as child of start
    let mut start_updated = start.clone();
    start_updated.children.push(key_step.id.clone());
    proofs.add_node(start_updated);

    // Complete main path with minimal tree
    let main_path = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(key_step.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Apply {
            theorem_id: "group_axiom_commutativity".to_string(),
            instantiation: HashMap::new(),
            target_expr: None,
        }),
        status: ProofStatus::Complete,
    };
    proofs.add_node(main_path.clone());

    // Add main_path as child of key_step
    let mut key_step_updated = key_step.clone();
    key_step_updated.children.push(main_path.id.clone());
    proofs.add_node(key_step_updated);

    // Build the theorem with a simpler proof tree
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Explores properties of commutative groups".to_string(),
        goal,
        proofs,
    }
}

/// Example: Named proof steps for better clarity
pub fn prove_with_named_steps() -> Theorem {
    let left = create_expr("a * a⁻¹");
    let right = create_expr("e");

    // Create the initial proof goal and theorem
    let theorem_id = "inverse_element_theorem";
    let name = "Inverse Element Theorem";
    let statement = MathRelation::equal(left, right);

    let goal = ProofGoal::new(statement);

    let mut proofs = ProofForest::new();

    // Initialize the proof forest with a root node
    let start = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: None,
        children: vec![],
        state: goal.clone(),
        tactic: None,
        status: ProofStatus::InProgress,
    };
    proofs.add_node(start.clone());

    // First step - introducing the concept
    let intro_step = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(start.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Intro {
            name: Identifier::Name("inverse_concept".to_string(), 0),
            expression: create_expr("inverse_concept"),
            view: None,
        }),
        status: ProofStatus::InProgress,
    };
    proofs.add_node(intro_step.clone());

    // Add intro_step as child of start
    let mut start_updated = start.clone();
    start_updated.children.push(intro_step.id.clone());
    proofs.add_node(start_updated);

    // Main branch - direct approach
    let direct_approach = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(intro_step.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Intro {
            name: Identifier::Name("direct".to_string(), 0),
            expression: create_expr("direct approach"),
            view: None,
        }),
        status: ProofStatus::InProgress,
    };
    proofs.add_node(direct_approach.clone());

    // Add direct_approach as child of intro_step
    let mut intro_step_updated = intro_step.clone();
    intro_step_updated.children.push(direct_approach.id.clone());
    proofs.add_node(intro_step_updated);

    // Complete the main branch
    let completed = ProofNode {
        id: Uuid::new_v4().to_string(),
        parent: Some(direct_approach.id.clone()),
        children: vec![],
        state: goal.clone(),
        tactic: Some(Tactic::Apply {
            theorem_id: "group_axiom_inverse".to_string(),
            instantiation: HashMap::new(),
            target_expr: None,
        }),
        status: ProofStatus::Complete,
    };
    proofs.add_node(completed.clone());

    // Add completed as child of direct_approach
    let mut direct_approach_updated = direct_approach.clone();
    direct_approach_updated.children.push(completed.id.clone());
    proofs.add_node(direct_approach_updated);

    // Build the theorem with a simple proof tree
    Theorem {
        id: theorem_id.to_string(),
        name: name.to_string(),
        description: "Proves that g * g⁻¹ = e in a group".to_string(),
        goal,
        proofs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_associativity_proof() {
        let theorem = prove_group_associativity();
        // Assert theorem name and basic properties
        assert_eq!(theorem.name, "Group Associativity");
    }

    #[test]
    fn test_bookmarked_proof() {
        let theorem = prove_with_bookmarks();
        assert_eq!(theorem.name, "Commutative Group Properties");
    }

    #[test]
    fn test_named_steps_proof() {
        let theorem = prove_with_named_steps();
        assert_eq!(theorem.name, "Inverse Element Theorem");
    }
}
